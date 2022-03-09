from _mpk import lib

cfg = lib.mpk_config_load(b"~/mpk/mpk.toml")
print(cfg)

lib.mpk_config_write(cfg, b"/tmp/mpk.toml")
