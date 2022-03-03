try:
    from cffi import FFI
except ImportError:
    print("pip install cffi, included with PyPy")

import os
import re

def parse_header(header):
  h = open(header, "r").read().lstrip()
  cdef = re.sub(r'^(#|\s*\/*\*|extern).*[\r\n]|.*\"C\"$|^(?:[\t ]*(?:\r?\n|\r))+', '', h, flags=re.MULTILINE)
  return cdef

def init_ffi(cdef):
  ffi = FFI()
  ffi.set_source("mpk_ffi",
                 """
                 #include "mpk_ffi.h"
                 """,
                 libraries=['mpk_ffi'], library_dirs=['.'], include_dirs=['./include'])

  ffi.cdef(cdef)

  return ffi

def compile(ffi, lib_dir, v):
  os.environ["LD_RUN_PATH"] = os.path.abspath(lib_dir)
  ffi.compile(verbose=v)


if __name__ == "__main__":
  cdef = parse_header("./include/mpk_ffi.h")
  compile(init_ffi(cdef),".",True)
