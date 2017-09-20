---
title: Habitat Studio Artifact Caching
date: 2017-07-28
author: fnichol
tags: studio, habitat, update
category: studio
classes: body-article
---

Habitat's Studio now supports artifact caching across Studios!

![tenor-101473861](https://user-images.githubusercontent.com/261548/28220704-bf87b39a-687d-11e7-9d38-34509d5c6be2.gif)

This change adds a new capability to the Studio software which allows the downloaded Habitat artifacts (i.e. `*.hart` files) to be shared between different Studio instances or the between setup and tear downs of the same Studio instance. The sharing is accomplished by mounting in a common artifact cache directory into each Studio so that all artifact downloads get combined into one location for the benefit of all instances.

For non-root users using Linux and for Windows and Mac users, a directory of `$HOME/.hab/cache/artifacts` will now be created and mounted into the Studio's artifact cache directory which is always at `/hab/cache/artifacts`. For a root user using Linux, the host's system `/hab/cache/artifacts` directory will be mounted into the Studio--the assumption we make in Habitat is that the root user would have their own home directory configuration that is different from the system's.

The net effect of this change is that users of the Studio should see a substantial reduction in bandwidth usage when building software. A couple of things to note however:

* The install logic will still (as before) check Builder to ensure that we still have the latest artifact cached. If a newer artifact exists in Builder, it will still be downloaded. Therefore, as of this feature, you will still need internet access when running Studio builds. Any potential future "offline" feature would require this caching as a prerequisite which is why it has been implemented first.
* While an artifact may be pre-cached, the Studio code still needs to install it into the Studio instance, meaning that on-disk extraction and artifact verification is still being performed every time (as before).
* While the benefits of saving on re-downloads is great, we introduce a new resource issue for users to keep an eye on: the size of their `~/.hab/cache/artifacts` directory.

![tenor-27372330](https://user-images.githubusercontent.com/261548/28220761-e961c5d4-687d-11e7-8441-008c69e6ef39.gif)

Studio Usage Additions
----------------------

The following new flags and options are added to the `hab-studio` program which gets called directly on Linux when using `hab studio` or `hab pkg build` calls:

* `-N` Do not mount the source artifact cache path into the Studio (default: mount the path).
* `-a <ARTIFACT_PATH>` Sets the source artifact cache path (default: `/hab/cache/artifacts`). For non-root users the default will be `$HOME/.hab/cache/artifacts`.

As well, and to match the patterns already established in the Studio codebase, the following Studio-only environment variables are added:

* `ARTIFACT_PATH` - Sets the source artifact cache path (\`-a' option overrides). This is named to line up with the `SRC_PATH` environment variable already in use.
* `NO_ARTIFACT_PATH` - If set, do not mount the source artifact cache path (\`-N' flag overrides).

Want to know more? You can [check out the PR & read the code here!](https://github.com/habitat-sh/habitat/pull/2737)
