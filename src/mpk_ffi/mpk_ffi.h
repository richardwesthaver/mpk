#ifndef mpk_ffi_h
#define mpk_ffi_h

/* Generated with cbindgen:0.20.0 */

/* DO NOT TOUCH */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Media Database
 */
typedef struct Mdb Mdb;

struct Mdb *mdb_new(const char *path);

void mdb_init(const struct Mdb *db);

int64_t mdb_insert_track(const struct Mdb *db, const char *path);

#endif /* mpk_ffi_h */
