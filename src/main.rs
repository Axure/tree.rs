use std::io;
use std::fs::{self};
use std::path::Path;
use std::env;

///
/// TODO: Maybe I can cache the string.
fn tree_print_dir(dir_name: &Path, depth: usize, vector: &mut Vec<bool>) -> io::Result<()> {
  let prefix: String = vector.iter().fold(String::new(), |sum, x| sum + match x {
    &true => "│   ",
    &false => "    "
  });
  let dirs = try!(fs::read_dir(dir_name));
  let mut next_dirs = try!(fs::read_dir(dir_name));
  next_dirs.next();
  for possible_dir in dirs {
    match possible_dir {
      Ok(dir) => {
        let next = next_dirs.next();
        match next {
          Some(Ok(ref v)) => {
            println!("{}├──{:?}", prefix, dir.file_name());
          },
          _ => {
            println!("{}└──{:?}", prefix, dir.file_name());
          }
        }
        if dir.path().is_dir() {
          match next {
            Some(Ok(ref v)) => {
              vector.push(true);
            },
            _ => {
              vector.push(false);
            }
          }
          tree_print_dir(&dir.path(), depth + 1, vector);
          vector.pop();
        }
      }
      _ => {}
    }
  }
  Ok(())
}

///
/// TODO: I need to record the problems that I solve, I think.
fn main() {
  if let Some(arg1) = env::args().nth(1) {
    println!("The first argument is {}", arg1);
  }
  let mut vec: Vec<bool> = Vec::new();
  tree_print_dir(Path::new("./"), 0, &mut vec);
}
