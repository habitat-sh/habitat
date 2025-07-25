ARG PACKAGE_TARGET
FROM habitat-${PACKAGE_TARGET}:hab-base AS hab
ENV PATH=${PATH}:/hab/bin
ARG BLDR_CHANNEL=stable
ARG BLDR_URL=https://bldr.habitat.sh
ARG HAB_LICENSE=no-accept

RUN --mount=type=secret,id=hat HAB_AUTH_TOKEN=$(cat /run/secrets/hat) hab pkg install --url ${BLDR_URL} --channel ${BLDR_CHANNEL} core/hab-backline
RUN --mount=type=secret,id=hat HAB_AUTH_TOKEN=$(cat /run/secrets/hat) hab pkg binlink core/bash -d /hab/bin
RUN --mount=type=secret,id=hat HAB_AUTH_TOKEN=$(cat /run/secrets/hat) hab pkg binlink core/hab -d /hab/bin

FROM scratch
COPY --from=hab /hab /hab
COPY --from=hab /hab/bin /bin
COPY --from=hab /bin/hab /bin/
COPY ./etc/ /etc/
ADD ./entrypoint.sh /
ADD ./profile /etc/
ADD ./profile.enter /etc/
ADD ./build /bin/
ENTRYPOINT ["/entrypoint.sh"]
CMD ["enter"]
WORKDIR /src
