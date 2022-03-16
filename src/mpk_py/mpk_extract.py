#!/usr/bin/env python3
import os
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
        "-c",
        "--cfg",
        type=Path,
        help="config file",
    )
    parser.add_argument(
        "-t",
        "--type",
        help="input type of audio",
        default="track",
        choices=["track", "sample"],
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
            "spectrograms",
            "metadata",
            "all",
        ],
    )
    parser.add_argument("--version", action="version", version="%(prog)s 0.1.0")

    return parser.parse_args()


if __name__ == "__main__":
    args = run()
    cfg = Config(args.cfg)
    if args.db:
        db = Mdb(cfg.db_path())
        db.init()

    files = [i for s in [collect_files(f) for f in args.input] for i in s]
    if args.type == "track":
      data = bulk_extract(files, track=True)
    else if args.type == "sample":
      data = bulk_extract(files, track=False)

    for k, v in data.items():
        if args.type == "track":
            try:
                mb_tags = musicbrainz_tags(
                    [
                        v["metadata"]["tags"][k][0]
                        for k in (
                            "musicbrainz_albumartistid",
                            "musicbrainz_albumid",
                            "musicbrainz_albumstatus",
                            "musicbrainz_albumtype",
                            "musicbrainz_artistid",
                            "musicbrainz_releasegroupid",
                            "musicbrainz_releasetrackid",
                            "musicbrainz_trackid",
                        )
                    ]
                )
            except Exception as err:
                mb_tags = None
                print("error while building musicbrainz_tags: {0}".format(err))

            try:
                tags = track_tags(
                    [
                        v["metadata"]["tags"][k][0]
                        for k in ("artist", "title", "album", "genre", "year")
                    ]
                )
            except Exception as err:
                tags = None
                print("error while building track_tags: {0}".format(err))

        lowlevel = lowlevel_features(list(v["lowLevel"].values()))

        try:
            rhythm = rhythm_features(list(v["rhythm"].values()))
        except Exception as err:
            rhythm = None
            print("error while building rhythm_features: {0}".format(err))

        sfx = sfx_features(list(v["sfx"].values()))
        tonal = tonal_features(list(v["tonal"].values()))
        specs = spectrograms(
            [
                np.float32(v["mel_spec"]),
                np.float32(v["log_spec"]),
                np.float32(v["freq_spec"]),
            ]
        )

        if args.db:
            if args.type == "track":
                id = db.insert_track(os.path.realpath(k))
                if tags is not None:
                    try:
                        db.insert_track_tags(id, tags)
                    except Exception as err:
                        print("error during insert_track_tags: {0}".format(err))

                if mb_tags is not None:
                    try:
                        db.insert_track_tags_musicbrainz(id, mb_tags)
                    except Exception as err:
                        print(
                            "error during insert_track_tags_musicbrainz: {0}".format(
                                err
                            )
                        )

                db.insert_track_featues_lowlevel(id, lowlevel)

                if rhythm is not None:
                    try:
                        db.insert_track_features_rhythm(id, rhythm)
                    except Exception as err:
                        print(
                            "error during insert_track_features_rhythm: {0}".format(err)
                        )

                db.insert_track_features_sfx(id, sfx)
                db.insert_track_features_tonal(id, tonal)
                db.insert_track_images(id, specs)

            elif args.type == "sample":
                id = db.insert_sample(os.path.realpath(k))
                db.insert_sample_featues_lowlevel(id, lowlevel)

                if rhythm is not None:
                    try:
                        db.insert_sample_features_rhythm(id, rhythm)
                    except Exception as err:
                        print(
                            "error during insert_sample_features_rhythm: {0}".format(
                                err
                            )
                        )

                db.insert_sample_features_sfx(id, sfx)
                db.insert_sample_features_tonal(id, tonal)
                db.insert_sample_images(id, specs)

    print("...Done")
