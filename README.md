# Habitat

[![Build Status](https://api.travis-ci.org/habitat-sh/habitat.svg?branch=master)](https://travis-ci.org/habitat-sh/habitat)
[![Build status](https://ci.appveyor.com/api/projects/status/ejn8d6bkhiml16al/branch/master?svg=true)](https://ci.appveyor.com/project/chef/habitat/branch/master)
[![Slack](http://slack.habitat.sh/badge.svg)](http://slack.habitat.sh/)

Want to try Habitat? [Get started here](https://www.habitat.sh/tutorials/get-started/demo/).

Habitat is an application automation framework that allows you to build applications that have automation built-in. This provides modern applications that:

* Provide repeatable builds
* Run from single, immutable assets
* Allow for runtime configuration for multiple deployment scenarios
* Are agnostic to operating environment (works the same on bare metal, virtualization, containers, PaaS)
* Provide idempotent behavior (the same inputs to the same asset provide the same outcome)
* Provide convergent behavior (each service makes progress towards the correct behavior in the face of failure)
* Expose promises to those who rely on it
* Provide a low barrier to entry
* Are language agnostic

To learn more about Habitat, please visit the [Habitat website](https://www.habitat.sh).

Participation in the Habitat community is governed by the [code of conduct](https://github.com/habitat-sh/habitat/blob/master/CODE_OF_CONDUCT.md).

# Table of Contents
* [Install](#install)
* [Contribute](#contribute)
* [Documentation](#documentation)
* [Code Organization](#repo-organization)
* [Roadmap](#roadmap)
* [Community and support](#community-and-support)
* [Building](#building)
* [Further reference material](#further-reference-material)
* [License](#license)

## Install

You can download Habitat from the [Habitat downloads page](https://www.habitat.sh/docs/get-habitat/).

Once you have downloaded it, follow the instructions on the page for your specific operating system.

If you are running MacOS and use [Homebrew](https://brew.sh), you can use our official [Homebrew tap](https://github.com/habitat-sh/homebrew-habitat).
```
$ brew tap habitat-sh/habitat
$ brew install hab
```

If you are running Windows and use [Chocolatey](https://chocolatey.org), you can install our [chocolatey package](https://chocolatey.org/packages/habitat)
```
C:\> choco install habitat
```

If you do _not_ run Homebrew or Chocolatey, or if you use Linux, you can use the [Habitat install
script](https://github.com/habitat-sh/habitat/blob/master/components/hab/install.sh) from a bash shell.

```
$ curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
```

## Contribute

We are always looking for more opportunities for community involvement. Interested in contributing? Check out our [CONTRIBUTING.md](CONTRIBUTING.md) to get started!

## Documentation

Get started with the [Habitat tutorials](https://www.habitat.sh/tutorials/) or plunge into the [complete documentation](https://www.habitat.sh/docs/).

## Code Organization

### Core Plans

The Habitat plans that are built and maintained by Habitat's Core Team are in [their own repo.](https://github.com/habitat-sh/core-plans)

### Habitat Supervisor, Builder, and other core components

The code for the Habitat Supervisor, Builder, and other core components are in the [components directory](https://github.com/habitat-sh/habitat/tree/master/components).

### Web Application

The code for the Habitat Builder web application is in the components/builder-web directory. See [its README](components/builder-web/README.md) for more information.

### Docs

Habitat's website and documentation source is located in the `www` directory of the Habitat source code. See [its README](www/README.md) for more information.

## Roadmap

The Habitat project's roadmap is public and is on our [community page](https://www.habitat.sh/community/).

The Habitat core team's project tracker is also public and on [Github.](https://github.com/habitat-sh/habitat/projects/1)

## Community and support

* [Habitat Slack](http://slack.habitat.sh)
* [Forums]()
* Community triage is every Tuesday at 10am Pacific. The link to participate is shared in the [Habitat Slack channel](http://slack.habitat.sh), and videos are posted on the [Habitat YouTube channel](https://youtube.com/channel/UC0wJZeP2dfPZaDUPgvpVpSg).

## Builder Dev Env

See [BUILDER_DEV.md](BUILDER_DEV.md) for information on setting up a Builder Dev Environment

## Building
See [BUILDING.md](BUILDING.md) for platform specific info on building Habitat from source.

## Further reference material

* [The Rust Programming Language](http://doc.rust-lang.org/book/)
* [Rust by Example](http://rustbyexample.com/)
* [Introduction to Bash programming](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html)
* [Advanced Bash-Scripting Guide](http://www.tldp.org/LDP/abs/html/)
* [Bash Cheat Sheet](http://tldp.org/LDP/abs/html/refcards.html)
* [Writing Robust Bash Shell Scripts](http://www.davidpashley.com/articles/writing-robust-shell-scripts/)
* [Wikibook: Bourne Shell Scripting](https://en.wikibooks.org/wiki/Bourne_Shell_Scripting)
* [What is the difference between test, \[ and \[\[ ?](http://mywiki.wooledge.org/BashFAQ/031)
* [POSIX Shell Command Language](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)

## License

Copyright (c) 2016 Chef Software Inc. and/or applicable contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
