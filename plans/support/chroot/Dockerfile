FROM alpine
MAINTAINER The Bldr Maintainers <bldr@chef.io>

RUN apk add --update \
    make \
    sudo \
  && rm -rf /var/cache/apk/*

COPY Makefile /tmp-plans/
COPY lfs-tools /tmp-plans/lfs-tools
COPY scripts /tmp-plans/scripts

ENV CHROOT /mnt/chroot
ENV PLANS /plans

ARG use_bootstrap_tools
ENV use_bootstrap_tools ${use_bootstrap_tools:-}

RUN cd /tmp-plans \
  && make chroot NO_MOUNT=true \
  && rm -rf /tmp-plans
