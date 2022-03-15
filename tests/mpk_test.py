import mpk
import numpy as np

tags_raw = ["artist", "title", "album", "genre", 2022]
tags_musicbrainz_raw = ["0", "0", "status", "type", "0", "0", "0", "0"]
lowlevel_raw = [0.0, np.float32([0.0]) * 33]
rhythm_raw = [
    0.0,
    0.0,
    0.0,
    np.float32([0.0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, np.float32([0.0]) * 6),
]
sfx_raw = [0.0, 0.0, 0.0, 0.0, np.float32([0.0]) * 3]
tonal_raw = [
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    np.float32([0.0]),
    np.float32([0.0]),
    np.float32([0.0]),
    np.float32([0.0]),
    "test",
    "test",
    "test",
    "test",
    "test",
]
specs_raw = [
    np.float32([0.0, 0.5, 1.0]),
    np.float32([0.0, 0.5, 1.0]),
    np.float32([0.0, 0.5, 1.0]),
]
