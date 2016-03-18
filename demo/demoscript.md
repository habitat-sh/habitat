# The bldr demo script

## Preparing for the demo: short version

You can download a canned `chef/redis` container from quay.io. You'll need to be added to the bldr
team there; drop into #bldr-support in Slack and someone can help you.

After that:

```
docker login quay.io
# enter your quay username, password, and email
docker pull quay.io/bldr/redis
```

Make sure it works:

```
docker run -it quay.io/bldr/redis
```

You're ready to demo.

## Pre-bldr world

```
docker run -it redis
```

Look, so easy! Look how much it abstracted. However, what happens when you
want to inject runtime configuration data? e.g. how do you fix this:

```
WARNING: The TCP backlog setting of 511 cannot be enforced because /proc/sys/net/core/somaxconn is set to the lower value of 128.
```

First, Google for what this means. Now you know you need to modify the
`tcp-backlog` setting to 128. How do you do this?

Now you need to eat the entire Dockerfile and all of its complexity (a copy
is included in this dir for reference)

## With bldr

```
docker run -it quay.io/bldr/redis
```

Looks pretty much the same, except with a supervisor wrapping the run and
some extra colorization. Ok...

But we can ask the supervisor what things are configurable:

```
docker run -it quay.io/bldr/redis config chef/redis
```

Get a TOML output with all the configuration values.

Run the container again overriding the configuration:

```
docker run -e BLDR_REDIS='tcp-backlog = 128' -it quay.io/bldr/redis
```

See the message gone.

## Making the configuration permanent

*** This part is currently broken due to the switch from etcd ***

(Once this is reimplemented we can inject this config into the supervisor
by asking it to load a TOML file from the host machine. You can track the
progress of fixing this by following
[BLDR-86]([https://chefio.atlassian.net/browse/BLDR-86) on the dev board.)

## Built-in topology awareness discovery using gossip

You'll want three terminal windows for this.

First let's start the primary:

```
docker run quay.io/bldr/redis start chef/redis --gossip-permanent  --topology leader
```

This prints out the IP of the container, so you can run two more:

```
docker run -it quay.io/bldr/redis start chef/redis --gossip-permanent --gossip-peer WHATEVER-THE-IP-OF-THE-FIRST-IS --topology leader
```

(run that twice)

See how the cluster automatically voted once there was quorum on a leader,
picked one as the master, the others are followers.

You could start more than three instances; it doesn't matter. If you do that you can safely kill one
(kill the master for fun) to force an election and show how easy it is.

## Sidecar

Need to do this from another container for now since the bldr containers
are so bare and don't have any of these tools.

```
docker run -it fedora /bin/sh
# Then inside the container
dnf -y install curl jq
```

Show the gossip endpoint of the sidecar.

```
curl http://some-peer:9631/gossip | jq .
```

Show how we will show status

```
curl http://some-peer:9631/gossip | jq '.member_list.members | map({ip: .ip, health: .health})'
```

Refer to how easy it is to make a custom callback for the app when showing the packaging system.

# Show the Packaging System

From the root of the bldr GitHub repository:

```
make shell
studio enter
# Talk about what this is, then rebuild something
cd plans
make gpg
# You need Internet access for this part - not guest Wi-Fi because
# until the depot is up, it runs on a weird port
build rust
```

Talk about how to make a Docker container out of this!

```
dockerize chef/rust
```

Talk about how easy it is for a developer to write a health check that gets invoked by the sidecar:

```
cat plans/bldr-web/hooks/health_check
```

# To-Do on Gossip

* Show partitions
* Show how peers are loosely connected

# Appendix

If you want to build the `chef/redis` container from scratch (e.g. 'cause you want a newer one than the
one on Quay, e.g. 'cause there's newer features you want to show off), here the instructions for building
a new one. Do this from the root of `bldr`:

```
make shell
studio enter
cd plans
# downloads pre-built userland, enough for package building
build redis
dockerize chef/redis
```

Now remember that the name of this local container is `chef/redis`, so don't use `quay.io/bldr/redis` in your
demo for the container image name.
