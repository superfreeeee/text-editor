#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction
)]

mod editor;
use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
