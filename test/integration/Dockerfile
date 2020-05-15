FROM ubuntu:18.04

# Minimal Docker container for running Habitat's BATS integration
#  tests.
#
# jq and curl are used by our custom BATS helper functions; busybox
# and the various lib* are needed by the hab binaries themselves; the
# intended usecase is to exercise the individual binaries built by
# cargo, and not Habitat-packaged binaries (which of course include
# all dependencies).
#
# BATS, of course, is needed to run BATS :P

RUN useradd hab

RUN apt-get update && \
    apt-get -y install jq curl git && \
    apt-get -y install libczmq-dev busybox && \
    git clone https://github.com/sstephenson/bats /bats && \
    cd /bats && \
    git checkout v0.4.0 && \
    ./install.sh /usr/local
