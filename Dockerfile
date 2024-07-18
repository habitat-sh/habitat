# 17.10 (artful) will be EOL July 2018; update FROM directive before then
FROM ubuntu:20.04
MAINTAINER The Habitat Maintainers <humans@habitat.sh>

ENV CARGO_HOME /cargo-cache
ENV PATH $PATH:$CARGO_HOME/bin:/root/.cargo/bin

ARG HAB_BLDR_URL
ENV HAB_BLDR_URL ${HAB_BLDR_URL:-}

ENV DEBIAN_FRONTEND=noniteractive
ENV TZ='Europe/London'

COPY components/hab/install.sh \
  support/linux/install_dev_0_ubuntu_latest.sh \
  support/linux/install_dev_9_linux.sh \
  /tmp/
COPY support/devshell_profile.sh /root/.bash_profile

RUN apt-get update
RUN apt-get install -yq --no-install-recommends sudo \
  && sh /tmp/install_dev_0_ubuntu_latest.sh \
  && sh /tmp/install_dev_9_linux.sh \
  && useradd -m -s /bin/bash -G sudo jdoe && echo jdoe:1234 | chpasswd \
  && rm -rf \
    /tmp/install.sh \
    /tmp/install_dev_0_ubuntu_latest.sh \
    /tmp/install_dev_9_linux.sh \
    /hab/cache \
    /root/.cargo/registry \
    /var/lib/apt/lists/*

WORKDIR /src
CMD ["bash", "-l"]
