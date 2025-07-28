FROM ubuntu

# Channel from which to install Habitat-related packages
ARG CHANNEL=dev-v1.6

# Bootstrap the installation of Habitat
RUN apt-get update && apt-get -y install curl
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN /bin/bash -c ./install.sh -o core -c "${CHANNEL}"

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Ensure that the desired version of `hab` is actually in the image.
RUN --mount=type=secret,id=hat HAB_AUTH_TOKEN=$(cat /run/secrets/hat) hab pkg install core/hab --binlink --force --channel="${CHANNEL}"

# For tests where the testing container needs access to Docker to test
# things. If that describes your test, remember to mount /var/run/docker.sock!
RUN hab pkg install core/docker --binlink --force

RUN hab pkg install core/powershell --binlink --force
RUN hab pkg install core/pester
