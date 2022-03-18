use crate::err::{Error, Result};
use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, Value, ValueRef};
use std::fmt;
use std::str::FromStr;
pub use uuid::Uuid;

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

#[derive(Debug, Clone)]
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

impl Iterator for VecReal {
  type Item = f32;
  fn next(&mut self) -> Option<Self::Item> {
    self.into_iter().next()
  }
}

impl From<MatrixReal> for VecReal {
  fn from(m: MatrixReal) -> Self {
    let data: Vec<f32> = m.into_iter().flatten().collect();
    VecReal(data)
  }
}

impl<'a> From<&'a MatrixReal> for VecReal {
  fn from(m: &'a MatrixReal) -> Self {
    m.to_vec()
  }
}

#[derive(Debug)]
pub struct MatrixReal(pub Vec<VecReal>);

impl MatrixReal {
  pub fn new(v: VecReal, s: usize) -> Self {
    let data = v.0.chunks_exact(s).collect::<MatrixReal>();
    data
  }
  pub fn to_vec(&self) -> VecReal {
    self.into()
  }
  pub fn frame_len(&self) -> usize {
    self.0.first().unwrap().len()
  }
}

impl Iterator for MatrixReal {
  type Item = VecReal;
  fn next(&mut self) -> Option<Self::Item> {
    self.into_iter().next()
  }
}

impl FromIterator<VecReal> for MatrixReal {
  fn from_iter<I: IntoIterator<Item=VecReal>>(iter: I) -> Self {
    let mut mtx = MatrixReal(Vec::new());
    for i in iter {
      mtx.0.push(i);
    }
    mtx
  }
}

impl<'a> FromIterator<&'a [f32]> for MatrixReal {
  fn from_iter<I: IntoIterator<Item=&'a [f32]>>(iter: I) -> Self {
    let mut mtx = MatrixReal(Vec::new());
    for i in iter {
      mtx.0.push(VecReal(i.into()));
    }
    mtx
  }
}

impl fmt::Display for MatrixReal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let first = self.0.first().unwrap();
    let last = self.0.last().unwrap();
    write!(
      f,
      "matrix([{0} ... {1}], len={2})",
      first,
      last,
      self.0.len()
    )
  }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct AudioData {
  pub path: String,
  pub filesize: Option<usize>,
  pub duration: Option<f64>,
  pub channels: Option<u8>,
  pub bitrate: Option<u32>,
  pub samplerate: Option<u32>,
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
    write!(
      f,
      "path: {}
filesize: {}
duration: {}
channels: {}
bitrate: {}
samplerate: {}",
      self.path, filesize, duration, channels, bitrate, samplerate
    )
  }
}

#[derive(Debug)]
pub struct TrackTags {
  pub artist: Option<String>,
  pub title: Option<String>,
  pub album: Option<String>,
  pub genre: Option<String>,
  pub year: Option<i16>,
}

impl fmt::Display for TrackTags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let artist = self.artist.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let title = self.title.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let album = self.album.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let genre = self.genre.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let year = self
      .year
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    write!(
      f,
      "artist: {}
title: {}
album: {}
genre: {}
year: {}",
      artist, title, album, genre, year
    )
  }
}

#[derive(Debug)]
pub struct MusicbrainzTags {
  pub albumartistid: Uuid,
  pub albumid: Uuid,
  pub albumstatus: String,
  pub albumtype: String,
  pub artistid: Uuid,
  pub releasegroupid: Uuid,
  pub releasetrackid: Uuid,
  pub trackid: Uuid,
}

impl fmt::Display for MusicbrainzTags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "albumartistid: {}
albumid: {}
albumstatus: {}
albumtype: {}
artistid: {}
releasegroupid: {}
releasetrackid: {}
trackid: {}",
      self.albumartistid,
      self.albumid,
      self.albumstatus,
      self.albumtype,
      self.artistid,
      self.releasegroupid,
      self.releasetrackid,
      self.trackid
    )
  }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Spectrograms {
  pub mel_spec: MatrixReal,
  pub log_spec: MatrixReal,
  pub freq_spec: MatrixReal,
}

impl fmt::Display for Spectrograms {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "mel_spec: {}
log_spec: {}
freq_spec: {}",
      self.mel_spec, self.log_spec, self.freq_spec
    )
  }
}

#[derive(Debug)]
pub enum SpecType {
  Mel,
  Log,
  Freq,
}

#[derive(Debug)]
pub enum QueryBy {
  Id,
  Path,
  Title,
  Artist,
  Album,
  Genre,
  Year,
  SampleRate,
}

#[derive(Debug)]
pub enum QueryType {
  Info,
  Tags,
  Musicbrainz,
  Lowlevel,
  Rhythm,
  Sfx,
  Tonal,
  Spectrograms,
  All,
  Raw,
}

impl FromStr for QueryType {
  type Err = Error;
  fn from_str(input: &str) -> Result<QueryType> {
    match input {
      "info" => Ok(QueryType::Info),
      "tags" => Ok(QueryType::Tags),
      "musicbrainz" | "mb" => Ok(QueryType::Musicbrainz),
      "lowlevel" => Ok(QueryType::Lowlevel),
      "rhythm" => Ok(QueryType::Rhythm),
      "sfx" => Ok(QueryType::Sfx),
      "tonal" => Ok(QueryType::Tonal),
      "spectrograms" | "specs" => Ok(QueryType::Spectrograms),
      "all" => Ok(QueryType::All),
      "raw" => Ok(QueryType::Raw),
      e => Err(Error::BadQType(e.to_string())),
    }
  }
}
