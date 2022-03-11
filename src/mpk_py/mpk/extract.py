import essentia
import essentia.standard as es
from librosa import load, to_mono
import numpy as np
import os
import fnmatch

FILE_EXT = (".mp3", ".flac", ".ogg", ".wav", ".aiff", ".alac", ".aac", ".mp4")


def walk_dir(dir, exts=FILE_EXT):
    for root, dirs, files in os.walk(os.path.expanduser(dir)):
        for base in files:
            if base.lower().endswith(exts):
                yield os.path.join(root, base)


def collect_files(dir, exts=FILE_EXT):
    files = [f for f in walk_dir(dir, exts)]
    print("found %d audio files in %s" % (len(files), dir))
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


def bulk_extract(files, sr=44100, mono=False):
    result = {}
    for f in files:
        try:
            with Extract(f, sr, mono) as extractor:
                print("extracting:", f)
                extractor.metadata()
                extractor.features()
                extractor.mel_spec()
                extractor.freq_spec()
                extractor.log_spec()
                extractor.inverse_spec()
                result.update({f: pool_to_dict(extractor.pool)})
        except Exception as e:
            print(str(e))
    return result


class AudioFile(object):
    def __init__(self, file, sr=44100, mono=False):
        audio, sr = load(file, sr, mono)
        self.path = os.path.expanduser(file)
        self.sr = sr
        self.audio = audio

    def mono(self):
        return to_mono(self.audio)


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
        self.pool.merge(es.MetadataReader(filename=self.path)()[7])
        print("added metadata to pool.")

    def features(self):
        extractor = es.Extractor()
        self.pool.merge(extractor(self.mono()))
        print("added features to pool.")

    def freq_spec(self):
        windowing = es.Windowing(type="blackmanharris62", zeroPadding=2048)
        spectrum = es.Spectrum()
        amp2db = es.UnaryOperator(type="lin2db", scale=2)
        for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
            frame_spec = spectrum(windowing(frame))
        self.pool.add("freq_spec", amp2db(frame_spec))
        print("added freq_spec to pool.")

    def mel_spec(self):
        windowing = es.Windowing(type="blackmanharris62", zeroPadding=2048)
        spectrum = es.Spectrum()
        melbands = es.MelBands(
            numberBands=96, lowFrequencyBound=0, highFrequencyBound=11000
        )
        amp2db = es.UnaryOperator(type="lin2db", scale=2)
        for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
            frame_spec = spectrum(windowing(frame))
            frame_mel = melbands(frame_spec)
        self.pool.add("mel_spec", amp2db(frame_mel))
        print("added mel_spec to pool.")

    def log_spec(self):
        windowing = es.Windowing(type="blackmanharris62", zeroPadding=2048)
        spectrum = es.Spectrum()
        logfreq = es.LogSpectrum(binsPerSemitone=1)
        amp2db = es.UnaryOperator(type="lin2db", scale=2)
        for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
            frame_spec = spectrum(windowing(frame))
            frame_logfreq, _, _ = logfreq(frame_spec)
        self.pool.add("log_spec", amp2db(frame_logfreq))
        print("added log_spec to pool.")

    def inverse_spec(self):
        frameSize = 1024
        hopSize = 512
        fs = 44100
        w = es.Windowing(type="hamming", normalized=False)
        # make sure these are same for MFCC and IDCT computation
        NUM_BANDS = 26
        DCT_TYPE = 2
        LIFTERING = 0
        NUM_MFCCs = 13
        spectrum = es.Spectrum()
        mfcc = es.MFCC(
            numberBands=NUM_BANDS,
            numberCoefficients=NUM_MFCCs,  # make sure you specify first N mfcc: the less, the more lossy (blurry) the smoothed mel spectrum will be
            weighting="linear",  # computation of filter weights done in Hz domain (optional)
            normalize="unit_max",  #  htk filter normaliation to have constant height = 1 (optional)
            dctType=DCT_TYPE,
            logType="log",
            liftering=LIFTERING,
        )  # corresponds to htk default CEPLIFTER = 22
        idct = es.IDCT(
            inputSize=NUM_MFCCs,
            outputSize=NUM_BANDS,
            dctType=DCT_TYPE,
            liftering=LIFTERING,
        )
        all_melbands_smoothed = []
        for frame in es.FrameGenerator(
            self.mono(), frameSize=frameSize, hopSize=hopSize
        ):
            spect = spectrum(w(frame))
            melbands, mfcc_coeffs = mfcc(spect)
            melbands_smoothed = np.exp(
                idct(mfcc_coeffs)
            )  # inverse the log taken in MFCC computation
            all_melbands_smoothed.append(melbands_smoothed)
        # transpose to have it in a better shape
        # we need to convert the list to an essentia.array first (== numpy.array of floats)
        # mfccs = essentia.array(pool['MFCC']).T

        self.pool.add("inverse_spec", essentia.array(all_melbands_smoothed).T)
        print("added inverse_spec to pool.")

    def rhythm(self):
        rhythm_extractor = es.RhythmExtractor2013(method="multifeature")
        bpm, beats, confidence, _, intervals = rhythm_extractor(self.mono())
        self.pool.add("bpm", bpm[0])
        self.pool.add("beats", beats)
        self.pool.add("bpm_confidence", confidence)
        self.pool.add("bpm_intervals", intervals)
        print("added rhythm descriptors to pool.")

    def loudness(self):
        windowing = es.Windowing(type="blackmanharris62", zeroPadding=2048)
        spectrum = es.Spectrum()
        rms = es.RMS()
        hfc = es.HFC()
        for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
            frame_spectrum = spectrum(windowing(frame))
            self.pool.add("rms", rms(frame))
            self.pool.add("rms_spectrum", rms(frame_spectrum))
            self.pool.add("hfc", hfc(frame_spectrum))
        print("added loudness descriptors to pool.")

    def write_json(self, out_file, format="json"):
        print("writing to file: ", out_file)
        es.YamlOutput(filename=out_file, format=format, writeVersion=False)(self.pool)

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
