FROM alpine
ARG HAB_VERSION=
ARG PACKAGE_TARGET
RUN set -ex \
  && apk add --no-cache --virtual .build-deps \
    ca-certificates \
    gnupg \
    wget \
    bash \
  \
  && cd /tmp \
  && wget https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh \
  && bash install.sh -t ${PACKAGE_TARGET} \
  && rm -rf install.sh /hab/cache /root/.wget-hsts /root/.gnupg \
  && apk del .build-deps
