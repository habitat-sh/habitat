---
title: Multi-Service Supervision
date: 2017-04-12
author: Ian Henry
tags: supervisor, gossip
category: supervisor
classes: body-article
---

Those of you that have been following along with our regular release cadence might have noticed a couple of pretty big features dropped [last week with version 0.20.0](https://forums.habitat.sh/t/habitat-0-20-0-released/317). In fact one of those features was a breaking change to the way the Supervisor functions. If you didn't get a chance to read those [release notes](https://forums.habitat.sh/t/habitat-0-20-0-released/317) I would highly suggest you do! This change is going to open up a huge amount of possibility for the way you run and manage Chef Habitat Supervisor rings and the services they're responsible for. Actually one of our community members [Adam Leff](https://github.com/adamleff) (who also happens to be the Technical Community Advocate for the Chef Inspec project) even wrote a [blog post about using Chef Habitat and Chef Inspec together](https://blog.chef.io/2017/03/30/inspec-habitat-and-continuous-compliance/) that explores one of the many ways you'll benefit from this new functionality.

What I want to go over here first is the *potentially breaking behavior* we introduced, as well as how you can start taking advantage of this feature today. So, let's start with the breaking change.

This issue will *only* impact Chef Habitat users who have been running more than one Supervisor per host/VM/container/etc. If you're a container purist and stuck to your guns on a single process per container, then you wont run into this. In the case where you are a user that *is* running more than one Supervisor (you'll know if this is you because you would have had to explicitly set non-default values for the `--listen-gossip` and `--listen-http` options) you have to ways to update to the new Supervisor behavior:
  * Eliminate the multiple Supervisors launching via SystemD, init, etc., launch a single Supervisor with `hab sup run` (note the lack of package identifiers, topology strategies, etc.), and run a `hab svc load` per service.
  * Alternatively, for each existing Supervisor, add an `--override-name` option to the command with some unique value. This will allow the Supervisors to write their own state information to distinctly different directories. Once stable, you are highly encouraged to migrate to the single Multi-Service Supervisor strategy as explained directly above.
If all of that sounds like a foreign language to you try not to worry. Let's take a look at how the Supervisor functions now.

### Running Multiple Services with a single Supervisor
The Chef Habitat Supervisor is designed to supervise more than one service concurrently. So now if you're running Chef Habitat on bare metal or a virtual machine (or even if you're packing containers with sidecar services) there is only a need for one Supervisor per compute instance.

#### Starting only the Supervisor
Whoa, whoa, whoa. Starting _only_ the Supervisor? Well, yes. As part of the work that went into multi-service supervision we found we really needed a way for supervisors to be started in a state that's completely divorced from a Chef Habitat package. There are certainly some neat things that can be achieved as byproduct of these changes including running supervisors in a ring without any services at all in permanent peer mode. There are some other patterns that I'm sure will spring up as we all start digging into this new functionality in our unique environments but for now lets look at how to do all the things.

Starting the Supervisor is as simple as running:

```shell
$ hab sup run
```

The `hab sup run` command will also allow you to override default gossip and http gateway binding ports, just like when using `hab start`.

```shell
OPTIONS:
        --listen-gossip <LISTEN_GOSSIP>    The listen address for the gossip system [default: 0.0.0.0:9638]
        --listen-http <LISTEN_HTTP>        The listen address for the HTTP gateway [default: 0.0.0.0:9631]
        --override-name <NAME>             The name for the state directory if launching more than one Supervisor [default: default]
        --org <ORGANIZATION>               The organization that the Supervisor and it's subsequent services are part of [default: default]
        --peer <PEER>...                   The listen address of an initial peer (IP[:PORT])
    -r, --ring <RING>                      Ring key name
```

#### Loading a Service for Supervision

Adding services to a Supervisor is accomplished with the `hab svc load` subcommand. It's going to support most of the same flags and options as `hab start` so theres nothing totally new to learn here. If for example you need to load `yourorigin/yourpkg` in a leader topology with a rolling update strategy and a Group of "acme" you could likely easily guess the syntax:

```shell
$ hab svc load yourorigin/yourpkg --topology leader --strategy rolling --group acme
```
Heres where the magic actually happens. Running any subsequent `hab svc load` commands with different package identifiers  is going to result in the Supervisor turning on and managing multiple services. So lets pretend for a moment that `yourorigin/yourpkg` runs in conjunction with a postgres database. Let's add `core/postgresql` to teh Supervisor for some fun:

```shell
$ hab svc load core/postgresql
```
The Supervisor will (like you're probably used to at this point) pull down the package and start the service except that your single Supervisor is now managing *both* services!

#### Unloading a Service from Supervision
If there are situations where you'll want a Supervisor to run multiple services then there will likely be situations where you'll want to completely _unload_ a service. In Chef Habitat when you `unload` a service from supervision, you use the `hab svc unload` subcommand. If the service specified was in a running state it will first be stopped, and then removed. This means the next time the Supervisor starts (or perhaps restarts) it will not run this unloaded service.

```shell
$ hab svc unload yourorigin/yourpkg
```

#### Stopping and Starting Loaded Services
Once your service is loaded and running you might have a situation where you need to temporarily *stop*  service for some reason. Maybe thats during a maintenance window, or perhaps in testing a development cluster. Rather than completely removing the service like you saw with `unload` you can actually use the `hab svc stop` subcommand. Executing this call will shut down a running service and leave it in this satte until you start it again. This means that all your service-related options like topology and update strategy are preserved until the service is started again!

```shell
$ hab svc stop core/postgresql
```
To resume running a service that was stopped with the `hab svc stop` subcommand you can use the `hab svc start` command. So once your maintenance window is over, or you're ready to turn your persistence back on you can simply run:

```shell
$ hab svc start core/redis
```

If you're a user that's leveraging your host's init system to kick off the Supervisor, hopefully this has given you an idea for how to tweak your init files. For more information on this subject [check out our docs on Multi-Service Supervision](/docs/using-habitat/#run-multiple-services-with-one-supervisor)

Thanks for following along and as always Happy Chef Habitat-ing!
