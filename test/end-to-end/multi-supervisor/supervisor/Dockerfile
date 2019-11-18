FROM alpine as build_image

# All Supervisor-related packages will be downloaded from this channel
# on Builder.
ARG CHANNEL=stable

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Bootstrap us enough to get a `hab` binary on the box, but then clear
# out the /hab directory. The `hab` binary isn't necessarily the one
# that we want in the final image.
RUN apk add bash \
  && wget https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh \
  && bash install.sh \
  && mv /hab/pkgs/core/hab/*/*/bin/hab /bin/ \
  && rm -Rf /hab

# Install only what's needed to run the Supervisor
# TODO: need a way to parameterize busybox
RUN  hab pkg install --channel="${CHANNEL}" core/hab \
  && hab pkg install --channel="${CHANNEL}" core/hab-sup \
  && hab pkg install --channel="${CHANNEL}" core/hab-launcher \
  && hab pkg install --channel="stable" core/busybox \
  && hab pkg binlink core/hab -d /hab/bin

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
RUN hab pkg binlink core/busybox --dest=/tmp/rootfs/bin

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
