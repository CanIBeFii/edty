use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn close_program(error: std::io::Error) {
	panic!("{}", error);
}

fn main() {
	let _stdout =stdout().into_raw_mode().unwrap();

	for key in io::stdin().keys() {
		match key {
			Ok(key) => match key {
				Key::Char(c) => {
					println!("{} ({:?})\r", c, c as u8);
				}
				Key::Ctrl('q') => break,
				_ => println!("{:?}\r", key),
			}
			Err(error) => close_program(error),
		}
	}
}
