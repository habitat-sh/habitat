FROM ubuntu:wily
MAINTAINER The Habitat Maintainers <humans@habitat.sh>

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    dh-autoreconf \
    file \
    gawk \
    gdb \
    gnupg \
    iproute2 \
    libarchive-dev \
    libclang-dev \
    libncurses5-dev \
    libncursesw5-dev \
    libgpgme11-dev \
    libsodium-dev \
    libssl-dev \
    libssl-doc \
    man \
    m4 \
    npm \
    patchutils \
    pkg-config \
    rsync \
    sudo \
    wget \
    vim \
  && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME /cargo-cache

ARG HAB_DEPOT_URL
ENV HAB_DEPOT_URL ${HAB_DEPOT_URL:-}

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y && rustc -V
RUN curl -sSL https://get.docker.io | sh && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN (adduser --system hab || true) && (addgroup --system hab || true)

COPY .delivery/scripts/ssh_wrapper.sh /usr/local/bin
COPY .delivery/scripts/git_src_checkout.sh /usr/local/bin
# COPY components/studio/install.sh /tmp
# RUN /tmp/install.sh \
#   && hab-bpm install chef/hab-bpm \
#   && hab-bpm binlink chef/hab-bpm hab-bpm \
#   && rm -f /tmp/install.sh
RUN (cd /tmp && wget https://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/core-hab-studio-0.1.0-20160415194717.tar.xz) \
  && (cd / && tar xvJf /tmp/core-hab-studio-0.1.0-20160415194717.tar.xz) \
  && ln -snfv /hab/pkgs/core/hab-studio/0.1.0/20160415194717/bin/hab-studio /usr/bin/hab-studio \
  && rm -fv /tmp/core-hab-studio-0.1.0-20160415194717.tar.xz

WORKDIR /src
CMD ["bash"]
