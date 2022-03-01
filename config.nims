# MPK dev tools
--hints:off

import std/distros
import std/os
import std/sequtils
from std/strutils import join

const
  v {.booldefine.} = false
  pkg {.strdefine.}: string = ""
  ffi {.booldefine.} = false
  MPK_BIN = "src/mpk"
var
  target_dir = "target/debug"
  build_dir = "build"

proc DepMissing(dep: string) =
  var d = case dep:
            of "jackd": "jack"
            of "rustc": "rust"
            else: dep
  echo "dependency not found: ", d
  let cmd = foreignDepInstallCmd(d)
  let prompt = "install with:\t"
  case cmd[1]:
    of true: echo prompt & "sudo " & cmd[0]
    of false: echo prompt & cmd[0]

proc checkDeps() =
  var deps = ["rustc", "jackd", "emacs"]
  for d in deps:
    let exe = findExe(d)
    if exe == "":
      DepMissing(d)
    else:
      echo "found ", d, ": ", exe

proc hostInfo() =
  echo "Current Host ::"
  echo "\tOS: ", hostOS
  echo "\tCPU: ", hostCPU
  echo "\tcpuEndian: ", cpuEndian
  echo "\tNimVersion: ", NimVersion
  echo "\t", gorge("rustc --version")
  echo "\t", gorge("cargo --version")
  echo "\temacs ", gorge("emacs -Q --batch --eval '(message emacs-version)'")

task info, "print system, dependency, and project info":
  hostInfo()
  echo ""
  checkDeps()

task build, "build MPK":
  var args: seq[string]
  when defined(release):
    args.insert(" --release")
    target_dir = "target/release"
  exec "cargo build" & args.join
  when defined(Windows):
    let ext = ".dll"
  elif defined(Linux):
    let ext = ".so"
  elif defined(MacOsX):
    let ext = ".dylib"
  let 
    ffi_lib = "libmpk_ffi" & ext
    ffi_h = "mpk_ffi.h"
    include_dir = build_dir / "include"
  mkDir("build/include")
  cpFile(target_dir / ffi_lib, build_dir / ffi_lib)
  if fileExists(ffi_h):
    mvFile(ffi_h, include_dir / ffi_h)

task run, "run MPK binary":
  var args: seq[string]
  when defined(release):
    args.insert(" --release")
  exec "cargo run" & args.join

task install, "install MPK":
  exec "cargo install --release --path " & MPK_BIN

task clean, "clean build artifacts":
  exec "cargo clean"
  rmDir(build_dir)
  rmFile("Cargo.lock")

task test, "run MPK tests":
  if not dirExists(build_dir):
    buildTask()
  var args: seq[string]
  when defined(pkg):
    args.add(" -p " & pkg)
  when defined(v):
    args.add(" -- --nocapture")

  exec "cargo test" & args.join

  when defined(ffi):
    let test_build_dir = build_dir / "tests"
    let ffi_test = "mpk_ffi_test"
    if not fileExists(test_build_dir / "mpk_ffi_test"):
      mkDir(test_build_dir)
      exec "gcc tests/mpk_ffi_test.c -Ibuild/include -Lbuild -lmpk_ffi -o " & test_build_dir / ffi_test
    exec test_build_dir / ffi_test
    rmFile("/tmp/mpk.toml")
