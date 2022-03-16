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

#[derive(Debug)]
pub struct VecReal(pub Vec<f32>);

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

#[derive(Debug)]
pub struct MatrixReal(pub Vec<VecReal>);

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
  pub format: Option<String>,
  pub channels: Option<i16>,
  pub filesize: Option<usize>,
  pub bitrate: Option<u32>,
  pub bitdepth: Option<u8>,
  pub duration: Option<u32>,
  pub samplerate: Option<u32>,
}

impl fmt::Display for AudioData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let format = self.format.as_ref().map(|s| s.as_str()).unwrap_or("NULL");
    let channels = self
      .channels
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let filesize = self
      .filesize
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let bitrate = self
      .bitrate
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let bitdepth = self
      .bitdepth
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let duration = self
      .duration
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    let samplerate = self
      .samplerate
      .map(|n| n.to_string())
      .unwrap_or("NULL".to_string());
    write!(
      f,
      "path: {}
format: {}
channels: {}
filesize: {}
bitrate: {}
bitdepth: {}
duration: {}
samplerate: {}",
      self.path, format, channels, filesize, bitrate, bitdepth, duration, samplerate
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
  pub average_loudness: f32,
  pub barkbands_kurtosis: VecReal,
  pub barkbands_skewness: VecReal,
  pub barkbands_spread: VecReal,
  pub barkbands: VecReal,
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
  pub mfcc: VecReal,
  pub sccoeffs: VecReal,
  pub scvalleys: VecReal,
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
  pub bpm: f32,
  pub confidence: f32,
  pub onset_rate: f32,
  pub beats_loudness: VecReal,
  pub first_peak_bpm: f32,
  pub first_peak_spread: f32,
  pub first_peak_weight: f32,
  pub second_peak_bpm: f32,
  pub second_peak_spread: f32,
  pub second_peak_weight: f32,
  pub beats_position: VecReal,
  pub bpm_estimates: VecReal,
  pub bpm_intervals: VecReal,
  pub onset_times: VecReal,
  pub beats_loudness_band_ratio: VecReal,
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
  pub pitch_after_max_to_before_max_energy_ratio: f32,
  pub pitch_centroid: f32,
  pub pitch_max_to_total: f32,
  pub pitch_min_to_total: f32,
  pub inharmonicity: VecReal,
  pub oddtoevenharmonicenergyratio: VecReal,
  pub tristimulus: VecReal,
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
  pub chords_change_rate: f32,
  pub chords_number_rate: f32,
  pub key_strength: f32,
  pub tuning_diatonic_strength: f32,
  pub tuning_equal_tempered_deviation: f32,
  pub tuning_frequency: f32,
  pub tuning_nontempered_tuning_ratio: f32,
  pub chords_strength: VecReal,
  pub chords_histogram: VecReal,
  pub thpcp: VecReal,
  pub hpcp: VecReal,
  pub chords_key: String,
  pub chords_scale: String,
  pub key_key: String,
  pub key_scale: String,
  pub chord_progression: VecText,
}

#[derive(Debug)]
pub struct Spectrograms {
  pub mel_spec: VecReal,
  pub log_spec: VecReal,
  pub freq_spec: VecReal,
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
