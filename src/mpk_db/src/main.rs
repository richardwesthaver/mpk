use mpk_db::{Mdb, Id3};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
  cmd: Option<String>,
  #[clap(short,long)]
  db: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  let mdb = Mdb::new(args.db)?;

  mdb.exec("create table if not exists tracks (
            path text not null,
            title text not null,
            artist text not null,
            album text,
            genre,
            year)", [])?;

  let song = Id3::new("Reference-The_Best_Day_In_Detroit.mp3")?;
  let path = Some(String::from(song.path.to_str().unwrap()));
  let artist = song.get_tag("TPE1");
  let title = song.get_tag("TIT2");
  let album = song.get_tag("TALB");
  let genre = song.get_tag("TCON");
  let year = song.get_tag("TDRC");

  mdb.exec("insert into tracks (path, title, artist, album, genre, year)
            values (?,?,?,?,?,?)", [path, title, artist, album, genre, year])?;
  Ok(())
}
