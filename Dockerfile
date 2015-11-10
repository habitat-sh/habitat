FROM ubuntu:latest
MAINTAINER Adam Jacob <adam@chef.io>

ENV TRIPLE x86_64-unknown-linux-gnu

RUN apt-get update && apt-get install -y --no-install-recommends \
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
  && rm -rf /var/lib/apt/lists/*

ENV SHELL /bin/bash
ENV CARGO_HOME /bldr-cargo-cache

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y && rustc -V
RUN curl -sSL https://get.docker.io | sh && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN adduser --system bldr
RUN addgroup --system bldr

WORKDIR /src
CMD ["bash"]
