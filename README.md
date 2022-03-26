- [Status](#org735f2cc)
- [Dependencies](#org71204a7)
- [Modules](#org5b14af7)
  - [`mpk`](#org8f6b62e)
  - [`mpk_config`](#org31fd1da)
  - [`mpk_db`](#org074bf7b)
  - [`mpk_py`](#orge3447a9)
  - [`mpk_ffi`](#org12e364d)
  - [`mpk_audio`](#org3c82103)
  - [`mpk_flate`](#orgce98567)
  - [`mpk_codec`](#orgd2a5a14)
  - [`mpk_gear`](#org51eaace)
  - [`mpk_jack`](#org7364907)
  - [`mpk_sesh`](#orgbf988e6)
  - [`mpk_midi`](#orga28102b)
  - [`mpk_http`](#org89a4627)
  - [`mpk_osc`](#orgc6c575d)

`mpk` is my *Media Programming Kit* &#x2013; a development kit for digital media, taking lessons learned from the software industry and applying them to creative pursuits. It is a flexible ecosystem designed to organize my workflow involving hardware, software, and data. Batteries are **not** included.


<a id="org735f2cc"></a>

# Status

This project is quite young and will only deal with audio for quite some time since that's the medium I'm most interested in. There are future plans for image/video support followed by VR/AR. The core APIs are written in Rust but there are bindings for C and Python (see [mpk<sub>ffi</sub>](#org12e364d)).

Right now my focus is on the SQLite<sup><a id="fnr.1" class="footref" href="#fn.1" role="doc-backlink">1</a></sup> database and cataloging libraries of audio tracks and samples. The database is designed to capture as much information as possible with minimal user configuration and input. The libraries have a fairly flat directory structure &#x2013; a far cry from most music library programs which encourage a deeply nested structure (`Tracks -> Artist -> Album -> track.wav`).

Once I'm happy with the database I'll work on the MIDI module ([mpk<sub>midi</sub>](#orga28102b)), add playback/record/transcode capabilities ([mpk<sub>audio</sub>](#org3c82103)/[mpk<sub>codec</sub>](#orgd2a5a14)), and then get started on session management functionality ([mpk<sub>sesh</sub>](#orgbf988e6)).

mpk<sub>audio</sub> also includes a metronome and sample chainer<sup><a id="fnr.2" class="footref" href="#fn.2" role="doc-backlink">2</a></sup> which I plan to tweak over the next few weeks since they have well-defined requirements.


<a id="org71204a7"></a>

# Dependencies

-   **[Rust](https://www.rust-lang.org/tools/install):** use `cargo` to install `mpk`
-   **Python3:** use `pip` to install `mpk_extract` and `mpk` python package.
    -   your mileage may vary on Py3.10. If installation fails try it on 3.9.
-   <span class="underline">Dev Dependencies</span>
    -   **essentia:** try a `pip install` from the [github repo](https://github.com/MTG/essentia), if that doesn't work you will need to [install from source](https://essentia.upf.edu/installing.html). If you have issues just contact me.
    -   **numpy:** you will need a version <1.22, for example `pip install numpy==1.21.5`.
    -   **poetry:** `pip` or OS package manager
    -   **black:** `pip` or OS package manager
    -   **Nim:** OS package manager
        -   used as a build tool via [NimScript](https://nim-lang.org/docs/nims.html).
    -   **C Compiler:** GCC or LLVM
        -   **Valgrind:** OS package manager
            -   used to detect issues with FFI memory management.
    -   **SQLite:** OS package manager
        -   required by `mpk_db`
    -   **JACK:** OS package manager
        -   required by `mpk_jack`


<a id="org5b14af7"></a>

# Modules


<a id="org8f6b62e"></a>

## `mpk`

The MPK binary providing CLI access to the library features.


<a id="org31fd1da"></a>

## `mpk_config`

User configuration with read/write support for TOML (typically from `mpk.toml`). Used to initialize other modules at runtime (for example `DbConfig` for `Mdb::new_with_config`).


<a id="org074bf7b"></a>

## `mpk_db`

The `Mdb` struct provides an API to the underlying SQLite database which works with the custom structs defined in `src/types.rs`.


<a id="orge3447a9"></a>

## `mpk_py`

The MIR<sup><a id="fnr.3" class="footref" href="#fn.3" role="doc-backlink">3</a></sup> tool (`mpk_extract.py`) uses Python as a bridge between Essentia<sup><a id="fnr.4" class="footref" href="#fn.4" role="doc-backlink">4</a></sup> for feature extraction and the MPK database. There are a huge amount of features stored in the database (*97* at time of writing), but the feature set will be reduced in future iterations as I find the features which are most useful to me. As for the extraction algorithms, My plan is to RiiR<sup><a id="fnr.5" class="footref" href="#fn.5" role="doc-backlink">5</a></sup> and reduce DB size by applying zstd<sup><a id="fnr.6" class="footref" href="#fn.6" role="doc-backlink">6</a></sup> compression.

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


<a id="org12e364d"></a>

## `mpk_ffi`

C-compatible MPK FFI with C-header and python binding generators.


<a id="org3c82103"></a>

## `mpk_audio`

The audio module leverages [cpal](https://github.com/RustAudio/cpal) and [rodio](https://github.com/RustAudio/rodio) for audio playback and recording. It provides high-level standalone tools with simple use cases such as playing an audio file on disk and isn't designed for low-level DSP.


<a id="orgce98567"></a>

## `mpk_flate`

Zstd compression and Tar archival utilities.


<a id="orgd2a5a14"></a>

## TODO `mpk_codec`

Audio file encoding and decoding.


<a id="org51eaace"></a>

## TODO `mpk_gear`

MPK interface for hardware devices connected via USB.

-   Elektron Octatrack MKII
-   Elektron Analog Rytm MKII
-   DSI Prophet Rev2
-   Korg SV-1


<a id="org7364907"></a>

## TODO `mpk_jack`

MPK interface for JACK.


<a id="orgbf988e6"></a>

## TODO `mpk_sesh`

MPK session management. Inspired by NSM<sup><a id="fnr.7" class="footref" href="#fn.7" role="doc-backlink">7</a></sup>.


<a id="orga28102b"></a>

## TODO `mpk_midi`

MPK MIDI interface supporting real-time processing, encoding/decoding, and Sysex patching.


<a id="org89a4627"></a>

## TODO `mpk_http`

HTTP client APIs for MPK. Currently includes freesound.org and musicbrainz.org.


<a id="orgc6c575d"></a>

## TODO `mpk_osc`

OSC APIs for MPK. Includes an API client for NSM (Non-Session Manager).

## Footnotes

<sup><a id="fn.1" class="footnum" href="#fnr.1">1</a></sup> [SQLite Home Page](https://www.sqlite.org/index.html)

<sup><a id="fn.2" class="footnum" href="#fnr.2">2</a></sup> [GitHub - KaiDrange/OctaChainer](https://github.com/KaiDrange/OctaChainer)

<sup><a id="fn.3" class="footnum" href="#fnr.3">3</a></sup> [Music information retrieval - Wikipedia](https://en.wikipedia.org/wiki/Music_information_retrieval)

<sup><a id="fn.4" class="footnum" href="#fnr.4">4</a></sup> [Essentia - Music Technology Group - Universitat Pompeu Fabra](https://essentia.upf.edu/)

<sup><a id="fn.5" class="footnum" href="#fnr.5">5</a></sup> [ansuz - /random/RIIR](https://transitiontech.ca/random/RIIR)

<sup><a id="fn.6" class="footnum" href="#fnr.6">6</a></sup> [Zstandard - Real-time data compression algorithm](http://facebook.github.io/zstd/)

<sup><a id="fn.7" class="footnum" href="#fnr.7">7</a></sup> [Non Session Manager](http://non.tuxfamily.org/wiki/Non%20Session%20Manager)