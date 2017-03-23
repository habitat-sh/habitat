# Habitat

[![Build Status](https://api.travis-ci.org/habitat-sh/habitat.svg?branch=master)](https://travis-ci.org/habitat-sh/habitat)
[![Build status](https://ci.appveyor.com/api/projects/status/ejn8d6bkhiml16al/branch/master?svg=true)](https://ci.appveyor.com/project/chef/habitat/branch/master)
[![Slack](http://slack.habitat.sh/badge.svg)](http://slack.habitat.sh/)

Want to try Habitat? [Get started here](https://www.habitat.sh/try/).

Habitat is an application automation framework that allows you to build
applications that have automation built-in. This provides modern
applications that:

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

## Installation

There are a couple of different way to install Habitat.

You can download it from the [Habitat downloads page](https://www.habitat.sh/docs/get-habitat/).

Once you have downloaded it, follow the instructions on the page for your specific operating system.

If you are running on a Mac or Linux, you can also use the Habitat install script.

```
  $ curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
```

###


## Web Application

The Habitat Builder web application is in the components/builder-web directory. See
[its README](components/builder-web/README.md) for more information.

## Documentation

Habitat's website and documentation is located in the `www` directory of the Habitat source code. See [its README](www/README.md) for more information.

## Contributing
We are always looking for more opportunities for community involvement. Interested in contributing? Check out our [CONTRIBUTING.md](CONTRIBUTING.md) the [Rustlang](https://rust-lang.org) reference materials below or our [UX_PRINCIPLES doc](UX_PRINCIPLES.md)to get started!

## Building
See [BUILDING.md](BUILDING.md) for platform specific info on building Habitat from source.

## Useful reference material

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

Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
