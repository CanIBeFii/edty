use std::num::Saturating;

use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
	pub x: usize,
	pub y: usize,
}

pub struct Editor {
	quit: bool,
	terminal: Terminal,
	cursor_position: Position,
}

impl Editor {
	pub fn default() -> Self {
		Self {
			quit: false,
			terminal: Terminal::new().expect("Failed to initialize terminal :0"),
			cursor_position: Position { x: 0, y: 0 },
		}
	}

	pub fn run(&mut self) {
		loop {
			if let Err(error) = self.refresh_screen() {
				close_program(&error);
			}
			if self.quit {
				break;
			}
			if let Err(error) = self.process_keypress() {
				close_program(&error);
			}
		}
	}

	fn refresh_screen(&self) -> Result<(), std::io::Error> {
		Terminal::hide_cursor();
		Terminal::cursor_position(&Position{ x: 0, y: 0 });
		if self.quit {
			Terminal::clear_screen();
			println!("Bye bye >:3\r");
		} else {
			self.draw_rows();
			Terminal::cursor_position(&self.cursor_position);
		}
		Terminal::show_cursor();
		Terminal::flush()
	}

	fn process_keypress(&mut self) -> Result<(), std::io::Error> {
		let pressed_key = Terminal::read_key()?;
		match pressed_key {
			Key::Ctrl('q') => self.quit = true,
			Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
			_ => (),
		}
		Ok(())
	}

	fn move_cursor(&mut self, key: Key) {
		let Position{ mut y, mut x } = self.cursor_position;
		let size = self.terminal.size();
		let height = size.height.saturating_sub(1) as usize;
		let width = size.width.saturating_sub(1) as usize;

		match key {
			Key::Up => y = y.saturating_sub(1),
			Key::Down => {
				if y < height {
					y = y.saturating_add(1);
				}
			}
			Key::Left => x = x.saturating_sub(1),
			Key::Right => {
				if x < width {
					x = x.saturating_add(1);
				}	
			}
			_ => (),
		}
		self.cursor_position = Position { x, y }
	}

	fn draw_welcome_message(&self) {
		let mut welcome_message = format!("Eddy editor -- version {}\r", VERSION);
		let width = self.terminal.size().width as usize;
		let len = welcome_message.len();
		let padding = width.saturating_sub(len) / 2;
		let spaces = " ".repeat(padding.saturating_sub(1));
		welcome_message = format!("~{}{}", spaces, welcome_message);
		welcome_message.truncate(width);
		println!("{}\r", welcome_message);
	}

	fn draw_rows(&self) {
		let height = self.terminal.size().height;

		for row in 0..height - 1 {
			Terminal::clear_current_line();
			if row == height / 2 {
				self.draw_welcome_message();
			} else {
				println!("~\r");
			}
		}
	}
}

fn close_program(error: &std::io::Error) {
	Terminal::clear_screen();
	panic!("{}", error);
}