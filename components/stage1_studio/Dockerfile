FROM ubuntu:20.04

ENV PATH=${PATH}:/hab/bin

RUN apt-get update && apt-get install -y libzmq3-dev wget xz-utils curl

RUN mkdir -vp /hab/bin
ADD src/hab /bin
ADD src/hab-launch /bin
ADD src/hab-sup /bin
ADD src/hab-pkg-export-tar /bin
ADD src/hab-pkg-export-container /bin
ADD src/hab-plan-build /hab/bin/hab-plan-build
ADD src/rq /bin
RUN ln -sf /hab/bin/hab-plan-build/hab-plan-build.sh /bin/hab-plan-build
COPY ./default/etc/ /etc/
ADD ./default/entrypoint.sh /
ADD ./default/profile /etc/
ADD ./default/profile.enter /etc/

ENTRYPOINT ["/entrypoint.sh"]
CMD ["enter"]
WORKDIR /src