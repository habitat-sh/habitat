# The bldr demo script

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
docker run -it chef/redis
```

Looks pretty much the same, except with a supervisor wrapping the run and
some extra colorization. Ok...

But we can ask the supervisor what things are configurable:

```
docker run -it chef/redis config redis
```

Get a TOML output with all the configuration values.

Run the container again overriding the configuration:

```
docker run -e BLDR_redis='tcp-backlog = 128' -it chef/redis
```

See the message gone.

## Making the configuration permanent

*** This part is currently broken due to the switch from etcd ***

(Once this is reimplemented we can inject this config into the supervisor
by asking it to load a TOML file from the host machine)

## Built-in topology awareness discovery using gossip

You'll want three terminal windows for this.

First let's start the primary:

```
docker run chef/redis start chef/redis --gossip-permanent  --topology leader
```

This prints out the IP of the container, so you can run two more:

```
docker run -it chef/redis start chef/redis --gossip-permanent --gossip-peer 172.17.0.3 --topology leader
```

(run that twice)

See how the cluster automatically voted once there was quorum on a leader,
picked one as the master, the others are followers.

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

# Show the Packaging System

```
cd studio && make docker-studio
# Talk about what this is, then rebuild something
make gpg
# You need Internet access for this part - not guest Wi-Fi because
# until the depot is up, it runs on a weird port
build rust
```

Talk about how to make a Docker container out of this!

```
dockerize chef/rust
```

# To-Do on Gossip

* Show partitions
* Show how peers are loosely connected
