mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
// this is a note
