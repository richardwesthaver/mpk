`mpk` is a *Media Programming Kit* &#x2013; a development kit for digital media, taking lessons learned from software engineering and applying them to creative pursuits. It is a flexible ecosystem designed to organize my workflow involving hardware, software, and data.

This application is intended for artists, hackers, and composers. Batteries are not included.

-   **Status:** `ALPHA` This project is in alpha and will only deal with audio for quite some time since that's the medium I'm most interested in. There are future plans for image/video/text support.


# Table of Contents

1.  [Features](#orgd752b39)
2.  [Usage](#org75118b3)
3.  [Installation](#org6628308)
4.  [Initialization](#org225e233)
5.  [Configuration](#orgb15c76d)
6.  [Dev Dependencies](#org5b7f47c)


<a id="orgd752b39"></a>

# Features


<a id="org75118b3"></a>

# Usage

MPK is meant to be used on a Linux box. In this example we'll be using Arch Linux.

MPK also runs on MacOS but some of the project management functionality isn't available. Most notably, you can't run the [NSM](https://new-session-manager.jackaudio.org) server on MacOS, but you can still interact with one remotely. All other features are supported on both platforms.


<a id="org6628308"></a>

# Installation

First install the dependencies:

```shell
# install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly

# for processing analysis files
curl https://essentia.upf.edu/extractors/essentia-extractors-v2.1_beta2-linux-x86_64.tar.gz -o essentia-extractors.tar.gz
tar -xf essentia-extractors.tar.gz essentia-extractors-v2.1_beta2/{streaming_extractor_freesound,streaming_extractor_music}
mv essentia-extractors-v2.1_beta2/{streaming_extractor_freesound,streaming_extractor_music} /usr/local/bin/ && rm -r essentia-extractors-v2.1_beta2

sudo pacman -S nim alsa-lib jack2 ffmpeg new-session-manager supercollider
```

It is recommended to install [Nim](https://nim-lang.org/) so that you can run the build scripts in `config.nims`.

Simply run `nim install` in the project root to install the mpk binaries in `~/.cargo`.

Run `nim help` to see the other commands and flags available.


<a id="org225e233"></a>

# Initialization

Once the binary is installed run `mpk init` to initialize the app directories at `~/mpk` as well as the database and TOML config file.

```shell
analysis
db
mpk.toml
patches
plugins
samples
sesh
thumpers
tracks
```


<a id="orgb15c76d"></a>

# Configuration

The default `mpk.toml` config file looks like this:

```conf-toml
[fs]
root = '~/mpk'

[db]
path = '~/mpk/db'
mode = 'Fast'
cache_capacity = 1073741824
print_on_drop = false
use_compression = false
compression_factor = 5

[jack]
name = 'mpk'
audio = 'alsa'
midi = 'seq'
device = 'default'
realtime = true
autoconnect = ' '
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

[sesh]
client_addr = '127.0.0.1:0'

[net]
socket = '127.0.0.1:0'

[engine]
socket = '127.0.0.1:0'

[gear]
```

Much of the configuration can be overridden by CLI flags but you may want to change some of the default values. Some of the optional settings aren't included in the default file:

-   fs.{`ext_samples`, `ext_tracks`, `ext_projects`, `ext_plugins`, `ext_patches`}
-   gear.{`octatrack`, `analog_rytm`, `op_1`}
-   metro.{`tic`, `toc`}
-   net.{`freesound`, `musicbrainz`, `youtube`, `spotify`}.{`client_id`, `client_secret`, `redirect_url`}


<a id="org5b7f47c"></a>

# Dev Dependencies

`*` := *use your OS package manager (apt, brew, pacman, etc)*

-   **[Rust](https://www.rust-lang.org/tools/install):** install with [rustup.rs](https://rustup.rs/)
-   **C Compiler:** [GCC](https://gcc.gnu.org/) or [LLVM](https://llvm.org/) \*
-   **[Nim](https://nim-lang.org/):** \*
    -   used as a build tool via [NimScript](https://nim-lang.org/docs/nims.html).
-   **[JACK](https://jackaudio.org/):** \*
-   **[NSM](https://new-session-manager.jackaudio.org):** \*
-   **[Valgrind](https://valgrind.org/):** \*
    -   used to detect issues with FFI memory management.