#!/usr/bin/env -S nim --skipProjCfg --skipParentCfg --skipUserCfg --hints:on
# MPK dev tools
import std/distros
import std/os

echo commandLineParams()

proc askInstall(dep: string) =
  echo "dependency not found: ", dep
  let cmd = foreignDepInstallCmd(dep)
  let prompt = "install with:\t"
  case cmd[1]:
    of true: echo prompt & "sudo " & cmd[0]
    of false: echo prompt & cmd[0]

proc checkDeps() =
  for d in ["rustc", "cargo", "jackd", "emacs"]:
    let exe = findExe(d)
    if exe == "":
      askInstall(d)
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

echo ""
hostInfo()
echo ""
checkDeps()
