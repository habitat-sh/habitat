ARG PACKAGE_TARGET
FROM habitat-${PACKAGE_TARGET}:hab-base as hab
ARG HAB_AUTH_TOKEN
ENV PATH=${PATH}:/hab/bin
ARG BLDR_CHANNEL=stable
ARG BLDR_URL=https://bldr.habitat.sh
ARG HAB_LICENSE=no-accept
RUN export HAB_AUTH_TOKEN=$HAB_AUTH_TOKEN \
  && hab pkg install --url ${BLDR_URL} --channel ${BLDR_CHANNEL} chef/hab-backline \
  && hab pkg binlink core/bash -d /hab/bin \
  && hab pkg binlink chef/hab -d /hab/bin \
  && unset HAB_AUTH_TOKEN

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
