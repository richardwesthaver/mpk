import essentia
import essentia.standard as es
from pylab import plot, show, figure, imshow
import matplotlib.pyplot as plt
import librosa

audio_file = "heartless.flac"

audio,sr = librosa.load(audio_file, sr=44100, mono=False)
mono_audio = librosa.to_mono(audio)

pool = essentia.Pool()

windowing = es.Windowing(type='blackmanharris62', zeroPadding=2048)
spectrum = es.Spectrum()

rms = es.RMS()
hfc = es.HFC()

for frame in es.FrameGenerator(mono_audio, frameSize=2048, hopSize=1024):
    frame_spectrum = spectrum(windowing(frame))

    pool.add('rms', rms(frame))
    pool.add('rms_spectrum', rms(frame_spectrum))
    pool.add('hfc', hfc(frame_spectrum))
    
plot(audio)
plt.title("Waveform")

envelope = es.Envelope()
plot(envelope(mono_audio))
plt.title("Signal envelope")

plot(pool['rms'])
plt.title("RMS")
plt.xlim(0)

plot(pool['rms_spectrum'])
plt.title("Spectrum RMS")
plt.xlim(0)

plot(pool['hfc'])
plt.title("High-frequency content")
plt.xlim(0)


ebu_momentary, ebu_shortterm, ebu_integrated, dr = es.LoudnessEBUR128(hopSize=1024/44100, startAtZero=True)(audio)
plot(ebu_momentary)
plot(ebu_shortterm)
plt.title("EBU R128 momentary and short-term loudness")
plt.xlim(0)
plt.show()
