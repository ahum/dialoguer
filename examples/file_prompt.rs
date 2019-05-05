extern crate dialoguer;

use dialoguer::{theme::CustomPromptCharacterTheme, FileInput};
use std::env;
use std::path::PathBuf;

fn main() {
    let theme = CustomPromptCharacterTheme::new('>');
    let input: PathBuf = FileInput::with_theme(&theme)
        .with_prompt("Choose file")
        .default(env::current_dir().expect("Got a dir"))
        .interact()
        .unwrap();
    println!("Hello {}!", input.to_string_lossy());
}
