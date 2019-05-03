
extern crate dialoguer;

use dialoguer::{theme::CustomPromptCharacterTheme, FileInput};
use std::env;

fn main() {

    let theme = CustomPromptCharacterTheme::new('>');
    let input: env::PathBuf = FileInput::with_theme(&theme)
        .with_prompt("Choose file")
        .default(env::current_dir().expect("Got a dir"))
        .interact()
        .unwrap();
    println!("Hello {}!", input);
}
