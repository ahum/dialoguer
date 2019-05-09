extern crate ahum_dialoguer;

use ahum_dialoguer::{theme::CustomPromptCharacterTheme, Input};

fn main() {
    let theme = CustomPromptCharacterTheme::new('>');
    let input: String = Input::with_theme(&theme)
        .with_prompt("Your name")
        .interact()
        .unwrap();
    println!("Hello {}!", input);
}
