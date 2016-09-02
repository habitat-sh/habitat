# Building Habitat from source

*NOTE:* This document is likely to change a lot over the coming months.

*NOTE:* For Rust editing, I'd recommend Visual Studio Code and have [a blog post on how to set up the best Rust editing experience](http://stevenmurawski.com/powershell/2016/08/rustify).

This document is based on Windows 10 1607 (Anniversary update) or newer.  Most of it will probably work downlevel.

All commands are in PowerShell unless otherwise stated.  It is assumed that you have `git` installed and configured.  Posh-Git is a handy PowerShell module for making `git` better in your PowerShell console (`install-module posh-git`).

```
# install chocolatey
iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

# get the Visual Studio 2013 C++ Runtime (required for the MSVC-based Rust toolchain)
choco install vcredist2013

# get the Visual Studio 2015 C++ Build Tools
# this is what Rust will use to compile MSVC compatible code.
choco install visualcppbuildtools --version 14.0.25123

# several cmake based builds
choco install cmake.portable

## a few more pre-reqs for the OpenSSL builds
choco install StrawberryPerl
choco install nasm

# Install rustup to manage your Rust toolchain.
invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile "~/Downloads/rustup-init.exe"
~/Downloads/rustup-init.exe

# Get the nightly MSVC version of Rust
# Latest seems to have issues.  The last nightly I've tested is 
# nightly-2016-08-22-x86_64-pc-windows-msvc
rustup install nightly-2016-08-22-x86_64-pc-windows-msvc

# Install the psake PowerShell module
# I'm currently using psake until we can transition to 
# habitat builds natively
import-module PowerShellGet -force
install-module psake

# Move to where you want to download the source
# for this example, I'll use ~/source
cd ~/source

# Grab the current build scripts
git clone https://github.com/smurawski/hab-build-script

# Grab the latest source
git clone https://github.com/habitat-sh/habitat.git
cd habitat 
rustup override set nightly-2016-08-22-x86_64-pc-windows-msvc 
```
## To Build

You'll want to start in a fresh PowerShell instance, with the Visual C++ Build Tools paths and environment variables set.

I use a handy `Start-VsDevShell` function in my profile.

```
function Start-VsDevShell {
  cmd.exe --% /k ""C:\Program Files (x86)\Microsoft Visual C++ Build Tools\vcbuildtools.bat" amd64" & powershell
}
```

```
# Build
Start-VsDevShell
cd ~/source/hab-build-script
invoke-psake
```

