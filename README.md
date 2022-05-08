`mpk` is a *Media Programming Kit* &#x2013; a development kit for digital media, taking lessons learned from software engineering and applying them to creative pursuits. It is a flexible ecosystem designed to organize my workflow involving hardware, software, and data.

This application is intended for artists, hackers, and composers. Batteries are not included.

-   **Status:** `ALPHA` This project is in alpha and will only deal with audio for quite some time since that's the medium I'm most interested in. There are future plans for image/video/text support.


# Table of Contents

1.  [On Digital Audio Workstations](#orge55a03e)
    1.  [DAW Workflows](#orge7fb161)
    2.  [DAWs as Instruments](#orga28e89e)
    3.  [A new paradigm](#orgf05c3fa)
2.  [Usage](#orga0f8584)
    1.  [Installation](#orgd57c302)
    2.  [Initialization](#orgf28206c)
    3.  [Configuration](#org085d4db)
3.  [Dev Dependencies](#org55e2717)


<a id="orge55a03e"></a>

# TODO On Digital Audio Workstations

The DAW (Digital Audio Workstation) has existed for only a moment in the continuum of creative mediums. DAWs started appearing in the late 1970's, thanks to developments made by dedicated engineers such as Max Matthews (AT&T), Hal Chamberlin, and David Cox (MTU). These early DAWs were born from the commercial need for precise control of audio on computers; government funded speech research, commercial telephone research, and University computer music synthesis centers. The very first DAWs were actually used in US Government funded Speech Research for Sonar and the CIA<sup><a id="fnr.1" class="footref" href="#fn.1" role="doc-backlink">1</a></sup>.

Nowadays the DAW is the cornerstone of the studio. It handles audio recording, sequencing, mixing and resource management. With such a powerful tool, there's rarely a need to work outside of the 'box'. With a laptop and some inspiration you can get a lot done.

Like any analog equivalent that has been digitized, users have thoroughly benefited from the convenience and ease of use that the DAW provides. While at UConn, I would often go to the library and make beats on my laptop between classes, sometimes I would even do so while attending class. This level of creative portability was unheard of 30 years ago, and will only get better as mobile device manufacturers develop smaller and more powerful chips.

Another benefit of the DAW is its efficacy in education. Most Music Production classes today can be taught without ever entering an analog studio. Lectures become project templates and students can follow along in their own in-box studios. It has never been easier to learn how to make music.

Indeed, the DAW has been an important evolution in the ways we make music. For all the luxuries it endows us with, there's hardly an argument to be made against the paradigm. Despite this, I will be making one, if only for argument's sake.

-   [Digital sound revolution - Wikipedia](https://en.wikipedia.org/wiki/Digital_sound_revolution)
-   [CCRMA - Music 192B: Week 2, Digital Audio Workstations](https://ccrma.stanford.edu/courses/192b/ProTools-Logic%20Lec.pdf)


<a id="orge7fb161"></a>

## DAW Workflows

Let’s take a moment to consider an elementary DAW workflow. For this example, we will be using Ableton Live. The process is as follows:

-   **Jam:** First, we create. This is the most thrilling part of the process and part of the reason many computer musicians have trouble finishing projects. There is no commitment at this stage and much of what we create won’t make it to the finished product. We’re free to turn all the knobs, make controversial choices, revise, delete, and forget.

-   **Record:** If you ever want to make art, you must make decisions. This is what the next step is all about. We record our Audio and MIDI clips in the Session View and loosely arrange them into sections. This involves trimming the fat from our jam sesh and curating the collection of clips we’re left with. We must be acutely aware of our audience, spectral balance, and song structure, make tough design decisions, and commit to our ideas. This step is deceptively simple, as doing this step well will make the remaining steps flow quite easily and save you a lot of time. Don’t rush it, as moving on to the next step too early will cause you to keep coming back.

-   **Arrange:** Next, we switch to Arrangement View and begin dragging our clips into the timeline. During this process we think in two primary dimensions: length, and depth. In the first dimension, each section must be of appropriate length as well as the total length of our song. In the second, we must assure there is a range of depth over time – both spectral and dynamic. Here we develop cohesion in our song. Sections should flow from one to the next with the intended musical effect, and the song should feel structurally sound (pun intended).

-   **Mix:** Finally, we do the mix. One should take an objective approach to this task. Our goal is to trim unintended spectral artifacts and make our mix sound good when reproduced on different speaker systems.

There is a healthy level of variation in how these steps are performed, but the structure is relatively the same.


<a id="orga28e89e"></a>

## DAWs as Instruments


### Trackers

-   [Mod love | Salon.com](https://www.salon.com/1999/04/29/mod_trackers/)


### Patchers

-   [freesoftware@ircam - A brief history of MAX](https://web.archive.org/web/20090603230029/http://freesoftware.ircam.fr/article.php3?id_article=5)
-   [Miller Puckette, IRCAM - The Patcher](http://msp.ucsd.edu/Publications/icmc88.pdf)


<a id="orgf05c3fa"></a>

## A new paradigm

-   [JACK Audio Connection Kit API](https://jackaudio.org/api/)
-   [FAQ · Wiki · PipeWire](https://gitlab.freedesktop.org/pipewire/pipewire/-/wikis/FAQ)
-   [zita-j2n, zita-n2j - Manpage](http://manpages.ubuntu.com/manpages/bionic/man1/zita-njbridge.1.html)
-   [Non Session Management API](http://non.tuxfamily.org/nsm/API.html)
-   [OpenSoundControl.org](https://ccrma.stanford.edu/groups/osc/index.html)


<a id="orga0f8584"></a>

# Usage

MPK is meant to be used on a Linux box. In this example we'll be using Arch Linux.

MPK also runs on MacOS but some of the project management functionality isn't available. Most notably, you can't run the [NSM](https://new-session-manager.jackaudio.org) server on MacOS, but you can still interact with one remotely. All other features are supported on both platforms.


<a id="orgd57c302"></a>

## Installation

First install the dependencies:

```shell
# install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# only required if you intend to run benchmarks (requires the unstable 'test' feature)
rustup default nightly

# for processing analysis files
curl https://essentia.upf.edu/extractors/essentia-extractors-v2.1_beta2-linux-x86_64.tar.gz -o essentia-extractors.tar.gz
tar -xf essentia-extractors.tar.gz essentia-extractors-v2.1_beta2/streaming_extractor_freesound
mv essentia-extractors-v2.1_beta2/streaming_extractor_freesound /usr/bin/essentia_streaming_extractor_freesound && rm -r essentia-extractors-v2.1_beta2

sudo pacman -S nim alsa-lib jack2 ffmpeg new-session-manager
```

It is recommended to install [Nim](https://nim-lang.org/) so that you can run the build scripts in `config.nims`.

Simply run `nim install` in the project root to install the mpk binaries in `~/.cargo`.

Run `nim help` to see the other commands and flags available.


<a id="orgf28206c"></a>

## Initialization

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


<a id="org085d4db"></a>

## Configuration

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

[sesh]
client_addr = '127.0.0.1:0'

[net]
socket = '127.0.0.1:0'

[engine]
socket = '127.0.0.1:0'

```

Much of the configuration can be overridden by CLI flags but you may want to change some of the default values. Some of the optional settings aren't included in the default file:

-   fs.{`ext_samples`, `ext_tracks`, `ext_projects`, `ext_plugins`, `ext_patches`}
-   `extractor.path`
-   `metro.tic`
-   `metro.toc`
-   net.{`freesound`, `musicbrainz`, `youtube`, `spotify`}


<a id="org55e2717"></a>

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

## Footnotes

<sup><a id="fn.1" class="footnum" href="#fnr.1">1</a></sup> [Digital Audio Workstation - The Evolution](http://www.mtu.com/support/mtudawevolution.htm)