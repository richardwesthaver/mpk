//! MPK_DB TYPES
//!
//! SQLite accepts only TEXT, INTEGER, REAL, NULL, or BLOB where BLOB is a
//! byte-array.  the rusqlite crate handles conversions for common
//! types (String, usize, f64, [u8], Option<>) as well as a few custom types
//! (UUID) via feature flags. The FromSql and ToSql traits allow
//! querying/insertion with SQLite so our custom types to implement
//! these traits as well or be converted to a more primitive type.
//!
//! The low-level types which we need to use with SQLite are VecReal
//! and MatrixReal, which are 1D and 2D float-arrays respectively,
//! stored as BLOBs. MatrixReals are actually VecReals but with some
//! additional indexing information stored in the struct (frame_size
//! AKA column count).
//!
//! The high-level types are AudioData, TrackTags, MusicbrainzTags,
//! LowlevelFeatures, RhythmFeatures, SfxFeatures, TonalFeatures, and
//! Spectrograms. AudioData and TrackTags are the simplest, containing
//! only primitives. MusicbrainzTags is similar but includes Uuids
//! which are stored in the DB as BLOBs. The remaining types contain
//! our custom low-level types.
//!
//!  There are also some auxiliary types: DbValue, DbValues, QueryBy,
//!  and QueryType. These are all used to simplify Interactions with
//!  the DB.
use crate::err::{Error, Result};
use mpk_hash::Checksum;
use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, Value, ValueRef};
use std::fmt;
use std::ops::{Index, Range};
use std::str::FromStr;
use std::path::PathBuf;
use std::time::Duration;
pub use uuid::Uuid;

/// Display wrapper for SQLite Value
#[derive(Debug)]
pub struct DbValue(pub Value);

impl fmt::Display for DbValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.0 {
      Value::Null => write!(f, "NULL"),
      Value::Integer(i) => write!(f, "{}", i),
      Value::Real(i) => write!(f, "{}", i),
      Value::Text(ref i) => write!(f, "{}", i),
      Value::Blob(ref x) => write!(f, "<Blob[u8;{}]>", x.len()),
    }
  }
}

/// Display wrapper for a Vec of SQLite Values
#[derive(Debug)]
pub struct DbValues(pub Vec<DbValue>);

impl fmt::Display for DbValues {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for i in &self.0 {
      write!(f, "| {} ", i)?;
    }
    write!(f, "|")
  }
}

/// A Vec<f32> for SQLite
#[derive(Debug, Default, Clone, PartialEq)]
pub struct VecReal(pub Vec<f32>);

impl VecReal {
  pub fn len(&self) -> usize {
    self.0.len()
  }
}

impl FromSql for VecReal {
  fn column_result(value: ValueRef) -> FromSqlResult<Self> {
    value
      .as_blob()
      .and_then(|blob| Ok(VecReal(unsafe { blob.align_to::<f32>().1.to_vec() })))
  }
}

impl ToSql for VecReal {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(unsafe { self.0.align_to::<u8>().1 }))
  }
}

impl fmt::Display for VecReal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut iter = self.0.iter();
    let mut riter = self.0.iter().rev();
    write!(
      f,
      "vec([{0}, {1}, {2}, {3} ... {7}, {6}, {5}, {4}], len={8})",
      iter.next().unwrap(),
      iter.next().unwrap(),
      iter.next().unwrap(),
      iter.next().unwrap(),
      riter.next().unwrap(),
      riter.next().unwrap(),
      riter.next().unwrap(),
      riter.next().unwrap(),
      self.0.len()
    )
  }
}

impl From<Vec<f32>> for VecReal {
  fn from(v: Vec<f32>) -> Self {
    VecReal(v)
  }
}

impl Iterator for VecReal {
  type Item = f32;
  fn next(&mut self) -> Option<Self::Item> {
    self.into_iter().next()
  }
}

impl Index<usize> for VecReal {
  type Output = f32;
  fn index(&self, idx: usize) -> &Self::Output {
    &self.0[idx]
  }
}

impl Index<Range<usize>> for VecReal {
  type Output = [f32];
  fn index(&self, idx: Range<usize>) -> &Self::Output {
    &self.0[idx]
  }
}

impl From<MatrixReal> for VecReal {
  fn from(m: MatrixReal) -> Self {
    m.vec
  }
}

/// A Matrix of f32s for SQLite. Implemented as a flat Vec with a frame_size
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MatrixReal {
  pub vec: VecReal,
  pub frame_size: usize,
}

impl MatrixReal {
  pub fn new(vec: VecReal, frame_size: usize) -> Self {
    MatrixReal { vec, frame_size }
  }
  pub fn to_vec(&self) -> VecReal {
    self.clone().into()
  }
  pub fn frames(&self) -> usize {
    self.vec.len() / self.frame_size
  }
}

impl Index<usize> for MatrixReal {
  type Output = [f32];
  fn index(&self, idx: usize) -> &Self::Output {
    &self.vec[idx * self.frame_size..idx + 1 * self.frame_size]
  }
}

impl fmt::Display for MatrixReal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "matrix([{}, ...], frame_size={}, frames={})",
      VecReal::from(self[0].to_vec()),
      //      VecReal::from(self[1].to_vec()),
      self.frame_size,
      self.frames()
    )
  }
}

//TODO
#[derive(Debug)]
pub enum Note {
  C = 0,
  Db = 1,
  D = 2,
  Eb = 3,
  E = 4,
  F = 5,
  Gb = 6,
  G = 7,
  Ab = 8,
  A = 9,
  Bb = 10,
  B = 11,
}

impl FromStr for Note {
  type Err = Error;
  fn from_str(input: &str) -> Result<Note> {
    match input {
      "C" => Ok(Note::C),
      "Db" => Ok(Note::Db),
      "D" => Ok(Note::D),
      "Eb" => Ok(Note::Eb),
      "E" => Ok(Note::E),
      "F" => Ok(Note::F),
      "Gb" => Ok(Note::Gb),
      "G" => Ok(Note::G),
      "Ab" => Ok(Note::Ab),
      "A" => Ok(Note::A),
      "Bb" => Ok(Note::Bb),
      "B" => Ok(Note::B),
      e => Err(Error::BadNote(e.to_string())),
    }
  }
}

#[derive(Debug, Default)]
pub struct VecNote(pub Vec<Note>);

#[derive(Debug, Default)]
pub struct VecText(pub Vec<String>);

impl FromSql for VecText {
  fn column_result(value: ValueRef) -> FromSqlResult<Self> {
    value.as_str().and_then(|text| {
      Ok(VecText(
        text
          .split("|")
          .collect::<Vec<_>>()
          .iter()
          .map(|s| s.to_string())
          .collect(),
      ))
    })
  }
}

impl ToSql for VecText {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(self.0.join("|")))
  }
}

/// Generic audio file information
/// tables: [tracks, samples]
#[derive(Debug, Default)]
pub struct AudioData {
  pub path: String,
  pub filesize: Option<usize>,
  pub duration: Option<f64>,
  pub channels: Option<u8>,
  pub bitrate: Option<u32>,
  pub samplerate: Option<u32>,
  pub checksum: Option<Checksum>,
}

impl fmt::Display for AudioData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let filesize = self
      .filesize
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let duration = self
      .duration
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let channels = self
      .channels
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let bitrate = self
      .bitrate
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let samplerate = self
      .samplerate
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let checksum = self
      .checksum
      .map(|c| c.to_hex())
      .unwrap_or("NULL".to_string());
    write!(
      f,
      "path: {}
filesize: {} bytes
duration: {} ms
channels: {}
bitrate: {}
samplerate: {} hz
checksum: {}",
      self.path, filesize, duration, channels, bitrate, samplerate, checksum
    )
  }
}

/// Track tags retrieved from file headers (i.e. ID3).
/// tables: [track_tags]
#[derive(Debug, Default)]
pub struct TrackTags {
  pub artist: Option<String>,
  pub title: Option<String>,
  pub album: Option<String>,
  pub genre: Option<String>,
  pub date: Option<String>,
  pub tracknumber: Option<String>,
  pub format: Option<String>,
  pub language: Option<String>,
  pub country: Option<String>,
  pub label: Option<String>,
  pub producer: Option<String>,
  pub engineer: Option<String>,
  pub mixer: Option<String>,
}

impl fmt::Display for TrackTags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let artist = self.artist.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let title = self.title.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let album = self.album.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let genre = self.genre.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let date = self.date.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let tracknumber = self
      .tracknumber
      .as_ref()
      .map(|s| s.as_str())
      .unwrap_or("NULL");
    let format = self.format.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let language = self.language.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let country = self.country.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let label = self.label.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let producer = self.producer.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let engineer = self.engineer.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let mixer = self.mixer.as_ref().map(|s| s.as_str()).unwrap_or("NULL");

    write!(
      f,
      "artist: {}
title: {}
album: {}
genre: {}
date: {}
tracknumber: {}
format: {}
language: {}
country: {}
label: {}
producer: {}
engineer: {}
mixer: {}",
      artist,
      title,
      album,
      genre,
      date,
      tracknumber,
      format,
      language,
      country,
      label,
      producer,
      engineer,
      mixer
    )
  }
}

/// Track tags specific to musicbrainz.org. Used for looking up tracks
/// on the internet.
/// tables: [track_tags_musicbrainz]
#[derive(Debug, Default)]
pub struct MusicbrainzTags {
  pub albumartistid: Option<Uuid>,
  pub albumid: Option<Uuid>,
  pub albumstatus: Option<String>,
  pub albumtype: Option<String>,
  pub artistid: Option<Uuid>,
  pub releasegroupid: Option<Uuid>,
  pub releasetrackid: Option<Uuid>,
  pub trackid: Option<Uuid>,
  pub asin: Option<String>,
  pub musicip_puid: Option<Uuid>,
}

impl fmt::Display for MusicbrainzTags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let albumartistid = self
      .albumartistid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let albumid = self
      .albumid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let albumstatus = self
      .albumstatus
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let albumtype = self
      .albumtype
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let artistid = self
      .artistid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let releasegroupid = self
      .releasegroupid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let releasetrackid = self
      .releasetrackid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let trackid = self
      .trackid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let asin = self
      .asin
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    let musicip_puid = self
      .musicip_puid
      .as_ref()
      .map(|s| s.to_string())
      .unwrap_or("NULL".to_string());
    write!(
      f,
      "albumartistid: {}
albumid: {}
albumstatus: {}
albumtype: {}
artistid: {}
releasegroupid: {}
releasetrackid: {}
trackid: {}
asin: {}
musicip_puid: {}",
      albumartistid,
      albumid,
      albumstatus,
      albumtype,
      artistid,
      releasegroupid,
      releasetrackid,
      trackid,
      asin,
      musicip_puid,
    )
  }
}

/// Lowlevel features containg a variety of spectral features.
/// tables: [track_features_lowlevel, sample_features_lowlevel]
#[derive(Debug, Default)]
pub struct LowlevelFeatures {
  pub average_loudness: f64,
  pub barkbands_kurtosis: VecReal,
  pub barkbands_skewness: VecReal,
  pub barkbands_spread: VecReal,
  pub barkbands: MatrixReal,
  pub dissonance: VecReal,
  pub hfc: VecReal,
  pub pitch: VecReal,
  pub pitch_instantaneous_confidence: VecReal,
  pub pitch_salience: VecReal,
  pub silence_rate_20db: VecReal,
  pub silence_rate_30db: VecReal,
  pub silence_rate_60db: VecReal,
  pub spectral_centroid: VecReal,
  pub spectral_complexity: VecReal,
  pub spectral_crest: VecReal,
  pub spectral_decrease: VecReal,
  pub spectral_energy: VecReal,
  pub spectral_energyband_high: VecReal,
  pub spectral_energyband_low: VecReal,
  pub spectral_energyband_middle_high: VecReal,
  pub spectral_energyband_middle_low: VecReal,
  pub spectral_flatness_db: VecReal,
  pub spectral_flux: VecReal,
  pub spectral_kurtosis: VecReal,
  pub spectral_rms: VecReal,
  pub spectral_rolloff: VecReal,
  pub spectral_skewness: VecReal,
  pub spectral_spread: VecReal,
  pub spectral_strongpeak: VecReal,
  pub zerocrossingrate: VecReal,
  pub mfcc: MatrixReal,
  pub sccoeffs: MatrixReal,
  pub scvalleys: MatrixReal,
}

impl fmt::Display for LowlevelFeatures {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "average_loudness: {}
barkbands_kurtosis: {}
barkbands_skewness: {}
barkbands_spread: {}
barkbands: {}
dissonance: {}
hfc: {}
pitch: {}
pitch_instantaneous_confidence: {}
pitch_salience: {}
silence_rate_20db: {}
silence_rate_30db: {}
silence_rate_60db: {}
spectral_centroid: {}
spectral_complexity: {}
spectral_crest: {}
spectral_decrease: {}
spectral_energy: {}
spectral_energyband_high: {}
spectral_energyband_low: {}
spectral_energyband_middle_high: {}
spectral_energyband_middle_low: {}
spectral_flatness_db: {}
spectral_flux: {}
spectral_kurtosis: {}
spectral_rms: {}
spectral_rolloff: {}
spectral_skewness: {}
spectral_spread: {}
spectral_strongpeak: {}
zerocrossingrate: {}
mfcc: {}
sccoeffs: {}
scvalleys: {}",
      self.average_loudness,
      self.barkbands_kurtosis,
      self.barkbands_skewness,
      self.barkbands_spread,
      self.barkbands,
      self.dissonance,
      self.hfc,
      self.pitch,
      self.pitch_instantaneous_confidence,
      self.pitch_salience,
      self.silence_rate_20db,
      self.silence_rate_30db,
      self.silence_rate_60db,
      self.spectral_centroid,
      self.spectral_complexity,
      self.spectral_crest,
      self.spectral_decrease,
      self.spectral_energy,
      self.spectral_energyband_high,
      self.spectral_energyband_low,
      self.spectral_energyband_middle_high,
      self.spectral_energyband_middle_low,
      self.spectral_flatness_db,
      self.spectral_flux,
      self.spectral_kurtosis,
      self.spectral_rms,
      self.spectral_rolloff,
      self.spectral_skewness,
      self.spectral_spread,
      self.spectral_strongpeak,
      self.zerocrossingrate,
      self.mfcc,
      self.sccoeffs,
      self.scvalleys
    )
  }
}

/// Rhythm features for audio including bpm and onsets
/// tables: [track_features_rhythm, sample_features_rhythm]
#[derive(Debug, Default)]
pub struct RhythmFeatures {
  pub bpm: f64,
  pub confidence: f64,
  pub onset_rate: f64,
  pub beats_loudness: VecReal,
  pub first_peak_bpm: f64,
  pub first_peak_spread: f64,
  pub first_peak_weight: f64,
  pub second_peak_bpm: f64,
  pub second_peak_spread: f64,
  pub second_peak_weight: f64,
  pub beats_position: VecReal,
  pub bpm_estimates: VecReal,
  pub bpm_intervals: VecReal,
  pub onset_times: VecReal,
  pub beats_loudness_band_ratio: MatrixReal,
  pub histogram: VecReal,
}

impl fmt::Display for RhythmFeatures {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "bpm: {},
confidence: {}
onset_rate: {}
beats_loudness: {}
first_peak_bpm: {}
first_peak_spread: {}
first_peak_weight: {}
second_peak_bpm: {}
second_peak_spread: {}
second_peak_weight: {}
beats_position: {}
bpm_estimates: {}
bpm_intervals: {}
onset_times: {}
beats_loudness_band_ratio: {}
histogram: {}",
      self.bpm,
      self.confidence,
      self.onset_rate,
      self.beats_loudness,
      self.first_peak_bpm,
      self.first_peak_spread,
      self.first_peak_weight,
      self.second_peak_bpm,
      self.second_peak_spread,
      self.second_peak_weight,
      self.beats_position,
      self.bpm_estimates,
      self.bpm_intervals,
      self.onset_times,
      self.beats_loudness_band_ratio,
      self.histogram
    )
  }
}

/// SFX features
/// tables: [track_features_sfx, sample_features_sfx]
#[derive(Debug, Default)]
pub struct SfxFeatures {
  pub pitch_after_max_to_before_max_energy_ratio: f64,
  pub pitch_centroid: f64,
  pub pitch_max_to_total: f64,
  pub pitch_min_to_total: f64,
  pub inharmonicity: VecReal,
  pub oddtoevenharmonicenergyratio: VecReal,
  pub tristimulus: MatrixReal,
}

impl fmt::Display for SfxFeatures {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "pitch_after_max_to_before_max_energy_ratio: {}
pitch_centroid: {}
pitch_max_to_total: {}
pitch_min_to_total: {}
inharmonicity: {}
oddtoevenharmonicenergyratio: {}
tristimulus: {}",
      self.pitch_after_max_to_before_max_energy_ratio,
      self.pitch_centroid,
      self.pitch_max_to_total,
      self.pitch_min_to_total,
      self.inharmonicity,
      self.oddtoevenharmonicenergyratio,
      self.tristimulus
    )
  }
}

/// Tonal features including key and chord progression estimates
/// tables: [track_features_tonal, sample_features_tonal]
#[derive(Debug, Default)]
pub struct TonalFeatures {
  pub chords_changes_rate: f64,
  pub chords_number_rate: f64,
  pub key_strength: f64,
  pub tuning_diatonic_strength: f64,
  pub tuning_equal_tempered_deviation: f64,
  pub tuning_frequency: f64,
  pub tuning_nontempered_energy_ratio: f64,
  pub chords_strength: VecReal,
  pub chords_histogram: VecReal,
  pub thpcp: VecReal,
  pub hpcp: MatrixReal,
  pub chords_key: String,
  pub chords_scale: String,
  pub key_key: String,
  pub key_scale: String,
  pub chords_progression: VecText,
}

/// Audio spectrograms. Mel-weighted spectrogram is particularly
/// useful for analysis.
/// tables: [track_images, sample_images]
#[derive(Debug, Default)]
pub struct Spectrograms {
  pub mel_spec: Option<MatrixReal>,
  pub log_spec: Option<MatrixReal>,
  pub freq_spec: Option<MatrixReal>,
}

impl fmt::Display for Spectrograms {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "mel_spec: {}
log_spec: {}
freq_spec: {}",
      self
        .mel_spec
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or("NULL".to_string()),
      self
        .log_spec
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or("NULL".to_string()),
      self
        .freq_spec
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or("NULL".to_string()),
    )
  }
}

/// An identifier for spectrograms. Currently unused.
#[derive(Debug, Copy, Clone)]
pub enum SpecType {
  Mel,
  Log,
  Freq,
}

/// An identifier for audio types.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AudioType {
  Track,
  Sample,
}

impl AudioType {
  pub fn table_name(&self) -> &str {
    match self {
      AudioType::Track => "tracks",
      AudioType::Sample => "samples",
    }
  }
  pub fn track_else(&self, sql: &str) -> Result<String> {
    match self {
      AudioType::Track => {
	Ok(sql.into())
      },
      AudioType::Sample => {
	Err(Error::BadType("sample".into()))
      }
    }
  }
}

impl FromStr for AudioType {
  type Err = Error;
  fn from_str(input: &str) -> Result<AudioType> {
    match input {
      "track" | "tracks" => Ok(AudioType::Track),
      "sample" | "samples" => Ok(AudioType::Sample),
      e => Err(Error::BadType(e.to_string())),
    }
  }
}

impl fmt::Display for AudioType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AudioType::Track => write!(f, "track"),
      AudioType::Sample => write!(f, "sample"),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum QueryType {
  Single,
  Like,
  Range,
}

impl Default for QueryType {
  fn default() -> Self {
    QueryType::Single
  }
}

/// Columns to query by.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryBy {
  Id(u64),
  Path(PathBuf),
  Title(String),
  Artist(String),
  Album(String),
  Genre(String),
  Date(String),
  SampleRate(u32),
  Bpm(f64),
  Label(String),
}

impl QueryBy {
  pub fn as_query(&self, ty: AudioType, fr: QueryFor) -> Result<String> {
    match self {
      QueryBy::Id(n) => {
	Ok(format!("{} where id = {}", fr.as_query(ty)?, n))
      },
      QueryBy::Path(p) => {
	Ok(format!("{} where id in (select id from {} where path = '{}')", fr.as_query(ty)?, ty.table_name(), p.display()))
      },
      QueryBy::Title(s) => {
	ty.track_else(
	  format!("{} where id in (select id from track_tags where title = '{}')", fr.as_query(ty)?, s).as_str()
	)
      },
      QueryBy::Artist(s) => {
	ty.track_else(
	  format!("{} where id in (select id from track_tags where artist = '{}')", fr.as_query(ty)?, s).as_str()
	)
      },
      QueryBy::Album(s) => {
	ty.track_else(
	  format!("{} where id in (select id from track_tags where album = '{}')", fr.as_query(ty)?, s).as_str()
	)
      },
      QueryBy::Genre(s) => {
	ty.track_else(
	  format!("{} where id in (select id from track_tags where genre = '{}')", fr.as_query(ty)?, s).as_str()
	)
      },
      QueryBy::Date(s) => {
	ty.track_else(
	  format!("{} where id in (select id from track_tags where date = '{}')", fr.as_query(ty)?, s).as_str()
	)
      },
      QueryBy::SampleRate(n) => {
	Ok(format!("{} where id in (select id from {} where samplerate = {})", fr.as_query(ty)?, ty.table_name(), n))	
      },
      QueryBy::Bpm(n) => {
	Ok(format!("{} where id in (select id from {}_features_rhythm where bpm = {})", fr.as_query(ty)?, ty, n))	      },
      QueryBy::Label(s) => {
	ty.track_else(
	  format!("{} where id in (select id from track_tags where label = '{}')", fr.as_query(ty)?, s).as_str()
	)
      },
    }
  }
}

/// The type of query result. Determines which tables are returned by a query.
#[derive(Debug, Copy, Clone)]
pub enum QueryFor {
  Info,
  Tags,
  Musicbrainz,
  Lowlevel,
  Rhythm,
  Sfx,
  Tonal,
  Spectrograms,
  All,
}

impl QueryFor {
  pub fn as_query(&self, ty: AudioType) -> Result<String> {
    match self {
      QueryFor::Info => {
	Ok(format!("select * from {}", ty.table_name()))
      },
      QueryFor::Tags => {
	ty.track_else("select * from track_tags")
      },
      QueryFor::Musicbrainz => {
	ty.track_else("select * from track_tags_musicbrainz")
      },
      QueryFor::Lowlevel => {
	Ok(format!("select * from {}_features_lowlevel", ty))
      },
      QueryFor::Rhythm => {
	Ok(format!("select * from {}_features_rhythm", ty))
      },
      QueryFor::Sfx => {
	Ok(format!("select * from {}_features_sfx", ty))
      },
      QueryFor::Tonal => {
	Ok(format!("select * from {}_features_tonal", ty))
      },
      QueryFor::Spectrograms => {
	Ok(format!("select * from {}_images", ty))
      },
      QueryFor::All => {
	let mut q = format!("select * from {} ", ty.table_name());
	if ty == AudioType::Track {
	  q.push_str("join track_tags on tracks.id = track_tags
join track_tags_musicbrainz on tracks.id = track_tags_musicbrainz.track_id
");
	}
	q.push_str(format!("join {ty}_features_lowlevel on tracks.id = {ty}_features_lowlevel.{ty}_id
join {ty}_features_rhythm on {typ}.id = {ty}_features_rhythm.{ty}_id
join {ty}_features_sfx on {typ}.id = {ty}_features_sfx.{ty}_id
join {ty}_features_tonal on {typ}.id = {ty}_features_sfx.{ty}_id
join {ty}_images on {typ}.id = {ty}_images.{ty}_id", typ = ty.table_name()).as_str());	
	Ok(q)
      }
    }
  }
}

impl FromStr for QueryFor {
  type Err = Error;
  fn from_str(input: &str) -> Result<QueryFor> {
    match input {
      "info" => Ok(QueryFor::Info),
      "tags" => Ok(QueryFor::Tags),
      "musicbrainz" | "mb" => Ok(QueryFor::Musicbrainz),
      "lowlevel" => Ok(QueryFor::Lowlevel),
      "rhythm" => Ok(QueryFor::Rhythm),
      "sfx" => Ok(QueryFor::Sfx),
      "tonal" => Ok(QueryFor::Tonal),
      "spectrograms" | "specs" => Ok(QueryFor::Spectrograms),
      "all" => Ok(QueryFor::All),
      e => Err(Error::BadType(e.to_string())),
    }
  }
}
