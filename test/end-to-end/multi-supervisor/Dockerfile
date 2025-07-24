FROM ubuntu

# Channel from which to install Habitat-related packages 
ARG CHANNEL=stable
ARG HAB_AUTH_TOKEN

# Bootstrap the installation of Habitat
RUN apt-get update && apt-get -y install curl ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN export HAB_AUTH_TOKEN="${HAB_AUTH_TOKEN}" \
    && ./install.sh -c acceptance -o chef \
    && unset HAB_AUTH_TOKEN

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Install core Habitat tools
RUN export HAB_AUTH_TOKEN="${HAB_AUTH_TOKEN}" \
    && hab pkg install chef/hab --binlink --force --channel="${CHANNEL}" \
    && unset HAB_AUTH_TOKEN

# Install Docker CLI for tests (via hab if available)
RUN export HAB_AUTH_TOKEN="${HAB_AUTH_TOKEN}" \
    && hab pkg install core/docker --binlink --force -c base-2025 \
    && unset HAB_AUTH_TOKEN

RUN export HAB_AUTH_TOKEN="${HAB_AUTH_TOKEN}" \
    && hab pkg install core/powershell --binlink --force -c base-2025 \
    && hab pkg install core/pester -c base-2025 \
    && unset HAB_AUTH_TOKEN
