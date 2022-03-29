try:
    from ._mpk import lib, ffi
except ImportError:
    print("mpk python bindings missing")

import numpy as np

NULL = ffi.NULL


class Config:
    def __init__(self, file=None):
        if file:
            self.cfg = lib.mpk_config_load(str(file).encode())
        else:
            self.cfg = lib.mpk_config_new(NULL, NULL, NULL)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.mpk_config_free(self.cfg)

    def write(self, file):
        print("writing config to " + file)
        lib.mpk_config_write(self.cfg, file.encode())

    def load(self, file):
        print("loaded config from " + file)
        self.cfg = lib.mpk_config_load(file.encode())

    def db_flags(self):
        return lib.mpk_db_config_flags(self.cfg)

    def db_path(self):
        return ffi.string(lib.mpk_db_config_path(self.cfg)).decode()

    def get_path(self, path):
        return ffi.string(lib.mpk_fs_config_get_path(self.cfg, path.encode())).decode()


class Mdb:
    def __init__(self, file=NULL):
        self.db = lib.mdb_new(str(file).encode())

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.mdb_free(self.db)

    def init(self):
        lib.mdb_init(self.db)
        print("initialized Mdb")

    def exec_batch(self, sql):
        return lib.mdb_exec_batch(self.db, sql.encode())

    def insert_track(self, data):
        print("inserting track:", data)
        return lib.mdb_insert_track(self.db, data)

    def insert_track_tags(self, id, tags):
        if tags is None:
            print("no track_tags found for track_id:", id)
            return
        else:
            print("inserting tags:", tags, "for track_id:", id)
            return lib.mdb_insert_track_tags(self.db, id, tags)

    def insert_track_tags_musicbrainz(self, id, tags):
        print("inserting musicbrainz_tags:", tags, "for track_id:", id)
        return lib.mdb_insert_track_tags_musicbrainz(self.db, id, tags)

    def insert_track_featues_lowlevel(self, id, features):
        print("inserting lowlevel_features:", features, "for track_id:", id)
        return lib.mdb_insert_track_features_lowlevel(self.db, id, features)

    def insert_track_features_rhythm(self, id, features):
        if features is None:
            print("no track_features_rhythm found for track_id:", id)
            return
        else:
            print("inserting rhythm_features:", features, "for track_id:", id)
            return lib.mdb_insert_track_features_rhythm(self.db, id, features)

    def insert_track_features_sfx(self, id, features):
        print("inserting sfx_features:", features, "for track_id:", id)
        return lib.mdb_insert_track_features_sfx(self.db, id, features)

    def insert_track_features_tonal(self, id, features):
        print("inserting tonal_features:", features, "for track_id:", id)
        return lib.mdb_insert_track_features_tonal(self.db, id, features)

    def insert_track_images(self, id, images):
        print("inserting spectrograms:", images, "for track_id:", id)
        return lib.mdb_insert_track_images(self.db, id, images)

    def insert_sample(self, data):
        print("inserting sample:", data)
        return lib.mdb_insert_sample(self.db, data)

    def insert_sample_featues_lowlevel(self, id, features):
        print("inserting lowlevel_features for sample_id:", id)
        return lib.mdb_insert_sample_features_lowlevel(self.db, id, features)

    def insert_sample_features_rhythm(self, id, features):
        if features is None:
            print("no sample_features_rhythm found for track_id:", id)
            return
        else:
            print("inserting rhythm_features for sample_id:", id)
            return lib.mdb_insert_sample_features_rhythm(self.db, id, features)

    def insert_sample_features_sfx(self, id, features):
        print("inserting sfx_features for sample_id:", id)
        return lib.mdb_insert_sample_features_sfx(self.db, id, features)

    def insert_sample_features_tonal(self, id, features):
        print("inserting tonal_features for sample_id:", id)
        return lib.mdb_insert_sample_features_tonal(self.db, id, features)

    def insert_sample_images(self, id, images):
        print("inserting spectrograms for sample_id:", id)
        return lib.mdb_insert_sample_images(self.db, id, images)


def vectorize(arr):
    if type(arr) is list:
        arr = np.float32(arr).flatten()
    buf = ffi.from_buffer("float[]", arr)
    return lib.mdb_vecreal_new(ffi.cast("const float *", buf), len(buf))


def matrixize(mtx):
    if type(mtx) is list:
        mtx = np.float32(mtx).flatten()
    buf = ffi.from_buffer("float[]", mtx)
    return lib.mdb_matrixreal_new(ffi.cast("const float *", buf), len(buf))


def audio_data(path, filesize, duration, channels, bitrate, samplerate):
    return lib.mdb_audio_data_new(
        path.encode(), filesize, duration, channels, bitrate, samplerate
    )


def track_tags(tags):
    if all(i is None for i in tags):
        return
    else:
        for idx, v in enumerate(tags):
            if v is None:
                tags[idx] = NULL
            else:
                tags[idx] = v.encode()
        return lib.mdb_track_tags_new(*tags)


def musicbrainz_tags(tags):
    if all(i is None for i in tags):
        return
    else:
        for idx, v in enumerate(tags):
            if v is None:
                tags[idx] = NULL
            elif type(v) is str:
                tags[idx] = v.encode()
        return lib.mdb_musicbrainz_tags_new(*tags)


def lowlevel_features(features):
    if features is None:
        return
    features[1:4] = [vectorize(x) for x in features[1:4]]
    features[5] = matrixize(features[5])
    features[6:32] = [vectorize(x) for x in features[6:32]]
    features[33] = matrixize(features[33])
    features[35] = matrixize(features[35])
    features[37] = matrixize(features[37])
    return lib.mdb_lowlevel_features_new(*features)


def rhythm_features(features):
    if any(i is None for i in features):
        return
    else:
        features[3] = vectorize(features[3])
        features[4:10] = [
            x[0] for x in features[4:10] if isinstance(x, (list, np.ndarray))
        ]
        features[10:14] = [vectorize(x) for x in features[10:14]]
        features[15] = matrixize(features[15])
        features[16] = vectorize(features[16])
        return lib.mdb_rhythm_features_new(*features)


def sfx_features(features):
    features[4:6] = [vectorize(x) for x in features[4:6]]
    features[6] = matrixize(features[6])
    return lib.mdb_sfx_features_new(*features)


def tonal_features(features):
    features[7:10] = [vectorize(x) for x in features[7:10]]
    features[11] = matrixize(features[11])
    features[12:16] = [x.encode() for x in features[12:16]]
    features[16] = "|".join(features[16]).encode()
    return lib.mdb_tonal_features_new(*features)


def spectrograms(specs):
    for idx, v in enumerate(specs):
        if idx in [1, 3, 5]:
            if v is not None:
                specs[idx] = matrixize(v)
            elif v is None:
                specs[idx] = matrixize([0])
        if idx in [0, 2, 4]:
            if v is None:
                specs[idx] = 0

    return lib.mdb_spectrograms_new(*specs)
