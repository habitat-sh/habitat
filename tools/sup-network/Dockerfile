FROM ubuntu:24.04

# Set non-interactive mode to prevent prompts during build
ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/bin:/usr/bin:${PATH}"
ENV HAB_LICENSE=accept

# Combine RUN commands to reduce layers and add cleanup
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libczmq-dev \
        libssl-dev \
        jq \
        curl \
        ca-certificates && \
    # Clean up apt cache to reduce image size
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# This useradd mirrors the one used in our on-prem-builder repo
RUN useradd --system --no-create-home hab

# install stable hab
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh | bash
RUN hab pkg install core/hab-sup
RUN hab pkg install core/hab-launcher
