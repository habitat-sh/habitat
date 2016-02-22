FROM ubuntu:latest
MAINTAINER The Bldr Maintainers <bldr@chef.io>

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    dh-autoreconf \
    file \
    gawk \
    gdb \
    gnupg \
    libarchive-dev \
    libclang-dev \
    libncurses5-dev \
    libncursesw5-dev \
    libgpgme11-dev \
    libssl-dev \
    libssl-doc \
    man \
    m4 \
    npm \
    patchutils \
    pkg-config \
    rsync \
    wget \
  && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME /bldr-cargo-cache

ARG BLDR_REPO
ENV BLDR_REPO ${BLDR_REPO:-}

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y && rustc -V
RUN curl -sSL https://get.docker.io | sh && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN adduser --system bldr || true
RUN addgroup --system bldr || true

COPY .delivery/scripts/ssh_wrapper.sh /usr/local/bin/ssh_wrapper.sh
COPY .delivery/scripts/git_src_checkout.sh /usr/local/bin/git_src_checkout.sh
COPY studio/studio-install.sh /tmp
RUN /tmp/studio-install.sh && bpm install chef/bldr && bpm install chef/bpm

WORKDIR /src
CMD ["bash"]
