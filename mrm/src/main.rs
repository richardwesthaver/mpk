use std::{fs, path::Path, std::io, collections::HashSet};

fn dedup<P: AsRef<Path>>(path: P, dry_run: bool) -> io::Result<()> {
  let path = path.as_ref();
  let mut files: Vec<Path> = vec![];
  if path.is_file() {
    
  } else if path.is_dir() {
    
  }
}

fn main() {
  
}
