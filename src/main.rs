#![warn(clippy::all, clippy::pedantic, clippy::restriction)]

mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}
