try:
    from cffi import FFI
except ImportError:
    print("pip install cffi, included with PyPy")

import os
import re
import pathlib


def parse_header(header):
    h = open(header, "r").read().lstrip()
    cdef = re.sub(
        r"^(#|\s*\/*\*|extern).*[\r\n]|.*\"C\"$|^(?:[\t ]*(?:\r?\n|\r))+",
        "",
        h,
        flags=re.MULTILINE,
    )
    return cdef


def init_ffi(cdef):
    ffi = FFI()
    ffi.set_source(
        "_mpk",
        """
                 #include "mpk_ffi.h"
                 """,
        libraries=["mpk_ffi"],
        library_dirs=["."],
        include_dirs=["./include"],
    )

    ffi.cdef(cdef)

    return ffi


def compile(ffi, lib_dir, v):
    os.environ["LD_RUN_PATH"] = os.path.abspath(lib_dir)
    ffi.compile(verbose=v)


if __name__ == "__main__":
    build_dir = pathlib.Path(__file__).parent
    cdef = parse_header(build_dir / "include/mpk_ffi.h")
    print(cdef)
    compile(init_ffi(cdef), build_dir, True)
