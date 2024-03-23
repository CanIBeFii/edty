#![warn(clippy::all)]
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
