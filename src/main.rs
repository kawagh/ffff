extern crate termion;
use atty::Stream;
use clap::Parser;
use std::fs;

use std::io::{stdin, stdout, BufRead, Write};
use termion::event::{Event, Key};

use termion::terminal_size;
use termion::{clear, color, cursor, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};

struct Cursor {
    x: u16,
}

impl Cursor {
    fn new(x: u16) -> Self {
        Self { x }
    }
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser)]
    file: Option<String>,
}

#[allow(clippy::format_push_string)]
fn draw_names(names: &Vec<String>, text_input: &String, matched_name_index: usize) {
    for (i, name) in names.iter().enumerate() {
        print!("{}", cursor::Goto(1, 3 + i as u16));
        let mut name_line = if i == matched_name_index {
            format!("{}>{} ", color::Fg(color::Cyan), color::Fg(color::Reset))
        } else {
            ("- ").to_string()
        };
        if !text_input.is_empty() {
            for c in name.chars() {
                if text_input.contains(c) {
                    name_line.push_str(&format!(
                        "{}{}{}",
                        color::Fg(color::Yellow),
                        c,
                        color::Fg(color::Reset),
                    ));
                } else {
                    name_line.push(c);
                }
            }
        } else {
            name_line.push_str(name);
        }
        print!("{}", name_line)
    }
}

fn draw_text_input(cursor: &mut Cursor, text_input: &String) {
    let text_input_header = "input: ";
    print!(
        "{}{}{}{}{}{}",
        cursor::Goto(1, 2),
        clear::CurrentLine,
        color::Fg(color::Cyan),
        text_input_header,
        color::Fg(color::Reset),
        text_input,
    );
    cursor.x = 1 + (text_input_header.len() + text_input.len()) as u16;
}

fn update_scores(scores: &mut [i32], names: &Vec<String>, text_input: &String) {
    for (i, name) in names.iter().enumerate() {
        scores[i] = scoring(name, text_input);
    }
}
fn scoring(name: &str, input: &String) -> i32 {
    // scoring logic
    let length_diff = input.len().abs_diff(name.len()) as i32;
    -length_diff
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
fn get_names_from_stdin_or_pipe() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut stdin_locked = stdin().lock();
    let mut line = String::new();
    while let Ok(n_byte) = BufRead::read_line(&mut stdin_locked, &mut line) {
        if n_byte == 0 {
            eprintln!("debug: 0byte");
            break;
        }
        if line == String::from("\n") {
            eprintln!("debug: return");
            break;
        }
        names.push(line.clone());
        line.clear();
    }
    if atty::is(Stream::Stdin) {
        names.push("from stdin\n".to_string());
        names
    } else {
        names.push("from redirect\n".to_string());
        names
    }
}

fn main() {
    let args = Args::parse();
    let names: Vec<String> = if let Some(file_path) = args.file {
        let content = fs::read_to_string(file_path).expect("could not read file");
        content
            .lines()
            .take(20)
            .map(|line| line.to_string())
            .collect()
    } else {
        get_names_from_stdin_or_pipe()
    };
    let mut scores = vec![0; names.len()];

    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let terminal_size = terminal_size().unwrap();
    print!("{}", clear::All);
    print!("{}ffff (Ctrl-Q: Quit)", cursor::Goto(1, 1));
    let mut text_input = String::new();

    let mut cursor = Cursor::new(1);
    let mut selected = false;

    update_scores(&mut scores, &names, &text_input);
    let mut matched_name_index: usize = find_most_match_index(&scores);

    draw_text_input(&mut cursor, &text_input);
    draw_names(&names, &text_input, matched_name_index);
    print!("{}", cursor::Goto(cursor.x, 2));

    screen.flush().unwrap();
    for event in stdin.events() {
        eprintln!("inside event loop");
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => break,
            Event::Key(Key::Char('\n')) => {
                selected = true;
                break;
            }
            Event::Key(Key::Down) | Event::Key(Key::Ctrl('n')) => {
                if matched_name_index + 1 < names.len() {
                    matched_name_index += 1;
                }
            }
            Event::Key(Key::Up) | Event::Key(Key::Ctrl('p')) => {
                if matched_name_index > 0 {
                    matched_name_index -= 1;
                }
            }
            Event::Key(Key::Right) | Event::Key(Key::Ctrl('l')) => {
                if cursor.x < terminal_size.0 {
                    cursor.x += 1;
                }
            }
            Event::Key(Key::Backspace | Key::Ctrl('h')) => {
                text_input.pop();
                update_scores(&mut scores, &names, &text_input);
                matched_name_index = find_most_match_index(&scores);
            }
            Event::Key(Key::Char(c)) => {
                text_input.push(c);
                update_scores(&mut scores, &names, &text_input);
                matched_name_index = find_most_match_index(&scores);
            }
            _ => {}
        }
        draw_text_input(&mut cursor, &text_input);
        draw_names(&names, &text_input, matched_name_index);
        print!("{}", cursor::Goto(cursor.x, 2));
        screen.flush().unwrap();
    }

    drop(screen);
    if selected {
        println!("{}", names[matched_name_index]);
    }
}
