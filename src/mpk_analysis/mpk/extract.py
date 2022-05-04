import essentia
import essentia.standard as es
import librosa as lr
import numpy as np
import os
import fnmatch

FILE_EXT = (".mp3", ".flac", ".ogg", ".wav", ".aif", ".alac", ".aac", ".mp4")


def walk_dir(dir, exts=FILE_EXT):
    for root, dirs, files in os.walk(os.path.expanduser(dir)):
        for base in files:
            if base.lower().endswith(exts):
                yield os.path.join(root, base)


def collect_files(dir, exts=FILE_EXT):
    dir = os.path.realpath(dir)
    if os.path.isdir(dir):
        files = [f for f in walk_dir(dir, exts)]
        print("found %d audio files in %s" % (len(files), dir))
    else:
        files = [dir]
        print("found 1 audio file: %s" % dir)
    return files


def isMatch(name, patterns):
    if not patterns:
        return False
    for pattern in patterns:
        if fnmatch.fnmatch(name, pattern):
            return True
    return False


def add_to_dict(dict, keys, value):
    for key in keys[:-1]:
        dict = dict.setdefault(key, {})
    dict[keys[-1]] = value


def pool_to_dict(pool, include_descs=None, ignore_descs=None):
    """a workaround to convert Pool to dict"""
    descs = pool.descriptorNames()
    if include_descs:
        descs = [d for d in descs if isMatch(d, include_descs)]
    if ignore_descs:
        descs = [d for d in descs if not isMatch(d, ignore_descs)]
    result = {}
    for d in descs:
        keys = d.split(".")
        value = pool[d]
        if type(value) is np.ndarray:
            value = value.tolist()
        add_to_dict(result, keys, value)
    return result


def bulk_extract(files, sr=44100, mono=False, descs=None):
    result = {}
    for f in files:
        try:
            with Extract(f, sr, mono) as extractor:
                print("extracting:", f)
                if descs is not None:
                    if "all" in descs:
                        extractor.metadata()
                        extractor.features()
                        extractor.mel_spec()
                        extractor.freq_spec()
                        extractor.log_spec()
                    else:
                        if "info" in descs or "tags" in descs:
                            extractor.metadata()
                            if (
                                "features" in descs
                                or "rhythm" in descs
                                or "lowlevel" in descs
                                or "sfx" in descs
                                or "tonal" in descs
                            ):
                                extractor.features()
                            if "specs" in descs:
                                extractor.mel_spec()
                                extractor.freq_spec()
                                extractor.log_spec()
                            elif "specs" not in descs:
                                if "mel_spec" in descs:
                                    extractor.mel_spec()
                                if "freq_spec" in descs:
                                    extractor.freq_spec()
                                if "log_spec" in descs:
                                    extractor.log_spec()
                else:
                    return result
                result.update({f: pool_to_dict(extractor.pool)})
        except Exception as e:
            print(str(e))
    return result


class AudioFile(object):
    def __init__(self, file, sr=44100, mono=False):
        path = os.path.expanduser(file)
        audio, sr = lr.load(path, sr=sr, mono=mono)
        self.path = os.path.realpath(path)
        self.filesize = os.path.getsize(file)
        self.original_sr = lr.get_samplerate(file)
        self.duration = (
            lr.get_duration(y=audio, sr=self.original_sr) * 1000
        )  # duration in ms
        self.sr = sr
        self.audio = audio

    def mono(self):
        return lr.to_mono(self.audio)


class Extract(AudioFile):
    def __init__(self, file, sr=44100, mono=False):
        super().__init__(file, sr, mono)
        self.pool = essentia.Pool()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        if exc_type:
            print(f"exc_type: {exc_type}")
            print(f"exc_value: {exc_value}")
            print(f"exc_traceback: {exc_traceback}")

    def metadata(self):
        metadata = es.MetadataReader(filename=self.path)()
        self.pool.merge(metadata[7])
        self.pool.add("metadata.tags.filesize", self.filesize)
        self.pool.add("metadata.tags.duration", self.duration)
        self.pool.add("metadata.tags.bitrate", metadata[9])
        self.pool.add("metadata.tags.samplerate", self.original_sr)
        self.pool.add("metadata.tags.channels", metadata[11])
        self.pool.add("metadata.tags.path", self.path)
        print("added metadata to pool.")

    def features(self):
        extractor = es.Extractor()
        self.pool.merge(extractor(self.mono()))
        print("added features to pool.")

    def mel_spec(
        self,
        bands=96,
        lfbound=0,
        hfbound=11000,
        window="hann",
        framesize=2048,
        hopsize=1024,
    ):
        windowing = es.Windowing(type=window)
        spectrum = es.Spectrum()
        melbands = es.MelBands(
            numberBands=bands, lowFrequencyBound=lfbound, highFrequencyBound=hfbound
        )
        amp2db = es.UnaryOperator(type="lin2db", scale=2)
        for frame in es.FrameGenerator(
            self.mono(), frameSize=framesize, hopSize=hopsize
        ):
            frame_spec = spectrum(windowing(frame))
            frame_mel = melbands(frame_spec)
            self.pool.add("mel_spec", amp2db(frame_mel))
        print("added mel_spec to pool.")

    def log_spec(self, window="hann", framesize=2048, hopsize=1024):
        windowing = es.Windowing(type=window)
        spectrum = es.Spectrum()
        logfreq = es.LogSpectrum(binsPerSemitone=1)
        amp2db = es.UnaryOperator(type="lin2db", scale=2)
        for frame in es.FrameGenerator(
            self.mono(), frameSize=framesize, hopSize=hopsize
        ):
            frame_spec = spectrum(windowing(frame))
            frame_logfreq, _, _ = logfreq(frame_spec)
            self.pool.add("log_spec", amp2db(frame_logfreq))
        print("added log_spec to pool.")

    def freq_spec(self, window="hann", framesize=2048, hopsize=1024):
        windowing = es.Windowing(type=window)
        spectrum = es.Spectrum()
        amp2db = es.UnaryOperator(type="lin2db", scale=2)
        for frame in es.FrameGenerator(
            self.mono(), frameSize=framesize, hopSize=hopsize
        ):
            frame_spec = spectrum(windowing(frame))
            self.pool.add("freq_spec", amp2db(frame_spec))
        print("added freq_spec to pool.")

    def rhythm(self):
        rhythm_extractor = es.RhythmExtractor2013(method="multifeature")
        bpm, beats, confidence, _, intervals = rhythm_extractor(self.mono())
        self.pool.add("bpm", bpm)
        self.pool.add("beats", beats)
        self.pool.add("bpm_confidence", confidence)
        self.pool.add("bpm_intervals", intervals)
        print("added rhythm descriptors to pool.")

    def loudness(
        self, window="blackmanharris62", padding=2048, framesize=2048, hopsize=1024
    ):
        windowing = es.Windowing(type=window, zeroPadding=padding)
        spectrum = es.Spectrum()
        rms = es.RMS()
        hfc = es.HFC()
        for frame in es.FrameGenerator(
            self.mono(), frameSize=framesize, hopSize=hopsize
        ):
            frame_spectrum = spectrum(windowing(frame))
            self.pool.add("rms", rms(frame))
            self.pool.add("rms_spectrum", rms(frame_spectrum))
            self.pool.add("hfc", hfc(frame_spectrum))
        print("added loudness descriptors to pool.")

    def write_json(self, out_file):
        print("writing to file: ", out_file)
        es.YamlOutput(filename=out_file, format="json", writeVersion=False)(self.pool)

    def descriptors(self):
        return self.pool.descriptorNames()

    def get(self, key):
        return self.pool.__getitem__(key)

    def remove(self, key):
        self.pool.remove(key)
        print("removed '" + key + "' from pool")

    def clear(self):
        self.pool.clear()
        print("cleared the pool.")
