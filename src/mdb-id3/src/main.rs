use id3::Tag;
use clap::Parser;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::Serialize;

/// CLI Args
#[derive(Parser, Debug)]
struct Args {
  path: PathBuf,
  output: Option<PathBuf>,
}

fn id3_walk(path: &Path) -> io::Result<HashMap<PathBuf, Option<Tag>>> {
  let mut coll = HashMap::new();
  if path.is_dir() {
    for elt in fs::read_dir(path)? {
      let elt = elt?;
      let p = elt.path();
      if p.is_dir() {
	id3_walk(&p)?;
      } else if p.is_file() {
	println!("parsing {:?}", p);
	coll.insert(path.to_path_buf(),
		    if let Ok(t) = Tag::read_from_path(&p) {
		      Some(t)
		    } else { None }
	);
      }
    }
  } else if path.is_file() {
    coll.insert(path.to_path_buf(),
		if let Ok(t) = Tag::read_from_path(&path)
		{ Some(t) }
		else { None });
    
  }
  Ok(coll)
}

#[derive(Debug, Serialize)]
struct JsonId3 {
  path: PathBuf,
  tags: Option<HashMap<String, String>>
}

impl JsonId3 {
  fn new(path: PathBuf, tag: Option<Tag>) -> Result<JsonId3, id3::Error> {

    let tags = if let Some(tag) = tag { 
      let mut map = HashMap::new();
      for f in tag.frames() {
	map.insert(f.id().to_string(), f.content().to_string());
      }
      Some(map)
    } else { None };

    Ok(JsonId3 {
      path,
      tags,
    })
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();
  println!("parsing contents of {:?}", args.path.canonicalize()?);

  let coll = id3_walk(args.path.as_path())?;

  for (k,v) in coll {
    let json = JsonId3::new(k,v)?;
    if let Some(ref f) = args.output {
      let mut out = fs::File::create(f)?;
      serde_json::to_writer(&mut out, &json)?;
    } else {
      println!("{}", serde_json::to_string_pretty(&json)?);
    }
  }

  Ok(())
}
