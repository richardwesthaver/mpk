#include <stdio.h>
#include "../mpk_ffi.h"

int main() {
  Mdb* db = mdb_new("test_ffi.db");
  mdb_init(db);
  printf("success\n");
  int64_t last_id = mdb_insert_track(db, "./mpk_test.c");
  printf("%lld\n", last_id);
}
