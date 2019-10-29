FROM chefes/buildkite:latest

# Minimal Docker container for running BATS tests on our CI tooling.
#
# BATS, of course, is needed to run BATS :P

RUN git clone https://github.com/sstephenson/bats /bats && \
    cd /bats && \
    git checkout v0.4.0 && \
    ./install.sh /usr/local
RUN mkdir /test_helper
RUN git clone https://github.com/ztombol/bats-support --branch=v0.3.0 /test_helper/bats-support
RUN git clone https://github.com/ztombol/bats-assert --branch=v0.3.0 /test_helper/bats-assert
RUN git clone https://github.com/ztombol/bats-file --branch=v0.2.0 /test_helper/bats-file

ENV HAB_LICENSE accept-no-persist

# Install modern hab... can get rid of this when chefes/buildkite has
# >= hab 0.88.0
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN /bin/bash -c ./install.sh
