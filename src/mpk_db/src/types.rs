use rusqlite::types::{ValueRef, FromSql, FromSqlResult, ToSql, ToSqlOutput};
// use crate::err::Result;
pub use uuid::Uuid;

mod id3;
pub use self::id3::{Id3, id3_walk};

#[derive(Clone)]
pub struct VecReal(pub Vec<f32>);

impl FromSql for VecReal {
  fn column_result(value: ValueRef) -> FromSqlResult<Self> {
    value.as_blob().and_then(|blob| {
      Ok(VecReal(unsafe {blob.align_to::<f32>().1.to_vec()}))
    })
  }
}

impl ToSql for VecReal {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(unsafe {self.0.align_to::<u8>().1}))
  }
}

pub struct VecText(pub Vec<String>);

impl FromSql for VecText {
  fn column_result(value: ValueRef) -> FromSqlResult<Self> {
    value.as_str().and_then(|text| {
      Ok(VecText(text.split("|").collect::<Vec<_>>()
		 .iter().map(|s| s.to_string()).collect()))
    })
  }
}

impl ToSql for VecText {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(self.0.join("|")))
  }
}

pub struct TrackTags {
  pub artist: Option<String>,
  pub title: Option<String>,
  pub album: Option<String>,
  pub genre: Option<String>,
  pub year: Option<i16>,
}

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

pub struct LowlevelFeatures {
  pub average_loudness: f32,
  pub barkbanks_kurtosis: VecReal,
  pub barkbanks_skewness: VecReal,
  pub barkbanks_spread: VecReal,
  pub barkbanks: VecReal,
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

pub struct SfxFeatures {
  pub pitch_after_max_to_before_max_energy_ratio: f32,
  pub pitch_centroid: f32,
  pub pitch_max_to_total: f32,
  pub pitch_min_to_total: f32,
  pub inharmonicity: VecReal,
  pub oddtoevenharmonicenergyratio: VecReal,
  pub tristimulus: VecReal,
}

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

pub enum SpecType {
  Mel,
  Log,
  Freq,
}

pub struct Spectograms {
  pub mel_spec: VecReal,
  pub log_spec: VecReal,
  pub freq_spec: VecReal,
}
