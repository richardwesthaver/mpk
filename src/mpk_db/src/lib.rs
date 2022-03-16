use mpk_config::DbConfig;
use rusqlite::{types::Value, Connection, OpenFlags, ToSql};
use std::path::Path;
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

    Ok(Mdb { conn })
  }

  pub fn new_with_config(cfg: DbConfig) -> Result<Mdb> {
    let flags: OpenFlags = OpenFlags::from_bits(cfg.flags().unwrap()).unwrap();
    let conn = match cfg.path() {
      Some(p) => Connection::open_with_flags(p, flags)?,
      None => Connection::open_in_memory_with_flags(flags)?,
    };

    Ok(Mdb { conn })
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

  pub fn last_track_id(&self, path: &str) -> i64 {
    self
      .conn
      .query_row("select id from tracks where path = ?", [path], |row| {
        row.get(0)
      })
      .unwrap()
  }

  pub fn last_sample_id(&self, path: &str) -> i64 {
    self
      .conn
      .query_row("select id from samples where path = ?", [path], |row| {
        row.get(0)
      })
      .unwrap()
  }

  pub fn init(&self) -> Result<()> {
    let sql = include_str!("init.sql");

    self.exec_batch(sql)
  }

  pub fn insert_track(&self, path: &str) -> Result<i64> {
    self.exec(
      "insert into tracks (path) values (?1)
on conflict do update set path = ?1",
      &[&path],
    )?;
    Ok(self.last_track_id(path))
  }

  pub fn insert_track_tags(&self, id: i64, tags: &TrackTags) -> Result<()> {
    self.exec(
      "
insert into track_tags values (?1,?2,?3,?4,?5,?6)
on conflict do update
set artist = ?2,
    title = ?3,
    album = ?4,
    genre = ?5,
    year =  ?6
where track_id = ?1",
      &[
        &id,
        &tags.artist,
        &tags.title,
        &tags.album,
        &tags.genre,
        &tags.year,
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_tags_musicbrainz(
    &self,
    id: i64,
    tags: &MusicbrainzTags,
  ) -> Result<()> {
    self.exec(
      "
insert into track_tags_musicbrainz values (?1,?2,?3,?4,?5,?6,?7,?8,?9)
on conflict do update
set albumartistid = ?2,
    albumid = ?3,
    albumstatus = ?4,
    albumtype = ?5,
    artistid = ?6,
    releasegroupid = ?7,
    releasetrackid = ?8,
    trackid = ?9
where track_id = ?1",
      &[
        &id,
        &tags.albumartistid,
        &tags.albumid,
        &tags.albumstatus,
        &tags.albumtype,
        &tags.artistid,
        &tags.releasegroupid,
        &tags.releasetrackid,
        &tags.trackid,
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_features_lowlevel(
    &self,
    id: i64,
    features: &LowlevelFeatures,
  ) -> Result<()> {
    self.exec("insert into track_features_lowlevel
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35)
on conflict do update
set average_loudness = ?2,
barkbands_kurtosis = ?3,
barkbands_skewness = ?4,
barkbands_spread = ?5,
barkbands = ?6,
dissonance = ?7,
hfc = ?8,
pitch = ?9,
pitch_instantaneous_confidence = ?10,
pitch_salience = ?11,
silence_rate_20db = ?12,
silence_rate_30db = ?13,
silence_rate_60db = ?14,
spectral_centroid = ?15,
spectral_complexity = ?16,
spectral_crest = ?17,
spectral_decrease = ?18,
spectral_energy = ?19,
spectral_energyband_high = ?20,
spectral_energyband_low = ?21,
spectral_energyband_middle_high = ?22,
spectral_energyband_middle_low = ?23,
spectral_flatness_db = ?24,
spectral_flux = ?25,
spectral_kurtosis = ?26,
spectral_rms = ?27,
spectral_rolloff = ?28,
spectral_skewness = ?29,
spectral_spread = ?30,
spectral_strongpeak = ?31,
zerocrossingrate = ?32,
mfcc = ?33,
sccoeffs = ?34,
scvalleys = ?35
where track_id = ?1",
	      &[&id,
		&features.average_loudness,
		&features.barkbands_kurtosis,
		&features.barkbands_skewness,
		&features.barkbands_spread,
		&features.barkbands,
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

  pub fn insert_track_features_rhythm(
    &self,
    id: i64,
    features: &RhythmFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into track_features_rhythm
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
on conflict do update
set bpm = ?2,
confidence = ?3,
onset_rate = ?4,
beats_loudness = ?5,
first_peak_bpm = ?6,
first_peak_spread = ?7,
first_peak_weight = ?8,
second_peak_bpm = ?9,
second_peak_spread = ?10,
second_peak_weight = ?11,
beats_position = ?12,
bpm_estimates = ?13,
bpm_intervals = ?14,
onset_times = ?15,
beats_loudness_band_ratio = ?16,
histogram = ?17
where track_id = ?1",
      &[
        &id,
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
        &features.histogram,
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_features_sfx(
    &self,
    id: i64,
    features: &SfxFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into track_features_sfx
values (?1,?2,?3,?4,?5,?6,?7,?8)
on conflict do update
set pitch_after_max_to_before_max_energy_ratio = ?2,
pitch_centroid = ?3,
pitch_max_to_total = ?4,
pitch_min_to_total = ?5,
inharmonicity = ?6,
oddtoevenharmonicenergyratio = ?7,
tristimulus = ?8
where track_id = ?1
",
      &[
        &id,
        &features.pitch_after_max_to_before_max_energy_ratio,
        &features.pitch_centroid,
        &features.pitch_max_to_total,
        &features.pitch_min_to_total,
        &features.inharmonicity,
        &features.oddtoevenharmonicenergyratio,
        &features.tristimulus,
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_features_tonal(
    &self,
    id: i64,
    features: &TonalFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into track_features_tonal
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
on conflict do update
set chords_change_rate = ?2,
chords_number_rate = ?3,
key_strength = ?4,
tuning_diatonic_strength = ?5,
tuning_equal_tempered_deviation = ?6,
tuning_frequency = ?7,
tuning_nontempered_tuning_ratio = ?8,
chords_strength = ?9,
chords_histogram = ?10,
thpcp = ?11,
hpcp = ?12,
chords_key = ?13,
chords_scale = ?14,
key_key = ?15,
key_scale = ?16,
chord_progression = ?17
where track_id = ?1",
      &[
        &id,
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
        &features.chord_progression,
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_images(&self, id: i64, images: &Spectrograms) -> Result<()> {
    self.exec(
      "insert into track_images values (?,?,?,?)
on conflict do update
set mel_spec = ?2,
log_spec = ?3,
freq_spec = ?4
where track_id = ?1",
      &[&id, &images.mel_spec, &images.log_spec, &images.freq_spec],
    )?;
    Ok(())
  }

  pub fn insert_track_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample(&self, path: &str) -> Result<i64> {
    self.exec(
      "insert into samples (path) values (?1)
on conflict(path) do update set path = ?1",
      &[&path],
    )?;
    Ok(self.last_sample_id(path))
  }

  pub fn insert_sample_features_lowlevel(
    &self,
    id: i64,
    features: &LowlevelFeatures,
  ) -> Result<()> {
    self.exec("insert into sample_features_lowlevel
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35)
on conflict do update
set average_loudness = ?2,
barkbands_kurtosis = ?3,
barkbands_skewness = ?4,
barkbands_spread = ?5,
barkbands = ?6,
dissonance = ?7,
hfc = ?8,
pitch = ?9,
pitch_instantaneous_confidence = ?10,
pitch_salience = ?11,
silence_rate_20db = ?12,
silence_rate_30db = ?13,
silence_rate_60db = ?14,
spectral_centroid = ?15,
spectral_complexity = ?16,
spectral_crest = ?17,
spectral_decrease = ?18,
spectral_energy = ?19,
spectral_energyband_high = ?20,
spectral_energyband_low = ?21,
spectral_energyband_middle_high = ?22,
spectral_energyband_middle_low = ?23,
spectral_flatness_db = ?24,
spectral_flux = ?25,
spectral_kurtosis = ?26,
spectral_rms = ?27,
spectral_rolloff = ?28,
spectral_skewness = ?29,
spectral_spread = ?30,
spectral_strongpeak = ?31,
zerocrossingrate = ?32,
mfcc = ?33,
sccoeffs = ?34,
scvalleys = ?35
where sample_id = ?1",
	      &[&id,
		&features.average_loudness,
		&features.barkbands_kurtosis,
		&features.barkbands_skewness,
		&features.barkbands_spread,
		&features.barkbands,
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

  pub fn insert_sample_features_rhythm(
    &self,
    id: i64,
    features: &RhythmFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into sample_features_rhythm
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
on conflict do update
set bpm = ?2,
confidence = ?3,
onset_rate = ?4,
beats_loudness = ?5,
first_peak_bpm = ?6,
first_peak_spread = ?7,
first_peak_weight = ?8,
second_peak_bpm = ?9,
second_peak_spread = ?10,
second_peak_weight = ?11,
beats_position = ?12,
bpm_estimates = ?13,
bpm_intervals = ?14,
onset_times = ?15,
beats_loudness_band_ratio = ?16,
histogram = ?17
where sample_id = ?1",
      &[
        &id,
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
        &features.histogram,
      ],
    )?;
    Ok(())
  }

  pub fn insert_sample_features_sfx(
    &self,
    id: i64,
    features: &SfxFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into sample_features_sfx
values (?1,?2,?3,?4,?5,?6,?7,?8)
on conflict do update
set pitch_after_max_to_before_max_energy_ratio = ?2,
pitch_centroid = ?3,
pitch_max_to_total = ?4,
pitch_min_to_total = ?5,
inharmonicity = ?6,
oddtoevenharmonicenergyratio = ?7,
tristimulus = ?8
where sample_id = ?1
",
      &[
        &id,
        &features.pitch_after_max_to_before_max_energy_ratio,
        &features.pitch_centroid,
        &features.pitch_max_to_total,
        &features.pitch_min_to_total,
        &features.inharmonicity,
        &features.oddtoevenharmonicenergyratio,
        &features.tristimulus,
      ],
    )?;
    Ok(())
  }

  pub fn insert_sample_features_tonal(
    &self,
    id: i64,
    features: &TonalFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into sample_features_tonal
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
on conflict do update
set chords_change_rate = ?2,
chords_number_rate = ?3,
key_strength = ?4,
tuning_diatonic_strength = ?5,
tuning_equal_tempered_deviation = ?6,
tuning_frequency = ?7,
tuning_nontempered_tuning_ratio = ?8,
chords_strength = ?9,
chords_histogram = ?10,
thpcp = ?11,
hpcp = ?12,
chords_key = ?13,
chords_scale = ?14,
key_key = ?15,
key_scale = ?16,
chord_progression = ?17
where sample_id = ?1",
      &[
        &id,
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
        &features.chord_progression,
      ],
    )?;
    Ok(())
  }

  pub fn insert_sample_images(&self, id: i64, images: &Spectrograms) -> Result<()> {
    self.exec(
      "insert into track_images values (?1,?2,?3,?4)
on conflict do update
set mel_spec = ?2,
log_spec = ?3,
freq_spec = ?4
where sample_id = ?1",
      &[&id, &images.mel_spec, &images.log_spec, &images.freq_spec],
    )?;
    Ok(())
  }

  pub fn insert_sample_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project(&self, name: &str, path: &str, ty: &str) -> Result<()> {
    self.exec(
      "insert into projects (name, path, type) values (?,?,?)",
      &[&name, &path, &ty],
    )?;
    Ok(())
  }

  pub fn insert_project_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn query_track(&self, id: i64) -> Result<AudioData> {
    let res =
      self
        .conn
        .query_row("select * from tracks where id = ?", [id], |row| {
          Ok(AudioData {
            path: row.get(1)?,
            format: row.get(2)?,
            channels: row.get(3)?,
            filesize: row.get(4)?,
            bitrate: row.get(5)?,
            bitdepth: row.get(6)?,
            duration: row.get(7)?,
            samplerate: row.get(8)?,
          })
        })?;
    Ok(res)
  }

  pub fn query_track_tags(&self, id: i64) -> Result<TrackTags> {
    let res = self.conn.query_row(
      "select * from track_tags where track_id = ?",
      [id],
      |row| {
        Ok(TrackTags {
          artist: Some(row.get(1)?),
          title: Some(row.get(2)?),
          album: Some(row.get(3)?),
          genre: Some(row.get(4)?),
          year: Some(row.get(5)?),
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_tags_musicbrainz(&self, id: i64) -> Result<MusicbrainzTags> {
    let res = self.conn.query_row(
      "select * from track_tags_musicbrainz where track_id = ?",
      [id],
      |row| {
        Ok(MusicbrainzTags {
          albumartistid: row.get(1)?,
          albumid: row.get(2)?,
          albumstatus: row.get(3)?,
          albumtype: row.get(4)?,
          artistid: row.get(5)?,
          releasegroupid: row.get(6)?,
          releasetrackid: row.get(7)?,
          trackid: row.get(8)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_features_lowlevel(&self, id: i64) -> Result<LowlevelFeatures> {
    let res = self.conn.query_row(
      "select * from track_features_lowlevel where track_id = ?",
      [id],
      |row| {
        Ok(LowlevelFeatures {
          average_loudness: row.get(1)?,
          barkbands_kurtosis: row.get(2)?,
          barkbands_skewness: row.get(3)?,
          barkbands_spread: row.get(4)?,
          barkbands: row.get(5)?,
          dissonance: row.get(6)?,
          hfc: row.get(7)?,
          pitch: row.get(8)?,
          pitch_instantaneous_confidence: row.get(9)?,
          pitch_salience: row.get(10)?,
          silence_rate_20db: row.get(11)?,
          silence_rate_30db: row.get(12)?,
          silence_rate_60db: row.get(13)?,
          spectral_centroid: row.get(14)?,
          spectral_complexity: row.get(15)?,
          spectral_crest: row.get(16)?,
          spectral_decrease: row.get(17)?,
          spectral_energy: row.get(18)?,
          spectral_energyband_high: row.get(19)?,
          spectral_energyband_low: row.get(20)?,
          spectral_energyband_middle_high: row.get(21)?,
          spectral_energyband_middle_low: row.get(22)?,
          spectral_flatness_db: row.get(23)?,
          spectral_flux: row.get(24)?,
          spectral_kurtosis: row.get(25)?,
          spectral_rms: row.get(26)?,
          spectral_rolloff: row.get(27)?,
          spectral_skewness: row.get(28)?,
          spectral_spread: row.get(29)?,
          spectral_strongpeak: row.get(30)?,
          zerocrossingrate: row.get(31)?,
          mfcc: row.get(32)?,
          sccoeffs: row.get(33)?,
          scvalleys: row.get(34)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_features_rhythm(&self, id: i64) -> Result<RhythmFeatures> {
    let res = self.conn.query_row(
      "select * from track_features_rhythm where track_id = ?",
      [id],
      |row| {
        Ok(RhythmFeatures {
          bpm: row.get(1)?,
          confidence: row.get(2)?,
          onset_rate: row.get(3)?,
          beats_loudness: row.get(4)?,
          first_peak_bpm: row.get(5)?,
          first_peak_spread: row.get(6)?,
          first_peak_weight: row.get(7)?,
          second_peak_bpm: row.get(8)?,
          second_peak_spread: row.get(9)?,
          second_peak_weight: row.get(10)?,
          beats_position: row.get(11)?,
          bpm_estimates: row.get(12)?,
          bpm_intervals: row.get(13)?,
          onset_times: row.get(14)?,
          beats_loudness_band_ratio: row.get(15)?,
          histogram: row.get(16)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_images(&self, id: i64) -> Result<Spectrograms> {
    let res = self.conn.query_row(
      "select * from track_images where track_id = ?",
      [id],
      |row| {
        Ok(Spectrograms {
          mel_spec: row.get(1)?,
          log_spec: row.get(2)?,
          freq_spec: row.get(3)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_sample(&self, id: i64) -> Result<AudioData> {
    let res =
      self
        .conn
        .query_row("select * from samples where id = ?", [id], |row| {
          Ok(AudioData {
            path: row.get(1)?,
            format: row.get(2)?,
            channels: row.get(3)?,
            filesize: row.get(4)?,
            bitrate: row.get(5)?,
            bitdepth: row.get(6)?,
            duration: row.get(7)?,
            samplerate: row.get(8)?,
          })
        })?;
    Ok(res)
  }

  pub fn query_sample_features_lowlevel(&self, id: i64) -> Result<LowlevelFeatures> {
    let res = self.conn.query_row(
      "select * from sample_features_lowlevel where sample_id = ?",
      [id],
      |row| {
        Ok(LowlevelFeatures {
          average_loudness: row.get(1)?,
          barkbands_kurtosis: row.get(2)?,
          barkbands_skewness: row.get(3)?,
          barkbands_spread: row.get(4)?,
          barkbands: row.get(5)?,
          dissonance: row.get(6)?,
          hfc: row.get(7)?,
          pitch: row.get(8)?,
          pitch_instantaneous_confidence: row.get(9)?,
          pitch_salience: row.get(10)?,
          silence_rate_20db: row.get(11)?,
          silence_rate_30db: row.get(12)?,
          silence_rate_60db: row.get(13)?,
          spectral_centroid: row.get(14)?,
          spectral_complexity: row.get(15)?,
          spectral_crest: row.get(16)?,
          spectral_decrease: row.get(17)?,
          spectral_energy: row.get(18)?,
          spectral_energyband_high: row.get(19)?,
          spectral_energyband_low: row.get(20)?,
          spectral_energyband_middle_high: row.get(21)?,
          spectral_energyband_middle_low: row.get(22)?,
          spectral_flatness_db: row.get(23)?,
          spectral_flux: row.get(24)?,
          spectral_kurtosis: row.get(25)?,
          spectral_rms: row.get(26)?,
          spectral_rolloff: row.get(27)?,
          spectral_skewness: row.get(28)?,
          spectral_spread: row.get(29)?,
          spectral_strongpeak: row.get(30)?,
          zerocrossingrate: row.get(31)?,
          mfcc: row.get(32)?,
          sccoeffs: row.get(33)?,
          scvalleys: row.get(34)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_sample_features_rhythm(&self, id: i64) -> Result<RhythmFeatures> {
    let res = self.conn.query_row(
      "select * from sample_features_rhythm where sample_id = ?",
      [id],
      |row| {
        Ok(RhythmFeatures {
          bpm: row.get(1)?,
          confidence: row.get(2)?,
          onset_rate: row.get(3)?,
          beats_loudness: row.get(4)?,
          first_peak_bpm: row.get(5)?,
          first_peak_spread: row.get(6)?,
          first_peak_weight: row.get(7)?,
          second_peak_bpm: row.get(8)?,
          second_peak_spread: row.get(9)?,
          second_peak_weight: row.get(10)?,
          beats_position: row.get(11)?,
          bpm_estimates: row.get(12)?,
          bpm_intervals: row.get(13)?,
          onset_times: row.get(14)?,
          beats_loudness_band_ratio: row.get(15)?,
          histogram: row.get(16)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_sample_images(&self, id: i64) -> Result<Spectrograms> {
    let res = self.conn.query_row(
      "select * from sample_images where sample_id = ?",
      [id],
      |row| {
        Ok(Spectrograms {
          mel_spec: row.get(1)?,
          log_spec: row.get(2)?,
          freq_spec: row.get(3)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_raw(&self, sql: &str) -> Result<DbValues> {
    let mut stmt = self.conn.prepare(sql)?;
    let count = stmt.column_count();
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    while let Some(r) = rows.next()? {
      for i in 0..count {
        res.push(DbValue(r.get(i)?));
      }
    }
    Ok(DbValues(res))
  }

  pub fn track_count(&self) -> Result<usize> {
    let res = self
      .conn
      .query_row("select count(*) from tracks", [], |row| row.get(0))?;
    Ok(res)
  }

  pub fn sample_count(&self) -> Result<usize> {
    let res = self
      .conn
      .query_row("select count(*) from samples", [], |row| row.get(0))?;
    Ok(res)
  }

  pub fn close(self) {
    self.conn.close().unwrap();
  }
}
