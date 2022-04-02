//! MPK_DB
//!
//! Types for interacting with the SQLite DB. The schema is defined in 'init.sql'.
use mpk_config::DbConfig;
use mpk_hash::Checksum;
use rusqlite::{version, Connection, OpenFlags, ToSql};
use std::path::Path;
mod err;
pub use err::{Error, Result};
use rusqlite::backup::{Backup, Progress};
pub use rusqlite::DatabaseName;
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

  pub fn version() -> &'static str {
    version()
  }

  pub fn new_with_config(cfg: DbConfig) -> Result<Mdb> {
    let flags: OpenFlags = OpenFlags::from_bits(cfg.flags().unwrap()).unwrap();
    let conn = match cfg.path() {
      Some(p) => Connection::open_with_flags(p, flags)?,
      None => Connection::open_in_memory_with_flags(flags)?,
    };

    let mut db = Mdb { conn };
    if cfg.trace {
      db.set_tracer(Some(|x| println!("{}", x)))
    }
    if cfg.profile {
      db.set_profiler(Some(|x, y| println!("{} -- {}ms", x, y.as_millis())))
    }

    Ok(db)
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

  pub fn update_path<P: AsRef<Path>>(
    &self,
    path: P,
    checksum: Checksum,
    ty: AudioType,
  ) -> Result<()> {
    let sql = format!(
      "update {} set path = ?1 where checksum = ?2",
      ty.table_name()
    );
    self.exec(
      &sql,
      &[
        &path.as_ref().to_str().unwrap(),
        &checksum.to_hex().as_str(),
      ],
    )?;
    Ok(())
  }

  pub fn insert_track(&self, file: &AudioData) -> Result<i64> {
    self.exec(
      "insert into tracks (path, filesize, duration, channels, bitrate, samplerate, checksum) values (?1,?2,?3,?4,?5,?6,?7)
on conflict do update
set filesize = ?2,
duration = ?3,
channels = ?4,
bitrate = ?5,
samplerate = ?6,
checksum = ?7
where id = ?1",
      &[&file.path, &file.filesize, &file.duration, &file.channels, &file.bitrate, &file.samplerate, &file.checksum.map(|c| c.to_hex())],
    )?;
    Ok(self.last_track_id(&file.path))
  }

  pub fn insert_track_tags(&self, id: i64, tags: &TrackTags) -> Result<()> {
    self.exec(
      "
insert into track_tags values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)
on conflict do update
set artist = ?2,
    title = ?3,
    album = ?4,
    genre = ?5,
    date =  ?6,
    tracknumber = ?7,
    format = ?8,
    language = ?9,
    country = ?10,
    label = ?11,
    producer = ?12,
    engineer = ?13,
    mixer = ?14
where id = ?1",
      &[
        &id,
        &tags.artist,
        &tags.title,
        &tags.album,
        &tags.genre,
        &tags.date,
        &tags.tracknumber,
        &tags.format,
        &tags.language,
        &tags.country,
        &tags.label,
        &tags.producer,
        &tags.engineer,
        &tags.mixer,
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
insert into track_tags_musicbrainz values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
on conflict do update
set albumartistid = ?2,
albumid = ?3,
albumstatus = ?4,
albumtype = ?5,
artistid = ?6,
releasegroupid = ?7,
releasetrackid = ?8,
trackid = ?9,
asin = ?10,
musicip_puid = ?11
where id = ?1",
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
        &tags.asin,
        &tags.musicip_puid,
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
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38,?39)
on conflict do update
set average_loudness = ?2,
barkbands_kurtosis = ?3,
barkbands_skewness = ?4,
barkbands_spread = ?5,
barkbands_frame_size = ?6,
barkbands = ?7,
dissonance = ?8,
hfc = ?9,
pitch = ?10,
pitch_instantaneous_confidence = ?11,
pitch_salience = ?12,
silence_rate_20db = ?13,
silence_rate_30db = ?14,
silence_rate_60db = ?15,
spectral_centroid = ?16,
spectral_complexity = ?17,
spectral_crest = ?18,
spectral_decrease = ?19,
spectral_energy = ?20,
spectral_energyband_high = ?21,
spectral_energyband_low = ?22,
spectral_energyband_middle_high = ?23,
spectral_energyband_middle_low = ?24,
spectral_flatness_db = ?25,
spectral_flux = ?26,
spectral_kurtosis = ?27,
spectral_rms = ?28,
spectral_rolloff = ?29,
spectral_skewness = ?30,
spectral_spread = ?31,
spectral_strongpeak = ?32,
zerocrossingrate = ?33,
mfcc_frame_size = ?34,
mfcc = ?35,
sccoeffs_frame_size = ?36,
sccoeffs = ?37,
scvalleys_frame_size = ?38,
scvalleys = ?39
where id = ?1",
	      &[&id,
		&features.average_loudness,
		&features.barkbands_kurtosis,
		&features.barkbands_skewness,
		&features.barkbands_spread,
		&features.barkbands.frame_size,
		&features.barkbands.to_vec(),
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
		&features.mfcc.frame_size,
		&features.mfcc.to_vec(),
		&features.sccoeffs.frame_size,
		&features.sccoeffs.to_vec(),
		&features.scvalleys.frame_size,
		&features.scvalleys.to_vec()])?;
    Ok(())
  }

  pub fn insert_track_features_rhythm(
    &self,
    id: i64,
    features: &RhythmFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into track_features_rhythm
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)
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
beats_loudness_band_ratio_frame_size = ?16,
beats_loudness_band_ratio = ?17,
histogram = ?18
where id = ?1",
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
        &features.beats_loudness_band_ratio.frame_size,
        &features.beats_loudness_band_ratio.to_vec(),
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
where id = ?1
",
      &[
        &id,
        &features.pitch_after_max_to_before_max_energy_ratio,
        &features.pitch_centroid,
        &features.pitch_max_to_total,
        &features.pitch_min_to_total,
        &features.inharmonicity,
        &features.oddtoevenharmonicenergyratio,
        &features.tristimulus.to_vec(),
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
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)
on conflict do update
set chords_changes_rate = ?2,
chords_number_rate = ?3,
key_strength = ?4,
tuning_diatonic_strength = ?5,
tuning_equal_tempered_deviation = ?6,
tuning_frequency = ?7,
tuning_nontempered_energy_ratio = ?8,
chords_strength = ?9,
chords_histogram = ?10,
thpcp = ?11,
hpcp_frame_size = ?12,
hpcp = ?13,
chords_key = ?14,
chords_scale = ?15,
key_key = ?16,
key_scale = ?17,
chords_progression = ?18
where id = ?1",
      &[
        &id,
        &features.chords_changes_rate,
        &features.chords_number_rate,
        &features.key_strength,
        &features.tuning_diatonic_strength,
        &features.tuning_equal_tempered_deviation,
        &features.tuning_frequency,
        &features.tuning_nontempered_energy_ratio,
        &features.chords_strength,
        &features.chords_histogram,
        &features.thpcp,
        &features.hpcp.frame_size,
        &features.hpcp.to_vec(),
        &features.chords_key,
        &features.chords_scale,
        &features.key_key,
        &features.key_scale,
        &features.chords_progression,
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_images(&self, id: i64, images: &Spectrograms) -> Result<()> {
    self.exec(
      "insert into track_images values (?,?,?,?,?,?,?)
on conflict do update
set mel_frame_size = ?2,
mel_spec = ?3,
log_frame_size = ?4,
log_spec = ?5,
freq_frame_size = ?6,
freq_spec = ?7
where id = ?1",
      &[
        &id,
        &images.mel_spec.as_ref().map(|s| s.frame_size),
        &images.mel_spec.as_ref().map(|s| s.to_vec()),
        &images.log_spec.as_ref().map(|s| s.frame_size),
        &images.log_spec.as_ref().map(|s| s.to_vec()),
        &images.freq_spec.as_ref().map(|s| s.frame_size),
        &images.freq_spec.as_ref().map(|s| s.to_vec()),
      ],
    )?;
    Ok(())
  }

  pub fn insert_track_user_notes(&self, _note: &str, _append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_user_tags(&self, _tag: &str, _append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample(&self, file: &AudioData) -> Result<i64> {
    self.exec("insert into samples (path, filesize, duration, channels, bitrate, samplerate, checksum) values (?1,?2,?3,?4,?5,?6,?7)
on conflict do update
set filesize = ?2,
duration = ?3,
channels = ?4,
bitrate = ?5,
samplerate = ?6,
checksum = ?7
where id = ?1",
	      &[&file.path, &file.filesize, &file.duration, &file.channels, &file.bitrate, &file.samplerate, &file.checksum.map(|c| c.to_hex())],
    )?;
    Ok(self.last_sample_id(&file.path))
  }

  pub fn insert_sample_features_lowlevel(
    &self,
    id: i64,
    features: &LowlevelFeatures,
  ) -> Result<()> {
    self.exec("insert into sample_features_lowlevel
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38,?39)
on conflict do update
set average_loudness = ?2,
barkbands_kurtosis = ?3,
barkbands_skewness = ?4,
barkbands_spread = ?5,
barkbands_frame_size = ?6,
barkbands = ?7,
dissonance = ?8,
hfc = ?9,
pitch = ?10,
pitch_instantaneous_confidence = ?11,
pitch_salience = ?12,
silence_rate_20db = ?13,
silence_rate_30db = ?14,
silence_rate_60db = ?15,
spectral_centroid = ?16,
spectral_complexity = ?17,
spectral_crest = ?18,
spectral_decrease = ?19,
spectral_energy = ?20,
spectral_energyband_high = ?21,
spectral_energyband_low = ?22,
spectral_energyband_middle_high = ?23,
spectral_energyband_middle_low = ?24,
spectral_flatness_db = ?25,
spectral_flux = ?26,
spectral_kurtosis = ?27,
spectral_rms = ?28,
spectral_rolloff = ?29,
spectral_skewness = ?30,
spectral_spread = ?31,
spectral_strongpeak = ?32,
zerocrossingrate = ?33,
mfcc_frame_size = ?34,
mfcc = ?35,
sccoeffs_frame_size = ?36,
sccoeffs = ?37,
scvalleys_frame_size = ?38,
scvalleys = ?39
where id = ?1",
	      &[&id,
		&features.average_loudness,
		&features.barkbands_kurtosis,
		&features.barkbands_skewness,
		&features.barkbands_spread,
		&features.barkbands.frame_size,
		&features.barkbands.to_vec(),
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
		&features.mfcc.frame_size,
		&features.mfcc.to_vec(),
		&features.sccoeffs.frame_size,
		&features.sccoeffs.to_vec(),
		&features.scvalleys.frame_size,
		&features.scvalleys.to_vec()])?;
    Ok(())
  }

  pub fn insert_sample_features_rhythm(
    &self,
    id: i64,
    features: &RhythmFeatures,
  ) -> Result<()> {
    self.exec(
      "insert into sample_features_rhythm
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)
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
beats_loudness_band_ratio_frame_size = ?16,
beats_loudness_band_ratio = ?17,
histogram = ?18
where id = ?1",
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
        &features.beats_loudness_band_ratio.frame_size,
        &features.beats_loudness_band_ratio.to_vec(),
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
where id = ?1
",
      &[
        &id,
        &features.pitch_after_max_to_before_max_energy_ratio,
        &features.pitch_centroid,
        &features.pitch_max_to_total,
        &features.pitch_min_to_total,
        &features.inharmonicity,
        &features.oddtoevenharmonicenergyratio,
        &features.tristimulus.to_vec(),
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
values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)
on conflict do update
set chords_changes_rate = ?2,
chords_number_rate = ?3,
key_strength = ?4,
tuning_diatonic_strength = ?5,
tuning_equal_tempered_deviation = ?6,
tuning_frequency = ?7,
tuning_nontempered_energy_ratio = ?8,
chords_strength = ?9,
chords_histogram = ?10,
thpcp = ?11,
hpcp_frame_size = ?12,
hpcp = ?13,
chords_key = ?14,
chords_scale = ?15,
key_key = ?16,
key_scale = ?17,
chords_progression = ?18
where id = ?1",
      &[
        &id,
        &features.chords_changes_rate,
        &features.chords_number_rate,
        &features.key_strength,
        &features.tuning_diatonic_strength,
        &features.tuning_equal_tempered_deviation,
        &features.tuning_frequency,
        &features.tuning_nontempered_energy_ratio,
        &features.chords_strength,
        &features.chords_histogram,
        &features.thpcp,
        &features.hpcp.frame_size,
        &features.hpcp.to_vec(),
        &features.chords_key,
        &features.chords_scale,
        &features.key_key,
        &features.key_scale,
        &features.chords_progression,
      ],
    )?;
    Ok(())
  }

  pub fn insert_sample_images(&self, id: i64, images: &Spectrograms) -> Result<()> {
    self.exec(
      "insert into sample_images values (?1,?2,?3,?4,?5,?6,?7)
on conflict do update
set mel_frame_size = ?2,
mel_spec = ?3,
log_frame_size = ?4,
log_spec = ?5,
freq_frame_size = ?6,
freq_spec = ?7
where id = ?1",
      &[
        &id,
        &images.mel_spec.as_ref().map(|s| s.frame_size),
        &images.mel_spec.as_ref().map(|s| s.to_vec()),
        &images.log_spec.as_ref().map(|s| s.frame_size),
        &images.log_spec.as_ref().map(|s| s.to_vec()),
        &images.freq_spec.as_ref().map(|s| s.frame_size),
        &images.freq_spec.as_ref().map(|s| s.to_vec()),
      ],
    )?;
    Ok(())
  }

  pub fn insert_sample_user_notes(&self, _note: &str, _append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_user_tags(&self, _tag: &str, _append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project(&self, name: &str, path: &str, ty: &str) -> Result<()> {
    self.exec(
      "insert into projects (name, path, type) values (?,?,?)",
      &[&name, &path, &ty],
    )?;
    Ok(())
  }

  pub fn insert_project_user_notes(&self, _note: &str, _append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project_user_tags(&self, _tag: &str, _append: bool) -> Result<()> {
    Ok(())
  }

  pub fn query_track(&self, id: i64) -> Result<AudioData> {
    let res =
      self
        .conn
        .query_row("select * from tracks where id = ?", [id], |row| {
          Ok(AudioData {
            path: row.get(1)?,
            filesize: row.get(2)?,
            duration: row.get(3)?,
            channels: row.get(4)?,
            bitrate: row.get(5)?,
            samplerate: row.get(6)?,
            checksum: Some(Checksum::from_hex(row.get::<_, String>(7)?.as_str())),
          })
        })?;
    Ok(res)
  }

  pub fn query_track_tags(&self, id: i64) -> Result<TrackTags> {
    let res =
      self
        .conn
        .query_row("select * from track_tags where id = ?", [id], |row| {
          Ok(TrackTags {
            artist: row.get(1)?,
            title: row.get(2)?,
            album: row.get(3)?,
            genre: row.get(4)?,
            date: row.get(5)?,
            tracknumber: row.get(6)?,
            format: row.get(7)?,
            language: row.get(8)?,
            country: row.get(9)?,
            label: row.get(10)?,
            producer: row.get(11)?,
            engineer: row.get(12)?,
            mixer: row.get(13)?,
          })
        })?;
    Ok(res)
  }

  pub fn query_track_tags_musicbrainz(&self, id: i64) -> Result<MusicbrainzTags> {
    let res = self.conn.query_row(
      "select * from track_tags_musicbrainz where id = ?",
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
          asin: row.get(9)?,
          musicip_puid: row.get(10)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_features_lowlevel(&self, id: i64) -> Result<LowlevelFeatures> {
    let res = self.conn.query_row(
      "select * from track_features_lowlevel where id = ?",
      [id],
      |row| {
        let barkbands = MatrixReal::new(row.get(6)?, row.get(5)?);
        Ok(LowlevelFeatures {
          average_loudness: row.get(1)?,
          barkbands_kurtosis: row.get(2)?,
          barkbands_skewness: row.get(3)?,
          barkbands_spread: row.get(4)?,
          barkbands,
          dissonance: row.get(7)?,
          hfc: row.get(8)?,
          pitch: row.get(9)?,
          pitch_instantaneous_confidence: row.get(10)?,
          pitch_salience: row.get(11)?,
          silence_rate_20db: row.get(12)?,
          silence_rate_30db: row.get(13)?,
          silence_rate_60db: row.get(14)?,
          spectral_centroid: row.get(15)?,
          spectral_complexity: row.get(16)?,
          spectral_crest: row.get(17)?,
          spectral_decrease: row.get(18)?,
          spectral_energy: row.get(19)?,
          spectral_energyband_high: row.get(20)?,
          spectral_energyband_low: row.get(21)?,
          spectral_energyband_middle_high: row.get(22)?,
          spectral_energyband_middle_low: row.get(23)?,
          spectral_flatness_db: row.get(24)?,
          spectral_flux: row.get(25)?,
          spectral_kurtosis: row.get(26)?,
          spectral_rms: row.get(27)?,
          spectral_rolloff: row.get(28)?,
          spectral_skewness: row.get(29)?,
          spectral_spread: row.get(30)?,
          spectral_strongpeak: row.get(31)?,
          zerocrossingrate: row.get(32)?,
          mfcc: MatrixReal::new(row.get(34)?, row.get(33)?),
          sccoeffs: MatrixReal::new(row.get(36)?, row.get(35)?),
          scvalleys: MatrixReal::new(row.get(38)?, row.get(37)?),
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_features_rhythm(&self, id: i64) -> Result<RhythmFeatures> {
    let res = self.conn.query_row(
      "select * from track_features_rhythm where id = ?",
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
          beats_loudness_band_ratio: MatrixReal::new(row.get(16)?, row.get(15)?),
          histogram: row.get(17)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_track_images(&self, id: i64) -> Result<Spectrograms> {
    let res =
      self
        .conn
        .query_row("select * from track_images where id = ?", [id], |row| {
          Ok(Spectrograms {
            mel_spec: Some(MatrixReal::new(row.get(2)?, row.get(1)?)),
            log_spec: Some(MatrixReal::new(row.get(4)?, row.get(3)?)),
            freq_spec: Some(MatrixReal::new(row.get(6)?, row.get(5)?)),
          })
        })?;
    Ok(res)
  }

  pub fn query_sample(&self, id: i64) -> Result<AudioData> {
    let res =
      self
        .conn
        .query_row("select * from samples where id = ?", [id], |row| {
          Ok(AudioData {
            path: row.get(1)?,
            filesize: row.get(2)?,
            duration: row.get(3)?,
            channels: row.get(4)?,
            bitrate: row.get(5)?,
            samplerate: row.get(6)?,
            checksum: Some(Checksum::from_hex(row.get::<_, String>(7)?.as_str())),
          })
        })?;
    Ok(res)
  }

  pub fn query_sample_features_lowlevel(&self, id: i64) -> Result<LowlevelFeatures> {
    let res = self.conn.query_row(
      "select * from sample_features_lowlevel where id = ?",
      [id],
      |row| {
        Ok(LowlevelFeatures {
          average_loudness: row.get(1)?,
          barkbands_kurtosis: row.get(2)?,
          barkbands_skewness: row.get(3)?,
          barkbands_spread: row.get(4)?,
          barkbands: MatrixReal::new(row.get(6)?, row.get(5)?),
          dissonance: row.get(7)?,
          hfc: row.get(8)?,
          pitch: row.get(9)?,
          pitch_instantaneous_confidence: row.get(10)?,
          pitch_salience: row.get(11)?,
          silence_rate_20db: row.get(12)?,
          silence_rate_30db: row.get(13)?,
          silence_rate_60db: row.get(14)?,
          spectral_centroid: row.get(15)?,
          spectral_complexity: row.get(16)?,
          spectral_crest: row.get(17)?,
          spectral_decrease: row.get(18)?,
          spectral_energy: row.get(19)?,
          spectral_energyband_high: row.get(20)?,
          spectral_energyband_low: row.get(21)?,
          spectral_energyband_middle_high: row.get(22)?,
          spectral_energyband_middle_low: row.get(23)?,
          spectral_flatness_db: row.get(24)?,
          spectral_flux: row.get(25)?,
          spectral_kurtosis: row.get(26)?,
          spectral_rms: row.get(27)?,
          spectral_rolloff: row.get(28)?,
          spectral_skewness: row.get(29)?,
          spectral_spread: row.get(30)?,
          spectral_strongpeak: row.get(31)?,
          zerocrossingrate: row.get(32)?,
          mfcc: MatrixReal::new(row.get(34)?, row.get(33)?),
          sccoeffs: MatrixReal::new(row.get(36)?, row.get(35)?),
          scvalleys: MatrixReal::new(row.get(38)?, row.get(37)?),
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_sample_features_rhythm(&self, id: i64) -> Result<RhythmFeatures> {
    let res = self.conn.query_row(
      "select * from sample_features_rhythm where id = ?",
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
          beats_loudness_band_ratio: MatrixReal::new(row.get(16)?, row.get(15)?),
          histogram: row.get(17)?,
        })
      },
    )?;
    Ok(res)
  }

  pub fn query_sample_images(&self, id: i64) -> Result<Spectrograms> {
    let res =
      self
        .conn
        .query_row("select * from sample_images where id = ?", [id], |row| {
          let mut specs = Spectrograms::default();
          for i in [1, 3, 5] {
            let val = match row.get(i) {
              Ok(v) => Some(MatrixReal::new(row.get(i + 1)?, v)),
              Err(_) => None,
            };
            match i {
              1 => specs.mel_spec = val,
              3 => specs.log_spec = val,
              5 => specs.freq_spec = val,
              _ => (),
            }
          }
          Ok(specs)
        })?;
    Ok(res)
  }

  pub fn query_check_file<P: AsRef<Path>>(
    &self,
    path: P,
    checksum: Checksum,
    ty: AudioType,
  ) -> Result<String> {
    let q = format!(
      "select case when path = ?1 and checksum = ?2 then 'found'
when path = ?1 and checksum != ?2 then 'modified'
when path != ?1 and checksum = ?2 then 'moved'
end result
from {}
where path = ?1
or checksum = ?2",
      ty.table_name()
    );
    let res = self
      .conn
      .query_row(
        &q,
        [path.as_ref().to_str().unwrap(), checksum.to_hex().as_str()],
        |row| {
          let row = row.get::<_, String>(0)?;
          Ok(row)
        },
      )
      .unwrap_or("not found".into());
    Ok(res)
  }

  pub fn query(
    &self,
    ty: AudioType,
    by: QueryBy,
    fr: QueryFor,
  ) -> Result<Vec<Vec<rusqlite::types::Value>>> {
    let sql = by.as_query(ty, fr)?;
    let mut stmt = self.conn.prepare(sql.as_str())?;
    let count = stmt.column_count();
    let q = stmt.query_map([], |row| {
      let mut cols = Vec::with_capacity(count);
      for i in 0..count {
        cols.push(row.get::<_, rusqlite::types::Value>(i)?)
      }
      Ok(cols)
    })?;
    let mut rows = Vec::new();
    for i in q {
      rows.push(i?)
    }
    Ok(rows)
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

  pub fn backup<P: AsRef<Path>>(
    &self,
    dst: P,
    progress: Option<fn(Progress)>,
  ) -> Result<()> {
    let mut dst = Connection::open(&dst)?;
    let backup = Backup::new(&self.conn, &mut dst)?;
    backup.run_to_completion(2048, std::time::Duration::from_millis(10), progress)?;
    Ok(())
  }

  pub fn restore<P: AsRef<Path>>(
    &mut self,
    name: DatabaseName,
    src: P,
    progress: Option<fn(Progress)>,
  ) -> Result<()> {
    self.conn.restore(name.into(), src, progress)?;
    Ok(())
  }

  pub fn set_tracer(&mut self, tracer: Option<fn(_: &str)>) {
    self.conn.trace(tracer);
  }

  pub fn set_profiler(
    &mut self,
    profiler: Option<fn(_: &str, _: std::time::Duration)>,
  ) {
    self.conn.profile(profiler);
  }

  pub fn close(self) {
    self.conn.close().unwrap();
  }
}

pub fn print_progress(p: Progress) {
  let current: f32 =
    ((p.pagecount - p.remaining) as f32 / p.pagecount as f32) * 100 as f32;
  println!(
    "progress: {}/{} {}%",
    (p.pagecount - p.remaining),
    p.pagecount,
    current
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  fn new_mem_db() -> Mdb {
    let db = Mdb::new(None).unwrap();
    db.init().unwrap();
    db
  }
  #[test]
  fn test_init() {
    assert!(Mdb::new(None).unwrap().init().is_ok())
  }

  #[test]
  fn test_insert_track_tags() {
    let db = new_mem_db();
    let data = AudioData::default();
    let tags = TrackTags::default();
    let mb_tags = MusicbrainzTags::default();
    assert!(db.insert_track(&data).is_ok());
    assert!(db.insert_track_tags(1, &tags).is_ok());
    assert!(db.insert_track_tags_musicbrainz(1, &mb_tags).is_ok());
  }

  #[test]
  fn test_insert_track_features() {
    let db = new_mem_db();
    let data = AudioData::default();
    let low = LowlevelFeatures::default();
    let ryt = RhythmFeatures::default();
    let tnl = TonalFeatures::default();
    let sfx = SfxFeatures::default();
    let spc = Spectrograms::default();

    assert!(db.insert_track(&data).is_ok());
    assert!(db.insert_track_features_lowlevel(1, &low).is_ok());
    assert!(db.insert_track_features_rhythm(1, &ryt).is_ok());
    assert!(db.insert_track_features_tonal(1, &tnl).is_ok());
    assert!(db.insert_track_features_sfx(1, &sfx).is_ok());
    assert!(db.insert_track_images(1, &spc).is_ok());
  }

  #[test]
  fn test_insert_sample() {
    let db = new_mem_db();
    let data = AudioData::default();
    let low = LowlevelFeatures::default();
    let ryt = RhythmFeatures::default();
    let tnl = TonalFeatures::default();
    let sfx = SfxFeatures::default();
    let spc = Spectrograms::default();

    assert!(db.insert_track(&data).is_ok());
    assert!(db.insert_track_features_lowlevel(1, &low).is_ok());
    assert!(db.insert_track_features_rhythm(1, &ryt).is_ok());
    assert!(db.insert_track_features_tonal(1, &tnl).is_ok());
    assert!(db.insert_track_features_sfx(1, &sfx).is_ok());
    assert!(db.insert_track_images(1, &spc).is_ok());
  }

  #[test]
  fn test_matrix() {
    let vec = VecReal(vec![300.; 10000]);
    let mtx = MatrixReal::new(vec.clone(), 100);
    assert_eq!(vec, mtx.vec);
  }

  #[test]
  fn test_check_file() {
    let db = new_mem_db();
    let checksum = Checksum::rand();
    let d = ("/tmp/file", checksum);
    assert_eq!(
      db.query_check_file(d.0, d.1, AudioType::Track).unwrap(),
      "not found"
    );
    let mut data = AudioData::default();
    data.path = d.0.to_string();
    data.checksum = Some(checksum);
    db.insert_track(&data).unwrap();
    assert_eq!(
      db.query_check_file(d.0, d.1, AudioType::Track).unwrap(),
      "found"
    );
  }
}
