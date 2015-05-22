FROM ubuntu:latest
MAINTAINER Adam Jacob <adam@chef.io>
ENV TRIPLE x86_64-unknown-linux-gnu
RUN apt-get update \
  && apt-get install -y curl gdb file build-essential gnupg rsync libncurses5-dev libncursesw5-dev libssl-dev gawk wget man
ENV SHELL /bin/bash
RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- -y
ENV CARGO_HOME /bldr-cargo-cache
WORKDIR /src
CMD ["bash"]
