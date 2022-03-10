#include <stdio.h>
#include "mpk_ffi.h"

int main() {
  printf("starting mpk_ffi_test.c ...\n");
  FsConfig* fs_cfg = mpk_fs_config_new(NULL);
  DbConfig* db_cfg = mpk_db_config_new();
  JackConfig* jk_cfg = mpk_jack_config_new();
  Config* cfg = mpk_config_new(fs_cfg, db_cfg, jk_cfg);

  mpk_config_write(cfg, "/tmp/mpk.toml");
  mpk_config_load("/tmp/mpk.toml");
  printf("fs_root: %s\n", mpk_fs_config_get_path(fs_cfg, "root"));
  printf("db_flags: %d\n", mpk_db_config_flags(db_cfg));

  Mdb* db = mdb_new(NULL);

  mdb_init(db);
  printf("init success!\n");
  int64_t last_id = mdb_insert_track(db, "./mpk_test.c");
  printf("last_id: %lld\n", last_id);
  TrackTags* track_tags = mdb_track_tags_new("artist", "title", "album", "genre", 0);
  mdb_insert_track_tags(db, last_id, track_tags);
  mdb_exec_batch(db, "select * from track_tags"); // NOTE: no output

  mdb_free(db);
  mdb_track_tags_free(track_tags);
  mpk_jack_config_free(jk_cfg);
  mpk_db_config_free(db_cfg);
  mpk_fs_config_free(fs_cfg);
  mpk_config_free(cfg);

  printf("... Done\n");
}
