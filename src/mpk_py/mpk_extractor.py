#!/usr/bin/env python3
import os
import argparse
from pathlib import Path
import numpy as np
import multiprocessing as mp
from mpk import *


def run():
    parser = argparse.ArgumentParser(description="MPK_EXTRACTOR")
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
        "-d",
        help="descriptors to include",
        choices=[
            "metadata",
            "features",
            "lowlevel",
            "rhythm",
            "sfx",
            "tonal",
            "spectrograms",
            "mel_spec",
            "log_spec",
            "freq_spec",
            "all",
        ],
    )
    parser.add_argument(
        "-j", "--jobs", default=mp.cpu_count(), help="number of parallel jobs"
    )
    parser.add_argument(
        "-qs", "--queue_size", type=int, default=16, help="size of batch queue"
    )
    parser.add_argument(
        "-bs", "--batch_size", type=int, default=1, help="size of batch"
    )
    parser.add_argument("--version", action="version", version="%(prog)s 0.1.0")
    parser.add_argument("-f", "--force", help="skip file check, force insert or update")
    return parser.parse_args()


def get_audio_data(v):
    metadata = v["metadata"]["tags"]
    return audio_data(
        metadata["path"][0],
        int(metadata["filesize"][0]),
        metadata["duration"][0],
        int(metadata["channels"][0]),
        int(metadata["bitrate"][0]),
        int(metadata["samplerate"][0]),
    )


def get_lowlevel_features(v):
    ll = v["lowLevel"]
    return lowlevel_features(
        [
            ll["average_loudness"],
            ll["barkbands_kurtosis"],
            ll["barkbands_skewness"],
            ll["barkbands_spread"],
            len(ll["barkbands"][0]),
            ll["barkbands"],
            ll["dissonance"],
            ll["hfc"],
            ll["pitch"],
            ll["pitch_instantaneous_confidence"],
            ll["pitch_salience"],
            ll["silence_rate_20dB"],
            ll["silence_rate_30dB"],
            ll["silence_rate_60dB"],
            ll["spectral_centroid"],
            ll["spectral_complexity"],
            ll["spectral_crest"],
            ll["spectral_decrease"],
            ll["spectral_energy"],
            ll["spectral_energyband_high"],
            ll["spectral_energyband_low"],
            ll["spectral_energyband_middle_high"],
            ll["spectral_energyband_middle_low"],
            ll["spectral_flatness_db"],
            ll["spectral_flux"],
            ll["spectral_kurtosis"],
            ll["spectral_rms"],
            ll["spectral_rolloff"],
            ll["spectral_skewness"],
            ll["spectral_spread"],
            ll["spectral_strongpeak"],
            ll["zerocrossingrate"],
            len(ll["mfcc"][0]),
            ll["mfcc"],
            len(ll["sccoeffs"][0]),
            ll["sccoeffs"],
            len(ll["scvalleys"][0]),
            ll["scvalleys"],
        ]
    )


def get_rhythm_features(v):
    rhythm = []
    for i in (
        "bpm",
        "confidence",
        "onset_rate",
        "beats_loudness",
        "first_peak_bpm",
        "first_peak_spread",
        "first_peak_weight",
        "second_peak_bpm",
        "second_peak_spread",
        "second_peak_weight",
        "beats_position",
        "bpm_estimates",
        "bpm_intervals",
        "onset_times",
        "beats_loudness_band_ratio",
        "histogram",
    ):
        if i in v["rhythm"]:
            if i == "beats_loudness_band_ratio":
                rhythm.append(len(v["rhythm"][i][0]))
            rhythm.append(v["rhythm"][i])
        else:
            rhythm.append(None)
    rhythm = rhythm_features(rhythm)
    return rhythm


def get_sfx_features(v):
    sfx = sfx_features(list(v["sfx"].values()))
    return sfx


def get_tonal_features(v):
    tl = v["tonal"]
    tonal = tonal_features(
        [
            tl["chords_changes_rate"],
            tl["chords_number_rate"],
            tl["key_strength"],
            tl["tuning_diatonic_strength"],
            tl["tuning_equal_tempered_deviation"],
            tl["tuning_frequency"],
            tl["tuning_nontempered_energy_ratio"],
            tl["chords_strength"],
            tl["chords_histogram"],
            tl["thpcp"],
            len(tl["hpcp"][0]),
            tl["hpcp"],
            tl["chords_key"],
            tl["chords_scale"],
            tl["key_key"],
            tl["key_scale"],
            tl["chords_progression"],
        ]
    )
    return tonal


def get_spectrograms(v, descs):
    specs = [None, None, None, None, None, None]
    if descs is not None:
        if "mel_spec" in descs:
            specs[0:2] = [len(v["mel_spec"][0]), v["mel_spec"]]
        if "log_spec" in descs:
            specs[2:4] = [len(v["log_spec"][0]), v["log_spec"]]

        if "freq_spec" in descs:
            specs[4:6] = [len(v["freq_spec"][0]), v["freq_spec"]]
    else:
        specs = [
            len(v["mel_spec"][0]),
            v["mel_spec"],
            len(v["log_spec"][0]),
            v["log_spec"],
            len(v["freq_spec"][0]),
            v["freq_spec"],
        ]
    specs = spectrograms(specs)
    return specs


def get_musicbrainz_tags(v):
    mb_tags = []
    for i in (
        "musicbrainz_albumartistid",
        "musicbrainz_albumid",
        "musicbrainz_albumstatus",
        "musicbrainz_albumtype",
        "musicbrainz_artistid",
        "musicbrainz_releasegroupid",
        "musicbrainz_releasetrackid",
        "musicbrainz_trackid",
        "asin",
        "musicip_puid text",
    ):
        if i in v["metadata"]["tags"]:
            mb_tags.append(v["metadata"]["tags"][i][0])
        else:
            mb_tags.append(None)
    mb_tags = musicbrainz_tags(mb_tags)
    return mb_tags


def get_track_tags(v):
    tags = []
    for i in (
        "artist",
        "title",
        "album",
        "genre",
        "date",
        "tracknumber",
        "format",
        "language",
        "releasecountry",
        "label",
        "producer",
        "engineer",
        "mixer",
    ):
        if i in v["metadata"]["tags"]:
            tags.append(v["metadata"]["tags"][i][0])
        else:
            tags.append(None)
    tags = track_tags(tags)
    return tags


def prep_common(v, descs):
    audiodata = get_audio_data(v)
    lowlevel = None
    rhythm = None
    sfx = None
    tonal = None
    if descs is not None:
        if "features" in descs:
            lowlevel = get_lowlevel_features(v)
            rhythm = get_rhythm_features(v)
            sfx = get_sfx_features(v)
            tonal = get_tonal_features(v)
    specs = get_spectrograms(v, descs)
    return (audiodata, lowlevel, rhythm, sfx, tonal, specs)


def prep_track(v, descs):
    # collect common
    common = prep_common(v, descs)
    tags = get_track_tags(v)
    mb_tags = get_musicbrainz_tags(v)
    return common + (tags, mb_tags)


def insert_track(audiodata, lowlevel, rhythm, sfx, tonal, specs, tags, mb_tags):
    id = db.insert_track(audiodata)
    if tags is not None:
        try:
            db.insert_track_tags(id, tags)
        except Exception as err:
            print("error during insert_track_tags: {0}".format(err))

    if mb_tags is not None:
        try:
            db.insert_track_tags_musicbrainz(id, mb_tags)
        except Exception as err:
            print("error during insert_track_tags_musicbrainz: {0}".format(err))

    if lowlevel is not None:
        db.insert_track_featues_lowlevel(id, lowlevel)

    if rhythm is not None:
        try:
            db.insert_track_features_rhythm(id, rhythm)
        except Exception as err:
            print("error during insert_track_features_rhythm: {0}".format(err))

    if sfx is not None:
        db.insert_track_features_sfx(id, sfx)
    if tonal is not None:
        db.insert_track_features_tonal(id, tonal)
    if specs is not None:
        db.insert_track_images(id, specs)


def insert_sample(audiodata, lowlevel, rhythm, sfx, tonal, specs):
    id = db.insert_sample(audiodata)
    if lowlevel is not None:
        db.insert_sample_featues_lowlevel(id, lowlevel)

    if rhythm is not None:
        try:
            db.insert_sample_features_rhythm(id, rhythm)
        except Exception as err:
            print("error during insert_sample_features_rhythm: {0}".format(err))

    if sfx is not None:
        db.insert_sample_features_sfx(id, sfx)
    if tonal is not None:
        db.insert_sample_features_tonal(id, tonal)
    if specs is not None:
        db.insert_sample_images(id, specs)


def insert(data, descs):
    for k, v in data.items():
        if args.type == "track":
            (
                audiodata,
                lowlevel,
                rhythm,
                sfx,
                tonal,
                specs,
                tags,
                mb_tags,
            ) = prep_track(v, descs)
            insert_track(audiodata, lowlevel, rhythm, sfx, tonal, specs, tags, mb_tags)
        elif args.type == "sample":
            (audiodata, lowlevel, rhythm, sfx, tonal, specs) = prep_common(v, descs)
            insert_sample(audiodata, lowlevel, rhythm, sfx, tonal, specs)


def job(files, descs):
    print("spawning %d with files: %s" % (os.getpid(), files))
    return bulk_extract(files, descs=descs)


if __name__ == "__main__":
    args = run()
    cfg = Config(args.cfg)
    db = Mdb(cfg.db_path())

    files = [i for s in [collect_files(f) for f in args.input] for i in s]
    new = []
    modified = []
    moved = []

    if not args.force:
      for idx, f in enumerate(files):
        q = db.query_check_file(f, args.type)
        if q == "not found":
          print("%d: new file: %s" % (idx, f))
          new.append(f)
        elif q == "found":
          print("%d: file found: %s" % (idx, f))
        elif q == "modified":
          print("%d: file modified: %s" % (idx, f))
          modified.append(f)
        elif q == "moved":
          print("%d: file moved: %s" % (idx, f))
          moved.append(f)
        else:
          print('invalid check_file response')
    else:
      new = files

    if args.d:
        descs = args.d
    else:
        descs = None

    if not new or modified or moved:
        quit()

    print(
        "processing %d new, %d modified, %d moved..."
        % (len(new), len(modified), len(moved))
    )

    with mp.Pool(processes=args.jobs) as pool:
        queue_size = args.queue_size
        if queue_size > len(new):
            queue_size = len(new)
        batch_size = args.batch_size
        for i in range(0, len(new), queue_size):
            batch_files = new[i : i + queue_size]
            res = [
                pool.apply_async(
                    job,
                    (
                        batch_files[x : x + batch_size],
                        descs,
                    ),
                )
                for x in range(0, queue_size, batch_size)
            ]
            for r in res:
                insert(r.get(), descs)
            print("PROCESSED :: %d/%d" % (i + queue_size, len(new)))
    print("...DONE")
