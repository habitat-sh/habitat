# Building Habitat from source

## Not officially supported

**These setups are NOT officially supported by the Habitat maintainers - use at your own risk!**

**The officially supported setup can be found in the [Builder Dev](BUILDER_DEV.md) file.**

## Mac OS X for Native Development

These instructions assume you want to develop, build, and run the various
Habitat software components in a macOS environment. Components other than
the `hab` CLI itself aren't all supported on Mac, but it is sometimes useful
to run parts of the Habitat ecosystem without virtualization. We recommend
using another environment.

First clone the codebase and enter the directory:

```
git clone https://github.com/habitat-sh/habitat.git
cd habitat
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/mac/install_dev_0_mac_latest.sh
sh support/mac/install_dev_9_mac.sh
. ~/.profile
export PKG_CONFIG_PATH="/usr/local/opt/libarchive/lib/pkgconfig:/usr/local/opt/openssl/lib/pkgconfig:/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
export IN_DOCKER=false
make
```

For the builds to find the libarchive and openssl libraries, you will need to
set the `PKG_CONFIG_PATH` environment variable as above before running `cargo
build`, `cargo test`, etc. Additionally to use the Makefile on Mac and not have
the tasks execute in the Docker-based devshell (see above), you will need to
set `IN_DOCKER=false` in your environment. If you use an environment switcher
such as [direnv](https://direnv.net/), you can set up the following in the root
of the git repository:

```
echo 'export PKG_CONFIG_PATH="/usr/local/opt/libarchive/lib/pkgconfig:/usr/local/opt/openssl/lib/pkgconfig:/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"' > .envrc
direnv allow
```

## Windows

These instructions are based on Windows 10 1607 (Anniversary update) or newer.
Most of it will probably work downlevel.

All commands are in PowerShell unless otherwise stated.  It is assumed that you
have `git` installed and configured.  Posh-Git is a handy PowerShell module for
making `git` better in your PowerShell console (`install-module posh-git`).

```
# Clone the Habitat source
git clone https://github.com/habitat-sh/habitat.git

cd habitat
./build.ps1 components/hab -configure
```
