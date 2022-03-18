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
    data = bulk_extract(files)

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

        metadata = v["metadata"]["tags"]
        data = audio_data(metadata["path"][0],
                          int(metadata["filesize"][0]),
                          metadata["duration"][0],
                          int(metadata["channels"][0]),
                          int(metadata["bitrate"][0]),
                          int(metadata["samplerate"][0]))

        ll = v['lowLevel']
        lowlevel = lowlevel_features([ll['average_loudness'],
                                      ll['barkbands_kurtosis'],
                                      ll['barkbands_skewness'],
                                      ll['barkbands_spread'],
                                      len(ll['barkbands'][0]),
                                      ll['barkbands'],
                                      ll['dissonance'],
                                      ll['hfc'],
                                      ll['pitch'],
                                      ll['pitch_instantaneous_confidence'],
                                      ll['pitch_salience'],
                                      ll['silence_rate_20dB'],
                                      ll['silence_rate_30dB'],
                                      ll['silence_rate_60dB'],
                                      ll['spectral_centroid'],
                                      ll['spectral_complexity'],
                                      ll['spectral_crest'],
                                      ll['spectral_decrease'],
                                      ll['spectral_energy'],
                                      ll['spectral_energyband_high'],
                                      ll['spectral_energyband_low'],
                                      ll['spectral_energyband_middle_high'],
                                      ll['spectral_energyband_middle_low'],
                                      ll['spectral_flatness_db'],
                                      ll['spectral_flux'],
                                      ll['spectral_kurtosis'],
                                      ll['spectral_rms'],
                                      ll['spectral_rolloff'],
                                      ll['spectral_skewness'],
                                      ll['spectral_spread'],
                                      ll['spectral_strongpeak'],
                                      ll['zerocrossingrate'],
                                      len(ll['mfcc'][0]),
                                      ll['mfcc'],
                                      len(ll['sccoeffs'][0]),
                                      ll['sccoeffs'],
                                      len(ll['scvalleys'][0]),
                                      ll['scvalleys']
                                      ])

        try:
            rl = v["rhythm"]
            rhythm = rhythm_features([rl['bpm'],
                                      rl['confidence'],
                                      rl['onset_rate'],
                                      rl['beats_loudness'],
                                      rl['first_peak_bpm'],
                                      rl['first_peak_spread'],
                                      rl['first_peak_weight'],
                                      rl['second_peak_bpm'],
                                      rl['second_peak_spread'],
                                      rl['second_peak_weight'],
                                      rl['beats_position'],
                                      rl['bpm_estimates'],
                                      rl['bpm_intervals'],
                                      rl['onset_times'],
                                      len(rl['beats_loudness_band_ratio'][0]),
                                      rl['beats_loudness_band_ratio'],
                                      rl['histogram']])
        except Exception as err:
            rhythm = None
            print("error while building rhythm_features: {0}".format(err))

        sfx = sfx_features(list(v["sfx"].values()))

        tl = v["tonal"]
        tonal = tonal_features([tl['chords_changes_rate'],
                                tl['chords_number_rate'],
                                tl['key_strength'],
                                tl['tuning_diatonic_strength'],
                                tl['tuning_equal_tempered_deviation'],
                                tl['tuning_frequency'],
                                tl['tuning_nontempered_energy_ratio'],
                                tl['chords_strength'],
                                tl['chords_histogram'],
                                tl['thpcp'],
                                len(tl['hpcp'][0]),
                                tl['hpcp'],
                                tl['chords_key'],
                                tl['chords_scale'],
                                tl['key_key'],
                                tl['key_scale'],
                                tl['chords_progression']])

        specs = spectrograms(
            [
              len(v["mel_spec"][0]),
              v["mel_spec"],
              len(v["log_spec"][0]),
              v["log_spec"],
              len(v["freq_spec"][0]),
              v["freq_spec"]
            ]
        )

        if args.db:
            if args.type == "track":
                id = db.insert_track(data)
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


 #               db.insert_track_featues_lowlevel(id, lowlevel)

                # if rhythm is not None:
                #     try:
                #       db.insert_track_features_rhythm(id, rhythm)
                #     except Exception as err:
                #         print(
                #             "error during insert_track_features_rhythm: {0}".format(err)
                #         )

                # db.insert_track_features_sfx(id, sfx)
#                db.insert_track_features_tonal(id, tonal)
                db.insert_track_images(id, specs)

            elif args.type == "sample":
                id = db.insert_sample(data)
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
