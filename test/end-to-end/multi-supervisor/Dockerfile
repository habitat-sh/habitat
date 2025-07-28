# syntax=docker/dockerfile:1
FROM ubuntu

# Channel from which to install Habitat-related packages 
ARG CHANNEL=stable

# Bootstrap the installation of Habitat
RUN apt-get update && apt-get -y install curl ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && ./install.sh -c acceptance -o chef \
    && unset HAB_AUTH_TOKEN

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Install core Habitat tools
RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && hab pkg install chef/hab --binlink --force --channel="${CHANNEL}" \
    && unset HAB_AUTH_TOKEN

# Install Docker CLI for tests (via hab if available)
RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && hab pkg install core/docker --binlink --force -c base-2025 \
    && unset HAB_AUTH_TOKEN

RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && hab pkg install core/powershell -c base-2025 \
    && hab pkg binlink core/powershell pwsh --force \
    && hab pkg install core/pester -c base-2025 \
    && unset HAB_AUTH_TOKEN
