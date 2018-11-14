ARG PACKAGE_TARGET
FROM habitat-${PACKAGE_TARGET}:hab-base as hab
ENV PATH=${PATH}:/hab/bin
ARG BLDR_CHANNEL=stable
RUN hab pkg install -c ${BLDR_CHANNEL} core/hab-backline \
  && hab pkg binlink core/bash -d /hab/bin \
  && hab pkg binlink core/hab -d /hab/bin

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
