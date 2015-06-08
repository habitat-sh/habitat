# Bldr

# Demo

## Problem statement

How do we build, run, and manage our applications in a way that provides:

* Repeatable builds
* Single, immutable assets
* Runtime configuration for multiple deployment scenarios
* Agnostic to operating enviornment (works on bare metal, virtualiation, containers, PaaS)
* Idempotent behavior (the same inputs to the same asset provides the same outcome)
* Convergent behavior (each service makes progress towards the correct behavior in the face of failure)
* Exposes promises to those who rely on it
* Low barrier to entry
* Language agnostic

## Application artifact as closure

Historically, we build our applications as a conglomeration of upstream artifacts. We have the operating
system we used, which provides all of our build (and often run) time dependencies. We then layer in the
specific application (either one we wrote ourselves, or a version of someone elses software), and then we
layer in the details of how to configure and manage that application within its environment (with something
like Chef). Much of the complexity in the configuration layer comes from dealing with the large variety
in the upstream - with no consistent way to express what it means to be well managed, we are forced to
provide one.

What Bldr provides is the ability to have the application artifact as a closure of all of this behavior -
from how it is built to how it is configured and run. It takes a build description (which includes
dependencies,) an exhaustive set of configuration options, and a hosting platform for the service - wraps
them into a single, encrypted or signed artifact, and enables it to be configured dynamically when the
services are started.

The side effect is that the boundary for idempotency, convergence, and promises shifts from the invidual
details of the application stack to the artifact itself. Given the same input data (regardless of source)
we will run the application the same way everywhere, the artifact itself handles making best progress
towards its goal, and exposes consistent interfaces for health and monitoring.

## Hello world container

Let's contrast Bldr's approach with a similar style of work, the Dockerfile. The "hello world" of the
container ecosystem is Redis - a simple key/value store. Here is the dockerfile:

## What Bldr does for you

* Automatically build a minimal environment for your application
* Include dependencies as binary artifacts
* Specify all the configurable options for the application
* Configure them from a file, the environment, or a service discovery framework (etc/consul/chef) - in real time
* Ensure privilege seperation (supervisor de-privileges the service on your behalf)
* Integrates logging cleanly
* Provides pluggable interfaces for critical side-car behavior:
  * Status (up/down/etc)
  * Health checks
  * Smoke testing
  * Monitoring
  * Backups

With the same amount of effort required to put your applicatin in a Dockerfile. Or less.

# Try it

Below is a lot of rambling thoughts, some of which are now completely wrong,
but they were nice thoughts when I had them. If you want to try things out
right now, here is what you should try.

First, make sure you have the following things installed:

* docker
* boot2docker
* docker-compose

Make sure you can `boot2docker status` and get a good result.

## The fast way

```bash
$ make all
```

This does everything the slow way does, but tells you less about
how things work under the hood.

## The slow way

Create the data volumes you will need:

```bash
$ make volumes
```

Create the base container for bldr development:

```bash
$ make container
```

Create the bldr packages:

```bash
$ make packages
```

Then test bldr out:

```bash
$ make redis
```

Hit ctrl-c to exit.

## Development

Hack around. If you want to do a build and run tests:

```bash
$ make test
```

Will build and run the test suite in a container.

Two shells are available for your development pleasure. The first is for
making packages:

```bash
$ make pkg-shell
```

Will drop you into a shell with everythihng set to run bldr-build, and drops
packages off in the right location.

The second is for bldr development:

```
$ make shell
```

Which isn't all the fancy things you need for package development.

You can also build directly with cargo on OSX, but I doubt that package
development will work correctly.

## Making packages

You can make packages by entering the package shell, going to bldr-build, and doing:

```bash
$ gpg --import chef-private.gpg
$ ./bldr-build FILE
```

You can also build all the packages in one go with:

```bash
$ make world
```

From the bldr-build directory.

# Headline

Packaging for applications -< this is a bad headline!

# Thesis

We are moving to the idea of an application, along with any dependent software and configuration for it to run, as a single atomic asset. This shift currently starts with the existing operating system toolchain, and goes from there. For example, a typical Dockerfile:

```Dockerfile
FROM debian:wheezy

# add our user and group first to make sure their IDs get assigned consistently, regardless of whatever dependencies get added
RUN groupadd -r redis && useradd -r -g redis redis

RUN apt-get update \
    && apt-get install -y curl \
    && rm -rf /var/lib/apt/lists/*

# grab gosu for easy step-down from root
RUN gpg --keyserver pool.sks-keyservers.net --recv-keys B42F6819007F00F88E364FD4036A9C25BF357DD4
RUN curl -o /usr/local/bin/gosu -SL "https://github.com/tianon/gosu/releases/download/1.2/gosu-$(dpkg --print-architecture)" \
    && curl -o /usr/local/bin/gosu.asc -SL "https://github.com/tianon/gosu/releases/download/1.2/gosu-$(dpkg --print-architecture).asc" \
    && gpg --verify /usr/local/bin/gosu.asc \
    && rm /usr/local/bin/gosu.asc \
    && chmod +x /usr/local/bin/gosu

ENV REDIS_VERSION 3.0.0-rc6
ENV REDIS_DOWNLOAD_URL https://github.com/antirez/redis/archive/3.0.0-rc6.tar.gz
ENV REDIS_DOWNLOAD_SHA1 37409e04591472088afce2861909dd2e98e9c501

# for redis-sentinel see: http://redis.io/topics/sentinel
RUN buildDeps='gcc libc6-dev make'; \
    set -x \
    && apt-get update && apt-get install -y $buildDeps --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /usr/src/redis \
    && curl -sSL "$REDIS_DOWNLOAD_URL" -o redis.tar.gz \
    && echo "$REDIS_DOWNLOAD_SHA1 *redis.tar.gz" | sha1sum -c - \
    && tar -xzf redis.tar.gz -C /usr/src/redis --strip-components=1 \
    && rm redis.tar.gz \
    && make -C /usr/src/redis \
    && make -C /usr/src/redis install \
    && rm -r /usr/src/redis \
    && apt-get purge -y --auto-remove $buildDeps

RUN mkdir /data && chown redis:redis /data
VOLUME /data
WORKDIR /data

COPY docker-entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

EXPOSE 6379
CMD [ "redis-server" ]
```

The first line is 'FROM debian:wheezy', which brings along an entire minimal
distribution of debian, and its attendant toolchain. We then grab the tooling
needing to get a new package for redis, compile, do some shenanigans, and
eventually wind up with a working build of redis. Meanwhile, lets say we wanted
to re-use this same build of redis on 'regular' infrastructure - could we? The
only thing we loose is isolation, but might gain real benefits (for example,
the ability to have easy system level tuning.)

With Bldr, we take a different approach. We start from the idea that the application
itself is the real unit we want to make first class, and we want to decouple it from
a specific runtime environment. I don't want a Docker application, or a Red Hat
application, or a Mesos application, or a Cloud Foundry application - I just
want my application, and I want it to work (as best it can) in any runtime
environment I choose. I also want it to be trivially easy to build.

With Bldr, we can reduce the above container to the following:

```Dockerfile
FROM bldr
ENV BLDR_CONFIG_SERVICE etcd
RUN bldr install redis --version 3.0.0-rc6
EXPOSE 6379
CMD [ "bldr", "start", "redis" ]
```

If you were to open and compare the two, you would find that the original container
has an entire build toolchain, hundreds of unused libraries, and a huge amount of
wasted effort. Also: it has no standard way of configuring the container at runtime.
Meanwhile, the bldr built container has a tree like the following:

/opt/bldr/pkg/libc-2.20/45ce91c1170a3d968b7f91302590a2c9337347ac5d1c66f8832ac87d84b9f63d/MANIFEST
/opt/bldr/pkg/libc-2.20/45ce91c1170a3d968b7f91302590a2c9337347ac5d1c66f8832ac87d84b9f63d/lib/libc.2.20.so
/opt/bldr/pkg/libc-2.20/45ce91c1170a3d968b7f91302590a2c9337347ac5d1c66f8832ac87d84b9f63d/lib/libc.6.so -> libc.2.20.so
/opt/bldr/pkg/redis-3.3.0/28f7d996163813e507c89cd13aeeaa15ed8bbedcf5b35d8a69e7ba40542b6719/MANIFEST
/opt/bldr/pkg/redis-3.3.0/28f7d996163813e507c89cd13aeeaa15ed8bbedcf5b35d8a69e7ba40542b6719/bin/redis-server
/opt/bldr/pkg/redis-3.3.0/28f7d996163813e507c89cd13aeeaa15ed8bbedcf5b35d8a69e7ba40542b6719/config/default.yaml
/opt/bldr/pkg/redis-3.3.0/28f7d996163813e507c89cd13aeeaa15ed8bbedcf5b35d8a69e7ba40542b6719/config/redis.conf
/opt/bldr/redis/bin/redis-server -> /opt/bldr/pkg/redis-3.3.0/28f7d996163813e507c89cd13aeeaa15ed8bbedcf5b35d8a69e7ba40542b6719/bin/redis-server
/opt/bldr/redis/config

When the container starts, it passes any configuration files in config (other
than DEFAULT.toml) through a [mustache parser](http://mustache.github.io/), and
renders them into /opt/bldr/srvc/redis/config. The service automatically starts with
those configuration files.

It also creates /opt/bldr/srvc/redis/data (for any data it might build) and
/opt/bldr/srvc/redis/CURRENT, with a point to the current redis packages
directory.

If passed a BLDR_CONFIG_SERVICE variable, it will take connect to the correct
service (such as chef server, etcd, consul, etc.), and will look for the keys
to feed to the configuration from there, overriding any values in default.yaml.
Depending on the backend, it will either dynamically update the configuration
on change (for example consul and etc with watches) or peridoically (chef).

Finally, we will look for an environment variable that maps to BLDR_(service).
The value of this variable should be well-formatted TOML data, and it will be
used to override any other value passed to the service. For example, to set
the redis loglevel to debug from the environment:

```bash
$ BLDR_redis='loglevel = "debug"' bldr start redis
```

If you want to pass many options, you can use the shell to feed it well formed
toml from a file:

```bash
# toml in /tmp/config.toml
loglevel = "debug"
tcpbacklog = 121
# 
$ BLDR_redis=$(cat /tmp/config.toml) bldr start redis
```

In addition, bldr start runs a sidecar application in the container. It exposes
the current container configuration as HTML and JSON via HTTP, protected with
basic auth.

What if we don't want to have a container? What if we are going to work on
bare metal, and we don't need even the thin overhead of the container runtime?

Bldr works as native packages as well. Each pkg can be downloaded as a native
operating system package, with its native package management systme. When a
package exposes a service, we generate a systemd configuration file that calls
`bldr start`. It follows identical configuration rules to the container version,
but it looks at /etc/bldr.conf to set the environment variables for the 
CONFIG_SERVICE. These packages follow the following naming convention:

bldr-PKG-SHORTHASH-VERSION

Along with whatever other things are needed by the package. The result is you can
install multiple packages alongside each other, it's easy to see which ones are
related to bldr. When installed alongside each other, multiple Bldr services are
exposed through the same sidecar application.

Bldr packages are omnibus packages on steroids. :)

# build.rb
bldr "myapp" do
  depends "openssl"
  depends "erlang"
  service_start "foo"
  platform "rhel", "windows", "docker", "rkt"
end

# publish.rb
bldr_asset "*"

## Bldr Packages

The path is a sha256sum of:

  * The name of the software
  * The version of the software
  * The build source location
  * The build flags of the software (CFLAGS, LDFLAGS, LD_RUN_PATH)
  * The build file
  * The dependent hashes for deps

## Bldr paths

name: redis
version: 3.0.0
hash: name + version + build-meta + date
derivation: chef (optional)
date: 20150517171505

name!version!date!hash!platform!arch.bldr

redis!3.0.0!20150517171505!935616d14513262cd6bf76f0a29e5fc216b0acab87507c86a701f63917ccc7de!linux!x86_64.bldr
(Use cases for Bldr package metadata)
- latest package (name + version + date)
- latest derivation (name + version + date + derivation)
- specific pacakge (name + hash)

/opt/bldr/cache <- Binary package cache
/opt/bldr/store/NAME!VERSION!DATE!HASH <- The store of uncompressed packages
/opt/bldr/srvc/NAME/current <- Symlink to the store directory


## Bldr commands

### bldr install

```bash
$ bldr install redis
```

Installs the latest version of Redis.

```bash
$ bldr install redis --version 3.0.0
```

Installs the latest version of Redis 3.0.0.

```bash
$ bldr install redis --shasum 935616d14513262cd6bf76f0a29e5fc216b0acab87507c86a701f63917ccc7de
```

Installs the version of Redis built with this specific shasum.

You can specify a specific upstream:

```bash
$ bldr install redis -u http://localhost/935616d14513262cd6bf76f0a29e5fc216b0acab87507c86a701f63917ccc7de-redis-3.0.0.bldr
```

Dependencies are installed automatically.

# Creating new bldr packages

The best way to learn to create a new bldr package is by creating one. Lets port ncurses to bldr.

```bash
$ mkdir -p packages/ncurses
```

## Create the Bldrfile

A good first step is to download the source directly. Make a sha256sum of it. Then write the `packages/ncurses/Bldrfile` as follows:

```bash
pkg_name=ncurses
pkg_version=5.9
pkg_license=('ncurses')
pkg_source=http://ftp.gnu.org/gnu/ncurses/ncurses-5.9.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=9046298fb440324c9d4135ecea7879ffed8546dd1b58e59430ea07a4633f563b
pkg_gpg_key=3853DA6B
pkg_deps=(glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
```

If you were to stop here, we would download ncurses, verify the shasum, then run:

```bash
./configure --prefix=$pkg_prefix
make
make install
```

Since ncurses is a little more complex than that, we are going to override two
of the steps performed by bldr - `build()` and `install()`.

```bash
build() {
  ./configure --prefix=$pkg_prefix \
    --with-shared \
    --with-termlib \
    --with-cxx-binding \
    --with-cxx-shared \
    --without-ada \
    --enable-sigwinch \
    --enable-pc-files \
    --enable-symlinks \
    --enable-widec \
    --without-debug \
    --without-normal \
    --enable-overwrite
  make
}
```

The above configures ncurses appropriately, and calls 'make'

```bash
install() {
  make install
  for x in libform libmenu libncurses libtinfo libpanel; do
    ln -s $pkg_prefix/lib/${x}w.so $pkg_prefix/lib/${x}.so
    ln -s $pkg_prefix/lib/${x}w.so $pkg_prefix/lib/${x}.so.5
    ln -s $pkg_prefix/lib/${x}w.so $pkg_prefix/lib/${x}.so.5.9
  done
}
```

Then we want to actually install ncurses. Since we want every
library to have wide character support no matter what, we need to
link the libraries we have built with the "non wide character"
location (this is SOP).

Viola! Software ported.
