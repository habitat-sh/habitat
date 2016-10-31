# Building Habitat from source

## Mac OS X

1. [Install Docker](https://docs.docker.com/engine/installation/mac/#/docker-for-mac) (you'll need
   at least Docker 1.9.)
1. (Optional) Consider adding `eval "$(docker-machine env default)"` to your shell initialization
1. Checkout the source by running `git clone git@github.com:habitat-sh/habitat.git; cd habitat`
1. Run `make`
1. (Optional) Run `make test` if you want to run the tests. This will take a while.

Everything should come up green. Congratulations - you have a working Habitat development environment.

**Note:** The Makefile targets are documented. Run `make help` to show the output. Habitat requires `perl`.

**Optional:** This project compiles and runs inside Docker containers so while
installing the Rust language isn't strictly necessary, you might want a local
copy of Rust on your workstation (some editors' language support require an
installed version). To [install stable
Rust](https://www.rust-lang.org/install.html), run: `curl -sSf
https://static.rust-lang.org/rustup.sh | sh`. Additionally, the project
maintainers use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) for
code formatting. If you are submitting changes, please ensure that your work
has been run through rustfmt. An easy way to install it (assuming you have Rust
installed as above), is to run `cargo install rustfmt` and adding
`$HOME/.cargo/bin` to your `PATH`.


## Ubuntu: Xenial

This installation method uses as many packages from Ubuntu as possible. If you'd like to build additional components from source, see the next section.

```
apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    file \
    gdb \
    git \
    iproute2 \
    libarchive-dev \
    libprotobuf-dev \
    libsodium-dev \
    libssl-dev \
    libczmq-dev \
    man \
    musl-tools \
    npm \
    pkg-config \
    protobuf-compiler \
    sudo \
    wget

curl -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable \
  && rustup target add x86_64-unknown-linux-musl \
  && rustc -V
source $HOME/.cargo/env
(adduser --system hab || true) && (addgroup --system hab || true)
ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

git clone https://github.com/habitat-sh/habitat.git
cd habitat && make
```

- these docs were tested with:

		docker run -it ubuntu:xenial /bin/bash


## Ubuntu: 14.04+

This can be used to build and install on older versions of Ubuntu where libsodium and czmq aren't available.

```
apt-get update && apt-get install -y --no-install-recommends \
    autotools-dev \
    autoconf \
    automake \
    build-essential \
    ca-certificates \
    cmake \
    curl \
    file \
    gdb \
    git \
    iproute2 \
    libarchive-dev \
    libprotobuf-dev \
    libssl-dev \
    libtool \
    libunwind8-dev \
    man \
    musl-tools \
    npm \
    pkg-config \
    protobuf-compiler \
    sudo \
    uuid-dev \
    libpcre3-dev \
    wget

git clone https://github.com/jedisct1/libsodium.git
(cd libsodium && ./autogen.sh && ./configure && make && make install)

git clone git://github.com/zeromq/libzmq.git
(cd libzmq && ./autogen.sh && ./configure --with-libsodium && make install && ldconfig)

git clone https://github.com/zeromq/czmq.git
(cd czmq && ./autogen.sh && ./configure && make install && ldconfig)

curl -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable \
  && rustup target add x86_64-unknown-linux-musl \
  && rustc -V
source $HOME/.cargo/env
(adduser --system hab || true) && (addgroup --system hab || true)
ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

git clone https://github.com/habitat-sh/habitat.git
cd habitat && make
```

- these docs were tested with:

		docker run -it ubuntu:14.04 /bin/bash

## Centos 7

```
# Install the zeromq yum repo
curl http://download.opensuse.org/repositories/home:/fengshuo:/zeromq/CentOS_CentOS-6/home:fengshuo:zeromq.repo > /etc/yum.repos.d/zeromq.repo

# Install common development tools
yum groupinstall -y 'Development Tools'
# install sudo as the Rust installation needs it
yum install -y sudo libarchive-devel protobuf-devel openssl-devel zeromq-devel libczmq1-devel gpm-libs which wget

# pkg-config will be able to find libsodium with the following:
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
# needed for the Habitat binaries to find libsodium at runtime
export LD_LIBRARY_PATH=/usr/local/lib

# Install libsodium from source
git clone https://github.com/jedisct1/libsodium.git
(cd libsodium && ./autogen.sh && ./configure && make && make install)

# Install Rust
curl -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable \
  && rustup target add x86_64-unknown-linux-musl \
  && rustc -V
source $HOME/.cargo/env

# Setup hab user and group
useradd --system hab
groupadd --system hab

# Clone the Habitat source
git clone https://github.com/habitat-sh/habitat.git
cd habitat && make
```

- If you have issues with libsodium at runtime, ensure that you've set `LD_LIBRARY_PATH`:

	     export LD_LIBRARY_PATH=/usr/local/lib

- These docs were tested with:

		  docker run -it centos:centos7 /bin/bash


## Windows
These instructions are based on Windows 10 1607 (Anniversary update) or newer.  Most of it will probably work downlevel.

All commands are in PowerShell unless otherwise stated.  It is assumed that you have `git` installed and configured.  Posh-Git is a handy PowerShell module for making `git` better in your PowerShell console (`install-module posh-git`).

```
# Clone the Windows build script
git clone https://github.com/smurawski/hab-build-script.git

# Clone the Habitat source
git clone https://github.com/habitat-sh/habitat.git

# Install the psake PowerShell module
import-module PowerShellGet -force
install-module psake -force

# Change into build script directory
cd ./hab-build-script

# Build Habitat
invoke-psake
```


## General build notes

- Once make has finished, executables will exist in `/src/target/debug/foo`, where `foo` is the name of an executable (`hab`, `hab-sup`, `hab-depot`, etc).

- Executable names are specified in each components `Cargo.toml` file in a TOML table like this:

		[[bin]]
		name = "hab-depot"


## Windows build notes
The Windows build scripts referenced above and the default build task `invoke-psake` attempts to build the full habitat project as well as validates pre-reqs are installed. This is not always and ideal way to build or test. In some cases the following tasks may be more appropriate.

```
# Build all the currently ported crates
invoke-psake -tasklist build_all

# Build only the current crate in progress
invoke-psake -tasklist current_build

# Test all the currently ported crates
invoke-psake -tasklist test_all

# Run tests on the current crate in progress
invoke-psake -tasklist current_test
```

#### Building the native dependencies

You'll want to start in a fresh PowerShell instance, with the Visual C++ Build Tools paths and environment variables set.

I use a handy `Start-VsDevShell` function in my profile.

```
function Start-VsDevShell {
  cmd.exe --% /k ""C:\Program Files (x86)\Microsoft Visual C++ Build Tools\vcbuildtools.bat" amd64" & powershell
}
```

then

```
Start-VsDevShell
cd ~/source/hab-build-script
invoke-psake -tasklist build_native_deps
```
