#!/usr/bin/python3
import librosa, librosa.display
import matplotlib.pyplot as plt
import numpy as np
import os


path = "/srv/shed/stash/music/lib/wav/6720423_Hold_Tight_Original_Mix.wav"

y, sr = librosa.load(path,None)

D = librosa.stft(y)  # STFT of y
S_db = librosa.amplitude_to_db(np.abs(D), ref=np.max)

print("sample_rate:", librosa.get_samplerate(path))
print("duration:", librosa.get_duration(y,sr))
print("tempo:", librosa.beat.tempo(y,sr))
print("tuning:", librosa.estimate_tuning(y,sr))
print("onsets:", librosa.onset.onset_detect(y,sr))
print("spectral_centroid:", librosa.feature.spectral_centroid(y,sr))
print("mel_spectrogram:", librosa.feature.melspectrogram(y,sr))
print("tonnetz:", librosa.feature.tonnetz(y,sr))
