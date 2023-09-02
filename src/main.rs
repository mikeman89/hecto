#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    let e = Editor::default();
    e.run();
}
// this is a note
