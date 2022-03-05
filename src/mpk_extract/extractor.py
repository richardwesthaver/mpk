import essentia.standard as es
from essentia import Pool
import numpy as np
import librosa

audio_file = "heartless.flac"
result_file = "extractor.json"

y,sr = librosa.load(audio_file, sr=44100)
extractor = es.Extractor()
stats = extractor(y)
es.YamlOutput(filename=result_file, format="json")(stats)
