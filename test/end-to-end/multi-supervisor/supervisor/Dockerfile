# syntax=docker/dockerfile:1
FROM alpine as build_image

# All Supervisor-related packages will be downloaded from this channel
# on Builder.
ARG CHANNEL=stable

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Bootstrap us enough to get a `hab` binary on the box, but then clear
# out the /hab directory. The `hab` binary isn't necessarily the one
# that we want in the final image.
RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && apk add bash wget \
    && wget https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh \
    && bash install.sh -c acceptance \
    && mv /hab/pkgs/chef/hab/*/*/bin/hab /bin/ \
    && rm -Rf /hab \
    && unset HAB_AUTH_TOKEN

# Install only what's needed to run the Supervisor
# TODO: need a way to parameterize busybox
RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && hab pkg install --channel="${CHANNEL}" chef/hab \
    && hab pkg install --channel="${CHANNEL}" chef/hab-sup \
    && hab pkg install --channel="${CHANNEL}" chef/hab-launcher \
    && hab pkg install --channel="base-2025" core/busybox \
    && hab pkg binlink chef/hab -d /hab/bin \
    && unset HAB_AUTH_TOKEN

# Create enough of a filesystem for the Supervisor to operate. We
# can't do this in a `FROM scratch` image because there's no shell to
# run `mkdir`. Thus, we'll build it here, and then copy it into the
# final image later.
WORKDIR /tmp/rootfs
RUN mkdir tmp \
    && mkdir bin \
    && mkdir -p var/tmp \
    && mkdir root

# This is important because the Supervisor relies on busybox being
# present for some functionality. Without these present, some packages
# with auto-generated run hooks won't work.
RUN --mount=type=secret,id=hab_auth_token \
    export HAB_AUTH_TOKEN="$(cat /run/secrets/hab_auth_token)" \
    && hab pkg binlink core/busybox --dest=/tmp/rootfs/bin \
    && unset HAB_AUTH_TOKEN

########################################################################

FROM scratch

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Copy everything we need from the build image
COPY --from=build_image /tmp/rootfs/ /
COPY --from=build_image /hab /hab
COPY --from=build_image /hab/bin /bin
COPY etc/ /etc
ENTRYPOINT ["hab", "sup"]
CMD ["run"]
