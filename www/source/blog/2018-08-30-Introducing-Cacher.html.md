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

`cacher` make use of a Habitat package's ability to define an environment variable and "push" that variable to any package that depends on it. For more details on how that works see Christopher Maier's blog post [here](https://www.habitat.sh/blog/2017/11/runtime-environment-variables/).

The actual `plan.sh` for `cacher` relatively short:
```
pkg_origin=bixu
pkg_name=cacher
pkg_version="0.2.0"
pkg_maintainer="Blake Irvin <blake.irvin@gmail.com>"
pkg_license=("MIT")
pkg_deps=(
  core/coreutils
)


do_setup_environment() {
  # enable caching for Go dependencies
  mkdir --parents                  "/hab/cache/artifacts/studio_cache/go"
  set_runtime_env GOPATH           "/hab/cache/artifacts/studio_cache/go"

  # enable caching for NPM modules
  mkdir --parents                  "/hab/cache/artifacts/studio_cache/npm"
  set_runtime_env npm_config_cache "/hab/cache/artifacts/studio_cache/npm"

  # enable caching for Pip modules
  mkdir --parents                  "/hab/cache/artifacts/studio_cache/pip"
  set_runtime_env XDG_CACHE_HOME   "/hab/cache/artifacts/studio_cache/pip"
}

do_build() {
  return 0
}

do_install() {
  return 0
}

do_end() {
  build_line ""
  build_line "Cache settings:"
  build_line "$(hab pkg exec "$pkg_origin/$pkg_name" env | grep GOPATH)"
  build_line "$(hab pkg exec "$pkg_origin/$pkg_name" env | grep npm_config_cache)"
  build_line "$(hab pkg exec "$pkg_origin/$pkg_name" env | grep XDG_CACHE_HOME)"
  return $?
}
```

In this case, we are taking advantage of the fact that `pip`, the Python dependency manager that [smartB](http://www.smartb.eu) (my employer) uses, respects the `XDG_CACHE_HOME` environment variable. The directory `/hab/cache/artifacts` is loopback-mounted into the Studio Docker container, which means that we'll cache our `pip` modules in the same persistent location that Habitat uses to cache `.hart` artifacts. We use similar techniques for both NPM and Go.

Here's some performance improvement examples from my (old, slow) MacBook building our (relatively large) Python API, with examples both before and after adding `bixu/cacher` to our `pkg_build_deps`:
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

Right now, `cacher` only the depdency managers discussed here, but any dependency manager who's behavior can be configured using environment variables could be  supported in future. Pull requests are most welcome!
