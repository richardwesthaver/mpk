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
            # TODO
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
        return lib.mpk_db_config_flags(self.db)

    def get_path(self, path):
        return ffi.string(lib.mpk_fs_config_get_path(self.fs, path.encode())).decode()


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

    def insert_track(self, track):
        return lib.mdb_insert_track(self.db, track.encode())

    def insert_track_tags(self, id, tags, ptr=True):
        print("inserting tags:", tags, "for track_id:", id)
        return lib.mdb_insert_track_tags(self.db, id, tags)

    def insert_track_tags_musicbrainz(self, id, tags, ptr=True):
        print("inserting musicbrainz_tags:", tags, "for track_id:", id)
        return lib.mdb_insert_track_tags_musicbrainz(self.db, id, tags)

    def insert_track_featues_lowlevel(self, id, features, ptr=True):
        print("inserting lowlevel_features for track_id:", id)
        return lib.mdb_insert_track_features_lowlevel(self.db, id, features)

    def insert_track_features_rhythm(self, id, features, ptr=True):
        print("inserting rhythm_features for track_id:", id)
        return lib.mdb_insert_track_features_rhythm(self.db, id, features)

    def insert_track_features_sfx(self, id, features, ptr=True):
        print("inserting sfx_features for track_id:", id)
        return lib.mdb_insert_track_features_sfx(self.db, id, features)

    def insert_track_features_tonal(self, id, features, ptr=True):
        print("inserting tonal_features for track_id:", id)
        return lib.mdb_insert_track_features_tonal(self.db, id, features)

    def insert_track_images(self, id, images, ptr=True):
        print("inserting spectrograms for track_id:", id)
        return lib.mdb_insert_track_images(self.db, id, images)

    def insert_sample(self, sample):
        return lib.mdb_insert_sample(self.db, sample.encode())

    def insert_sample_featues_lowlevel(self, id, features, ptr=True):
        print("inserting lowlevel_features for sample_id:", id)
        return lib.mdb_insert_sample_features_lowlevel(self.db, id, features)

    def insert_sample_features_rhythm(self, id, features, ptr=True):
        print("inserting rhythm_features for sample_id:", id)
        return lib.mdb_insert_sample_features_rhythm(self.db, id, features)

    def insert_sample_features_sfx(self, id, features, ptr=True):
        print("inserting sfx_features for sample_id:", id)
        return lib.mdb_insert_sample_features_sfx(self.db, id, features)

    def insert_sample_features_tonal(self, id, features, ptr=True):
        print("inserting tonal_features for sample_id:", id)
        return lib.mdb_insert_sample_features_tonal(self.db, id, features)

    def insert_sample_images(self, id, images, ptr=True):
        print("inserting spectrograms for sample_id:", id)
        return lib.mdb_insert_sample_images(self.db, id, images)


def vectorize(arr):
    if type(arr) is list:
        arr = np.float32(arr)
    buf = ffi.from_buffer("float[]", arr)
    return lib.mdb_vecreal_new(ffi.cast("const float *", buf), len(buf))


def track_tags(tags):
    tags[0:4] = [x.encode() for x in tags[0:4]]
    tags[4] = int(tags[4])
    return lib.mdb_track_tags_new(*tags)


def musicbrainz_tags(tags):
    tags = [x.encode() for x in tags]
    return lib.mdb_musicbrainz_tags_new(*tags)


def lowlevel_features(features):
    features[1:] = [vectorize(x) for x in features[1:]]
    return lib.mdb_lowlevel_features_new(*features)


def rhythm_features(features):
    #    features[:3] = [x[0] for x in features[:3] if isinstance(x, (list, np.ndarray))]
    features[3] = vectorize(features[3])
    features[4:10] = [x[0] for x in features[4:10] if isinstance(x, (list, np.ndarray))]
    features[10:] = [vectorize(x) for x in features[10:]]
    return lib.mdb_rhythm_features_new(*features)


def sfx_features(features):
    features[4:] = [vectorize(x) for x in features[4:]]
    return lib.mdb_sfx_features_new(*features)


def tonal_features(features):
    features[7:11] = [vectorize(x) for x in features[7:11]]
    features[11:16] = [x[0].encode() for x in features[11:16]]
    return lib.mdb_tonal_features_new(*features)


def spectrograms(specs):
    specs = [vectorize(x) for x in specs]
    return lib.mdb_spectrograms_new(*specs)
