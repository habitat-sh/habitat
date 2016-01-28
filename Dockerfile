FROM ubuntu:latest
MAINTAINER Adam Jacob <adam@chef.io>

ENV TRIPLE x86_64-unknown-linux-gnu

RUN apt-get update && apt-get install -y --no-install-recommends \
    dh-autoreconf \
    build-essential \
    patchutils \
    ca-certificates \
    curl \
    file \
    gawk \
    gdb \
    gnupg \
    libncurses5-dev \
    libncursesw5-dev \
    libssl-dev \
    libssl-doc \
    man \
    npm \
    rsync \
    wget \
    m4 \
    pkg-config \
    libgpgme11-dev \
    libarchive-dev \
    libclang-dev \
  && rm -rf /var/lib/apt/lists/*

ENV SHELL /bin/bash
ENV CARGO_HOME /bldr-cargo-cache

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y && rustc -V
RUN curl -sSL https://get.docker.io | sh && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN adduser --system bldr || true
RUN addgroup --system bldr || true

COPY ssh_wrapper.sh /usr/local/bin/ssh_wrapper.sh
COPY git_src_checkout.sh /usr/local/bin/git_src_checkout.sh

WORKDIR /src
CMD ["bash"]
