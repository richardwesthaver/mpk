//! MPK FFI
//!
//! This crate provides FFI-safe bindings for MPK. The cdylib
//! generated from this crate can be used from other C-compatible
//! languages as you see fit.
//!
//! Cbindgen is used in the build.rs script to generate a C header
//! file ('mpk_ffi.h') which is also compatible with C++. This header
//! is in turn utilized by the Python package cffi in build.py to
//! generate Python-compatible bindings (_mpk.c, _mpk.o, and
//! _mpk.cpython-*.so). All of these files can be found in the build
//! directory at the project root after executing 'nim build'.
//!
//! The Python bindings are required by MPK_PY so if you plan to work
//! with the files mpk_extract.py, mpk/extract.py or mpk/lib.py
//! directly, be sure to build the project first. When the `dev` flag
//! is defined (default) the Python bindings will be automatically
//! copied to the appropriate directory.
use libc::{c_char, c_int, size_t};
use mpk_config::{Config, DbConfig, FsConfig, JackConfig};
use mpk_db::{
  AudioData, AudioType, LowlevelFeatures, MatrixReal, Mdb, MusicbrainzTags,
  RhythmFeatures, SfxFeatures, Spectrograms, TonalFeatures, TrackTags, Uuid, VecReal,
  VecText, FileChecksum
};
use mpk_hash::Checksum;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::slice;
use std::str::FromStr;

/// An array of bytes with a fixed length. Reprersents a BLAKE3 hash value
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CChecksum {
  pub ptr: *const u8,
  pub len: size_t,
}

impl From<CChecksum> for Checksum {
  fn from(c: CChecksum) -> Self {
    let b = unsafe {
      assert!(!c.ptr.is_null());
      slice::from_raw_parts(c.ptr, c.len)
    };
    Checksum::hash(b)
  }
}

impl From<Checksum> for CChecksum {
  fn from(r: Checksum) -> Self {
    let b = r.0.as_bytes();
    Self {
      ptr: b.as_ptr(),
      len: b.len(),
    }
  }
}
/// A f32 vector. In C this is represented as a raw pointer and
/// length.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CVecReal {
  pub ptr: *const f32,
  pub len: size_t,
}

impl From<VecReal> for CVecReal {
  fn from(v: VecReal) -> Self {
    CVecReal {
      ptr: v.0.as_ptr(),
      len: v.0.len(),
    }
  }
}

/// From<CVecReal> is implemented for VecReal so we can can convert
/// between these types
impl From<CVecReal> for VecReal {
  fn from(v: CVecReal) -> Self {
    VecReal(unsafe { slice::from_raw_parts(v.ptr, v.len) }.to_vec())
  }
}

/// A flattened f32 matrix. This is exactly the same as CVecReal but
/// defined as a separate struct for type assertions. It is the
/// developer's responsibility to capture the 'frame' or row size and
/// provide it to MatrixReal initializers.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CMatrixReal {
  pub ptr: *const f32,
  pub len: size_t,
}

/// From<CMatrixReal> is also implemented for VecReal since we can not
/// initialize a MatrixReal with only the fields in this struct.
impl From<CMatrixReal> for VecReal {
  fn from(m: CMatrixReal) -> Self {
    VecReal(unsafe { slice::from_raw_parts(m.ptr, m.len) }.to_vec())
  }
}

/// Build a CVecReal from PTR and LEN.
#[no_mangle]
pub extern "C" fn mdb_vecreal_new(ptr: *const f32, len: size_t) -> CVecReal {
  CVecReal { ptr, len }
}

/// Build a CMatrixReal from PTR and LEN.
#[no_mangle]
pub extern "C" fn mdb_matrixreal_new(ptr: *const f32, len: size_t) -> CMatrixReal {
  CMatrixReal { ptr, len }
}

/// Build a Blake3 Checksum from array BYTES of LEN.
#[no_mangle]
pub extern "C" fn mpk_checksum_new(ptr: *const u8, len: size_t) -> *mut CChecksum {
  let cs = CChecksum { ptr, len };
  Box::into_raw(Box::new(cs.into()))
}

/// Build a Blake3 Checksum from PATH given as c_char[].
#[no_mangle]
pub extern "C" fn mpk_checksum_path(path: *const c_char) -> *mut CChecksum {
  let p = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path).to_str().unwrap()
  };
  let cs = Checksum::from_path(p);
  Box::into_raw(Box::new(cs.into()))
}

#[no_mangle]
pub extern "C" fn mpk_checksum_free(ptr: *mut Checksum) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

/// Drop a c_char pointer
#[no_mangle]
pub extern "C" fn mpk_string_free(ptr: *mut c_char) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

/// Build a new Config from inner configs FS, DB, and JACK. Returns a
/// mutable pointer.
#[no_mangle]
pub extern "C" fn mpk_config_new(
  fs: *mut FsConfig,
  db: *mut DbConfig,
  jack: *mut JackConfig,
) -> *mut Config {
  if !fs.is_null() | !db.is_null() | !jack.is_null() {
    unsafe {
      let fs = &*fs;
      let db = &*db;
      let jack = &*jack;
      Box::into_raw(Box::new(
        Config::new(fs.to_owned(), db.to_owned(), jack.to_owned()).unwrap(),
      ))
    }
  } else {
    Box::into_raw(Box::new(Config::default()))
  }
}

/// Drop a Config
#[no_mangle]
pub extern "C" fn mpk_config_free(ptr: *mut Config) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

/// Load a Config from PATH. Returns mutable pointer to Config.
#[no_mangle]
pub extern "C" fn mpk_config_load(path: *const c_char) -> *mut Config {
  let cstr = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

  Box::into_raw(Box::new(Config::load(p).unwrap()))
}

/// Write a Config CFG to PATH.
#[no_mangle]
pub extern "C" fn mpk_config_write(cfg: *const Config, path: *const c_char) {
  let cstr = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
  let cfg = unsafe { &*cfg };
  cfg.write(p).unwrap()
}

/// Build a Config CFG
#[no_mangle]
pub extern "C" fn mpk_config_build(cfg: *const Config) {
  unsafe { &*cfg }.build().unwrap()
}

/// Build a FsConfig from ROOT. Returns a mutable pointer to FsConfig.
#[no_mangle]
pub extern "C" fn mpk_fs_config_new(root: *const c_char) -> *mut FsConfig {
  if !root.is_null() {
    let cstr = unsafe { CStr::from_ptr(root) };
    let r: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

    Box::into_raw(Box::new(FsConfig::new(r).unwrap()))
  } else {
    Box::into_raw(Box::new(FsConfig::default()))
  }
}

/// Drop a FsConfig
#[no_mangle]
pub extern "C" fn mpk_fs_config_free(ptr: *mut FsConfig) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

/// Get a PATH from FsConfig given CFG of type Config. Returns mutable
/// char pointer.
#[no_mangle]
pub extern "C" fn mpk_fs_config_get_path(
  cfg: *const Config,
  path: *const c_char,
) -> *mut c_char {
  let p = &unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  }
  .to_str()
  .unwrap();

  let cfg = unsafe { &*cfg };
  let res = cfg.fs.get_path(p).unwrap();
  CString::new(res.as_os_str().as_bytes()).unwrap().into_raw()
}

/// Build a DbConfig. Returns mutable pointer to DbConfig.
#[no_mangle]
pub extern "C" fn mpk_db_config_new() -> *mut DbConfig {
  Box::into_raw(Box::new(DbConfig::default()))
}

/// Drop a DbConfig
#[no_mangle]
pub extern "C" fn mpk_db_config_free(ptr: *mut DbConfig) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

/// Get the current DbConfig flags from CFG of type Config. Returns a
/// single int.
#[no_mangle]
pub extern "C" fn mpk_db_config_flags(cfg: *const Config) -> c_int {
  let cfg = unsafe { cfg.as_ref().unwrap() };
  cfg.db.flags().unwrap()
}

/// Get the DbConfig path from CFG of type Config. Returns a mutable
/// char pointer.
#[no_mangle]
pub extern "C" fn mpk_db_config_path(cfg: *const Config) -> *mut c_char {
  let cfg = unsafe { &*cfg };
  let res = cfg.db.path().unwrap();
  CString::new(res.as_os_str().as_bytes()).unwrap().into_raw()
}

/// Build JackConfig. Returns mutable JackConfig pointer.
#[no_mangle]
pub extern "C" fn mpk_jack_config_new() -> *mut JackConfig {
  Box::into_raw(Box::new(JackConfig::new().unwrap()))
}

/// Drop a JackConfig.
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
pub extern "C" fn mdb_audio_data_new(
  path: *const c_char,
  filesize: usize,
  duration: f64,
  channels: u8,
  bitrate: u32,
  samplerate: u32,
) -> *mut AudioData {
  let path = unsafe { CStr::from_ptr(path).to_str().unwrap().to_string() };
  let checksum = Checksum::from_path(path.as_str());
  let data = AudioData {
    path,
    filesize: Some(filesize),
    duration: Some(duration),
    channels: Some(channels),
    bitrate: Some(bitrate),
    samplerate: Some(samplerate),
    checksum: Some(FileChecksum::from(checksum)),
  };
  let box_data = Box::new(data);
  Box::into_raw(box_data)
}

#[no_mangle]
pub extern "C" fn mdb_audio_data_free(ptr: *mut AudioData) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn mdb_track_tags_new(
  artist: *const c_char,
  title: *const c_char,
  album: *const c_char,
  genre: *const c_char,
  date: *const c_char,
  tracknumber: *const c_char,
  format: *const c_char,
  language: *const c_char,
  country: *const c_char,
  label: *const c_char,
  producer: *const c_char,
  engineer: *const c_char,
  mixer: *const c_char,
) -> *mut TrackTags {
  let tags = TrackTags {
    artist: if artist.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(artist).to_str().unwrap().to_string() })
    },
    title: if title.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(title).to_str().unwrap().to_string() })
    },
    album: if album.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(album).to_str().unwrap().to_string() })
    },
    genre: if genre.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(genre).to_str().unwrap().to_string() })
    },
    date: if date.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(date).to_str().unwrap().to_string() })
    },
    tracknumber: if tracknumber.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(tracknumber).to_str().unwrap().to_string() })
    },
    format: if format.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(format).to_str().unwrap().to_string() })
    },
    language: if language.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(language).to_str().unwrap().to_string() })
    },
    country: if country.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(country).to_str().unwrap().to_string() })
    },
    label: if label.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(label).to_str().unwrap().to_string() })
    },
    producer: if producer.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(producer).to_str().unwrap().to_string() })
    },
    engineer: if engineer.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(engineer).to_str().unwrap().to_string() })
    },
    mixer: if mixer.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(mixer).to_str().unwrap().to_string() })
    },
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
pub extern "C" fn mdb_musicbrainz_tags_new(
  albumartistid: *const c_char,
  albumid: *const c_char,
  albumstatus: *const c_char,
  albumtype: *const c_char,
  artistid: *const c_char,
  releasegroupid: *const c_char,
  releasetrackid: *const c_char,
  trackid: *const c_char,
  asin: *const c_char,
  musicip_puid: *const c_char,
) -> *mut MusicbrainzTags {
  let tags = MusicbrainzTags {
    albumartistid: if albumartistid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(albumartistid).to_str().unwrap() })
          .unwrap(),
      )
    },
    albumid: if albumid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(albumid).to_str().unwrap() }).unwrap(),
      )
    },
    albumstatus: if albumstatus.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(albumstatus).to_str().unwrap() }.to_string())
    },
    albumtype: if albumtype.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(albumtype).to_str().unwrap() }.to_string())
    },
    artistid: if artistid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(artistid).to_str().unwrap() }).unwrap(),
      )
    },
    releasegroupid: if releasegroupid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(releasegroupid).to_str().unwrap() })
          .unwrap(),
      )
    },
    releasetrackid: if releasetrackid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(releasetrackid).to_str().unwrap() })
          .unwrap(),
      )
    },
    trackid: if trackid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(trackid).to_str().unwrap() }).unwrap(),
      )
    },
    asin: if asin.is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(asin).to_str().unwrap() }.to_string())
    },
    musicip_puid: if musicip_puid.is_null() {
      None
    } else {
      Some(
        Uuid::parse_str(unsafe { CStr::from_ptr(musicip_puid).to_str().unwrap() })
          .unwrap(),
      )
    },
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
pub extern "C" fn mdb_lowlevel_features_new(
  average_loudness: f64,
  barkbands_kurtosis: CVecReal,
  barkbands_skewness: CVecReal,
  barkbands_spread: CVecReal,
  barkbands_frame_size: usize,
  barkbands: CMatrixReal,
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
  mfcc_frame_size: usize,
  mfcc: CMatrixReal,
  sccoeffs_frame_size: usize,
  sccoeffs: CMatrixReal,
  scvalleys_frame_size: usize,
  scvalleys: CMatrixReal,
) -> *mut LowlevelFeatures {
  let features = LowlevelFeatures {
    average_loudness,
    barkbands_kurtosis: VecReal::from(barkbands_kurtosis),
    barkbands_skewness: VecReal::from(barkbands_skewness),
    barkbands_spread: VecReal::from(barkbands_spread),
    barkbands: MatrixReal::new(barkbands.into(), barkbands_frame_size),
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
    mfcc: MatrixReal::new(mfcc.into(), mfcc_frame_size),
    sccoeffs: MatrixReal::new(sccoeffs.into(), sccoeffs_frame_size),
    scvalleys: MatrixReal::new(scvalleys.into(), scvalleys_frame_size),
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
pub extern "C" fn mdb_rhythm_features_new(
  bpm: f64,
  confidence: f64,
  onset_rate: f64,
  beats_loudness: CVecReal,
  first_peak_bpm: f64,
  first_peak_spread: f64,
  first_peak_weight: f64,
  second_peak_bpm: f64,
  second_peak_spread: f64,
  second_peak_weight: f64,
  beats_position: CVecReal,
  bpm_estimates: CVecReal,
  bpm_intervals: CVecReal,
  onset_times: CVecReal,
  beats_loudness_band_ratio_frame_size: usize,
  beats_loudness_band_ratio: CMatrixReal,
  histogram: CVecReal,
) -> *mut RhythmFeatures {
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
    beats_loudness_band_ratio: MatrixReal::new(
      beats_loudness_band_ratio.into(),
      beats_loudness_band_ratio_frame_size,
    ),
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
pub extern "C" fn mdb_sfx_features_new(
  pitch_after_max_to_before_max_energy_ratio: f64,
  pitch_centroid: f64,
  pitch_max_to_total: f64,
  pitch_min_to_total: f64,
  inharmonicity: CVecReal,
  oddtoevenharmonicenergyratio: CVecReal,
  tristimulus: CMatrixReal,
) -> *mut SfxFeatures {
  let features = SfxFeatures {
    pitch_after_max_to_before_max_energy_ratio,
    pitch_centroid,
    pitch_max_to_total,
    pitch_min_to_total,
    inharmonicity: VecReal::from(inharmonicity),
    oddtoevenharmonicenergyratio: VecReal::from(oddtoevenharmonicenergyratio),
    tristimulus: MatrixReal::new(tristimulus.into(), 3),
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
pub extern "C" fn mdb_tonal_features_new(
  chords_changes_rate: f64,
  chords_number_rate: f64,
  key_strength: f64,
  tuning_diatonic_strength: f64,
  tuning_equal_tempered_deviation: f64,
  tuning_frequency: f64,
  tuning_nontempered_energy_ratio: f64,
  chords_strength: CVecReal,
  chords_histogram: CVecReal,
  thpcp: CVecReal,
  hpcp_frame_size: usize,
  hpcp: CMatrixReal,
  chords_key: *const c_char,
  chords_scale: *const c_char,
  key_key: *const c_char,
  key_scale: *const c_char,
  chords_progression: *const c_char,
) -> *mut TonalFeatures {
  let features = TonalFeatures {
    chords_changes_rate,
    chords_number_rate,
    key_strength,
    tuning_diatonic_strength,
    tuning_equal_tempered_deviation,
    tuning_frequency,
    tuning_nontempered_energy_ratio,
    chords_strength: VecReal::from(chords_strength),
    chords_histogram: VecReal::from(chords_histogram),
    thpcp: VecReal::from(thpcp),
    hpcp: MatrixReal::new(hpcp.into(), hpcp_frame_size),
    chords_key: unsafe { CStr::from_ptr(chords_key).to_str().unwrap() }.to_string(),
    chords_scale: unsafe { CStr::from_ptr(chords_scale).to_str().unwrap() }.to_string(),
    key_key: unsafe { CStr::from_ptr(key_key).to_str().unwrap() }.to_string(),
    key_scale: unsafe { CStr::from_ptr(key_scale).to_str().unwrap() }.to_string(),
    chords_progression: VecText(
      unsafe { CStr::from_ptr(chords_progression).to_str().unwrap() }
        .split("|")
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.to_string())
        .collect(),
    ),
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
pub extern "C" fn mdb_spectrograms_new(
  mel_frame_size: size_t,
  mel_spec: CMatrixReal,
  log_frame_size: size_t,
  log_spec: CMatrixReal,
  freq_frame_size: size_t,
  freq_spec: CMatrixReal,
) -> *mut Spectrograms {
  let mel_spec = if mel_frame_size.eq(&0) {
    None
  } else {
    Some(MatrixReal::new(mel_spec.into(), mel_frame_size))
  };

  let log_spec = if log_frame_size.eq(&0) {
    None
  } else {
    Some(MatrixReal::new(log_spec.into(), log_frame_size))
  };
  let freq_spec = if freq_frame_size.eq(&0) {
    None
  } else {
    Some(MatrixReal::new(freq_spec.into(), freq_frame_size))
  };
  let specs = Spectrograms {
    mel_spec,
    log_spec,
    freq_spec,
  };
  let specs_box = Box::new(specs);
  Box::into_raw(specs_box)
}

#[no_mangle]
pub extern "C" fn mdb_spectrograms_free(ptr: *mut Spectrograms) {
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
    let cstr = unsafe { CStr::from_ptr(path) };
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
pub extern "C" fn mdb_insert_track(db: *const Mdb, data: *const AudioData) -> i64 {
  let data = unsafe { &*data };
  let mdb = unsafe { &*db };
  mdb.insert_track(data).unwrap()
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_tags(
  db: *const Mdb,
  id: i64,
  tags: *const TrackTags,
) {
  let tags = unsafe { &*tags };
  let mdb = unsafe { &*db };
  mdb.insert_track_tags(id, tags).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_tags_musicbrainz(
  db: *const Mdb,
  id: i64,
  tags: *const MusicbrainzTags,
) {
  let tags = unsafe { &*tags };
  let mdb = unsafe { &*db };
  mdb.insert_track_tags_musicbrainz(id, tags).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_lowlevel(
  db: *const Mdb,
  id: i64,
  features: *const LowlevelFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_track_features_lowlevel(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_rhythm(
  db: *const Mdb,
  id: i64,
  features: *const RhythmFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_track_features_rhythm(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_sfx(
  db: *const Mdb,
  id: i64,
  features: *const SfxFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_track_features_sfx(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_features_tonal(
  db: *const Mdb,
  id: i64,
  features: *const TonalFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_track_features_tonal(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_images(
  db: *const Mdb,
  id: i64,
  images: *const Spectrograms,
) {
  let images = unsafe { &*images };
  let mdb = unsafe { &*db };
  mdb.insert_track_images(id, images).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_sample(db: *const Mdb, data: *const AudioData) -> i64 {
  let data = unsafe { &*data };
  let mdb = unsafe { &*db };
  mdb.insert_sample(&data).unwrap()
}

#[no_mangle]
pub extern "C" fn mdb_insert_sample_features_lowlevel(
  db: *const Mdb,
  id: i64,
  features: *const LowlevelFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_sample_features_lowlevel(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_sample_features_rhythm(
  db: *const Mdb,
  id: i64,
  features: *const RhythmFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_sample_features_rhythm(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_sample_features_sfx(
  db: *const Mdb,
  id: i64,
  features: *const SfxFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_sample_features_sfx(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_sample_features_tonal(
  db: *const Mdb,
  id: i64,
  features: *const TonalFeatures,
) {
  let features = unsafe { &*features };
  let mdb = unsafe { &*db };
  mdb.insert_sample_features_tonal(id, features).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_sample_images(
  db: *const Mdb,
  id: i64,
  images: *const Spectrograms,
) {
  let images = unsafe { &*images };
  let mdb = unsafe { &*db };
  mdb.insert_sample_images(id, images).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_update_path(
  db: *const Mdb,
  path: *const c_char,
  ty: *const c_char,
) {
  let cstr = unsafe { CStr::from_ptr(path) };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
  let t = AudioType::from_str(unsafe {
    assert!(!ty.is_null());
    CStr::from_ptr(ty).to_str().unwrap()
  })
  .unwrap();
  let c = Checksum::from_path(p);
  let db = unsafe { &*db };
  db.update_path(p, c, t).unwrap()
}

#[no_mangle]
pub extern "C" fn mdb_query_check_file(
  db: *const Mdb,
  path: *const c_char,
  ty: *const c_char,
) -> *mut c_char {
  let cstr = unsafe { CStr::from_ptr(path) };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
  let t = AudioType::from_str(unsafe {
    assert!(!ty.is_null());
    CStr::from_ptr(ty).to_str().unwrap()
  })
  .unwrap();
  let c = Checksum::from_path(p);
  let db = unsafe { &*db };
  let res = db.query_check_file(p, c, t).unwrap();
  CString::new(res.as_bytes()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn mdb_exec_batch(db: *const Mdb, sql: *const c_char) {
  let sql = unsafe { CStr::from_ptr(sql).to_str().unwrap() };
  let mdb = unsafe { &*db };
  mdb.exec_batch(sql).unwrap()
}
