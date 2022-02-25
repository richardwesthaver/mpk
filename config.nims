# MPK dev tools
--hints:off

import std/distros
import std/os

const
  OPTS = ["verbose", "release", "target"]
  CARGO_RELEASE = ["--release"]
  SRC_DIR = "src/"
  BUILD_DIR = "build/"
  MPK_BIN = SRC_DIR & "mpk"
proc DepMissing(dep: string) =
  var d = case dep:
            of "jackd": "jack"
            of "rustc", "cargo": "rust"
            else: dep
  echo "dependency not found: ", d
  let cmd = foreignDepInstallCmd(d)
  let prompt = "install with:\t"
  case cmd[1]:
    of true: echo prompt & "sudo " & cmd[0]
    of false: echo prompt & cmd[0]

proc checkDeps() =
  var deps = ["rustc", "cargo", "jackd", "emacs"]
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
  exec "cargo build"
task run, "run MPK binary":
  exec "cargo run"
task install, "install MPK":
  exec "cargo install --path " & MPK_BIN
task clean, "clean build artifacts":
  exec "cargo clean"
  rmDir(BUILD_DIR)
  rmFile("mpk_ffi.h")
  rmFile("Cargo.lock")
task test, "run MPK tests":
  exec "cargo test"
  exec "gcc tests/mpk_ffi_test.c -L. target/debug/libmpk_ffi.dylib -o tests/mpk_ffi_test"
  exec "tests/mpk_ffi_test"
  rmFile("tests/mpk_ffi_test")
  rmFile("/tmp/mpk.toml")
