# Building a hab Mac Binary

As Habitat currently does not have first class support for the Mac platform, a pragmatic approach has been taken to build a `hab` binary for Mac OS X. A wrapper script called `mac-build.sh` attempts to install any missing pre-requisites before invoking the `hab-plan-build.sh` program with a custom `PATH`. Currently, the following are required on the Mac system performing the build:

* Xcode CLI tools
* Homebrew
* `coreutils`, `gnu-tar`, and `wget` Homebrew packages
* Rust
* Cargo nightly

A final prerequisite of a `hab` binary itself is required by `hab-plan-build.sh` to sign the resulting Habitat artifact. The `mac-build.sh` will install the latest release from Bintray if `/bin/hab` cannot be found locally, otherwise the `hab` on your `PATH` will be used. As we are using the build program, a secret origin key is also required in the Mac's key cache under `/hab/cache/keys`.

## Usage

```sh
cd components/hab/mac
sudo ./mac-build.sh
```

Assuming success, this will produce a local `./results` directory with the artifact.

Alternatively, as `mac-build.sh` is a wrapper around the build program, it can be just as easily invoked from the root of the source tree with:

```sh
sudo ./components/hab/mac/mac-build.sh components/hab/mac
```
