import essentia.standard as es
import librosa
import matplotlib.pyplot as plt

audio_file = "heartless.flac"
y,sr = librosa.load(audio_file, sr=44100)

rhythm_extractor = es.RhythmExtractor2013(method="multifeature")
bpm, beats, beats_confidence, _, beats_intervals = rhythm_extractor(y)

print("BPM:", bpm)
print("Beat positions (sec.):", beats)
print("Beat estimation confidence:", beats_confidence)

# Mark beat positions in the audio and write it to a file.
# Use beeps instead of white noise to mark them, as it is more distinctive.
marker = es.AudioOnsetsMarker(onsets=beats, type='beep')
marked_audio = marker(y)

plt.plot(y)
for beat in beats:
    plt.axvline(x=beat*44100, color='red')
plt.xlabel('Time (samples)')
plt.title("Audio waveform and the estimated beat positions")
plt.show()

# an alternative
# Compute BPM.
bpm = es.PercivalBpmEstimator()(y)

print("BPM:", bpm)

# use this on short loops (pre-cut)
bpm = es.LoopBpmEstimator()(y)
print("Loop BPM:", bpm)

# using TempoCNN
# curl -SLO https://essentia.upf.edu/models/tempo/tempocnn/deeptemp-k16-3.pb
#global_bpm = es.TempoCNN(graphFilename='deeptemp-k16-3.pb')

#print('song BPM: {}'.format(global_bpm))
