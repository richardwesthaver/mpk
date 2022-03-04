import matplotlib.pyplot as plt

import essentia.standard as es
import essentia
import librosa

audio_file = "heartless.flac"

y,sr = librosa.load(audio_file, sr=44100)
windowing = es.Windowing(type='blackmanharris62', zeroPadding=2048)
spectrum = es.Spectrum()
melbands = es.MelBands(numberBands=96, lowFrequencyBound=0, highFrequencyBound=11000)
spectrum_logfreq = es.LogSpectrum(binsPerSemitone=1)

amp2db = es.UnaryOperator(type='lin2db', scale=2)
pool = essentia.Pool()

for frame in es.FrameGenerator(y, frameSize=2048, hopSize=1024):
  frame_spectrum = spectrum(windowing(frame))
  frame_mel = melbands(frame_spectrum)
  frame_spectrum_logfreq, _, _ = spectrum_logfreq(frame_spectrum)
  
  pool.add('spectrum_db', amp2db(frame_spectrum))
  pool.add('mel96_db', amp2db(frame_mel))
  pool.add('spectrum_logfreq_db', amp2db(frame_spectrum_logfreq))

# Plot all spectrograms.
fig, ((ax1, ax2, ax3)) = plt.subplots(3, 1, sharex=True, sharey=False, figsize=(15, 16))

ax1.set_title("Log-spectrogram (amp2db)")
ax1.set_xlabel("Time (frames)")
ax1.set_ylabel("Frequency bins")
ax1.imshow(pool['spectrum_db'].T, aspect = 'auto', origin='lower', interpolation='none')

ax2.set_title("Mel log-spectrogram (amp2db)")
ax2.set_xlabel("Time (frames)")
ax2.set_ylabel("Mel frequency bands")
ax2.imshow(pool['mel96_db'].T, aspect = 'auto', origin='lower', interpolation='none')

ax3.set_title("Log-frequency log-spectrogram (amp2db)")
ax3.set_xlabel("Time (frames)")
ax3.set_ylabel("Log-frequency bins")
ax3.imshow(pool['spectrum_logfreq_db'].T, aspect = 'auto', origin='lower', interpolation='none')

plt.show()
