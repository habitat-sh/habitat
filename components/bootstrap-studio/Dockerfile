FROM ubuntu:20.04

ENV PATH=${PATH}:/hab/bin

# Install the minimal set of critical dependencies required to bootstrap
# a linux ecosystem on a new environment. Since we want to do this all using
# habitat, we also need habitat runtime dependencies like libzmq3-dev, wget & xz-utils
RUN apt-get update && DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get install -y libzmq3-dev wget xz-utils curl make gcc file patch gawk bison python3 g++ texinfo

RUN mkdir -vp /hab/bin

# Copy the version
ADD build-tools-version-check.sh /bin/build-tools-version-check

# Copy all the habitat binaries
ADD files/hab /bin
ADD files/hab-launch /bin
ADD files/hab-sup /bin
ADD files/hab-pkg-export-tar /bin
ADD files/hab-pkg-export-container /bin

# Copy the habitat plan build shell scripts
ADD files/hab-plan-build /hab/bin/hab-plan-build

# Copy the record-query binary built using cargo
ADD files/rq /bin

# Copy the default bash environment configurations
COPY ./default/etc/ /etc/
ADD ./default/entrypoint.sh /
ADD ./default/profile /etc/
ADD ./default/profile.enter /etc/

# Additional changes
#   - Link the habitat plan build script as the build command
#   - Make the build-tools-version-check executable
#   - Fix the /bin/sh link to point to bash, required during the bootstrapping build process
RUN ln -sf /hab/bin/hab-plan-build/hab-plan-build.sh /bin/build && chmod +x /bin/build-tools-version-check && ln -sf bash /bin/sh

WORKDIR /src
ENTRYPOINT ["/entrypoint.sh"]
CMD ["enter"]
