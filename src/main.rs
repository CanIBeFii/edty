use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
	let _stdout =stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
		let b = b.unwrap();
		let letter = b as char;

		if letter.is_control() {
			println!("{:?} \r", b);
		} else {
			println!("{} ({:?})\r", letter, b);
		}
		if letter == 'q' {
			break;
		}
	}
}
