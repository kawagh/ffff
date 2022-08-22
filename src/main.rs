mod scoring;
use atty::Stream;
use clap::Parser;

use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, ExecutableCommand};
use std::fs::{self, ReadDir};

use std::io::{stdin, stdout, BufRead};
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser)]
    file: Option<String>,
}

fn draw_names(
    names: &[String],
    text_input: &String,
    matched_name_index: usize,
) -> crossterm::Result<()> {
    for (i, name) in names.iter().enumerate() {
        stdout().execute(MoveTo(0, 2 + i as u16))?;
        if i == matched_name_index {
            execute!(
                stdout(),
                SetBackgroundColor(Color::DarkGrey),
                Print("> ".to_string()),
            )?;
            draw_name(name, text_input)?;
            execute!(stdout(), ResetColor)?;
        } else {
            execute!(stdout(), Print("- ".to_string()))?;
            draw_name(name, text_input)?;
        }
    }
    Ok(())
}

fn draw_name(name: &String, text_input: &String) -> crossterm::Result<()> {
    for c in name.chars() {
        if text_input.contains(c) {
            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                Print(c),
                SetForegroundColor(Color::White),
            )?;
        } else {
            execute!(stdout(), Print(c),)?;
        }
    }
    Ok(())
}

fn find_most_match_index(scores: &[i32]) -> usize {
    scores
        .iter()
        .enumerate()
        .fold((usize::MIN, i32::MIN), |(ia, a), (ib, &b)| {
            if b > a {
                (ib, b)
            } else {
                (ia, a)
            }
        })
        .0
}
fn get_names_from_pipe() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut stdin_locked = stdin().lock();
    let mut line = String::new();
    while let Ok(n_byte) = BufRead::read_line(&mut stdin_locked, &mut line) {
        if n_byte == 0 {
            eprintln!("debug: 0byte");
            break;
        }
        names.push(line.clone());
        line.clear();
    }
    names[..20].to_vec()
}

fn get_paths_in_current_directory() -> Vec<String> {
    let paths: ReadDir = fs::read_dir("./").expect("could not read directory");
    paths
        .map(|p| p.unwrap().path().display().to_string())
        .collect()
}

fn main() -> crossterm::Result<()> {
    let args = Args::parse();
    let names: Vec<String> = if let Some(file_path) = args.file {
        let content = fs::read_to_string(file_path).expect("could not read file");
        content
            .lines()
            .take(20)
            .map(|line| line.to_string())
            .collect()
    } else if !atty::is(Stream::Stdin) {
        get_names_from_pipe()
    } else {
        get_paths_in_current_directory()
    };
    let mut scores = vec![0; names.len()];
    let mut text_input = String::new();
    let mut selected = false;
    scoring::update_scores(&mut scores, &names, &text_input);
    let mut matched_name_index: usize = find_most_match_index(&scores);

    terminal::enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    //  main loop
    loop {
        let input_line = format!("input: {}", text_input);
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(&input_line),
            MoveTo(0, 1),
            Print(format!("debug: {}", matched_name_index)),
        )?;

        draw_names(&names, &text_input, matched_name_index)?;

        // move cursor to textInput
        stdout().execute(MoveTo(input_line.len() as u16, 0))?;

        // handle events
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => match (event.code, event.modifiers) {
                    (KeyCode::Esc, _) => {
                        break;
                    }
                    (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        if matched_name_index > 0 {
                            matched_name_index -= 1;
                        }
                    }
                    (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        if matched_name_index + 1 < names.len() {
                            matched_name_index += 1;
                        }
                    }

                    (KeyCode::Enter, _) => {
                        selected = true;
                        break;
                    }

                    (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        text_input.pop();
                        scoring::update_scores(&mut scores, &names, &text_input);
                        matched_name_index = find_most_match_index(&scores);
                    }
                    (KeyCode::Char(c), _) => {
                        text_input.push(c);
                        scoring::update_scores(&mut scores, &names, &text_input);
                        matched_name_index = find_most_match_index(&scores);
                    }

                    (_, _) => {}
                },
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    if selected {
        println!("{}", names[matched_name_index]);
    }
    Ok(())
}
