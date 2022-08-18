extern crate termion;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};

use termion::terminal_size;
use termion::{clear, color, cursor, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn draw_names(text_input: &String) {
    let names = vec!["Alice", "Bob", "Carorl", "Dave", "Eve", "Frank"];
    for (i, name) in names.iter().enumerate() {
        if !text_input.is_empty() && name.contains(text_input) {
            print!(
                "{}{}{}{}",
                cursor::Goto(1, 3 + i as u16),
                color::Fg(color::Yellow),
                name,
                color::Fg(color::Reset),
            );
        } else {
            print!("{}{}", cursor::Goto(1, 3 + i as u16), name);
        }
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

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let terminal_size = terminal_size().unwrap();
    print!("{}", clear::All);
    print!("{}ffff (Ctrl-q: Quit)", cursor::Goto(1, 1));
    let mut text_input = String::new();

    let mut cursor = Cursor::new(1, 2);

    draw_text_input(&mut cursor, &text_input);
    draw_names(&text_input);

    screen.flush().unwrap();
    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => break,
            Event::Key(Key::Down) | Event::Key(Key::Ctrl('n')) => {
                if cursor.y < terminal_size.1 {
                    cursor.y += 1;
                }
            }
            Event::Key(Key::Up) | Event::Key(Key::Ctrl('p')) => {
                if cursor.y > 1 {
                    cursor.y -= 1;
                }
            }
            Event::Key(Key::Right) | Event::Key(Key::Ctrl('l')) => {
                if cursor.x < terminal_size.0 {
                    cursor.x += 1;
                }
            }
            Event::Key(Key::Backspace | Key::Ctrl('h')) => {
                text_input.pop();
                draw_text_input(&mut cursor, &text_input);
                draw_names(&text_input);
            }
            Event::Key(Key::Char(c)) => {
                text_input.push(c);
                draw_text_input(&mut cursor, &text_input);
                draw_names(&text_input);
            }
            _ => {}
        }
        print!("{}", cursor::Goto(cursor.x, cursor.y));
        screen.flush().unwrap();
    }
}
