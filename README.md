# Habitat

## Problem statement

How do we build, run, and manage our applications in a way that provides:

* Repeatable builds
* Single, immutable assets
* Runtime configuration for multiple deployment scenarios
* Agnostic to operating environment (works on bare metal, virtualization, containers, PaaS)
* Idempotent behavior (the same inputs to the same asset provide the same outcome)
* Convergent behavior (each service makes progress towards the correct behavior in the face of failure)
* Exposes promises to those who rely on it
* Low barrier to entry
* Language agnostic

## Application artifact as closure

Historically, we build our applications as a conglomeration of upstream artifacts. We have the operating
system we used, which provides all of our build (and often run) time dependencies. We then layer in the
specific application (either one we wrote ourselves, or a version of someone else's software), and then we
layer in the details of how to configure and manage that application within its environment (with something
like Chef.) Much of the complexity in the configuration layer comes from dealing with the large variety
in the upstream - with no consistent way to express what it means to be well managed, we are forced to
provide one.

What Habitat provides is the ability to have the application artifact as a closure of all of this behavior -
from how it is built to how it is configured and run. It takes a build description (which includes
dependencies,) an exhaustive set of configuration options, and a hosting platform for the service - wraps
them into a single, encrypted or signed artifact, and enables it to be configured dynamically when the
services are started.

The side effect is that the boundary for idempotency, convergence, and promises shifts from the individual
details of the application stack to the artifact itself. Given the same input data (regardless of source)
we will run the application the same way everywhere, the artifact itself handles making best progress
towards its goal, and exposes consistent interfaces for health and monitoring.

## What Habitat does for you

* Automatically build a minimal environment for your application
* Include dependencies as binary artifacts
* Specify all the configurable options for the application
* Configure them from a file, the environment, or a service discovery framework (etcd/consul/chef) - in real time
* Ensure privilege separation (supervisor de-privileges the service on your behalf)
* Integrates logging cleanly
* Provides pluggable interfaces for critical side-car behavior:
  * Status (up/down/etc.)
  * Health checks
  * Smoke testing
  * Monitoring
  * Backup

With the same amount of effort required to put your application in a Dockerfile. Or less.

## How does it do this?

Habitat provides a way to build an atomic `package` via `hab-plan-build`, and an
optional `container image` that is automatically configured to run it. It also
provides a supervisor, that handles running, configuring, and managing your
services (`hab-sup`).

## Documentation

The documentation for Habitat is hosted at https://www.habitat.sh/docs and
lives in [www](www).

# Working on Habitat

## Rules for working on Habitat

1. The principle of least abstraction. When possible, we use the tooling that is closest to the native
tooling for the platform, and provide as little abstraction as necessary. When we do choose an abstraction,
we choose one - and we make it the one that is most user-serviceable.
1. Keep it light. The runtime component of Habitat is used as a process supervisor - it needs to stay lean. No run-times.
1. Convention over configuration, with sane defaults. Where possible, we remove the need to configure things
by having a convention cover it. When we do need to configure things, we set sane defaults.
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

1. [Install Docker Toolbox](http://docs.docker.com/mac/step_one/) (you'll need
   at least Docker 1.9 and docker-machine 0.5. Both are included in Docker
   Toolbox)
1. [Install VMWare Fusion](https://www.vmware.com/products/fusion)
1. Delete the default docker-machine image: `docker-machine rm default`
1. Create a new one with vmware fusion: `docker-machine create -d vmwarefusion --vmwarefusion-memory-size 4096 --vmwarefusion-cpu-count 2 --vmwarefusion-disk-size 40960 default`. Feel free to increase the number of CPUs, RAM, or Disk space as needed. This determines how fast you can compile the project and build software. (Adam uses 4 CPUs, 8GB of RAM)
1. Consider adding `eval "$(docker-machine env default)"` to your shell initialization
1. Checkout the source by running `git clone git@github.com:habitat-sh/habitat.git; cd habitat`
1. Run `make` (or `make all` to be explicit)
1. Run `make test`

Everything should come up green. Congratulations - you have a working Habitat development environment.

**Note:** The Makefile targets are documented. Run `make help` to show the output. Requires `perl`.

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

**Optional:** This project currently uses GitHub integration with Delivery so
while the delivery-cli tool is not strictly necessary to initiate reviews, it
is highly recommended to have installed for the other useful subcommands.
Download the [delivery-cli
package](https://delivery-packages.s3.amazonaws.com/cli/deliverycli-20150819175041%2B20150819175041-1.pkg)),
install it, and you're done!

## Setup on native Linux

1. [Install Docker](https://docs.docker.com/linux/step_one/) **Note: You may need to logout and then login again after this step**
1. [Install Docker Compose](https://docs.docker.com/compose/install/)
1. Checkout the source by running `git clone git@github.com:habitat-sh/habitat.git; cd habitat`
1. Run `make` (or `make all` to be explicit)
1. Run `make test`

Everything should come up green. Congratulations - you have a working Habitat development environment.

**Note:** The Makefile targets are documented. Run `make help` to show the output. Requires `perl`.

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

**Optional:** This project currently uses GitHub integration with Delivery so
while the delivery-cli tool is not strictly necessary to initiate reviews, it
is highly recommended to have installed for the other useful subcommands.
Download the [delivery-cli
package](https://delivery-packages.s3.amazonaws.com/cli/deliverycli-20150819175041%2B20150819175041-1.pkg)),
install it, and you're done!

## Documentation

Run `make docs` to build the internal documentation for the Habitat Supervisor.

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

### Signing Your Commits

This project utilizes a Developer Certificate of Origin (DCO) to ensure that each commit was written by the author or that the author has the appropriate rights necessary to contribute the change.  The project utilizes [Developer Certificate of Origin, Version 1.1](http://developercertificate.org/)

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

The project requires that the name used is your real name.  No contributions utilizing pseudonyms will be accepted nor will anonymous contributions.

Git makes it easy to add this line to your commit messages.  Make sure the `user.name` and `user.email` are set in your git configs.  Use `-s` or `--signoff` to add the Signed-off-by line to the end of the commit message.

## Development environment with Habitat Studios

Habitat Studios provide an isolated environment with where packages can be
built.

Because all Habitat packages are cryptographically signed, you'll
need to make sure you have the signing key on hand for the origin your
package belongs to, and that it's installed in `/hab/cache/keys` inside
the Studio.

To build a package:

```bash
$ make shell
# Install the signing key in /hab/cache/keys before running the next step.
$ hab studio build plans/redis
```

To upload the resulting package

```bash
$ hab artifact upload ./results/<PKG>.hart
```

Alternatively, you can use the `last_build.env` metadata to fetch the full artifact name to upload:

```bash
$ hab artifact upload ./results/$(source ./results/last_build.env && echo $pkg_artifact)
```

To create a docker container of a package, either local or remote:

```bash
$ hab studio enter
$ hab install core/hab-pkg-dockerize
$ hab pkg exec core/hab-pkg-dockerize hab-pkg-dockerize core/redis
```

To develop Habitat itself, just work like you always did. If you want to,
for example, test that Redis is working with your development version of
the supervisor:

```bash
$ ./target/debug/hab-sup start core/redis
```

Will work just fine (as will running Habitat on other host operating
systems, cause thats all we're up to).

## Web Application

The Habitat Builder web application is in the components/builder-web directory. See
[its README](components/builder-web/README.md) for more information.

## Deploying

There is a [Terraform](https://www.terraform.io/) configuration in the terraform
directory.

This launches the habitat-builder-web app running on an instance behind a load
balancer. It also creates a load balancer for the builder API.

It current only works on the chef-aws account in the us-west-2 region.

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

## License

Copyright:: Copyright (c) 2015-2016, Chef Software, Inc.

The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
and the party accessing this file ("Licensee") apply to Licensee's use of
the Software until such time that the Software is made available under an
open source license such as the Apache 2.0 License.
