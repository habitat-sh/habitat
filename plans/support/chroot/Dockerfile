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

RUN cd /tmp-plans \
  && make metal NO_MOUNT=true \
  && rm -rf /tmp-plans
