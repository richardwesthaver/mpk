use rusqlite::{Connection, OpenFlags, ToSql};
use std::path::Path;
use mpk_config::DbConfig;

mod err;
pub use err::{Error, Result};

mod types;
pub use types::*;

/// MPK Database
#[derive(Debug)]
pub struct Mdb {
  conn: Connection,
}

impl Mdb {
  pub fn new(path: Option<&Path>) -> Result<Mdb> {
    let conn = match path {
      Some(p) => Connection::open(p)?,
      None => Connection::open_in_memory()?,
    };

    Ok(
      Mdb {
	conn
      }
    )
  }

  pub fn new_with_config(cfg: DbConfig) -> Result<Mdb> {
    let flags: OpenFlags = OpenFlags::from_bits(cfg.flags().unwrap()).unwrap();
    let conn = match cfg.path() {
      Some(p) => Connection::open_with_flags(p, flags)?,
      None => Connection::open_in_memory_with_flags(flags)?,
    };

    Ok(
      Mdb {
	conn
      }
    )
  }

  pub fn exec_batch(&self, sql: &str) -> Result<()> {
    self.conn.execute_batch(sql)?;
    Ok(())
  }

  pub fn exec(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize> {
    let res = self.conn.execute(sql, params)?;
    Ok(res)
  }


  pub fn last_insert_rowid(&self) -> i64 {
    self.conn.last_insert_rowid()
  }

  pub fn init(&self) -> Result<()> {
    let sql = include_str!("init.sql");

    self.exec_batch(sql)
  }

  pub fn insert_track(&self, path: &str) -> Result<i64> {
    self.exec("insert into tracks (path) values (?)", &[&path])?;
    Ok(self.last_insert_rowid())
  }

  pub fn insert_track_tags(&self, id: i64, tags: &TrackTags) -> Result<()> {
    self.exec("insert into track_tags values (?,?,?,?,?,?)",
	      &[&id, &tags.artist, &tags.title, &tags.album, &tags.genre, &tags.year])?;
    Ok(())
  }

  pub fn insert_track_tags_musicbrainz(&self, id: i64, tags: &MusicbrainzTags) -> Result<()> {
    self.exec("insert into track_tags_musicbrainz values (?,?,?,?,?,?,?,?,?)",
	      &[&id,
		&tags.albumartistid,
		&tags.albumid,
		&tags.albumstatus,
		&tags.albumtype,
		&tags.artistid,
		&tags.releasegroupid,
		&tags.releasetrackid,
		&tags.trackid])?;
    Ok(())
  }

  pub fn insert_track_features_lowlevel(&self, id: i64, features: &LowlevelFeatures) -> Result<()> {
    self.exec("insert into track_features_lowlevel
values (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
	      &[&id,
		&features.average_loudness,
		&features.barkbanks_kurtosis,
		&features.barkbanks_skewness,
		&features.barkbanks_spread,
		&features.barkbanks,
		&features.dissonance,
		&features.hfc,
		&features.pitch,
		&features.pitch_instantaneous_confidence,
		&features.pitch_salience,
		&features.silence_rate_20db,
		&features.silence_rate_30db,
		&features.silence_rate_60db,
		&features.spectral_centroid,
		&features.spectral_complexity,
		&features.spectral_crest,
		&features.spectral_decrease,
		&features.spectral_energy,
		&features.spectral_energyband_high,
		&features.spectral_energyband_low,
		&features.spectral_energyband_middle_high,
		&features.spectral_energyband_middle_low,
		&features.spectral_flatness_db,
		&features.spectral_flux,
		&features.spectral_kurtosis,
		&features.spectral_rms,
		&features.spectral_rolloff,
		&features.spectral_skewness,
		&features.spectral_spread,
		&features.spectral_strongpeak,
		&features.zerocrossingrate,
		&features.mfcc,
		&features.sccoeffs,
		&features.scvalleys])?;
    Ok(())
  }

  pub fn insert_track_features_rhythm(&self, id: i64, features: &RhythmFeatures) -> Result<()> {
    self.exec("insert into track_features_rhythm values (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
	      &[&id,
		&features.bpm,
		&features.confidence,
		&features.onset_rate,
		&features.beats_loudness,
		&features.first_peak_bpm,
		&features.first_peak_spread,
		&features.first_peak_weight,
		&features.second_peak_bpm,
		&features.second_peak_spread,
		&features.second_peak_weight,
		&features.beats_position,
		&features.bpm_estimates,
		&features.bpm_intervals,
		&features.onset_times,
		&features.beats_loudness_band_ratio,
		&features.histogram])?;
    Ok(())
  }

  pub fn insert_track_features_sfx(&self, id: i64, features: &SfxFeatures) -> Result<()> {
    self.exec("insert into track_features_sfx values (?,?,?,?,?,?,?,?)",
	      &[&id,
		&features.pitch_after_max_to_before_max_energy_ratio,
		&features.pitch_centroid,
		&features.pitch_max_to_total,
		&features.pitch_min_to_total,
		&features.inharmonicity,
		&features.oddtoevenharmonicenergyratio,
		&features.tristimulus])?;
    Ok(())
  }

  pub fn insert_track_features_tonal(&self, id: i64, features: &TonalFeatures) -> Result<()> {
    self.exec("insert into track_features_tonal (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
	      &[&id,
		&features.chords_change_rate,
		&features.chords_number_rate,
		&features.key_strength,
		&features.tuning_diatonic_strength,
		&features.tuning_equal_tempered_deviation,
		&features.tuning_frequency,
		&features.tuning_nontempered_tuning_ratio,
		&features.chords_strength,
		&features.chords_histogram,
		&features.thpcp,
		&features.hpcp,
		&features.chords_key,
		&features.chords_scale,
		&features.key_key,
		&features.key_scale,
		&features.chord_progression
	      ])?;
    Ok(())
  }

  pub fn insert_track_images(&self, id: i64, images: &Spectograms) -> Result<()> {
    self.exec("insert into track_images values (?,?,?,?)",
	      &[&id, &images.mel_spec, &images.log_spec, &images.freq_spec])?;
    Ok(())
  }

  pub fn insert_track_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample(&self, path: &str) -> Result<()> {
    self.exec("insert into samples (path) values (?)", &[&path])?;
    Ok(())
  }

  pub fn insert_sample_features_lowlevel(&self, id: i64, features: LowlevelFeatures) -> Result<()> {
    self.exec("insert into sample_features_lowlevel values (?)",
	      &[&id,
		&features.average_loudness,
		&features.barkbanks_kurtosis,
		&features.barkbanks_skewness,
		&features.barkbanks_spread,
		&features.barkbanks,
		&features.dissonance,
		&features.hfc,
		&features.pitch,
		&features.pitch_instantaneous_confidence,
		&features.pitch_salience,
		&features.silence_rate_20db,
		&features.silence_rate_30db,
		&features.silence_rate_60db,
		&features.spectral_centroid,
		&features.spectral_complexity,
		&features.spectral_crest,
		&features.spectral_decrease,
		&features.spectral_energy,
		&features.spectral_energyband_high,
		&features.spectral_energyband_low,
		&features.spectral_energyband_middle_high,
		&features.spectral_energyband_middle_low,
		&features.spectral_flatness_db,
		&features.spectral_flux,
		&features.spectral_kurtosis,
		&features.spectral_rms,
		&features.spectral_rolloff,
		&features.spectral_skewness,
		&features.spectral_spread,
		&features.spectral_strongpeak,
		&features.zerocrossingrate,
		&features.mfcc,
		&features.sccoeffs,
		&features.scvalleys])?;
    Ok(())
  }

  pub fn insert_sample_features_rhythm(&self, id: i64, features: RhythmFeatures) -> Result<()> {
    self.exec("insert into sample_features_rhythm values (?)",
	      &[&id,
		&features.bpm,
		&features.confidence,
		&features.onset_rate,
		&features.beats_loudness,
		&features.first_peak_bpm,
		&features.first_peak_spread,
		&features.first_peak_weight,
		&features.second_peak_bpm,
		&features.second_peak_spread,
		&features.second_peak_weight,
		&features.beats_position,
		&features.bpm_estimates,
		&features.bpm_intervals,
		&features.onset_times,
		&features.beats_loudness_band_ratio,
		&features.histogram])?;
    Ok(())
  }

  pub fn insert_sample_features_sfx(&self, id: i64, features: SfxFeatures) -> Result<()> {
    self.exec("insert into sample_features_sfx values (?)",
	      &[&id,
		&features.pitch_after_max_to_before_max_energy_ratio,
		&features.pitch_centroid,
		&features.pitch_max_to_total,
		&features.pitch_min_to_total,
		&features.inharmonicity,
		&features.oddtoevenharmonicenergyratio,
		&features.tristimulus])?;
    Ok(())
  }

  pub fn insert_sample_features_tonal(&self, id: i64, features: TonalFeatures) -> Result<()> {
    self.exec("insert into sample_features_tonal (?)",
	      &[&id,
		&features.chords_change_rate,
		&features.chords_number_rate,
		&features.key_strength,
		&features.tuning_diatonic_strength,
		&features.tuning_equal_tempered_deviation,
		&features.tuning_frequency,
		&features.tuning_nontempered_tuning_ratio,
		&features.chords_strength,
		&features.chords_histogram,
		&features.thpcp,
		&features.hpcp,
		&features.chords_key,
		&features.chords_scale,
		&features.key_key,
		&features.key_scale,
//		&features.chord_progression
	      ])?;
    Ok(())
  }

  pub fn insert_sample_images(&self, id: i64, images: Spectograms) -> Result<()> {
    self.exec("insert into track_images values (?)",
	      &[&id, &images.mel_spec, &images.log_spec, &images.freq_spec])?;
    Ok(())
  }

  pub fn insert_sample_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project(&self, name: &str, path: &str, ty: &str) -> Result<()> {
    self.exec("insert into projects (name, path, type) values (?,?,?)", &[&name, &path, &ty])?;
    Ok(())
  }

  pub fn insert_project_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }
}
