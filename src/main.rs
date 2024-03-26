#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
	clippy::missing_docs_in_private_items,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::needless_return,
	clippy::wildcard_enum_match_arm
)]

mod document;
mod row;
mod editor;
mod terminal;
use editor::Editor;
pub use document::Document;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;

fn main() {
	Editor::default().run();
}
