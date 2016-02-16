FROM busybox
MAINTAINER The Bldr Maintainers <bldr@chef.io>

COPY studio-install.sh /tmp/

ARG BLDR_REPO
ENV BLDR_REPO ${BLDR_REPO:-}

ARG STUDIO_TYPE
ENV STUDIO_TYPE ${STUDIO_TYPE:-bldr}

RUN /bin/sh /tmp/studio-install.sh \
  && mkdir -pv /src /cache \
  && cd /src \
  && env NO_MOUNT=true /usr/bin/studio -t $STUDIO_TYPE new \
  && rm -rfv /tmp/* /opt/studios/src/opt/bldr/cache/*

WORKDIR /src
CMD mkdir -p /opt/studios/src/opt/bldr/cache \
  && mount --bind /cache /opt/studios/src/opt/bldr/cache \
  && exec /usr/bin/studio enter
