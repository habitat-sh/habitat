# Habitat

[![Build Status](https://api.travis-ci.org/habitat-sh/habitat.svg?branch=master)](https://travis-ci.org/habitat-sh/habitat)
[![Slack](http://slack.habitat.sh/badge.svg)](http://slack.habitat.sh/)
[![Stories in Ready](https://badge.waffle.io/habitat-sh/habitat.png?label=ready&title=Ready)](https://waffle.io/habitat-sh/habitat)

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

The remainder of this README focuses on developers who want to modify
the source code of Habitat.

# Working on Habitat

## Rules for working on Habitat

1. The principle of least abstraction. When possible, we use the tooling that is closest to the native
tooling for the platform, and provide as little abstraction as necessary. When we do choose an abstraction,
we choose one - and we make it the one that is most user-serviceable.
1. Keep it light. The runtime component of Habitat is used as a process supervisor - it needs to stay lean. No run-times.
1. Convention over configuration, with reasonable defaults. Where possible, we remove the need to configure things
by having a convention cover it. When we do need to configure things, we set reasonable defaults.
1. Call things what they are.
1. It has to feel great to the end user. If it doesn't feel great, it's a bug.
1. Write documentation as you go. Internal and external.

## Useful reference material

* [The Rust Programming Language](http://doc.rust-lang.org/book/)
* [Rust by Example](http://rustbyexample.com/)
* [Introduction to Bash programming](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html)
* [Advanced Bash-Scripting Guide](http://www.tldp.org/LDP/abs/html/)
* [Writing Robust Bash Shell Scripts](http://www.davidpashley.com/articles/writing-robust-shell-scripts/)
* [Wikibook: Bourne Shell Scripting](https://en.wikibooks.org/wiki/Bourne_Shell_Scripting)
* [What is the difference between test, \[ and \[\[ ?](http://mywiki.wooledge.org/BashFAQ/031)

## Setup On Mac OS X

1. [Install Docker Toolbox](https://docs.docker.com/toolbox/toolbox_install_mac/) (you'll need
   at least Docker 1.9 and docker-machine 0.5. Both are included in Docker
   Toolbox)
1. [Install VMWare Fusion](https://www.vmware.com/products/fusion.html)
1. Delete the default docker-machine image: `docker-machine rm default`
1. Create a new one with vmware fusion: `docker-machine create -d vmwarefusion --vmwarefusion-memory-size 4096 --vmwarefusion-cpu-count 2 --vmwarefusion-disk-size 40960 default`. Feel free to increase the number of CPUs, RAM, or Disk space as needed. This determines how fast you can compile the project and build software. (Adam uses 4 CPUs, 8GB of RAM)
1. Consider adding `eval "$(docker-machine env default)"` to your shell initialization
1. Checkout the source by running `git clone git@github.com:habitat-sh/habitat.git; cd habitat`
1. Run `make`
1. (Optional) Run `make test` if you want to run the tests. This will take a while.

Everything should come up green. Congratulations - you have a working Habitat development environment.

**Note:** The Makefile targets are documented. Run `make help` to show the output. Habitat requires `perl`.

**Optional:** This project compiles and runs inside Docker containers so while
installing the Rust language isn't strictly necessary, you might want a local
copy of Rust on your workstation (some editors' language support require an
installed version). To [install stable
Rust](https://www.rust-lang.org/install.html), run: `curl -sSf
https://static.rust-lang.org/rustup.sh | sh`. Additionally, the project
maintainers use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) for
code formatting. If you are submitting changes, please ensure that your work
has been run through rustfmt. An easy way to install it (assuming you have Rust
installed as above), is to run `cargo install rustfmt` and adding
`$HOME/.cargo/bin` to your `PATH`.

## Setup on native Linux

see [docs/BUILDING.md](docs/BUILDING.md) for platform specific info.

## Web Application

The Habitat Builder web application is in the components/builder-web directory. See
[its README](components/builder-web/README.md) for more information.

## Documentation

Habitat's website and documentation is generated using [Middleman](https://middlemanapp.com/) and is located in the `www` directory of the Habitat source code. To work on the documentation, you will need to have a working [Ruby](https://ruby-lang.org) installation and Bundler. We recommend Ruby 2.3.1 or greater.

To install Middleman, follow these instructions:

1. Change to the `www` directory and type:

       bundle install --path=vendor

2. To build the documentation, either before or after you make your change, change to the `www` directory and type:

       bundle exec middleman build

3. The documentation is built into the `source` directory. You can instruct Middleman to serve the site by typing:

       bundle exec middleman serve

4. Middleman will start a small webserver on your computer and indicate what URL you should load in your browser to preview it.

       == View your site at "http://mylaptop.example.com:4567", "http://192.168.1.101:4567"

5. You can continue to make changes to the documentation files and Middleman will reload them live.
6. Press `Ctrl-C` to terminate the webserver when you are finished working with Middleman.

### Documentation for Rust Crates

The Rust crates also have their own internal developer documentation. From the root of the project, type `make docs` to build the internal Rust documentation.

Run `make serve-docs` to run a small web server that exposes the documentation
on port `9633`. You can then read the docs at `http://<DOCKER_HOST>:9633/`
(with working JavaScript-based search).

## Writing new features

1. Start a new feature branch
1. Open a terminal and run `make shell`
1. Change directory to a component `cd components/x`
1. Build with `cargo build` or `cargo test`
1. You can use `cargo run -- foobar` to pass options to the built binary
1. Sign and commit your change
1. Push your feature branch to GitHub, and create a Pull Request

## Running the Builder API locally

Run `make api-shell`. A shell will start with the API services running. The API
will be available on port 9636 of your Docker host.

Inside the shell, run the `api` command to manage the API server processes.

### Signing Your Commits

This project utilizes a Developer Certificate of Origin (DCO) to ensure that each commit was written by the
author or that the author has the appropriate rights necessary to contribute the change.  The project
utilizes [Developer Certificate of Origin, Version 1.1](http://developercertificate.org/)

```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
660 York Street, Suite 102,
San Francisco, CA 94110 USA

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

Each commit must include a DCO which looks like this

`Signed-off-by: Joe Smith <joe.smith@email.com>`

The project requires that the name used is your real name.  Neither anonymous contributors nor those
utilizing pseudonyms will be accepted.

Git makes it easy to add this line to your commit messages.  Make sure the `user.name` and
`user.email` are set in your git configs.  Use `-s` or `--signoff` to add the Signed-off-by line to
the end of the commit message.

## Pull Request Review and Merge Automation

Habitat uses several bots to automate the review and merging of pull
requests. Messages to and from the bots are brokered via the account
@thesentinels. First, we use Facebook's [mention bot](https://github.com/facebook/mention-bot)
to identify potential reviewers for a pull request based on the `blame`
information in the relevant diff. @thesentinels can also receive
incoming commands from reviewers to approve PRs. These commands are
routed to a [homu](https://github.com/barosl/homu) bot that will
automatically merge a PR when sufficient reviewers have provided a +1
(or r+ in homu terminology).


### Delegating pull request merge access

A Habitat core maintainer can delegate pull request merge access to a contributor via

	@thesentinels delegate=username

If you've been given approval to merge, you can do so by appending a comment to the pull request containing the following text:

	@thesentinels r+

Note: **do not** click the Merge Pull Request button if it's enabled.


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
