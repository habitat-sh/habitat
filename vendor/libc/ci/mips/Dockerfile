FROM ubuntu:15.10

RUN apt-get update
RUN apt-get install -y --force-yes --no-install-recommends \
        software-properties-common
RUN add-apt-repository ppa:angelsl/mips-cross
RUN apt-get update
RUN apt-get install -y --force-yes --no-install-recommends \
        gcc-5-mips-linux-gnu libc6-dev-mips-cross \
        gcc-5-mipsel-linux-gnu libc6-dev-mipsel-cross
RUN apt-get install -y --force-yes --no-install-recommends \
                build-essential qemu-user
