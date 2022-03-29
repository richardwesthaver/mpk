- [Status](#orge1711cf)
- [Dependencies](#org90ee879)
- [Crates](#orgce440b6)
  - [`mpk`](#org7e8dcef)
  - [`mpk_config`](#org08684f6)
  - [`mpk_db`](#org3e0a7dd)
  - [`mpk_py`](#orgc58e2d7)
  - [`mpk_ffi`](#orga24c1e7)
  - [`mpk_audio`](#orgafe7201)
  - [`mpk_flate`](#org4968284)
  - [`mpk_codec`](#orgd93fd52)
  - [`mpk_gear`](#org582f3eb)
  - [`mpk_jack`](#org9d87792)
  - [`mpk_sesh`](#org2fc3b1d)
  - [`mpk_midi`](#orgdd99b89)
  - [`mpk_http`](#org59e1028)
  - [`mpk_osc`](#org3b7d40c)
  - [`mpk_hash`](#org44c2dbf)

`mpk` is a *Media Programming Kit* &#x2013; a development kit for digital media, taking lessons learned from software engineering and applying them to creative pursuits. It is a flexible ecosystem designed to organize my workflow involving hardware, software, and data.

*Batteries are not included.*


<a id="orge1711cf"></a>

# Status

This project is quite young and will only deal with audio for quite some time since that's the medium I'm most interested in. There are future plans for image/video support followed by VR/AR. The core APIs are written in Rust but there are bindings for C and Python (see [mpk\_ffi](#orga24c1e7)).

Right now my focus is on the SQLite<sup><a id="fnr.1" class="footref" href="#fn.1" role="doc-backlink">1</a></sup> database and cataloging libraries of audio tracks and samples. The database is designed to capture as much information as possible with minimal user configuration and input. The libraries have a fairly flat directory structure &#x2013; a far cry from most music library programs which encourage a deeply nested structure (`Tracks -> Artist -> Album -> track.wav`).

Once I'm happy with the database I'll work on the MIDI module ([mpk\_midi](#orgdd99b89)), add playback/record/transcode capabilities ([mpk\_audio](#orgafe7201)/[mpk\_codec](#orgd93fd52)), and then get started on session management functionality ([mpk\_sesh](#org2fc3b1d)).


<a id="org90ee879"></a>

# Dependencies

-   **[Rust](https://www.rust-lang.org/tools/install):** [rustup.rs](https://rustup.rs/)
-   **[Python](https://www.python.org/)3.9:** \*
-   **C Compiler:** [GCC](https://gcc.gnu.org/) or [LLVM](https://llvm.org/) \*
-   **[essentia](https://essentia.upf.edu/):** try a `pip install` from the [github repo](https://github.com/MTG/essentia), if that doesn't work you will need to [install from source](https://essentia.upf.edu/installing.html). If you have issues just contact me.
-   **[numpy](https://numpy.org/):** you will need a version <1.22, for example `pip install numpy==1.21.5`.
-   **[SQLite](https://www.sqlite.org/index.html):** \*
-   **[JACK](https://jackaudio.org/):** \*
-   <span class="underline">Dev Dependencies</span>
    -   **[poetry](https://python-poetry.org/):** `pip` or \*
    -   **[black](https://black.readthedocs.io/en/stable/):** `pip` or \*
    -   **[Nim](https://nim-lang.org/):** \*
        -   used as a build tool via [NimScript](https://nim-lang.org/docs/nims.html).
    -   **[Valgrind](https://valgrind.org/):** \*
        -   used to detect issues with FFI memory management.

`*` &#x2013; /use your OS package manager (apt, brew, pacman, etc)


<a id="orgce440b6"></a>

# Crates


<a id="org7e8dcef"></a>

## `mpk`

The MPK binary providing CLI access to the library features.

    mpk 0.1.0
    ellis <ellis@rwest.io>
    media programming kit
    
    USAGE:
        mpk [OPTIONS] <SUBCOMMAND>
    
    OPTIONS:
        -c, --cfg <CFG>     [default: ~/mpk/mpk.toml]
            --db-trace      enable DB tracing
            --db-profile    enable DB profiling
        -h, --help          Print help information
        -V, --version       Print version information
    
    SUBCOMMANDS:
        init      Initialize MPK
        play      Play an audio file
        run       Run a service
        save      Save a session
        db        Interact with the database
        info      Print info
        pack      Package resources [.tar.zst]
        unpack    Unpackage resources [.tar.zst]
        quit      Shutdown services
        help      Print this message or the help of the given subcommand(s)


<a id="org08684f6"></a>

## `mpk_config`

User configuration with read/write support for TOML (typically from `mpk.toml`). Used to initialize other modules at runtime (for example `DbConfig` for `Mdb::new_with_config`).


<a id="org3e0a7dd"></a>

## `mpk_db`

The `Mdb` struct provides an API to the underlying SQLite database which works with the custom structs defined in [types.rs](src/mpk_db/src/types.rs).

-   **Tables**
    -   tracks
        
            id integer,
            path text,
            filesize integer,
            duration integer,
            channels integer,
            bitrate integer,
            samplerate integer,
            checksum text,
            updated datetime
    -   track\_tags
        
            track_id integer,
            artist text,
            title text,
            album text,
            genre text,
            date text,
            tracknumber text,
            format text,
            language text,
            country text,
            label text,
            producer text,
            engineer text,
            mixer text,
    -   track\_tags\_musicbrainz
        
            track_id integer,
            albumartistid text,
            albumid text,
            albumstatus text,
            albumtype text,
            artistid text,
            releasegroupid text,
            releasetrackid text,
            trackid text,
            asin text,
            musicip_puid text
    -   track\_features\_lowlevel
        
            track_id integer,
            average_loudness real,
            barkbands_kurtosis blob,
            barkbands_skewness blob,
            barkbands_spread blob,
            barkbands_frame_size integer,
            barkbands blob,
            dissonance blob,
            hfc blob,
            pitch blob,
            pitch_instantaneous_confidence blob,
            pitch_salience blob,
            silence_rate_20db blob,
            silence_rate_30db blob,
            silence_rate_60db blob,
            spectral_centroid blob,
            spectral_complexity blob,
            spectral_crest blob,
            spectral_decrease blob,
            spectral_energy blob,
            spectral_energyband_high blob,
            spectral_energyband_low blob,
            spectral_energyband_middle_high blob,
            spectral_energyband_middle_low blob,
            spectral_flatness_db blob,
            spectral_flux blob,
            spectral_kurtosis blob,
            spectral_rms blob,
            spectral_rolloff blob,
            spectral_skewness blob,
            spectral_spread blob,
            spectral_strongpeak blob,
            zerocrossingrate blob,
            mfcc_frame_size integer,
            mfcc blob,
            sccoeffs_frame_size integer,
            sccoeffs blob,
            scvalleys_frame_size integer,
            scvalleys blob,
    -   track\_features\_rhythm
        
            track_id integer,
            bpm real,
            confidence real,
            onset_rate real,
            beats_loudness blob,
            first_peak_bpm integer,
            first_peak_spread real,
            first_peak_weight real,
            second_peak_bpm integer,
            second_peak_spread real,
            second_peak_weight real,
            beats_position blob,
            bpm_estimates blob,
            bpm_intervals blob,
            onset_times blob,
            beats_loudness_band_ratio_frame_size integer,
            beats_loudness_band_ratio blob,
            histogram blob
    -   track\_features\_sfx
        
            track_id integer,
            pitch_after_max_to_before_max_energy_ratio real,
            pitch_centroid real,
            pitch_max_to_total real,
            pitch_min_to_total real,
            inharmonicity blob,
            oddtoevenharmonicenergyratio blob,
            tristimulus blob
    -   track\_features\_tonal
        
            track_id integer,
            chords_changes_rate real,
            chords_number_rate real,
            key_strength real,
            tuning_diatonic_strength real,
            tuning_equal_tempered_deviation real,
            tuning_frequency real,
            tuning_nontempered_energy_ratio real,
            chords_strength blob,
            chords_histogram blob,
            thpcp blob,
            hpcp_frame_size integer,
            hpcp blob,
            chords_key text,
            chords_scale text,
            key_key text,
            key_scale text,
            chords_progression blob,
    -   track\_images
        
            track_id integer,
            mel_frame_size integer,
            mel_spec blob,
            log_frame_size integer,
            log_spec blob,
            freq_frame_size integer,
            freq_spec blob
    -   track\_user\_data
        
            track_id integer,
            user_tags text,
            notes text,
    -   samples
        
            id integer,
            path text,
            filesize integer,
            duration integer,
            channels integer,
            bitrate integer,
            samplerate integer,
            checksum text
    -   sample\_features\_lowlevel
        
            sample_id integer,
            average_loudness real,
            barkbands_kurtosis blob,
            barkbands_skewness blob,
            barkbands_spread blob,
            barkbands_frame_size integer,
            barkbands blob,
            dissonance blob,
            hfc blob,
            pitch blob,
            pitch_instantaneous_confidence blob,
            pitch_salience blob,
            silence_rate_20db blob,
            silence_rate_30db blob,
            silence_rate_60db blob,
            spectral_centroid blob,
            spectral_complexity blob,
            spectral_crest blob,
            spectral_decrease blob,
            spectral_energy blob,
            spectral_energyband_high blob,
            spectral_energyband_low blob,
            spectral_energyband_middle_high blob,
            spectral_energyband_middle_low blob,
            spectral_flatness_db blob,
            spectral_flux blob,
            spectral_kurtosis blob,
            spectral_rms blob,
            spectral_rolloff blob,
            spectral_skewness blob,
            spectral_spread blob,
            spectral_strongpeak blob,
            zerocrossingrate blob,
            mfcc_frame_size integer,
            mfcc blob,
            sccoeffs_frame_size integer,
            sccoeffs blob,
            scvalleys_frame_size integer,
            scvalleys blob
    -   sample\_features\_rhythm
        
            sample_id integer,
            bpm real,
            confidence real,
            onset_rate real,
            beats_loudness blob,
            first_peak_bpm integer,
            first_peak_spread real,
            first_peak_weight real,
            second_peak_bpm integer,
            second_peak_spread real,
            second_peak_weight real,
            beats_position blob,
            bpm_estimates blob,
            bpm_intervals blob,
            onset_times blob,
            beats_loudness_band_ratio_frame_size integer,
            beats_loudness_band_ratio blob,
            histogram blob
    -   sample\_features\_sfx
        
            sample_id integer,
            pitch_after_max_to_before_max_energy_ratio real,
            pitch_centroid real,
            pitch_max_to_total real,
            pitch_min_to_total real,
            inharmonicity blob,
            oddtoevenharmonicenergyratio blob,
            tristimulus blob
    -   sample\_features\_tonal
        
            sample_id integer,
            chords_changes_rate real,
            chords_number_rate real,
            key_strength real,
            tuning_diatonic_strength real,
            tuning_equal_tempered_deviation real,
            tuning_frequency real,
            tuning_nontempered_energy_ratio real,
            chords_strength blob,
            chords_histogram blob,
            thpcp blob,
            hpcp_frame_size integer,
            hpcp blob,
            chords_key text,
            chords_scale text,
            key_key text,
            key_scale text,
            chords_progression blob
    -   sample\_images
        
            sample_id integer,
            mel_frame_size integer,
            mel_spec blob,
            log_frame_size integer,
            log_spec blob,
            freq_frame_size integer,
            freq_spec blob
    -   sample\_user\_data
        
            sample_id integer,
            user_tags text,
            notes text,
    -   projects
        
            id integer,
            name text,
            path text,
            type text
    -   project\_user\_data
        
            project_id integer,
            user_tags text,
            notes text


<a id="orgc58e2d7"></a>

## `mpk_py`

The MIR<sup><a id="fnr.2" class="footref" href="#fn.2" role="doc-backlink">2</a></sup> tool (`mpk_extract.py`) uses Python as a bridge between Essentia<sup><a id="fnr.3" class="footref" href="#fn.3" role="doc-backlink">3</a></sup> for feature extraction and the MPK database. There are a huge amount of features stored in the database (*97* at time of writing), but the feature set will be reduced in future iterations as I find the features which are most useful to me. As for the extraction algorithms, My plan is to RiiR<sup><a id="fnr.4" class="footref" href="#fn.4" role="doc-backlink">4</a></sup> and reduce DB size by applying zstd<sup><a id="fnr.5" class="footref" href="#fn.5" role="doc-backlink">5</a></sup> compression.

```artist
	 +------------------+                             
	 |  mpk_extract.py  |                            _____________        
	 +--------+---------+                           /             \       +--------+  +-----------------+
		  |                                 +-}| Extract(f[0]) |----->| POOL[0]|  |       DB        |
		  |                                /    \____________ /       |  -  -  |  | -  -  -  -  -  -|
		  |              +---------+      /      _____________    |   | POOL[1]|  |        |        |
	  +---------------+      |         |     /      /             \       |  -  -  |  |                 |
	  |collect_files()|{---->| [files] |----X-----}| Extract(f[1]) |----->|        |  | tracks | samples|
	  +---------------+      |         |     \      \____________ /       |[ .... ]|  |                 |
	       /    \            +---------+      \      _____________    |   |        |  |        |        |
	      /      \                             \    /             \       |  -  -  |  |                 |
	     /        \                             +-}| Extract(f[N]) |----->| POOL[N]|  |        |        |
	    o          o                                \____________ /       +--------+  +-----------------+
+-----------------+-----------------+                                             |                ^
|                 |                 |                                             v                |
|     tracks      |     samples     |                                       +------------+         |
|                 |                 |                                       | insert_*() |---------+
+-----------------+-----------------+                                       +------------+  

```


<a id="orga24c1e7"></a>

## `mpk_ffi`

C-compatible MPK FFI with C-header and python binding generators.


<a id="orgafe7201"></a>

## `mpk_audio`

The audio module leverages [cpal](https://github.com/RustAudio/cpal) and [rodio](https://github.com/RustAudio/rodio) for audio playback and recording. It provides high-level standalone tools with simple use cases such as playing an audio file on disk and isn't designed for low-level DSP.

-   **Modules**
    -   **metro:** a convenient metronome
    -   **chain:** sample chainer<sup><a id="fnr.6" class="footref" href="#fn.6" role="doc-backlink">6</a></sup>


<a id="org4968284"></a>

## `mpk_flate`

Zstd compression and Tar archival utilities.


<a id="orgd93fd52"></a>

## `mpk_codec`

Audio file encoding and decoding.


<a id="org582f3eb"></a>

## `mpk_gear`

MPK interface for hardware devices connected via USB.

-   Elektron Octatrack MKII
-   Elektron Analog Rytm MKII
-   DSI Prophet Rev2
-   Korg SV-1


<a id="org9d87792"></a>

## `mpk_jack`

MPK interface for JACK.


<a id="org2fc3b1d"></a>

## `mpk_sesh`

MPK session management. Inspired by NSM


<a id="orgdd99b89"></a>

## `mpk_midi`

MPK MIDI interface supporting real-time processing, encoding/decoding, and Sysex patching.


<a id="org59e1028"></a>

## `mpk_http`

HTTP client APIs for MPK. Currently includes [freesound.org](https://freesound.org/), [musicbrainz.org](https://musicbrainz.org/), and [coverartarchive.org](https://coverartarchive.org/).


<a id="org3b7d40c"></a>

## `mpk_osc`

OSC (Open Sound Control) APIs for MPK. Includes an API client for [NSM](https://new-session-manager.jackaudio.org/) (New/Non-Session Manager).


<a id="org44c2dbf"></a>

## `mpk_hash`

[BLAKE3](https://github.com/BLAKE3-team/BLAKE3) hashing utilities (for file checksums)

## Footnotes

<sup><a id="fn.1" class="footnum" href="#fnr.1">1</a></sup> [SQLite Home Page](https://www.sqlite.org/index.html)

<sup><a id="fn.2" class="footnum" href="#fnr.2">2</a></sup> [Music information retrieval - Wikipedia](https://en.wikipedia.org/wiki/Music_information_retrieval)

<sup><a id="fn.3" class="footnum" href="#fnr.3">3</a></sup> [Essentia - Music Technology Group - Universitat Pompeu Fabra](https://essentia.upf.edu/)

<sup><a id="fn.4" class="footnum" href="#fnr.4">4</a></sup> [ansuz - /random/RIIR](https://transitiontech.ca/random/RIIR)

<sup><a id="fn.5" class="footnum" href="#fnr.5">5</a></sup> [Zstandard - Real-time data compression algorithm](http://facebook.github.io/zstd/)

<sup><a id="fn.6" class="footnum" href="#fnr.6">6</a></sup> [GitHub - KaiDrange/OctaChainer](https://github.com/KaiDrange/OctaChainer)