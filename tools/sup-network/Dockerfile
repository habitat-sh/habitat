FROM ubuntu:18.04

RUN apt-get update

RUN apt-get install -y \
            libczmq-dev \
            libssl-dev

RUN apt-get install -y \
            jq \
            curl

RUN useradd --user-group hab
