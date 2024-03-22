use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
	quit: bool,
}

impl Editor {
	pub fn default() -> Self {
		Self { quit: false }
	}

	pub fn run(&mut self) {
		let _stdout =stdout().into_raw_mode().unwrap();

		loop {
			if let Err(error) = self.process_keypress() {
				close_program(&error);
			}
			if self.quit {
				break;
			}
		}
	}

	fn process_keypress(&mut self) -> Result<(), std::io::Error> {
		let pressed_key = read_key()?;
		match pressed_key {
			Key::Ctrl('q') => self.quit = true,
			_ => (),
		}
		Ok(())
	}
}

fn read_key() -> Result<Key, std::io::Error> {
	loop {
		if let Some(key) = io::stdin().lock().keys().next() {
			return key
		}
	}
}

fn close_program(error: &std::io::Error) {
	panic!("{}", error);
}