use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 1;

#[derive(Default, Debug, Copy, Clone)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

#[derive(Debug)]
struct StatusMessage {
	text: String,
	time: Instant,
}

impl StatusMessage {
	fn from(message: String) -> Self {
		Self {
			time: Instant::now(),
			text: message,
		}
	}
}

pub struct Editor {
	quit: bool,
	terminal: Terminal,
	cursor_position: Position,
	offset: Position,
	document: Document,
	status_message: StatusMessage,
	quit_times: u8,
}

impl Editor {
	pub fn default() -> Self {
		let args: Vec<String> = env::args().collect();
		let mut initial_status = String::from("HELP: Crtl-S = save | Crtl-Q = quit");
		let document = if args.len() > 1 {
			let file_name = &args[1];
			if let Ok(doc) =  Document::open(file_name) {
				doc
			} else {
				initial_status = format!("ERR: Could not open file: {file_name}");
				Document::default()
			}
		} else {
			Document::default()
		};

		return Self {
			quit: false,
			terminal: Terminal::new().expect("Failed to initialize terminal :0"),
			cursor_position: Position::default(),
			offset: Position::default(),
			document,
			status_message: StatusMessage::from(initial_status),
			quit_times: QUIT_TIMES,
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
		Terminal::cursor_position(&Position::default());
		if self.quit {
			Terminal::clear_screen();
			println!("Bye bye >:3\r");
		} else {
			self.draw_rows();
			self.draw_status_bar();
			self.draw_message_bar();
			Terminal::cursor_position(&Position {
				x: self.cursor_position.x.saturating_sub(self.offset.x),
				y: self.cursor_position.y.saturating_sub(self.offset.y),
			});
		}
		Terminal::show_cursor();
		return Terminal::flush()
	}

	fn save(&mut self) {
		if self.document.file_name.is_none() {
			let new_name = self.prompt("Save as: ").unwrap_or(None);
			if new_name.is_none() {
				self.status_message = StatusMessage::from("Save aborted.".to_string());
				return;
			}
			self.document.file_name = new_name;
		}

		if self.document.save().is_ok() {
			self.status_message = StatusMessage::from("File saved successfully".to_string());
		} else {
			self.status_message = StatusMessage::from("Error writing file!".to_string());
		}
	}

	fn process_keypress(&mut self) -> Result<(), std::io::Error> {
		let pressed_key = Terminal::read_key()?;
		match pressed_key {
			Key::Ctrl('q') => {
				if self.quit_times > 0 && self.document.is_dirty() {
					self.status_message = StatusMessage::from(
						"WARNING! File has unsaved changes, Press Ctrl-Q again to quit.".to_string()
					);
					self.quit_times -= 1;
					return Ok(());
				}
				self.quit = true;
			},
			Key::Ctrl('s') => self.save(),
			Key::Char(c) => {
				self.document.insert(&self.cursor_position, c);
				self.move_cursor(Key::Right);
			},
			Key::Delete => self.document.delete(&self.cursor_position),
			Key::Backspace => {
				if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
					self.move_cursor(Key::Left);
					self.document.delete(&self.cursor_position);
				}
			},
			Key::Up
			| Key::Down
			| Key::Left
			| Key::Right
			| Key::PageUp
			| Key::PageDown
			| Key::Home
			| Key::End => self.move_cursor(pressed_key),
			_ => (),
		}
		self.scroll();
		if self.quit_times < QUIT_TIMES {
			self.quit_times = QUIT_TIMES;
			self.status_message = StatusMessage::from(String::new());
		}
		Ok(())
	}

	fn scroll(&mut self) {
		let Position {x, y} = self.cursor_position;
		let width = self.terminal.size().width as usize;
		let height = self.terminal.size().height as usize;

		let offset = &mut self.offset;
		if y < offset.y {
			offset.y = y;
		} else if y >= offset.y.saturating_add(height) {
			offset.y = y.saturating_sub(height).saturating_add(1);
		} else {
			// We don't care if y < offset.y + height
		}

		if x < offset.x {
			offset.x = x;
		} else if x >= offset.x.saturating_add(width) {
			offset.x = x.saturating_sub(width).saturating_add(1);
		} else {
			// We don't care if x < offset.x + width
		}
	}

	fn move_cursor(&mut self, key: Key) {
		let terminal_height = self.terminal.size().height as usize;
		let Position{ mut y, mut x } = self.cursor_position;
		let height = self.document.len();
		let mut width = if let Some(row) = self.document.row(y) {
			row.len()
		} else {
			0
		};

		match key {
			Key::Up => y = y.saturating_sub(1),
			Key::Down => {
				if y < height {
					y = y.saturating_add(1);
				}
			}
			Key::Left => {
				
				if x > 0 {
					x -= 1;
				} else if y > 0 {
					y -= 1;
					if let Some(row) = self.document.row(y) {
						x = row.len();
					} else {
						x = 0;
					}
				} else {
					// We only care if x > 0 or y > 0
				}
			},
			Key::Right => {
				if x < width {
					x += 1;
				} else if y < height {
					y += 1;
					x = 0;
				} else {
					// We only care if x < width or y > height
				}
			}
			Key::PageUp => {
				y = if y > terminal_height {
					y.saturating_sub(terminal_height)
				} else {
					0
				}
			},
			Key::PageDown => {
				y = if y.saturating_add(terminal_height) < height {
					y.saturating_add(terminal_height)
				} else {
					height
				}
			},
			Key::Home => x = 0,
			Key::End => x = width,
			_ => (),
		}
		width = if let Some(row) = self.document.row(y) {
			row.len()
		} else {
			0
		};

		if x > width {
			x = width;
		}

		self.cursor_position = Position { x, y }
	}

	fn draw_welcome_message(&self) {
		let mut welcome_message = format!("Eddy editor -- version {VERSION}\r");
		let width = self.terminal.size().width as usize;
		let len = welcome_message.len();
		let padding = width.saturating_sub(len) / 2;
		let spaces = " ".repeat(padding.saturating_sub(1));
		welcome_message = format!("~{spaces}{welcome_message}");
		welcome_message.truncate(width);
		println!("{welcome_message}\r");
	}

	pub fn draw_row(&self, row: &Row) {
		let width = self.terminal.size().width as usize;
		let start = self.offset.x;
		let end = self.offset.x + width;
		let row = row.render(start, end);
		println!("{row}\r");
	}

	fn draw_rows(&self) {
		let height = self.terminal.size().height;

		for terminal_row in 0..height {
			Terminal::clear_current_line();
			if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
				self.draw_row(row); 
			} else if self.document.is_empty() && terminal_row == height / 2 {
				self.draw_welcome_message();
			} else {
				println!("~\r");
			}
		}
	}

	fn draw_status_bar(&self) {
		let mut status;
		let width = self.terminal.size().width as usize;
		let modified_indicator = if self.document.is_dirty() {
			" (modified)"
		} else {
			""
		};
		let mut file_name = "[No Name]".to_string();
		if let Some(name) = &self.document.file_name {
			file_name = name.clone();
			file_name.truncate(20);
		}
		status = format!(
			"{} - {} lines{}",
			file_name,
			self.document.len(),
			modified_indicator
		);
		let line_indicator = format!(
			"{}/{}",
			self.cursor_position.y.saturating_add(1),
			self.document.len()
		);
		let len = status.len() + line_indicator.len();
		if width > len {
			status.push_str(&" ".repeat(width - status.len()));
		}
		status = format!("{status}{line_indicator}");
		status.truncate(width);
		Terminal::set_bg_color(STATUS_BG_COLOR);
		Terminal::set_fg_color(STATUS_FG_COLOR);
		println!("{status}\r");
		Terminal::reset_fg_color();
		Terminal::reset_bg_color();
	}

	fn draw_message_bar(&self) {
		Terminal::clear_current_line();
		let message = &self.status_message;
		if message.time.elapsed() < Duration::new(5, 0) {
			let mut text = message.text.clone();
			text.truncate(self.terminal.size().width as usize);
			print!("{text}");
		}
	}

	fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
		let mut result = String::new();
		loop {
			self.status_message = StatusMessage::from(format!("{prompt}{result}"));
			self.refresh_screen()?;
			match Terminal::read_key()? {
				Key::Backspace => {
					if !result.is_empty() {
						result.pop();
					}
				},
				Key::Char('\n') => break,
				Key::Char(c) => {
					if !c.is_control() {
						result.push(c);
					}
				},
				Key::Esc => {
					result.clear();
					break;
				},
				_ => (),
			}
		}
		self.status_message = StatusMessage::from(String::new());
		if result.is_empty() {
			return Ok(None);
		}
		Ok(Some(result))
	}
}

fn close_program(error: &std::io::Error) {
	Terminal::clear_screen();
	panic!("{}", error);
}