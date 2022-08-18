extern crate termion;
use std::io::{stdin, stdout, Write};
use termion::event::Key;

use termion::terminal_size;
use termion::{clear, cursor, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let terminal_size = terminal_size().unwrap();
    print!("{}", clear::All);
    print!("{}ffff", cursor::Goto(1, 1));
    let mut cursor = Cursor::new(1, 2);
    print!("{}", cursor::Goto(cursor.x, cursor.y));
    screen.flush().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => {
                if cursor.x > 1 {
                    cursor.x -= 1;
                }
            }
            Key::Char('j') => {
                if cursor.y < terminal_size.1 {
                    cursor.y += 1;
                }
            }
            Key::Char('k') => {
                if cursor.y > 1 {
                    cursor.y -= 1;
                }
            }
            Key::Char('l') => {
                if cursor.x < terminal_size.0 {
                    cursor.x += 1;
                }
            }
            _ => {}
        }
        print!("{}", cursor::Goto(cursor.x, cursor.y));
        screen.flush().unwrap();
    }
}
