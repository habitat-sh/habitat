FROM ubuntu:wily
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
    npm \
    pkg-config \
    protobuf-compiler \
    sudo \
    tmux \
    vim \
    wget \
  && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME /cargo-cache
ENV PATH $PATH:$CARGO_HOME/bin:/root/.cargo/bin

ARG HAB_DEPOT_URL
ENV HAB_DEPOT_URL ${HAB_DEPOT_URL:-}

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y && rustc -V
RUN curl -sSL https://get.docker.io | sh && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN (adduser --system hab || true) && (addgroup --system hab || true)

RUN env -u CARGO_HOME cargo install protobuf && rm -rf /root/.cargo/registry

COPY .delivery/scripts/ssh_wrapper.sh /usr/local/bin
COPY .delivery/scripts/git_src_checkout.sh /usr/local/bin
COPY components/studio/install.sh /tmp
COPY support/init.sh /init.sh
RUN /tmp/install.sh \
  && hab-bpm install core/busybox-static \
  && (cd /tmp && curl -sLO https://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/core-20160423193745.pub) \
  && chmod 755 /init.sh \
  && rm -f /tmp/install.sh /hab/cache/artifacts/*

WORKDIR /src
# This entrypoint is temporary until origin key download on install is implemented
ENTRYPOINT ["/init.sh"]
CMD ["bash"]
