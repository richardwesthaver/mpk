#!/usr/bin/env python3
import argparse
from pathlib import Path
from mpk import FILE_EXT


def run():
    parser = argparse.ArgumentParser(description="MPK_EXTRACT")
    parser.add_argument(
        "input", type=Path, nargs="*", default=".", help="input file or directory"
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

    args = parser.parse_args()


if __name__ == "__main__":
    run()
