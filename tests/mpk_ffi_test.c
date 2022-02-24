#include <stdio.h>
#include "../mpk_ffi.h"

int main() {
  FsConfig* fs_cfg = mpk_fs_config_default();
  DbConfig* db_cfg = mpk_db_config_default();
  JackConfig* jk_cfg = mpk_jack_config_default();
  Config* cfg = mpk_config_new(fs_cfg, db_cfg, jk_cfg);

  mpk_config_write(cfg, "mpk.toml");
  mpk_config_load("mpk.toml");
  printf("fs_root: %s\n", mpk_fs_config_get_path(fs_cfg, "root"));
  printf("db_flags: %d\n", *mpk_db_config_flags(db_cfg));

  Mdb* db = mdb_new("test_ffi.db");

  mdb_init(db);
  printf("init success!\n");
  int64_t last_id = mdb_insert_track(db, "./mpk_test.c");
  printf("last_id: %lld\n", last_id);
  mdb_insert_track_tags(db, last_id, "artist", "title", "album", "genre", 1234);
  mdb_exec_batch(db, "select * from track_tags"); // NOTE: no output
}
