FROM ubuntu:18.10

RUN mkdir -p /hab/sup/default/config

COPY scripts/* /usr/bin/
COPY files/log.yml /hab/sup/default/config/log.yml

RUN apt-get update

RUN apt-get install -y \
            libarchive-dev \
            libczmq-dev \
            libsodium-dev \
            libssl1.0-dev

RUN apt-get install -y \
            jq \
            curl

RUN useradd --user-group hab
