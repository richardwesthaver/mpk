#!/usr/local/bin/bash
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    jackd -d alsa
elif [[ "$OSTYPE" == "darwin"* ]]; then
    jackd -d coreaudio
else
    echo "unimplemented"
fi
