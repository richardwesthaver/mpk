# MPK dev tools
--hints:off

import std/distros
import std/os
import std/sequtils
from std/strutils import join

const
  v {.booldefine.} = false # verbose
  p {.strdefine.}: string = "" # package
  m {.strdefine.}: string = ""
  ffi {.booldefine.} = false
  all {.booldefine.} = false
  release {.booldefine.} = false
  fastexport {.strdefine.}: string = expandTilde("~/stash/fast-export/hg-fast-export.sh")
  stash {.strdefine.}: string = expandTilde("~/stash")
  rs {.booldefine.} = true
  f {.booldefine.} = false
  MPK_BIN = "bin"

proc getVcRoot(): string =
  ## Try to get the path to the current VC root directory.
  ## Return ``projectDir()`` if a ``.hg`` or ``.git`` directory is not found.
  const
    maxAttempts = 10
  var
    path = projectDir()
    attempt = 0
  while (attempt < maxAttempts) and (not (dirExists(path / ".hg") or (dirExists(path / ".git")))):
    path = path / "../"
    attempt += 1
  if dirExists(path / ".hg"):
    result = path
  elif dirExists(path / ".git"):
    result = path
  else:
    echo "no VC root found, defaulting to projectDir"
    result = projectDir()  

var
  target_dir = "target/debug"
  build_dir = getVcRoot() / "build"

when defined(Windows):
  let ext = ".dll"
elif defined(Linux):
  let ext = ".so"
elif defined(MacOsX):
  let ext = ".dylib"
let 
  ffi_lib = "libmpk_ffi" & ext
  ffi_h = "mpk_ffi.h"
  mpk_py = "build.py"

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

proc rustTest() =
  var args: seq[string]
  when defined(p):
    args.add(" -p " & p)
  when defined(v):
    args.add(" -- --show-output")
  exec "cargo test" & args.join
  
proc ffiTest() =
  let ffi_test = "mpk_ffi_test"
  exec "LD_RUN_PATH=" & '"' & build_dir & '"' & " gcc tests/mpk_ffi_test.c -I" & build_dir & " -L" & build_dir & " -lmpk_ffi -o " & build_dir / ffi_test
  cpFile("tests" / ffi_test & ".py", build_dir / ffi_test & ".py")
  exec "cd " & build_dir & " && python3 " & build_dir / ffi_test & ".py"
  exec build_dir / ffi_test
  rmFile("/tmp/mpk.toml")  

task build, "build MPK":
  withDir getVcRoot():
    var args: seq[string]
    when release:
      args.insert(" --release")
      target_dir = "target/release"
    when defined(p):
      args.add(" -p " & p)
    exec "cargo build" & args.join
    mkDir(build_dir)
    cpFile(target_dir / ffi_lib, build_dir / ffi_lib)
    if fileExists(build_dir / mpk_py):
      exec "cd " & build_dir & " && " & "python3 " & mpk_py

task run, "run MPK binary":
  withDir getVcRoot():
    var args: seq[string]
    when release:
      args.insert("--release ")
    if p == "mpk" or not defined(p):
      args.add("--bin mpk ")
    elif p == "daemon" or p == "mpkd":
      args.add("--bin mpkd ")
    exec "cargo run " & args.join

task install, "install MPK":
  withDir getVcRoot():
    var args: seq[string]
    when defined(f):
      args.insert(" --force")
    when rs:
      args.insert(" --bins")
      exec "cargo install " & args.join " --path " & MPK_BIN

task clean, "clean build artifacts":
  withDir getVcRoot():
    exec "cargo clean"
    rmDir(build_dir)
    rmFile("Cargo.lock")
    echo "root cleaned"

task test, "run MPK tests":
  withDir getVcRoot():
    if not dirExists(build_dir):
      buildTask()
    if ffi:
      ffiTest()
    elif all:
      rustTest()
      ffiTest()
    else:
      rustTest()

task bench, "run MPK benchmarks":
  withDir getVcRoot():
    withDir "tests/benches":
      exec "cargo bench"

task info, "print system, dependency, and project info":
  hostInfo()
  echo ""
  checkDeps()

task status, "print hg status":
  withDir getVcRoot():
    exec "hg status"

task ar, "add/remove files":
  withDir getVcRoot():
    exec "hg addremove ."

task ci, "commit and push changes":
  withDir getVcRoot():
    echo "message: "
    exec("hg ci -m '" & readLineFromStdin() & "'")
    exec "hg push"

task pull, "pull changes from https://hg.rwest.io/mpk":
  withDir getVcRoot():
    exec("hg pull -u")

task fmt, "format code":
  withDir getVcRoot():
    exec "cargo fmt"
    exec "black ."

task check, "check code lints":
  withDir getVcRoot():
    exec "cargo clippy"

task ox, "export readme to GitHub-flavored Markdown":
  withDir getVcRoot():
    exec "emacs --eval '(progn (find-file \"org/README.org\") (org-gfm-export-to-markdown) (rename-file \"README.md\" \"../README.md\" t) (save-buffers-kill-terminal))'"

task mirror, "push code to github mirror":
  withDir stash:
    exec "git init mpk"
    withDir "mpk":
      exec "git config core.ignoreCase false"
      exec "git config push.followTags true"
      exec fastexport & " -r " & getVcRoot() & " -M default"
      exec "git checkout HEAD"
      exec "git remote add gh git@github.com:richardwesthaver/mpk.git"
      var args: seq[string]
      when defined(f):
        args.add("--force")
      exec "git push gh --all --force " & args.join
        
    rmDir("mpk")
