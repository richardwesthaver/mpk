try:
  from _mpk import lib, ffi
except ImportError:
  print("mpk python bindings missing")

from extract import AudioFile, Extract

NULL = ffi.NULL

class Config:
  def __init__(self):
    self.fs = lib.mpk_fs_config_new(NULL)
    self.db = lib.mpk_db_config_new()
    self.jack = lib.mpk_jack_config_new()
    self.cfg = lib.mpk_config_new(self.fs, self.db, self.jack)

  def write(self, file):
    print("writing config to "+file)
    lib.mpk_config_write(self.cfg, file.encode())

  def load(self, file):
    print("loaded config from "+file)
    self.cfg = lib.mpk_config_load(file.encode())

  def db_flags(self):
    return lib.mpk_db_config_flags(self.db)[0]

  def get_path(self, path):
    return ffi.string(lib.mpk_fs_config_get_path(self.fs, path.encode())).decode()

class Mdb:
  def __init__(self):
    self.db = lib.mdb_new(NULL)

  def init(self):
    lib.mdb_init(self.db)
    print("initialized Mdb")

  def exec_batch(self, sql):
    return lib.mdb_exec_batch(self.db, sql.encode())

  def insert_track(self, track):
    return lib.mdb_insert_track(self.db, track.encode())

  def insert_track_tags(self, id, **args):
    tags=[NULL]*5
    if 'artist' in args:
      tags[0]=args.get('artist').encode()
    if 'title' in args:
      tags[1]=args.get('title').encode()
    if 'album' in args:
      tags[2]=args.get('album').encode()
    if 'genre' in args:
      tags[3]=args.get('genre').encode()
    if 'year' in args:
      tags[4]=args.get('year')
    else:
      tags[4]=0
    print("inserting tags: ",tags," for track_id: ",id)
    return lib.mdb_insert_track_tags(self.db, id, *tags)
