#!/usr/bin/env nim --skipProjCfg --skipParentCfg --skipUserCfg --hints:off
# MPK dev tools
import std/distros
import std/os

proc checkDeps() =
  for d in ["rustc", "cargo", "jackd", "emacs"]:
    let exe = findExe(d)
    if exe == "":
      echo "dependency not found: ", d
    else:
      echo "found ", d, ": ", exe

proc hostInfo() =
  echo "Current Host ::",
     "\n\tOS: ", hostOS,
     "\n\tCPU: ", hostCPU,
     "\n\tcpuEndian: ", cpuEndian,
     "\n\tNimVersion: ", NimVersion,
     "\n\t", gorgeEx("rustc --version").output,
     "\n\t", gorgeEx("cargo --version").output,
     "\n\temacs ", gorgeEx("emacs -Q --batch --eval '(message emacs-version)'").output

hostInfo()
checkDeps()
echo foreignDepInstallCmd("jack")
