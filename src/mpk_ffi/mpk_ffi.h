#ifndef mpk_ffi_h
#define mpk_ffi_h

/* Generated with cbindgen:0.20.0 */

/* DO NOT TOUCH */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * MPK Configuration
 */
typedef struct Config Config;

typedef struct DbConfig DbConfig;

typedef struct FsConfig FsConfig;

typedef struct JackConfig JackConfig;

/**
 * MPK Database
 */
typedef struct Mdb Mdb;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct Config *mpk_config_new(const struct FsConfig *fs,
                              const struct DbConfig *db,
                              const struct JackConfig *jack);

struct Config *mpk_config_load(const char *path);

void mpk_config_write(const struct Config *cfg, const char *path);

void mpk_config_build(const struct Config *cfg);

struct FsConfig *mpk_fs_config_new(const char *root);

const char *mpk_fs_config_get_path(const struct FsConfig *cfg, const char *path);

struct DbConfig *mpk_db_config_new(void);

const int *mpk_db_config_flags(const struct DbConfig *cfg);

struct JackConfig *mpk_jack_config_new(void);

struct Mdb *mdb_new(const char *path);

void mdb_init(const struct Mdb *db);

int64_t mdb_insert_track(const struct Mdb *db, const char *path);

void mdb_insert_track_tags(const struct Mdb *db,
                           int64_t id,
                           const char *artist,
                           const char *title,
                           const char *album,
                           const char *genre,
                           int16_t year);

void mdb_exec_batch(const struct Mdb *db, const char *sql);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* mpk_ffi_h */
