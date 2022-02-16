#!/usr/local/bin/bash
ffmpeg -f concat -safe 0 -i <(for f in $1/*.wav; do echo "file '$f'"; done) -c copy $2
