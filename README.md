% Bldr README

# Bldr

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
  * Backup)

With the same amount of effort required to put your applicatin in a Dockerfile. Or less.

## How does it do this?

Bldr provides a way to build an atomic `package` via `bldr-build`, and an
optional `container` that is automatically configured to run it. It also
provides a supervisor, that handles running, configuring, and managing your
services (`bldr`).

# Working on Bldr

## Rules for working on Bldr

1. The principle of least abstraction. When possible, we use the tooling that is closest to the native
tooling for the platform, and provide as little abstraction as neccessary. When we do choose an abstraction,
we choose one - and we make it the one that is most user-servicable.
1. Keep it light. The runtime component of bldr is used as a process supervisor - it needs to stay lean. No runtimes.
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

## Setup

1. [Install Docker Toolbox](http://docs.docker.com/mac/step_one/)
1. Consider adding `eval "$(docker-machine env default)"` to your shell initialization
1. [Install Stable Rust](https://www.rust-lang.org/install.html) `curl -sSf https://static.rust-lang.org/rustup.sh | sh`
1. [Install the delivery-cli](https://delivery-packages.s3.amazonaws.com/cli/deliverycli-20150819175041%2B20150819175041-1.pkg)
1. [Install Node](https://nodejs.org/en/)
1. [Install docco](http://jashkenas.github.io/docco/)
1. Run `make`
1. Run `make test`

Everything should come up green. Congratulations - you have a working Bldr development environment.

## Documentation

Run `make doc` to build the internal documentation for bldr.

Run `doc-serve` to run a small webserver that exposes the documentation on port `9633`. You can then
read the docs at `http://localhost:9633/bldr` (with working javscript-based search).

## Writing new features

1. Start a new feature branch
1. Open a terminal and run `make pkg-shell`
1. Build with `cargo build` or `cargo test`
1. You can use `cargo run -- foobar` to pass options to the built binary
1. Commit your change
1. `delivery review`

# Demo commands

Start with the upstream docker redis container.

```bash
$ docker run -it redis
```

This is awesome because:

1. If you don't have redis, it downloads it
1. It runs the service, and returns the output to you directly
1. You don't know anything about, and don't care, what the "operating system" is

But things are not quite all as awesome as that first experience is. Right off
the bat, you configuration errors - and to fix it, you're told to update a
configuration file; but how?

The answer is you have to open up the Dockerfile, look at how it's constructed,
then fire up an instance of the redis container with a shell in it. Figure out
where the config file is, figure out the syntax, tweak it, then, make a choice:

1. Create a new Dockerfile that inherits FROM the upstream. Congratulations!
   You are the new maintainer!
1. Put your configuration file in a data-only container and cross-mount it.

But what would we want if we could have everything the way we were promised?

1. We would be able to ask the container what was configurable.
1. It would be 12 factor - we could configure anything we need to configure
   from the environment.

First, lets show a basic redis container - essentially exactly like the `redis`
container above.

```bash
$ docker run -it bldr/redis
```

Notice it has the same errors the 'default' container has. Lets see what
we can configure about it.

```bash
$ docker run -it bldr/redis config redis
```

What you see is a [toml](https://github.com/toml-lang/toml) file, which
documents every configurable option of our container. One error we see is the
`tcp-backlog` setting is wrong. Lets tweak that in the style of a 12 factor
app:

```bash
$ docker run -e BLDR_redis="tcp-backlog = 128" -it bldr/redis
```

Notice the error has gone away - we've gone from an opaque container we can't
manage to one that we can. Yay!

But we want to use this container not just in development, or one off - we want
to have it in production, and it has different tunings from the one we use in
development. Going back to the promise of having all of our tools in the
cloud, we want to use service discovery to solve this problem - centrally
store the configuration for our service, and then have it automatically
configure the container at runtime.

```
$ docker run --link=bldr_etcd_1:etcd -e BLDR_CONFIG_ETCD=http://etcd:4001 -it bldr/redis
```

This links us up to an `etcd` instance for service discovery. Open another
terminal window, and lets write our new configuration:

```toml
# Put this in /tmp/redis.toml
tcp-backlog = 128
loglevel = debug
```

To put this into etcd:

```bash
foo=$(cat /tmp/redis.toml); curl -L http://192.168.99.100:4001/v2/keys/bldr/redis/default/config -XPUT -d value="${foo}"
```

You'll notice that your redis instance sees the configuration has changed, and
automatically reconfigures iteslf. Neat!

But you don't want just one redis instance. You need a cluster.

Now, open three more terminal windows. (If you're using iterm, go ahead an put all
three in one split window, and then link their inputs things.)

Now run:

```bash
docker run --link=bldr_etcd_1:etcd -e BLDR_CONFIG_ETCD=http://etcd:4001 -it bldr/redis start redis -t leader
```

This will start 3 instances of redis, elect one as a leader, and the others
will automatcially become followers.

```bash
foo=$(cat /tmp/redis.toml); curl -L http://192.168.99.100:4001/v2/keys/bldr/redis/default -XPUT -d value="${foo}"
```

Then you can load balance them with a generic tcp proxy:

```bash
docker run --expose 6379 --link=bldr_etcd_1:etcd -e BLDR_CONFIG_ETCD=http://etcd:4001 -it bldr/haproxy start haproxy -w redis.default
```


