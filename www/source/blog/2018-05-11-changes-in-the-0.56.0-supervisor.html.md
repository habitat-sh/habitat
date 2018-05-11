---
title: "Changes in the Forthcoming 0.56.0 Supervisor: What You Need To Know"
date: 2018-05-11
author: Christopher Maier
tags: supervisor
classes: body-article
---

The 0.56.0 release of Habitat brings with it one of the largest changes to the Supervisor that we've ever made. We'd like to share with you what these changes are, why we needed to make them, and what steps you as a Supervisor user need to take to deal with these changes. They're big changes, but they will unlock a lot of exciting possibilities for us in the future.

Before the 0.56.0 release, the communication protocol between the `hab` command-line tool and a running Supervisor essentially consisted of modifying files on disk. While this is a very simple interaction model to implement and think about, it leaves much to be desired in a number of ways. It is difficult to achieve synchronous interactions between processes. It is difficult to validate interactions and abort them in the face of errors. It has introduced several difficult-to-reason-about edge-case bugs. It is inherently restricted to "same machine" interaction patterns. For these reasons and more, we needed to devise a new communication protocol.

With the 0.56.0 release, we have transitioned away from file-based communication to a defined TCP protocol between `hab` and a Supervisor process. Defining a protocol brings added rigor to the Supervisor codebase, which brings additional safety, enhanced backward-compatibility between releases going forward, and lays the groundwork for features that would otherwise have been impossible to implement. While this change has been introduced in a way that it can be largely transparent to current Habitat users in the near term, there are some manual steps that current users will need to take to ensure a smooth transition. We will highlight these changes now.

## Install `core/hab/0.56.0` on _all_ Supervisor Machines

Since communication with the new Supervisor takes place strictly via TCP, it needs a client that also speaks this same protocol; that client is the `hab` CLI tool. This means that in order to continue interacting with a Supervisor (starting services, stopping services, etc.), you will need to manually install `core/hab/0.56.0` on all machines running the 0.56.0 Supervisor. This can be done like so:

```
sudo hab pkg install core/hab/0.56.0 -b -f
```

This will install the new version of `hab`, and link it in your `$PATH`; if you wish to manage your binaries in another way, leave off the `-b` and `-f` flags.

If your Supervisors are configured to auto-update, then they will automatically begin running the 0.56.0 release, but you will still need to upgrade `core/hab`. In a forthcoming release, we will begin bundling these into a single package, but for now, you must take this manual step.

On the other hand, if your Supervisors are _not_ configured to auto-update, then you will not automatically receive the 0.56.0 Supervisor; in this case, you are free to continue using the older Supervisor, but you are encouraged to update at your earliest convenience.

## Revised Command Suite

With this reworking of the interaction model of Habitat, we had the opportunity to streamline and clarify the suite of commands for interacting with services. One of the most frequently-asked questions about Habitat has been "what's the difference between `hab start` and `hab load`?". There was a lot of overlap, but also some very subtle differences which made it difficult to clearly explain. Now we have very clearly differentiated _all_ the commands, in a way that we hope will be much more easy to reason about.

As a result, please review any scripts or automation you have that make use of these commands and update them as necessary.

`hab svc load` is how you inform a Supervisor to manage a new service. Once a new service is loaded, it will start running automatically.

`hab svc unload` is how you remove a service from the Supervisor. If the service is currently running, it will be stopped before being unloaded.

`hab svc start` is how you start a previously-stopped, loaded service. Previously, this could also start a Supervisor process, but that is no longer the case; see `hab sup run` below for more.

`hab svc stop` is how you stop a currently-running (and thus, loaded) service. A stopped service will not be restarted if you restart the Supervisor; you must run `hab svc start`.

`hab sup run` is now the _only_ way to start a Supervisor. As a convenience, `hab sup run` can also accept the same arguments as `hab svc load` in order to start a supervisor, and load and start a single service (for example, `hab sup run core/redis --strategy=at-once --group=dev --peer=192.168.1.1:9638`). This means that you are still able to start a Supervisor and service in a single command.

`hab sup term` shuts down the Supervisor and all currently-running services.

Another frequently-asked question centered around the difference between "persistent" and "transient" services, which happened to also be tied into the "start" vs. "load" question. This was also an area with lots of subtle distinctions which added to the confusion. Now, there is no longer any distinction between persistent and transient services; there are just services. The details of when they start and when they restart are completely determined by the behaviors of the above commands.

## Authentication of Communication

Now that Supervisor interaction is accomplished over TCP, rather than direct filesystem manipulation, we need to add some additional layers of safety.

First, the Supervisor listens on the `127.0.0.1` interface (port 9632) by default, so without any additional configuration, you will only be able to interact with a Supervisor from the same host (just as with older Supervisors). If you wish to open this up to allow remote control of your Supervisors, you'll want to start your Supervisor with the `--listen-ctl` option set to a different interface, e.g. `--listen-ctl=0.0.0.0:9632`.

The Supervisor communication protocol is also protected by a shared secret. When starting up, the Supervisor will look for the `/hab/sup/default/CTL_SECRET` file, which contains the secret it will use to authenticate any command messages it receives. If this file does not exist at startup, the Supervisor will generate a secret itself and write it to that location. If you only interact with your Supervisor from the same host, this will not be an issue, as the client can ultimately use this same secret transparently. However, if you _do_ wish to work with your Supervisors remotely, you'll need to manage your secrets a bit.

There is a new `hab sup secret generate` command that will generate a random secret (on standard output) that you can use. Placing this in the `/hab/sup/default/CTL_SECRET` file for your Supervisor(s) will take care of one half of the interaction. The `hab` client, on the other hand, can either use a secret placed in the `HAB_CTL_SECRET` environment variable, or in your `~/.hab/etc/cli.toml` file, under the `ctl_secret` key.

Finally, to target a remote supervisor with a command, you'll pass the `--remote-sup` flag, which tells `hab` the host and port at which to reach your Supervisor's control gateway; this should correspond to the value set with the `--listen-ctl` option on the Supervisor itself.

## Summary

That's a lot to digest, but you should be able to continue to use Habitat largely as you have in the past with a few small changes. You _must_ manually install the `core/hab/0.56.0` release on each machine that runs a Supervisor in order to be able to continue interacting with them locally. If that is all you wish to do, then you're done! If you wish to experiment with the remote control capabilities, you'll need to configure your Supervisors to listen remotely, and you'll need to manage your shared secrets. Please note that remote interaction is a strictly opt-in feature!

Given the refinement of the Supervisor command suite, you may also need to adjust any scripts or automation you may have in place that conflicts with the new command semantics.

These changes, while significant, lay a very important foundation for future work on the supervisor, including the aforementioned synchronous behavior options, multi-threaded process supervision, as well as the elimination of several edge-cases and bugs due to the older file-based protocols. In the near future, we also intend to begin packaging everything you need into a single Habitat package, which will eliminate the need to have to manually update `core/hab` on a Supervisor machine ever again.

We hope this preview has prepared you for the forthcoming 0.56.0 release, and also excited you for the changes we have in store. Thanks for using Habitat!

### Got questions?
* [Ask and answer questions on the Habitat forums](https://forums.habitat.sh/)
* [Chat with the Habitat Community on Slack](http://slack.habitat.sh/)
* [Learn more about Habitat](https://www.habitat.sh/)
