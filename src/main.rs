use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
	let byte = c as u8;

	byte & 0b0001_1111
}

fn close_program(error: std::io::Error) {
	panic!(error);
}

fn main() {
	let _stdout =stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
		match b {
			Ok(b) => {
				let letter = b as char;

				if letter.is_control() {
					println!("{:#b} \r", b);
				} else {
					println!("{} ({:#b})\r", letter, b);
				}

				if b == to_ctrl_byte('q') {
					break;
				}
			}
			Err(error) => close_program(error)
		}
	}
}
