# Building a hab Windows Binary

As Habitat currently does not have first class support for the Windows platform, a pragmatic approach has been taken to build a `hab` binary for Windows. A wrapper script called `win-build.ps1` attempts to install any missing pre-requisites and then building a `hab` binary. Currently, the following are required on the Mac system performing the build:

* Chocolatey
* Win32 builds of libarchive, libsodium, bzip2, zlib, xz, and openssl via a custom Chocolatey package 
* vcredist2013 and visualcppbuildtools Chocolatey packages
* Rust msvc nightly

## Usage

```powershell
cd components/hab/win
./win-build.ps1
```
