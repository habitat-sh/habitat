FROM alpine
ENV HAB_LICENSE=accept-no-persist
ARG HAB_VERSION=
ARG PACKAGE_TARGET
RUN set -ex \
  && apk add --no-cache --virtual .build-deps \
    ca-certificates \
    gnupg \
    curl \
    bash \
  \
  && cd /tmp \
  && curl https://raw.githubusercontent.com/habitat-sh/habitat/refs/heads/v1.6/components/hab/install.sh -o install.sh \
  && bash install.sh -t ${PACKAGE_TARGET} \
  && rm -rf install.sh /hab/cache /root/.wget-hsts /root/.gnupg \
  && apk del .build-deps

RUN --mount=type=secret,id=hat HAB_AUTH_TOKEN=$(cat /run/secrets/hat) hab pkg install --channel="dev-v1.6" core/hab