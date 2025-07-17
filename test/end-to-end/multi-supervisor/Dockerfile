FROM ubuntu

# Channel from which to install Habitat-related packages
ARG CHANNEL=stable
ARG HAB_AUTH_TOKEN

# Bootstrap the installation of Habitat
RUN apt-get update && apt-get -y install curl
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN export HAB_AUTH_TOKEN="${HAB_AUTH_TOKEN}" \
    && /bin/bash -c ./install.sh -c acceptance -o chef \
    && unset HAB_AUTH_TOKEN

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Ensure that the desired version of `hab` is actually in the image.
RUN --mount=type=secret,id=hab_auth_token \
export HAB_AUTH_TOKEN=$(cat /run/secrets/hab_auth_token) \
&& hab pkg install chef/hab --binlink --force --channel="${CHANNEL}" \
&& hab pkg install core/docker --binlink --force \
&& hab pkg install core/powershell --channel stable --binlink --force \
&& hab pkg install core/pester --channel stable

# For tests where the testing container needs access to Docker to test
# things. If that describes your test, remember to mount /var/run/docker.sock!
