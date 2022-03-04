import essentia
import essentia.standard as ess
import matplotlib.pyplot as plt
import numpy as np
import librosa

def extractor(filename):
  frameSize = 1024
  hopSize = 512
  fs = 44100

  y,sr = librosa.load(filename,sr=44100)
  w = ess.Windowing(type='hamming', normalized=False)

  # make sure these are same for MFCC and IDCT computation
  NUM_BANDS  = 26
  DCT_TYPE = 2
  LIFTERING = 0
  NUM_MFCCs = 13

  spectrum = ess.Spectrum()

  mfcc = ess.MFCC(numberBands = NUM_BANDS,
                  numberCoefficients=NUM_MFCCs, # make sure you specify first N mfcc: the less, the more lossy (blurry) the smoothed mel spectrum will be
                  weighting='linear', # computation of filter weights done in Hz domain (optional)
                  normalize='unit_max', #  htk filter normaliation to have constant height = 1 (optional)
                  dctType=DCT_TYPE,
                  logType='log',
                  liftering=LIFTERING) # corresponds to htk default CEPLIFTER = 22                  

  idct = ess.IDCT(inputSize=NUM_MFCCs, 
                  outputSize=NUM_BANDS, 
                  dctType = DCT_TYPE, 
                  liftering = LIFTERING)
  all_melbands_smoothed = []


  for frame in ess.FrameGenerator(y, frameSize=frameSize, hopSize=hopSize):
    spect = spectrum(w(frame))
    melbands, mfcc_coeffs = mfcc(spect)
    melbands_smoothed = np.exp(idct(mfcc_coeffs)) # inverse the log taken in MFCC computation
    all_melbands_smoothed.append(melbands_smoothed)

  # transpose to have it in a better shape
  # we need to convert the list to an essentia.array first (== numpy.array of floats)
  #mfccs = essentia.array(pool['MFCC']).T
  all_melbands_smoothed = essentia.array(all_melbands_smoothed).T
  
  # and plot
  plt.imshow(all_melbands_smoothed, aspect='auto') # ignore enery
  #plt.imshow(mfccs, aspect = 'auto', interpolation='none')
  plt.show() # unnecessary if you started "ipython --pylab"

extractor("heartless.flac")
