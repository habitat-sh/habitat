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
ENV PATH /cargo-cache/bin:$PATH

ARG HAB_DEPOT_URL
ENV HAB_DEPOT_URL ${HAB_DEPOT_URL:-}

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y && rustc -V
RUN curl -sSL https://get.docker.io | sh && docker -v
RUN ln -snf /usr/bin/nodejs /usr/bin/node && npm install -g docco && echo "docco `docco -V`"

RUN (adduser --system hab || true) && (addgroup --system hab || true)

RUN cargo install protobuf

COPY .delivery/scripts/ssh_wrapper.sh /usr/local/bin
COPY .delivery/scripts/git_src_checkout.sh /usr/local/bin
COPY components/studio/install.sh /tmp
RUN /tmp/install.sh && rm -f /tmp/install.sh

WORKDIR /src
CMD ["bash"]
