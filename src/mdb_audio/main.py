import argparse
import time
import logging
import torch
import torchaudio
if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description='Feature extraction for audio files')
    parser.add_argument('input')
    args = parser.parse_args()

    print(torch.__version__)
    print(torchaudio.__version__)
    print(args)
