use std::collections::HashSet;
use std::env;
use std::fs::read_dir;
use std::io::Write;
use std::iter::FromIterator;
use std::os::unix::fs::PermissionsExt;
use std::path::{is_separator, MAIN_SEPARATOR};
use std::sync::Arc;

use linefeed::complete::{Completer, Completion, Suffix};
use linefeed::terminal::Terminal;
use linefeed::Prompter;

use crate::tools;

pub struct PathCompleter;

/**
*  let cpl = Arc::new(path::PathCompleter);
       cpl.complete(word, reader, start, _end)
*/

// impl<Term: Terminal> Completer<Term> for PathCompleter {
//   fn complete(
//     &self,
//     word: &str,
//     // _reader: &Prompter<Term>,
//     // _start: usize,
//     // _end: usize,
//   ) -> Option<Vec<Completion>> {
//     Some(complete_path(word, false))
//   }
// }

/// Returns a sorted list of paths whose prefix matches the given path.
pub fn complete_path(path: &str, for_dir: bool) -> Vec<Completion> {
  let mut res = Vec::new();

  // let tokens = parsers::parser_line::cmd_to_tokens(word);
  // let (path, path_sep) = if tokens.is_empty() {
  //     (String::new(), String::new())
  // } else {
  //     let (ref _path_sep, ref _path) = tokens[tokens.len() - 1];
  //     (_path.clone(), _path_sep.clone())
  // };
  let path_sep = String::new();

  let (_dir_orig, _) = split_path(path);
  let dir_orig = if let Some(_dir) = _dir_orig { _dir } else { "" };
  let mut path_extended = path.clone();
  let (_dir_lookup, file_name) = split_path(&path_extended);
  let dir_lookup = _dir_lookup.unwrap_or(".");
  if let Ok(entries) = read_dir(".") {
    for entry in entries {
      if let Ok(entry) = entry {
        let pathbuf = entry.path();
        let is_dir = pathbuf.is_dir();
        if for_dir && !is_dir {
          continue;
        }

        let entry_name = entry.file_name();
        // TODO: Deal with non-UTF8 paths in some way
        if let Ok(_path) = entry_name.into_string() {
          if _path.starts_with(file_name) {
            let (name, display) = if dir_orig != "" {
              (
                format!("{}{}{}", dir_orig, MAIN_SEPARATOR, _path),
                Some(_path),
              )
            } else {
              (_path, None)
            };
            let mut name = str::replace(name.as_str(), "//", "/");
            if path_sep.is_empty() {
              name = tools::escape_path(&name);
            }
            let mut quoted = false;
            if !path_sep.is_empty() {
              name = tools::wrap_sep_string(&path_sep, &name);
              quoted = true;
            }
            let suffix = if is_dir {
              if quoted {
                name.pop();
              }
              Suffix::Some(MAIN_SEPARATOR)
            } else {
              Suffix::Default
            };
            res.push(Completion {
              completion: name,
              display,
              suffix,
            });
          }
        }
      }
    }
  }
  res
}

fn split_path(path: &str) -> (Option<&str>, &str) {
  match path.rfind(is_separator) {
    Some(pos) => (Some(&path[..=pos]), &path[pos + 1..]),
    None => (None, path),
  }
}

#[cfg(test)]
mod tests {
  use super::split_path;

  #[test]
  fn test_split_path() {
    assert_eq!(split_path(""), (None, ""));
    assert_eq!(split_path(""), (None, ""));
  }
}
