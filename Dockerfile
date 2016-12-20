FROM ubuntu:yakkety
MAINTAINER The Habitat Maintainers <humans@habitat.sh>

ENV CARGO_HOME /cargo-cache
ENV PATH $PATH:$CARGO_HOME/bin:/root/.cargo/bin

ARG HAB_DEPOT_URL
ENV HAB_DEPOT_URL ${HAB_DEPOT_URL:-}

COPY components/hab/install.sh \
  support/linux/install_dev_0_ubuntu_latest.sh \
  support/linux/install_dev_9_linux.sh \
  /tmp/
COPY support/devshell_profile.sh /root/.bash_profile

RUN apt-get update \
  && apt-get install -y --no-install-recommends sudo \
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
