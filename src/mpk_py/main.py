#!/usr/bin/env python3
import argparse
from pathlib import Path
import numpy as np
from mpk import *


def run():
    parser = argparse.ArgumentParser(description="MPK_EXTRACT")
    parser.add_argument(
        "input", type=Path, nargs="*", default=".", help="input file or directory"
    )
    parser.add_argument(
        "-c", "--cfg", type=Path, help="config file", 
    )
    parser.add_argument(
        "-t", help="input type of audio", default="track", choices=["track", "sample"]
    )
    parser.add_argument("--ext", help="extensions to include", choices=FILE_EXT)
    parser.add_argument(
        "--json", type=Path, help="write output to json", default="result.json"
    )
    parser.add_argument(
        "--db",
        action=argparse.BooleanOptionalAction,
        help="write output to MDB",
        default=True,
    )
    parser.add_argument(
        "-d",
        default="all",
        help="descriptors to include",
        choices=[
            "lowlevel",
            "rhythm",
            "sfx",
            "tonal",
            "spectograms",
            "metadata",
            "all",
        ],
    )
    parser.add_argument("--version", action="version", version="%(prog)s 0.1.0")

    return parser.parse_args()


if __name__ == "__main__":
    args = run()
    cfg = Config(args.cfg)
    db = Mdb("mdb.db")
    db.init()
    files = [i for s in [collect_files(f) for f in args.input] for i in s]
    data = bulk_extract(files)
    
    for k,v in data.items():
      id = db.insert_track(k)

      lowlevel = lowlevel_features(list(v['lowLevel'].values()))
      db.insert_track_featues_lowlevel(id, lowlevel)

      rhythm = rhythm_features(list(v['rhythm'].values()))
      db.insert_track_features_rhythm(id, rhythm)

      sfx = sfx_features(list(v['sfx'].values()))
      db.insert_track_features_sfx(id, sfx)

      tonal = tonal_features(list(v['tonal'].values()))
      db.insert_track_features_tonal(id, tonal)

      specs = spectograms([
        np.float32(v['mel_spec']),
        np.float32(v['log_spec']),
        np.float32(v['freq_spec'])
      ])
      db.insert_track_images(id, specs)
      
    print("...Done")
