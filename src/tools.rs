use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::IntoRawFd;
use std::path::Path;

use regex::Regex;


macro_rules! println_stderr {
    ($fmt:expr) => (
        match writeln!(&mut ::std::io::stderr(), $fmt) {
            Ok(_) => {}
            Err(e) => println!("write to stderr failed: {:?}", e)
        }
    );
    ($fmt:expr, $($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $fmt, $($arg)*) {
            Ok(_) => {}
            Err(e) => println!("write to stderr failed: {:?}", e)
        }
    );
}


pub fn wrap_sep_string(sep: &str, s: &str) -> String {
    let mut _token = String::new();
    let mut met_subsep = false;
    // let set previous_subsep to any char except '`' or '"'
    let mut previous_subsep = 'N';
    for c in s.chars() {
        // handle cmds like: export DIR=`brew --prefix openssl`/include
        // or like: export foo="hello world"
        if sep.is_empty() && (c == '`' || c == '"') {
            if !met_subsep {
                met_subsep = true;
                previous_subsep = c;
            } else if c == previous_subsep {
                met_subsep = false;
                previous_subsep = 'N';
            }
        }
        if c.to_string() == sep {
            _token.push('\\');
        }
        if c == ' ' && sep.is_empty() && !met_subsep {
            _token.push('\\');
        }
        _token.push(c);
    }
    format!("{}{}{}", sep, _token, sep)
}




pub fn escape_path(path: &str) -> String {
    let re;
    match Regex::new(r##"(?P<c>[!\(\)<>,\?\]\[\{\} \\'"`*\^#|$&;])"##) {
        Ok(x) => {
            re = x;
        }
        Err(_) => {
            return path.to_string();
        }
    }
    return re.replace_all(path, "\\$c").to_string();
}
