use std::ffi::{CStr, OsStr, CString};
use std::os::unix::ffi::OsStrExt;
use libc::{c_int, c_char, size_t};
use std::slice;
use std::path::Path;
use mpk_db::{Mdb, TrackTags, MusicbrainzTags, LowlevelFeatures, SfxFeatures, TonalFeatures, RhythmFeatures, Spectograms, Uuid, VecText, VecReal};
use mpk_config::{Config, DbConfig, FsConfig, JackConfig};

#[repr(C)]
#[derive(Clone)]
pub struct CVecReal {
  pub ptr: *const f32,
  pub len: size_t,
}

// impl std::ops::Deref for CVecReal {
//     type Target = [f32];

//     fn deref(&self) -> &[f32] {
//         unsafe { slice::from_raw_parts(self.ptr, self.len) }
//     }
// }

// impl Drop for CVecReal {
//     fn drop(&mut self) {
//       unsafe { Box::from_raw(&mut self.ptr) };
//     }
// }

impl From<VecReal> for CVecReal {
  fn from(v: VecReal) -> Self {
    CVecReal{
      ptr: v.0.as_ptr(),
      len: v.0.len(),
    }
  }
}

impl From<CVecReal> for VecReal {
  fn from(v: CVecReal) -> Self {
    VecReal(unsafe{slice::from_raw_parts(v.ptr, v.len)}.to_vec())
  }
}

#[no_mangle]
pub extern "C" fn mdb_vecreal_new(ptr: *const f32, len: size_t) -> CVecReal {
  CVecReal {
    ptr,
    len,
  }
//  let box_vec = Box::new(vec);
//  Box::into_raw(box_vec)
}

#[no_mangle]
pub extern "C" fn mpk_string_free(ptr: *mut c_char) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mpk_config_new(fs: *mut FsConfig, db: *mut DbConfig, jack: *mut JackConfig) -> *mut Config {
  if !fs.is_null() | !db.is_null() | !jack.is_null() {
    unsafe {
      let fs = &*fs;
      let db = &*db;
      let jack = &*jack;
      Box::into_raw(Box::new(Config::new(fs.to_owned(), db.to_owned(), jack.to_owned()).unwrap()))
    }
  } else {
      Box::into_raw(Box::new(Config::default()))
    }
}

#[no_mangle]
pub extern "C" fn mpk_config_free(ptr: *mut Config) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mpk_config_load(path: *const c_char) -> *mut Config {
  let cstr = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

  Box::into_raw(Box::new(Config::load(p).unwrap()))
}

#[no_mangle]
pub extern "C" fn mpk_config_write(cfg: *const Config, path: *const c_char) {
  let cstr = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
  let cfg = unsafe{&*cfg};
  cfg.write(p).unwrap()
}

#[no_mangle]
pub extern "C" fn mpk_config_build(cfg: *const Config) {
  unsafe{&*cfg}.build().unwrap()
}

#[no_mangle]
pub extern "C" fn mpk_fs_config_new(root: *const c_char) -> *mut FsConfig {
  if !root.is_null() {
    let cstr = unsafe {
      CStr::from_ptr(root)
    };
    let r: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

    Box::into_raw(Box::new(FsConfig::new(r).unwrap()))
  } else {
    Box::into_raw(Box::new(FsConfig::default()))
  }
}

#[no_mangle]
pub extern "C" fn mpk_fs_config_free(ptr: *mut FsConfig) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mpk_fs_config_get_path(cfg: *const FsConfig, path: *const c_char) -> *mut c_char {
  let p = &unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  }.to_str().unwrap();

  let cfg = unsafe{&*cfg};
  let res = cfg.get_path(p).unwrap();
  CString::new(res.as_os_str().as_bytes()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn mpk_db_config_new() -> *mut DbConfig {
  Box::into_raw(Box::new(DbConfig::default()))
}

#[no_mangle]
pub extern "C" fn mpk_db_config_free(ptr: *mut DbConfig) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mpk_db_config_flags(cfg: *const DbConfig) -> c_int {
  let cfg = unsafe {cfg.as_ref().unwrap()};
  cfg.flags().unwrap()
}

#[no_mangle]
pub extern fn mpk_jack_config_new() -> *mut JackConfig {
  Box::into_raw(Box::new(JackConfig::new().unwrap()))
}

#[no_mangle]
pub extern "C" fn mpk_jack_config_free(ptr: *mut JackConfig) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_track_tags_new(artist: *const c_char,
				     title: *const c_char,
				     album: *const c_char,
				     genre: *const c_char,
				     year: i16) -> *mut TrackTags {
  let tags = TrackTags {
    artist: if artist.is_null() {None} else {Some(unsafe {CStr::from_ptr(artist).to_str().unwrap().to_string()})},
    title: if title.is_null() {None} else {Some(unsafe {CStr::from_ptr(title).to_str().unwrap().to_string()})},
    album: if album.is_null() {None} else {Some(unsafe {CStr::from_ptr(album).to_str().unwrap().to_string()})},
    genre: if genre.is_null() {None} else {Some(unsafe {CStr::from_ptr(genre).to_str().unwrap().to_string()})},
    year: if year == 0 {None} else {Some(year)},
  };
  let tags_box = Box::new(tags);
  Box::into_raw(tags_box)
}

#[no_mangle]
pub extern "C" fn mdb_track_tags_free(ptr: *mut TrackTags) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_musicbrainz_tags_new(albumartistid: *const c_char,
					   albumid: *const c_char,
					   albumstatus: *const c_char,
					   albumtype: *const c_char,
					   artistid: *const c_char,
					   releasegroupid: *const c_char,
					   releasetrackid: *const c_char,
					   trackid: *const c_char) -> *mut MusicbrainzTags {
  assert!(!albumartistid.is_null()|!albumid.is_null()
	  |!albumstatus.is_null()|!albumtype.is_null()
	  |!artistid.is_null()|!releasegroupid.is_null()
	  |!releasetrackid.is_null()|!trackid.is_null());
  let tags = MusicbrainzTags {
    albumartistid: Uuid::parse_str(unsafe {CStr::from_ptr(albumartistid).to_str().unwrap()}).unwrap(),
    albumid: Uuid::parse_str(unsafe {CStr::from_ptr(albumid).to_str().unwrap()}).unwrap(),
    albumstatus: unsafe {CStr::from_ptr(albumstatus).to_str().unwrap()}.to_string(),
    albumtype: unsafe {CStr::from_ptr(albumtype).to_str().unwrap()}.to_string(),
    artistid: Uuid::parse_str(unsafe {CStr::from_ptr(artistid).to_str().unwrap()}).unwrap(),
    releasegroupid: Uuid::parse_str(unsafe {CStr::from_ptr(releasegroupid).to_str().unwrap()}).unwrap(),
    releasetrackid: Uuid::parse_str(unsafe {CStr::from_ptr(releasetrackid).to_str().unwrap()}).unwrap(),
    trackid: Uuid::parse_str(unsafe {CStr::from_ptr(trackid).to_str().unwrap()}).unwrap(),
  };
  let tags_box = Box::new(tags);
  Box::into_raw(tags_box)
}

#[no_mangle]
pub extern "C" fn mdb_musicbrainz_tags_free(ptr: *mut MusicbrainzTags) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_lowlevel_features_new(average_loudness: f32,
					    barkbanks_kurtosis: CVecReal,
					    barkbanks_skewness: CVecReal,
					    barkbanks_spread: CVecReal,
					    barkbanks: CVecReal,
					    dissonance: CVecReal,
					    hfc: CVecReal,
					    pitch: CVecReal,
					    pitch_instantaneous_confidence: CVecReal,
					    pitch_salience: CVecReal,
					    silence_rate_20db: CVecReal,
					    silence_rate_30db: CVecReal,
					    silence_rate_60db: CVecReal,
					    spectral_centroid: CVecReal,
					    spectral_complexity: CVecReal,
					    spectral_crest: CVecReal,
					    spectral_decrease: CVecReal,
					    spectral_energy: CVecReal,
					    spectral_energyband_high: CVecReal,
					    spectral_energyband_low: CVecReal,
					    spectral_energyband_middle_high: CVecReal,
					    spectral_energyband_middle_low: CVecReal,
					    spectral_flatness_db: CVecReal,
					    spectral_flux: CVecReal,
					    spectral_kurtosis: CVecReal,
					    spectral_rms: CVecReal,
					    spectral_rolloff: CVecReal,
					    spectral_skewness: CVecReal,
					    spectral_spread: CVecReal,
					    spectral_strongpeak: CVecReal,
					    zerocrossingrate: CVecReal,
					    mfcc: CVecReal,
					    sccoeffs: CVecReal,
					    scvalleys: CVecReal) -> *mut LowlevelFeatures {
  let features = LowlevelFeatures {
    average_loudness,
    barkbanks_kurtosis: VecReal::from(barkbanks_kurtosis),
    barkbanks_skewness: VecReal::from(barkbanks_skewness),
    barkbanks_spread: VecReal::from(barkbanks_spread),
    barkbanks: VecReal::from(barkbanks),
    dissonance: VecReal::from(dissonance),
    hfc: VecReal::from(hfc),
    pitch: VecReal::from(pitch),
    pitch_instantaneous_confidence: VecReal::from(pitch_instantaneous_confidence),
    pitch_salience: VecReal::from(pitch_salience),
    silence_rate_20db: VecReal::from(silence_rate_20db),
    silence_rate_30db: VecReal::from(silence_rate_30db),
    silence_rate_60db: VecReal::from(silence_rate_60db),
    spectral_centroid: VecReal::from(spectral_centroid),
    spectral_complexity: VecReal::from(spectral_complexity),
    spectral_crest: VecReal::from(spectral_crest),
    spectral_decrease: VecReal::from(spectral_decrease),
    spectral_energy: VecReal::from(spectral_energy),
    spectral_energyband_high: VecReal::from(spectral_energyband_high),
    spectral_energyband_low: VecReal::from(spectral_energyband_low),
    spectral_energyband_middle_high: VecReal::from(spectral_energyband_middle_high),
    spectral_energyband_middle_low: VecReal::from(spectral_energyband_middle_low),
    spectral_flatness_db: VecReal::from(spectral_flatness_db),
    spectral_flux: VecReal::from(spectral_flux),
    spectral_kurtosis: VecReal::from(spectral_kurtosis),
    spectral_rms: VecReal::from(spectral_rms),
    spectral_rolloff: VecReal::from(spectral_rolloff),
    spectral_skewness: VecReal::from(spectral_skewness),
    spectral_spread: VecReal::from(spectral_spread),
    spectral_strongpeak: VecReal::from(spectral_strongpeak),
    zerocrossingrate: VecReal::from(zerocrossingrate),
    mfcc: VecReal::from(mfcc),
    sccoeffs: VecReal::from(sccoeffs),
    scvalleys: VecReal::from(scvalleys),
  };
  let features_box = Box::new(features);
  Box::into_raw(features_box)
}

#[no_mangle]
pub extern "C" fn mdb_lowlevel_features_free(ptr: *mut LowlevelFeatures) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_rhythm_features_new(bpm: f32,
					  confidence: f32,
					  onset_rate: f32,
					  beats_loudness: CVecReal,
					  first_peak_bpm: i16,
					  first_peak_spread: f32,
					  first_peak_weight: f32,
					  second_peak_bpm: i16,
					  second_peak_spread: f32,
					  second_peak_weight: f32,
					  beats_position: CVecReal,
					  bpm_estimates: CVecReal,
					  bpm_intervals: CVecReal,
					  onset_times: CVecReal,
					  beats_loudness_band_ratio: CVecReal,
					  histogram:CVecReal) -> *mut RhythmFeatures {
  let features = RhythmFeatures {
    bpm,
    confidence,
    onset_rate,
    beats_loudness: VecReal::from(beats_loudness),
    first_peak_bpm,
    first_peak_spread,
    first_peak_weight,
    second_peak_bpm,
    second_peak_spread,
    second_peak_weight,
    beats_position: VecReal::from(beats_position),
    bpm_estimates: VecReal::from(bpm_estimates),
    bpm_intervals: VecReal::from(bpm_intervals),
    onset_times: VecReal::from(onset_times),
    beats_loudness_band_ratio: VecReal::from(beats_loudness_band_ratio),
    histogram: VecReal::from(histogram),
  };
  let features_box = Box::new(features);
  Box::into_raw(features_box)
}

#[no_mangle]
pub extern "C" fn mdb_rhythm_features_free(ptr: *mut RhythmFeatures) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_sfx_features_new(pitch_after_max_to_before_max_energy_ratio: f32,
				       pitch_centroid: f32,
				       pitch_max_to_total: f32,
				       pitch_min_to_total: f32,
				       inharmonicity: CVecReal,
				       oddtoevenharmonicenergyratio: CVecReal,
				       tristimulus: CVecReal) -> *mut SfxFeatures {
  let features = SfxFeatures {
    pitch_after_max_to_before_max_energy_ratio,
    pitch_centroid,
    pitch_max_to_total,
    pitch_min_to_total,
    inharmonicity: VecReal::from(inharmonicity),
    oddtoevenharmonicenergyratio: VecReal::from(oddtoevenharmonicenergyratio),
    tristimulus: VecReal::from(tristimulus),
  };
  let features_box = Box::new(features);
  Box::into_raw(features_box)
}

#[no_mangle]
pub extern "C" fn mdb_sfx_features_free(ptr: *mut SfxFeatures) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_tonal_features_new(chords_change_rate: f32,
					 chords_number_rate: f32,
					 key_strength: f32,
					 tuning_diatonic_strength: f32,
					 tuning_equal_tempered_deviation: f32,
					 tuning_frequency: f32,
					 tuning_nontempered_tuning_ratio: f32,
					 chords_strength: CVecReal,
					 chords_histogram: CVecReal,
					 thpcp: CVecReal,
					 hpcp: CVecReal,
					 chords_key: *const c_char,
					 chords_scale: *const c_char,
					 key_key: *const c_char,
					 key_scale: *const c_char,
					 chord_progression: *const c_char) -> *mut TonalFeatures {
  let features = TonalFeatures {
    chords_change_rate,
    chords_number_rate,
    key_strength,
    tuning_diatonic_strength,
    tuning_equal_tempered_deviation,
    tuning_frequency,
    tuning_nontempered_tuning_ratio,
    chords_strength: VecReal::from(chords_strength),
    chords_histogram: VecReal::from(chords_histogram),
    thpcp: VecReal::from(thpcp),
    hpcp: VecReal::from(hpcp),
    chords_key: unsafe{CStr::from_ptr(chords_key).to_str().unwrap()}.to_string(),
    chords_scale: unsafe {CStr::from_ptr(chords_scale).to_str().unwrap()}.to_string(),
    key_key: unsafe {CStr::from_ptr(key_key).to_str().unwrap()}.to_string(),
    key_scale: unsafe {CStr::from_ptr(key_scale).to_str().unwrap()}.to_string(),
    chord_progression: VecText(unsafe{CStr::from_ptr(chord_progression).to_str().unwrap()}.split("|").collect::<Vec<_>>().iter().map(|s| s.to_string()).collect()),
  };
  let features_box = Box::new(features);
  Box::into_raw(features_box)
}

#[no_mangle]
pub extern "C" fn mdb_tonal_features_free(ptr: *mut TonalFeatures) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_spectograms_new(mel_spec: CVecReal,
				      log_spec: CVecReal,
				      freq_spec: CVecReal) -> *mut Spectograms {
  let specs = Spectograms {
    mel_spec: VecReal::from(mel_spec),
    log_spec: VecReal::from(log_spec),
    freq_spec: VecReal::from(freq_spec),
  };
  let specs_box = Box::new(specs);
  Box::into_raw(specs_box)
}

#[no_mangle]
pub extern "C" fn mdb_spectograms_free(ptr: *mut Spectograms) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_new(path: *const c_char) -> *mut Mdb {
  let mdb: Mdb = if path.is_null() {
    Mdb::new(None).unwrap()
  } else {
    let cstr = unsafe {CStr::from_ptr(path)};
    let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
    Mdb::new(Some(p)).unwrap()
  };

  let mdb_box: Box<Mdb> = Box::new(mdb);

  Box::into_raw(mdb_box)
}

#[no_mangle]
pub extern "C" fn mdb_free(ptr: *mut Mdb) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub unsafe extern "C" fn mdb_init(db: *const Mdb) {
  db.as_ref().unwrap().init().unwrap()
}

#[no_mangle]
pub extern "C" fn mdb_insert_track(db: *const Mdb, path: *const c_char) -> i64 {
    let c_str = unsafe {
        assert!(!path.is_null());

        CStr::from_ptr(path)
    };
  let str = c_str.to_str().unwrap();
  let mdb = unsafe{&*db};

  mdb.insert_track(&str).unwrap()
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_tags(db: *const Mdb, 
					id: i64,
					tags: *const TrackTags) {
  let tags = unsafe{&*tags};
  let mdb = unsafe{&*db};
  mdb.insert_track_tags(id, &tags).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_tags_musicbrainz(db: *const Mdb,
						    id: i64,
						    tags: *const MusicbrainzTags) {
  let tags = unsafe{&*tags};
  let mdb = unsafe{&*db};
  mdb.insert_track_tags_musicbrainz(id, tags).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_lowlevel(db: *const Mdb,
						     id: i64,
						     features: *const LowlevelFeatures) {
  let features = unsafe{&*features};
  let mdb = unsafe{&*db};
  mdb.insert_track_features_lowlevel(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_rhythm(db: *const Mdb,
						   id: i64,
						   features: *const RhythmFeatures) {
  let features = unsafe{&*features};
  let mdb = unsafe{&*db};
  mdb.insert_track_features_rhythm(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_sfx(db: *const Mdb,
						id: i64,
						features: *const SfxFeatures) {
  let features = unsafe{&*features};
  let mdb = unsafe{&*db};
  mdb.insert_track_features_sfx(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_tonal(db: *const Mdb,
						  id: i64,
						  features: *const TonalFeatures) {
  let features = unsafe{&*features};
  let mdb = unsafe{&*db};
  mdb.insert_track_features_tonal(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_images(db: *const Mdb,
					  id: i64,
					  images: *const Spectograms) {
  let images = unsafe{&*images};
  let mdb = unsafe{&*db};
  mdb.insert_track_images(id, images).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_exec_batch(db: *const Mdb, sql: *const c_char) {
  let sql = unsafe {
    CStr::from_ptr(sql).to_str().unwrap()
  };

  let mdb = unsafe{&*db};
  mdb.exec_batch(sql).unwrap()
}
