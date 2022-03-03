from mpk_ffi import lib

cfg = lib.mpk_config_load(b"~/mpk/mpk.toml")
print(cfg)

lib.mpk_config_write(cfg, b"mpk.toml")
