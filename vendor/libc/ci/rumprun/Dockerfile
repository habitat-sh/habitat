FROM ubuntu:15.04

RUN apt-get update
RUN apt-get install -y binutils git g++ make qemu

WORKDIR /build
RUN git clone --recursive http://repo.rumpkernel.org/rumprun
WORKDIR /build/rumprun
RUN CC=cc ./build-rr.sh hw

ENV PATH=$PATH:/build/rumprun/rumprun/bin
WORKDIR /root
