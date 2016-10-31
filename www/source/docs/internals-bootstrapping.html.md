---
title: Bootstrapping Habitat
---

# Bootstrapping Habitat

This document provides developer documentation on how the Habitat system becomes self-sustaining. It is built upon the work from the [Linux from Scratch](http://www.linuxfromscratch.org/lfs/) project.

This instructions in this document may become rapidly out-of-date as we develop Habitat further. Should you have questions, please join us in [Slack](http://slack.habitat.sh/).

## Part I: Setup

In order to bootstrap the system from scratch, you should be familiar with how the Linux From Scratch project works.

We add the following software to augment the Linux From Scratch toolchain:

* Statically built [BusyBox](https://www.busybox.net/) - used for the unzip implementation
* Statically built [Wget](https://www.gnu.org/software/wget/) with OpenSSL support - used by the build program to download sources
* A copy of curl’s [cacert.pem](https://curl.haxx.se/ca/cacert.pem) certificates - used by wget when connecting to SSL-enabled websites

Finally, we place a recent last-known-good copy of the `hab` binary inside `tools/bin`.

The entire tarball of bootstrap "tools" lives inside the [stage1 studio](https://habitat-studio-stage1.s3.amazonaws.com/habitat-studio-stage1-20160612022150.tar.xz) tarball. This should be unpacked into `/tools` on a Linux host that will serve as the build environment until the system is self-sustaining through the rest of this procedure.

## Part II: Stage 0

### Freshening The stage1 tarball

From time to time and especially with breaking changes to `hab`’s core behavior it is a good idea to update the software in the `habitat-studio-stage1` tarball, even if that means skipping the work of rebuilding the toolchain.

~~~
> docker run --rm -ti -v `pwd`:/src ubuntu:xenial bash
~~~

~~~
# Install xz tools
apt-get update && apt-get install -y xz-utils

# Uncompress the tarball and remove old version of hab
tarxz=/src/habitat-studio-stage1-20160612022150.tar.xz
xz --decompress --keep $tarxz
tar --delete --file=${tarxz/.xz/} tools/bin/hab

# Extract new version of hab in correct path structure
hart=/src/core-hab-0.6.0-20160701014820-x86_64-linux.hart
mkdir -p /tmp/tools/bin
tail -n +6 $hart \
  | xzcat \
  | (cd /tmp/tools/bin && tar x --no-anchored bin/hab --strip-components=7)

# Append new version of hab into tarball
(cd /tmp && tar --append --file=${tarxz/.xz/} tools)

# Rename tarball to current date and recompress with xz
dst=/src/habitat-studio-stage1-$(date -u +%Y%m%d%H%M%S).tar.xz
mv ${tarxz/.xz/} ${dst/.xz/}
xz --compress -9 --threads=0 --verbose ${dst/.xz/}
~~~

## Part III: Stage 1

In this stage, we rebuild all the base packages needed by Habitat using the tools (compiler, etc.) from the existing tools tarball. You will need to have a depot locally running on your system, the latest version of the studio, and you'll need a copy of the core-plans on your local disk.

~~~
rm -rf /hab
./components/hab/install.sh
hab install ~ubuntu/results/core-hab-studio-0.6.0-20160701030246-x86_64-linux.hart
hab origin key generate core
~~~

~~~
git clone https://github.com/habitat-sh/core-plans.git plans
~~~

~~~
export STUDIO_TYPE=stage1
export HAB_STUDIO_ROOT=/hab/studios/stage1
export HAB_ORIGIN=core
hab studio enter
~~~

Now in the stage1 Studio:

~~~
export BUILD=/src/components/plan-build/bin/hab-plan-build.sh
export NO_INSTALL_DEPS=true
export DB_PREFIX=stage1-
rm -f /src/plans/tmp/${DB_PREFIX}*.db
cd /src/plans

time record ${DB_PREFIX}base make base
~~~

~~~
$BUILD libarchive
$BUILD libsodium
$BUILD zeromq
$BUILD ../components/depot
~~~

~~~
hab origin key upload -z userkey -u http://127.0.0.1:9632/v1 \
  /hab/cache/keys/core-*.pub
ls -1 launch-stage1-base-harts/*.hart \
  | while read hart; do hab pkg up -z userkey -u http://127.0.0.1:9632/v1 $hart; done
~~~

## Part IV: Stage 2

In this stage, we rebuild all the base packages needed by Habitat using the tools (compiler, etc.) from the previous stage, thus making the system self-sustaining.

~~~
export STUDIO_TYPE=default
export HAB_STUDIO_ROOT=/hab/studios/stage2
export HAB_ORIGIN=core
export HAB_DEPOT_URL=http://127.0.0.1:9632/v1
hab studio enter
~~~

~~~
export DB_PREFIX=stage2-
rm -f /src/plans/tmp/${DB_PREFIX}*.db
cd /src/plans

time record ${DB_PREFIX}base make base
~~~

~~~
build libarchive
build libsodium
build zeromq
build ../components/depot
~~~

~~~
hab origin key upload -z userkey -u http://127.0.0.1:9632/v1 \
  /hab/cache/keys/core-*.pub
ls -1 launch-stage2-base-harts/*.hart \
  | while read hart; do hab pkg up -z userkey -u http://127.0.0.1:9632/v1 $hart; done
~~~

## Part V: Remaining packages in world

In this stage, we rebuild all of the remaining packages using the base packages from the previous phase. We recommend that this stage be executed on a powerful machine, such as an `c4.4xlarge` on Amazon Web Services (AWS).

Update build host now:

~~~
hab install ~ubuntu/launch-stage2-base-harts/core-hab-0.6.0-20160612082139-x86_64-linux.hart
hab pkg binlink core/hab hab
hab install ~ubuntu/launch-stage2-base-harts/core-hab-studio-0.6.0-20160612082608-x86_64-linux.hart
~~~

~~~
apt-get update
apt-get install -y ruby2.0
find . -name plan.sh | ruby2.0 ./plans/build_order.rb --without-base | cut -d ' ' -f 2 > world_build_order
cp world_build_order all_order
~~~

~~~
export STUDIO_TYPE=default
export HAB_STUDIO_ROOT=/hab/studios/stage3
export HAB_ORIGIN=core
export HAB_DEPOT_URL=http://127.0.0.1:9632/v1
hab studio enter
~~~

~~~
cat all_order | while read plan; do build $plan || break; done
~~~
