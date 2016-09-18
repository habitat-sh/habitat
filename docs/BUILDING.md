# Building Habitat from source

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
wget -nv https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf cargo-nightly-x86_64-unknown-linux-gnu.tar.gz && \
  sudo cargo-nightly-x86_64-unknown-linux-gnu/install.sh &&
  rm -rf cargo-nightly-x86_64-unknown-linux-gnu \
    cargo-nightly-x86_64-unknown-linux-gnu.tar.gz
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
wget -nv https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf cargo-nightly-x86_64-unknown-linux-gnu.tar.gz && \
  sudo cargo-nightly-x86_64-unknown-linux-gnu/install.sh &&
  rm -rf cargo-nightly-x86_64-unknown-linux-gnu \
    cargo-nightly-x86_64-unknown-linux-gnu.tar.gz
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
yum install -y sudo libarchive-devel protobuf-devel openssl-devel zeromq-devel libczmq1-devel gpm-libs which

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
wget -nv https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf cargo-nightly-x86_64-unknown-linux-gnu.tar.gz && \
  sudo cargo-nightly-x86_64-unknown-linux-gnu/install.sh &&
  rm -rf cargo-nightly-x86_64-unknown-linux-gnu \
    cargo-nightly-x86_64-unknown-linux-gnu.tar.gz

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


## General build notes

- Once make has finished, executables will exist in `/src/target/debug/foo`, where `foo` is the name of an executable (`hab`, `hab-sup`, `hab-depot`, etc).

- Executable names are specified in each components `Cargo.toml` file in a TOML table like this:

		[[bin]]
		name = "hab-depot"
