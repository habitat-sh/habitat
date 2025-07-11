FROM ubuntu:24.04

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ruby ruby-dev build-essential jq zip unzip bats git curl file && \
    rm -rf /var/lib/apt/lists/*

RUN git clone --branch=v0.3.0 https://github.com/bats-core/bats-support /test_helper/bats-support && \
    git clone --branch=v2.1.0 https://github.com/bats-core/bats-assert /test_helper/bats-assert && \
    git clone --branch=v0.2.0 https://github.com/bats-core/bats-file /test_helper/bats-file

ENV HAB_LICENSE=accept-no-persist

COPY components/hab/install.sh /tmp/install.sh

RUN chmod +x /tmp/install.sh && /bin/bash /tmp/install.sh
