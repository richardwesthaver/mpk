- [Status](#org9772c9c)
- [On Digital Audio Workstations](#org46a47df)
  - [The Music Programmer](#orgf88103b)
    - [Trackers](#org8b897d3)
    - [Patchers](#org3f6e407)
  - [A new paradigm](#orgb462600)
- [Usage](#org6d9f0cf)
  - [Installation](#orgfc3780d)
  - [Initialization](#org017c815)
  - [Configuration](#orgf280898)
  - [The Database](#org7919f2c)
    - [Sync](#orgb398812)
    - [Query](#org2e1e262)
    - [Backup/Restore](#org0e9ce4b)
  - [Projects](#orgb332b61)
- [Dependencies](#orge460ec1)
- [Crates](#org784f7f8)
  - [`mpk`](#orgbb4b85f)
  - [`mpk_config`](#org26f8953)
  - [`mpk_db`](#org108b09c)
  - [`mpk_py`](#org1c15aef)
  - [`mpk_ffi`](#org4cdbefa)
  - [`mpk_audio`](#org864b1dd)
  - [`mpk_flate`](#org832dd75)
  - [`mpk_codec`](#org20f4c9a)
  - [`mpk_gear`](#orga6631a4)
  - [`mpk_jack`](#orgbe031d4)
  - [`mpk_sesh`](#org039e09a)
  - [`mpk_midi`](#org5e00c25)
  - [`mpk_http`](#org6fa4c12)
  - [`mpk_osc`](#org2fe0da3)
  - [`mpk_hash`](#orga84da88)

`mpk` is a *Media Programming Kit* &#x2013; a development kit for digital media, taking lessons learned from software engineering and applying them to creative pursuits. It is a flexible ecosystem designed to organize my workflow involving hardware, software, and data.

*Batteries are not included.*


<a id="org9772c9c"></a>

# Status

This project is quite young and will only deal with audio for quite some time since that's the medium I'm most interested in. There are future plans for image/video support followed by VR/AR. The core APIs are written in Rust but there are bindings for C and Python (see [mpk\_ffi](#org4cdbefa)).

Right now my focus is on the SQLite<sup><a id="fnr.1" class="footref" href="#fn.1" role="doc-backlink">1</a></sup> database and cataloging libraries of audio tracks and samples. The database is designed to capture as much information as possible with minimal user configuration and input. The libraries have a fairly flat directory structure &#x2013; a far cry from most music library programs which encourage a deeply nested structure (`Tracks -> Artist -> Album -> track.wav`).

Once I'm happy with the database I'll work on the MIDI module ([mpk\_midi](#org5e00c25)), add playback/record/transcode capabilities ([mpk\_audio](#org864b1dd)/[mpk\_codec](#org20f4c9a)), and then get started on session management functionality ([mpk\_sesh](#org039e09a)).


<a id="org46a47df"></a>

# TODO On Digital Audio Workstations

The DAW (Digital Audio Workstation) has existed for only a moment in the continuum of creative mediums. DAWs started appearing in the late 1970's, thanks to developments made by dedicated engineers such as Max Matthews (AT&T), Hal Chamberlin, and David Cox (MTU). These early DAWs were born from the commercial need for precise control of audio on computers; government funded speech research, commercial telephone research, and University computer music synthesis centers. The very first DAWs were actually used in US Government funded Speech Research for Sonar and the CIA<sup><a id="fnr.2" class="footref" href="#fn.2" role="doc-backlink">2</a></sup>.

Nowadays the DAW is the cornerstone of the studio. It handles audio recording, sequencing, mixing and resource management. With such a powerful tool, there's rarely a need to work outside of the 'box'. With a laptop and some inspiration you can get a lot done.

Like any analog equivalent that has been digitized, users have thoroughly benefited from the convenience and ease of use that the DAW provides. While at UConn, I would often go to the library and make beats on my laptop between classes, sometimes I would even do so while attending class. This level of creative portability was unheard of 30 years ago, and will only get better as mobile device manufacturers develop smaller and more powerful chips.

Another benefit of the DAW is its efficacy in education. Most Music Production classes today can be taught without ever entering an analog studio. Lectures become project templates and students can follow along in their own in-box studios. It has never been easier to learn how to make music.

Indeed, the DAW has been an important evolution in the ways we make music. For all the luxuries it endows us with, there's hardly an argument to be made against the paradigm. Despite this, I do think there is one: The DAW prevents Music Producers from being Music Programmers.


<a id="orgf88103b"></a>

## The Music Programmer

-   [Digital sound revolution - Wikipedia](https://en.wikipedia.org/wiki/Digital_sound_revolution)
-   [CCRMA - Music 192B: Week 2, Digital Audio Workstations](https://ccrma.stanford.edu/courses/192b/ProTools-Logic%20Lec.pdf)


<a id="org8b897d3"></a>

### Trackers

-   [Mod love | Salon.com](https://www.salon.com/1999/04/29/mod_trackers/)


<a id="org3f6e407"></a>

### Patchers

-   [freesoftware@ircam - A brief history of MAX](https://web.archive.org/web/20090603230029/http://freesoftware.ircam.fr/article.php3?id_article=5)
-   [Miller Puckette, IRCAM - The Patcher](http://msp.ucsd.edu/Publications/icmc88.pdf)


<a id="orgb462600"></a>

## A new paradigm

-   [JACK Audio Connection Kit API](https://jackaudio.org/api/)
-   [FAQ · Wiki · PipeWire](https://gitlab.freedesktop.org/pipewire/pipewire/-/wikis/FAQ)
-   [zita-j2n, zita-n2j - Manpage](http://manpages.ubuntu.com/manpages/bionic/man1/zita-njbridge.1.html)
-   [Non Session Management API](http://non.tuxfamily.org/nsm/API.html)
-   [OpenSoundControl.org](https://ccrma.stanford.edu/groups/osc/index.html)


<a id="org6d9f0cf"></a>

# Usage

MPK is meant to be used on a Linux box. In this example we'll be using Arch Linux.

MPK also runs on MacOS but some of the project management functionality isn't available. Most notably, you can't run the [NSM](https://new-session-manager.jackaudio.org) server on MacOS, but you can still interact with one remotely. All other features are supported on both platforms.


<a id="orgfc3780d"></a>

## Installation

First install the dependencies:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo pacman -S nim gcc sqlite jack2 new-session-manager
# cargo install paru or use another AUR installer
paru -S python39 # 3.10 is not supported yet

python -m pip install git+https://github.com/MTG/essentia.git
```

It is recommended to install [Nim](https://nim-lang.org/) so that you can run the build scripts in `config.nims`.

Simply run `nim install` in the project root to install the mpk binary in `~/.cargo` and the python modules in the default location (usually `/usr/local/lib/python3.9/site-packages`).

Run `nim help` to see the other commands and flags available.


<a id="org017c815"></a>

## Initialization

Once the binary is installed run `mpk init` to initialize the app directories at `~/mpk` as well as the database and TOML config file.

```shell
mpk.db
mpk.toml
patches
plugins
projects
samples
tracks
```


<a id="orgf280898"></a>

## Configuration

The default `mpk.toml` config file looks like this:

```conf-toml
[fs]
root = '~/mpk'

[db]
path = '~/mpk/mpk.db'
flags = [
    'readwrite',
    'create',
    'nomutex',
    'uri',
]
trace = false
profile = false

[jack]
name = 'mpk'
audio = 'alsa'
midi = 'seq'
device = 'default'
realtime = true
auto = ' '
temp = false
rate = 44100
period = 1024
n_periods = 2

[metro]
bpm = 120
time_sig = [
    4,
    4,
]

[extractor]
descriptors = ['mel_spec']
mono = false
sample_rate = 44100
windowing = 'hann'
frame_size = 2048
hop_size = 1024
mel_bands = 96
lf_bound = 0
hf_bound = 11000
```

XC Much of the configuration can be overridden by CLI flags but you may want to change some of the default values. Some of the optional settings aren't included in the default file:

-   **fs.{`ext_samples`, `ext_tracks`, `ext_projects`, `ext_plugins`, `ext_patches`}:** external directories
-   **`extractor.path`:** path to the `mpk_extract.py` script
-   **`metro.tic`:** audio file to play on metro downbeats
-   **`metro.toc`:** audio file to play on metro upbeats


<a id="org7919f2c"></a>

## The Database

The database is able to store a wide variety of audio descriptors including metadata, lowlevel features, and full spectrograms. Storing all of the descriptors for every audio file comes at a cost though:

-   *time* to process each file
-   *space* to store these descriptors in a single-file database

This is why the only optional descriptor enabled by default is the Mel Spectrogram. You can add additional descriptors via CLI or just add them to the config file. The full list includes the following:

    'lowlevel'
    'rhythm'
    'sfx'
    'tonal'
    'spectrograms'
    'mel_spec'
    'log_spec'
    'freq_spec'
    'all'

The samples and tracks tables always get populated, as well as track\_tags and track\_tags\_musicbrainz since they don't require heavy processing.

You can interact with the database via CLI:

    mpk-db 
    Interact with the database
    
    USAGE:
        mpk db <SUBCOMMAND>
    
    OPTIONS:
        -h, --help    Print help information
    
    SUBCOMMANDS:
        backup     
        help       Print this message or the help of the given subcommand(s)
        query      Query DB
        restore    
        sync       Sync resources with DB


<a id="orgb398812"></a>

### Sync

You can populate the database using `mpk db sync` which executes the `mpk_extract.py` script and updates any files that have changed based on checksums.


<a id="org2e1e262"></a>

### Query

Use `mpk db query` to query the database directly. You can get formatted output with the built-in commands. Raw queries are also supported but the output for Blobs are summarized with a length in bytes.


<a id="org0e9ce4b"></a>

### Backup/Restore

Use `mpk db backup` to backup the current database and `mpk db restore` to restore from a backup.


<a id="orgb332b61"></a>

## TODO Projects


<a id="orge460ec1"></a>

# Dependencies

`*` := *use your OS package manager (apt, brew, pacman, etc)*

-   **[Rust](https://www.rust-lang.org/tools/install):** install with [rustup.rs](https://rustup.rs/)
-   **[Python](https://www.python.org/)3.9:** use [pyenv](https://github.com/pyenv/pyenv) to switch Python version globally. Python 3.10 is not supported. \*
-   **C Compiler:** [GCC](https://gcc.gnu.org/) or [LLVM](https://llvm.org/) \*
-   **[Nim](https://nim-lang.org/):** \*
    -   used as a build tool via [NimScript](https://nim-lang.org/docs/nims.html).
-   **[SQLite](https://www.sqlite.org/index.html):** \*
-   **[JACK](https://jackaudio.org/):** \*
-   **[NSM](https://new-session-manager.jackaudio.org):** \*
-   <span class="underline">Dev Dependencies</span>
    -   **[poetry](https://python-poetry.org/):** `pip` or \*
    -   **[black](https://black.readthedocs.io/en/stable/):** `pip` or \*
    -   **[essentia](https://essentia.upf.edu/):** try a `pip install` from the [github repo](https://github.com/MTG/essentia), if that doesn't work you will need to [install from source](https://essentia.upf.edu/installing.html). If you have issues just contact me.
    -   **[Valgrind](https://valgrind.org/):** \*
        -   used to detect issues with FFI memory management.


<a id="org784f7f8"></a>

# Crates


<a id="orgbb4b85f"></a>

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


<a id="org26f8953"></a>

## `mpk_config`

User configuration with read/write support for TOML (typically from `mpk.toml`). Used to initialize other modules at runtime (for example `DbConfig` for `Mdb::new_with_config`).


<a id="org108b09c"></a>

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


<a id="org1c15aef"></a>

## `mpk_py`

The MIR<sup><a id="fnr.3" class="footref" href="#fn.3" role="doc-backlink">3</a></sup> tool (`mpk_extract.py`) uses Python as a bridge between Essentia<sup><a id="fnr.4" class="footref" href="#fn.4" role="doc-backlink">4</a></sup> for feature extraction and the MPK database. There are a huge amount of features stored in the database (*97* at time of writing), but the feature set will be reduced in future iterations as I find the features which are most useful to me. As for the extraction algorithms, My plan is to RWiR<sup><a id="fnr.5" class="footref" href="#fn.5" role="doc-backlink">5</a></sup> and reduce DB size by applying zstd<sup><a id="fnr.6" class="footref" href="#fn.6" role="doc-backlink">6</a></sup> compression.

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


<a id="org4cdbefa"></a>

## `mpk_ffi`

C-compatible MPK FFI with C-header and python binding generators.


<a id="org864b1dd"></a>

## `mpk_audio`

The audio module leverages [cpal](https://github.com/RustAudio/cpal) and [rodio](https://github.com/RustAudio/rodio) for audio playback and recording. It provides high-level standalone tools with simple use cases such as playing an audio file on disk and isn't designed for low-level DSP.

-   **Modules**
    -   **metro:** a convenient metronome
    -   **chain:** sample chainer<sup><a id="fnr.7" class="footref" href="#fn.7" role="doc-backlink">7</a></sup>


<a id="org832dd75"></a>

## `mpk_flate`

Zstd compression and Tar archival utilities.


<a id="org20f4c9a"></a>

## `mpk_codec`

Audio file encoding and decoding.


<a id="orga6631a4"></a>

## `mpk_gear`

MPK interface for hardware devices connected via USB.

-   Elektron Octatrack MKII
-   Elektron Analog Rytm MKII
-   DSI Prophet Rev2
-   Korg SV-1


<a id="orgbe031d4"></a>

## `mpk_jack`

MPK interface for JACK.


<a id="org039e09a"></a>

## `mpk_sesh`

MPK session management. Inspired by NSM


<a id="org5e00c25"></a>

## `mpk_midi`

MPK MIDI interface supporting real-time processing, encoding/decoding, and Sysex patching.


<a id="org6fa4c12"></a>

## `mpk_http`

HTTP client APIs for MPK. Currently includes [freesound.org](https://freesound.org/), [musicbrainz.org](https://musicbrainz.org/), and [coverartarchive.org](https://coverartarchive.org/).


<a id="org2fe0da3"></a>

## `mpk_osc`

OSC (Open Sound Control) APIs for MPK. Includes an API client for [NSM](https://new-session-manager.jackaudio.org/) (New/Non-Session Manager).


<a id="orga84da88"></a>

## `mpk_hash`

[BLAKE3](https://github.com/BLAKE3-team/BLAKE3) hashing utilities (for file checksums)

## Footnotes

<sup><a id="fn.1" class="footnum" href="#fnr.1">1</a></sup> [SQLite Home Page](https://www.sqlite.org/index.html)

<sup><a id="fn.2" class="footnum" href="#fnr.2">2</a></sup> [Digital Audio Workstation - The Evolution](http://www.mtu.com/support/mtudawevolution.htm)

<sup><a id="fn.3" class="footnum" href="#fnr.3">3</a></sup> [Music information retrieval - Wikipedia](https://en.wikipedia.org/wiki/Music_information_retrieval)

<sup><a id="fn.4" class="footnum" href="#fnr.4">4</a></sup> [Essentia - Music Technology Group - Universitat Pompeu Fabra](https://essentia.upf.edu/)

<sup><a id="fn.5" class="footnum" href="#fnr.5">5</a></sup> [ansuz - /random/RIIR](https://transitiontech.ca/random/RIIR)

<sup><a id="fn.6" class="footnum" href="#fnr.6">6</a></sup> [Zstandard - Real-time data compression algorithm](http://facebook.github.io/zstd/)

<sup><a id="fn.7" class="footnum" href="#fnr.7">7</a></sup> [GitHub - KaiDrange/OctaChainer](https://github.com/KaiDrange/OctaChainer)