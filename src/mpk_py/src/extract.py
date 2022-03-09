import essentia
import essentia.standard as es
from librosa import load, to_mono
import numpy as np
import os

class AudioFile(object):
  def __init__(self, file, sr=44100, mono=False):
    audio,sr = load(file, sr, mono)
    self.path = os.path.realpath(file)
    self.sr = sr
    self.audio = audio

  def mono(self):
    return to_mono(self.audio)
    
class Extract(AudioFile):
  def __init__(self, file, sr=44100, mono=False):
    super().__init__(file, sr, mono)
    self.pool = essentia.Pool()

  def metadata(self):
    self.pool.merge(es.MetadataReader(filename=self.path)()[7])
    print("added metadata to pool.")

  def features(self):
    extractor = es.Extractor()
    self.pool.merge(extractor(self.mono()))
    print("added features to pool.")

  def freq_spec(self):
    windowing = es.Windowing(type='blackmanharris62', zeroPadding=2048)
    spectrum = es.Spectrum()
    amp2db = es.UnaryOperator(type='lin2db', scale=2)
    for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
      frame_spec = spectrum(windowing(frame))
    self.pool.add('freq_spec', amp2db(frame_spec))
    print("added freq_spec to pool.")
    
  def mel_spec(self):
    windowing = es.Windowing(type='blackmanharris62', zeroPadding=2048)
    spectrum = es.Spectrum()
    melbands = es.MelBands(numberBands=96, lowFrequencyBound=0, highFrequencyBound=11000)
    amp2db = es.UnaryOperator(type='lin2db', scale=2)
    for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
      frame_spec = spectrum(windowing(frame))
      frame_mel = melbands(frame_spec)
    self.pool.add('mel_spec', amp2db(frame_mel))
    print("added mel_spec to pool.")

  def log_spec(self):
    windowing = es.Windowing(type='blackmanharris62', zeroPadding=2048)
    spectrum = es.Spectrum()
    logfreq = es.LogSpectrum(binsPerSemitone=1)
    amp2db = es.UnaryOperator(type='lin2db', scale=2)
    for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
      frame_spec = spectrum(windowing(frame))
      frame_logfreq,_,_ = logfreq(frame_spec)
    self.pool.add('log_spec', amp2db(frame_logfreq))
    print("added log_spec to pool.")
    
  def inverse_spec(self):
    frameSize = 1024
    hopSize = 512
    fs = 44100
    w = es.Windowing(type='hamming', normalized=False)
    # make sure these are same for MFCC and IDCT computation
    NUM_BANDS  = 26
    DCT_TYPE = 2
    LIFTERING = 0
    NUM_MFCCs = 13
    spectrum = es.Spectrum()
    mfcc = es.MFCC(numberBands = NUM_BANDS,
                    numberCoefficients=NUM_MFCCs, # make sure you specify first N mfcc: the less, the more lossy (blurry) the smoothed mel spectrum will be
                    weighting='linear', # computation of filter weights done in Hz domain (optional)
                    normalize='unit_max', #  htk filter normaliation to have constant height = 1 (optional)
                    dctType=DCT_TYPE,
                    logType='log',
                    liftering=LIFTERING) # corresponds to htk default CEPLIFTER = 22                  
    idct = es.IDCT(inputSize=NUM_MFCCs, 
                    outputSize=NUM_BANDS, 
                    dctType = DCT_TYPE, 
                    liftering = LIFTERING)
    all_melbands_smoothed = []
    for frame in es.FrameGenerator(self.mono(), frameSize=frameSize, hopSize=hopSize):
      spect = spectrum(w(frame))
      melbands, mfcc_coeffs = mfcc(spect)
      melbands_smoothed = np.exp(idct(mfcc_coeffs)) # inverse the log taken in MFCC computation
      all_melbands_smoothed.append(melbands_smoothed)
    # transpose to have it in a better shape
    # we need to convert the list to an essentia.array first (== numpy.array of floats)
    #mfccs = essentia.array(pool['MFCC']).T

    self.pool.add('inverse_spec', essentia.array(all_melbands_smoothed).T)
    print("added inverse_spec to pool.")

  def rhythm(self):
    rhythm_extractor = es.RhythmExtractor2013(method="multifeature")
    bpm, beats, confidence, _, intervals = rhythm_extractor(self.mono())
    self.pool.add('bpm', bpm[0])
    self.pool.add('beats', beats)
    self.pool.add('bpm_confidence', confidence)
    self.pool.add('bpm_intervals', intervals)
    print("added rhythm descriptors to pool.")

  def loudness(self):
    windowing = es.Windowing(type='blackmanharris62', zeroPadding=2048)
    spectrum = es.Spectrum()
    rms = es.RMS()
    hfc = es.HFC()
    for frame in es.FrameGenerator(self.mono(), frameSize=2048, hopSize=1024):
      frame_spectrum = spectrum(windowing(frame))
      self.pool.add('rms', rms(frame))
      self.pool.add('rms_spectrum', rms(frame_spectrum))
      self.pool.add('hfc', hfc(frame_spectrum))
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
    print("removed \'"+key+"\' from pool")

  def clear(self):
    self.pool.clear()
    print("cleared the pool.")
