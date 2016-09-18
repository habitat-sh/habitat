FROM ubuntu:xenial
MAINTAINER The Habitat Maintainers <humans@habitat.sh>

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    cmake \
    curl \
    file \
    gdb \
    iproute2 \
    libarchive-dev \
    libprotobuf-dev \
    libsodium-dev \
    libssl-dev \
    libczmq-dev \
    man \
    musl-tools \
    net-tools \
    npm \
    pkg-config \
    protobuf-compiler \
    redis-server \
    software-properties-common \
    sudo \
    tmux \
    vim \
    wget

ENV CARGO_HOME /cargo-cache
ENV PATH $PATH:$CARGO_HOME/bin:/root/.cargo/bin

ARG HAB_DEPOT_URL
ENV HAB_DEPOT_URL ${HAB_DEPOT_URL:-}

RUN curl -sSf https://sh.rustup.rs \
    | env -u CARGO_HOME sh -s -- -y --no-modify-path --default-toolchain stable \
  && env -u CARGO_HOME rustup target add x86_64-unknown-linux-musl \
  && rustc -V
RUN URL=https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz \
  && mkdir -p /prep/cargo \
  && curl -s $URL | tar xvzfm - -C /prep \
  && /prep/cargo-nightly-x86_64-unknown-linux-gnu/install.sh \
  && rm -rf /prep \
  && cargo --version
RUN env -u CARGO_HOME cargo install protobuf && rm -rf /root/.cargo/registry

RUN curl -sSL https://get.docker.io | sh && rm -rf /var/lib/apt/lists/* && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN (adduser --system hab || true) && (addgroup --system hab || true)

COPY support/devshell_profile.sh /root/.bash_profile
COPY .delivery/scripts/ssh_wrapper.sh /usr/local/bin
COPY components/hab/install.sh /tmp
RUN /tmp/install.sh \
  && hab install core/busybox-static \
  && rm -rf /tmp/install.sh /hab/cache

WORKDIR /src
CMD ["bash", "-l"]
