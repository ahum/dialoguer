extern crate ahum_dialoguer;

use ahum_dialoguer::Editor;

fn main() {
    if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
        println!("Your message:");
        println!("{}", rv);
    } else {
        println!("Abort!");
    }
}
