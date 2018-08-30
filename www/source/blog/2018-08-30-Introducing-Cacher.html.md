---
title: Introducing Cacher
date: 2018-08-30
author: Blake Irvin
tags: studio, caching, dependencies, build, build performance, local dev
category: Community
classes: body-article
---

"If you wish to make apple pie from scratch, you must first create the universe", and if you wish to build an app using the interpreted language framework of your choice, you usually have to install a whole universe of dependencies. If your build environment is persistent, you probably don't feel the pain of waiting for the universe to download once the initial `pip install` is complete. Subsequent invocations of `pip` will see the local modules you previously downloaded and avoid re-downloading and installing them. For "clean room" build environments, this pain is felt each time we build.

The build performance improvements of persistent build environments bring with them some important caveats. If you don't perform a "clean room" build there's always a chance some stray bits or accidental config changes have crept into the build environment, leading to inconsistent build output. To deal with this, many of us use tools like Habitat Builder or Travis CI that _do_ create the universe each time they bake an apple pie, in Builder's case starting with an empty chroot environment for each build.

While universe-creation ensures consistent results (this is a very good thing in Builder), it's sometimes annoyingly slow for local Studio-based development. A Habitat Studio `build` must perform a brand-new `pip install` download of every Python module our project depends on, on every build. This can add many seconds or even minutes to each build. Frustration with these long waits led to the creation of `cacher`, a package that speeds up local Studio-based development.

`cacher` make use of a Habitat package's ability to define an environment variable and 'push' that variable to any package that depends on it. For more details on how that works see Christopher Maier's blog post [here](https://www.habitat.sh/blog/2017/11/runtime-environment-variables/).

The actual `plan.sh` for `cacher` is only a few lines long:
```
pkg_origin=bixu
pkg_name=cacher
pkg_version="0.1.0"
pkg_maintainer="Blake Irvin <blake.irvin@gmail.com>"
pkg_license=("MIT")
pkg_lib_dirs=("lib")

do_setup_environment() {
  mkdir --parents "/hab/cache/artifacts/studio_cache"
  set_runtime_env XDG_CACHE_HOME "/hab/cache/artifacts/studio_cache"
}

do_build() {
  return 0
}

do_install() {
  return 0
}
```

In this case, we are taking advantage of the fact that `pip`, the Python dependency manager that [smartB](http://www.smartb.eu) (my employer) uses, respects the `XDG_CACHE_HOME` environment variable. The directory `/hab/cache/artifacts` is loopback-mounted into the Studio Docker container, which means that we'll cache our `pip` modules in the same persistent location that Habitat uses to cache `.hart` artifacts.

Here's some performance improvement examples from my (old, slow) MacBook building our (relatively large) Python API, with example both before and after adding `bixu/cacher` to our `pkg_build_deps`:
```
Without cacher:
  api: Build time: 7m59s

With cacher, first run:
  api: Build time: 9m31s

With cacher, second run:
  api: Build time: 6m56s

With cacher, third run:
  api: Build time: 6m4s
```
**Build performance improves in this case by ~25%. ❤**

Right now, `cacher` only supports `pip`-managed dependencies, but any dependency manager who's behavior can be configured using environment variables could be  supported. Pull requests are most welcome!
