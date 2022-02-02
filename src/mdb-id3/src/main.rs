//! mdb-id3 --- ID3 parsing utils
// TXXX tag???
use id3::Tag;
use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::Serialize;

/// mdb-id3
#[derive(Parser, Debug)]
struct Args {
  path: PathBuf,
  output: Option<PathBuf>,
}

fn id3_walk(path: &Path, coll: &mut HashMap<PathBuf, Option<Tag>>) -> Result<(), io::Error> {
  if path.is_dir() {
    for elt in fs::read_dir(path)? {
      let elt = elt?;
      let p = elt.path();
      if p.is_dir() {
	id3_walk(&p, coll).expect("failed to walk path");
      } else if p.is_file() {
	println!("parsing {:?}", p);
	coll.insert(p.to_path_buf(),
		    if let Ok(t) = Tag::read_from_path(&p) {
		      Some(t)
		    } else { None }
	);
      } 
    }
  } else if path.is_file() {
    coll.insert(path.to_path_buf(),
		if let Ok(t) = Tag::read_from_path(&path) {
		  Some(t)
		} else { None });
  }
  Ok(())
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
      for f in tag.frames().into_iter() {
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

  let mut coll = HashMap::new();
  id3_walk(args.path.as_path(), &mut coll)?;

  let mut buff = Vec::new();

  if let Some(ref f) = args.output {
    let mut out = fs::OpenOptions::new()
      .create_new(true)
      .append(true)
      .open(f)?;

    for (k,v) in coll.into_iter() {
      let json = JsonId3::new(k,v)?;
      serde_json::to_writer_pretty(&mut buff, &json)?;
    }
    out.write_all(&buff)?;
  } else {
      for (k,v) in coll.into_iter() {
	let json = JsonId3::new(k,v)?;
	println!("{}", serde_json::to_string_pretty(&json)?);
      }
    }

  Ok(())
}
