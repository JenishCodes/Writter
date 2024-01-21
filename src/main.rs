mod document;
mod editor;
mod row;
mod terminal;
mod filetype;
mod highlighting;

pub use document::Document;
use editor::{Editor, SearchDirection, Position};
pub use row::Row;
pub use terminal::Terminal;
pub use filetype::FileType;

fn main() {
    Editor::default().run();
}
