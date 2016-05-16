FROM ubuntu:xenial
MAINTAINER The Habitat Maintainers <humans@habitat.sh>

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
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
    npm \
    pkg-config \
    protobuf-compiler \
    sudo \
    tmux \
    vim \
    wget

ENV CARGO_HOME /cargo-cache
ENV PATH $PATH:$CARGO_HOME/bin:/root/.cargo/bin

ARG HAB_DEPOT_URL
ENV HAB_DEPOT_URL ${HAB_DEPOT_URL:-}

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y \
  && RUST_VERSION=$(rustc -V | cut -d ' ' -f 2) \
  && URL=http://static.rust-lang.org/dist/rust-std-${RUST_VERSION}-x86_64-unknown-linux-musl.tar.gz \
  && mkdir -p /prep/rust-std-musl \
  && (cd /prep && curl -LO $URL) \
  && tar xf /prep/$(basename $URL) -C /prep/rust-std-musl --strip-components=1 \
  && (cd /prep/rust-std-musl && ./install.sh --prefix=$(rustc --print sysroot)) \
  && rm -rf /prep \
  && rustc -V
RUN env -u CARGO_HOME cargo install protobuf && rm -rf /root/.cargo/registry

RUN curl -sSL https://get.docker.io | sh && rm -rf /var/lib/apt/lists/* && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN (adduser --system hab || true) && (addgroup --system hab || true)

COPY .delivery/scripts/ssh_wrapper.sh /usr/local/bin
COPY .delivery/scripts/git_src_checkout.sh /usr/local/bin
COPY components/studio/install.sh /tmp
COPY support/init.sh /init.sh
RUN /tmp/install.sh \
  && hab-bpm install core/busybox-static \
  && (cd /tmp && curl -sLO https://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/core-20160423193745.pub) \
  && chmod 755 /init.sh \
  && rm -rf /tmp/install.sh /hab/cache/artifacts

WORKDIR /src
# This entrypoint is temporary until origin key download on install is implemented
ENTRYPOINT ["/init.sh"]
CMD ["bash"]
