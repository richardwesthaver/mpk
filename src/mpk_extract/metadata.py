import essentia.standard as es
from essentia import Pool
import librosa
audio_file = "heartless.flac"
result_file = "metadata.json"

result = Pool()

y,sr = librosa.load(audio_file, sr=44100)

# metadata
es.MetadataReader(filename=audio_file)()
pool = es.MetadataReader(filename=audio_file)()[7]

#for d in pool.descriptorNames():
#  print(d, pool[d])

es.YamlOutput(filename=result_file, format='json', doubleCheck=True, writeVersion=False)(pool)  
