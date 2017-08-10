---
title: Use Shield to backup and restore Redis
date: 2017-05-22
author: Justin Carter
tags: community
category: Community
classes: body-article
---

In a recent [blog post](http://starkandwayne.com/blog/intro-tour-of-habitat-2/) I briefly discussed how to build, export and run a service packaged via a Habitat plan.

In this post we will take a look at running Redis and backing it up via Shield.

## Running Redis

To play around with the starkandwayne/redis release you can bring it up in the habitat studio:

```console
$ hab studio enter
[1][default:/src:0]# hab svc load starkandwayne/redis
(...)
[2][default:/src:127]# hab pkg binlink starkandwayne/redis
» Symlinking redis-check-rdb from starkandwayne/redis into /bin
★ Binary redis-check-rdb from starkandwayne/redis/3.2.8/20170522110804 symlinked to /bin/redis-check-rdb
» Symlinking redis-server from starkandwayne/redis into /bin
★ Binary redis-server from starkandwayne/redis/3.2.8/20170522110804 symlinked to /bin/redis-server
(...)
[2][default:/src:0]# /bin/redis-cli -a password SET hello world
OK
[3][default:/src:0]# /bin/redis-cli -a password GET hello
"world"
```

Typing `sl` will give you the log output of the background supervisor that got started when you entered the studio:

```console
[4][default:/src:0]# sl
--> Tailing the Habitat Supervisor's output (use 'Ctrl+c' to stop)
redis.default(O):  |    `-._`-._        _.-'_.-'    |
redis.default(O):   `-._    `-._`-.__.-'_.-'    _.-'
redis.default(O):       `-._    `-.__.-'    _.-'
redis.default(O):           `-._        _.-'
redis.default(O):               `-.__.-'
redis.default(O):
redis.default(O): 168:M 22 May 13:11:55.082 # Server started, Redis version 3.2.8
redis.default(O): 168:M 22 May 13:11:55.082 * The server is now ready to accept connections on port 6379
(...)
```
## Running Shield daemon

Since Shield is a bit more complex system with a few moving parts I will run it via the pre-exported docker images in docker-compose.

First lets bring up the shield-daemon connected to a database. The daemon is the main coordinator of shield. It triggers backups as needed and persists the state of all created archives and backup jobs.  

```console
$ mkdir redis-hab-demo && cd redis-hab-demo
$ cat <<EOF > docker-compose.yml
version: '3'

services:
  shield:
    ports:
    - 443:443
    image: starkandwayne/shield
    command: "start starkandwayne/shield --peer database --bind database:postgresql.shield"
    links:
    - database
  database:
    image: starkandwayne/postgresql
    command: "start starkandwayne/postgresql --group shield"
EOF

docker-compose up
```

You can use the `shield` cli to interact with the daemon. Download it from the [github-release](https://github.com/starkandwayne/shield/releases).

From another terminal:

```console
$ shield create-backend hab https://localhost
Successfully created backend 'hab', pointing to 'https://localhost'

Using https://localhost (hab) as SHIELD backend
$ export SHIELD_API_TOKEN=autoprovision
```

To actually backup a system you need to create a few entities in shield such as a policy, schedule and store. Lets create a schedule that takes a backup every day at 4am via the cli:

```
$ shield create-schedule -k
Schedule Name: daily
Summary:
Time Spec (i.e. 'daily 4am'): daily 4am

Schedule Name:                daily
Summary:
Time Spec (i.e. 'daily 4am'): daily 4am

Really create this schedule? [y/n] y
Created new schedule
Name:     daily
Summary:
Timespec: daily 4am
$ shield schedules -k
Name   Summary  Frequency / Interval (UTC)
====   =======  ==========================
daily           daily 4am
```

Because creating all entities manually is error prone we can also automate it by using the shield-agent.

## Running Shield agent
The shield-agent is another component of Shield which is typically co-located with the data store you want to backup. You can configure it to automatically provision the elements that shield needs to run a backup.

Stop the docker-compose system via:
```
docker-compose stop && docker-compose rm -f
```

Use an EDITOR to add the agent to the docker-compose file. Add the `agent` service under the already existing `services:` key:

```YAML
services:
  agent: # to autoprovision the dependant entities
    image: starkandwayne/shield-agent
    command: "start starkandwayne/shield-agent --bind daemon:shield.default --peer database"
    environment:
      HAB_SHIELD_AGENT: |
        [[stores]]
        name='local'
        plugin='fs'
        [stores.config]
        base_dir='/backups'
        [schedules]
        daily='daily 4am'
        [retention-policies]
        shortterm='86400'
    links:
    - database
```

Bring it up and lets see if it worked:

```
$ docker-compose up
```

Once everything is runnin you can see the configured entities in another terminal:

```
$ shield policies -k
Name       Summary  Expires in
====       =======  ==========
shortterm           1 days
$ shield stores -k
Name   Summary  Plugin  Configuration
====   =======  ======  =============
local           fs      {
                          "base_dir": "/backups"
                        }
```

Excellent we have now automatically configured a _store_. For the demo we are using the `fs` plugin to store backups in a local folder (`/backups`). In production you would want to use a plugin that can store the backups on a cloud based object store like `s3`.

## Auto-configuring Redis
Now that we have a schedule, policy and store in place we can bring up Redis and have it automatically configure Shield to run backups.

Again stop the running system:

```
docker-compose stop && docker-compose rm -f
```
And add Redis to the `docker-compose.yml`. Again the `redis` service belongs under the _already existing_ `services:` key. The `volumes` key new:

```
services:
  redis:
  image: starkandwayne/redis:edge
  volumes:
  - backups-volume:/backups
  ports:
  - 6379:6379
  command: "start starkandwayne/redis --peer shield --bind shield:shield.default"
  environment:
    HAB_REDIS: |
      bootstrap_from_backup=true
      backups_schedule='daily'
      backups_retention='shortterm'
      backups_store='local'
  links:
  - shield

volumes:
backups-volume: {}
```

Bring it up and have a look:

```
$ docker-compose up
```
It can take a while for the whole system to come up but eventually you should see:

```
$ shield jobs -k
Name           P?  Summary  Retention Policy  Schedule  Remote IP        Target
====           ==  =======  ================  ========  =========        ======
redis-default  N            shortterm         daily     172.27.0.5:5444  {
                                                                           "base_dir": "/hab/svc/redis/data"
                                                                         }
```
So the Redis service we just added was able to configure its own backup job just by binding to a running Shield daemon. Cool!
Lets write a value, take a backup and see if it works:

```
$ redis-cli -a password SET hello world
OK
$ shield run redis-default -k
Scheduled immediate run of job
To view task, type shield task f82752ae-8066-4bca-9c71-47dc35464c80
$ shield archives -k
UUID                                  Target              Restore IP         Store         Taken at                         Expires at                       Status  Notes
====                                  ======              ==========         =====         ========                         ==========                       ======  =====
fb2b2b0b-925b-4e69-8083-ab649760048e  redis-default (fs)  192.168.16.5:5444  default (fs)  Tue, 16 May 2017 13:29:02 +0000  Wed, 17 May 2017 13:29:02 +0000  valid
```
So we set a value and manually took a backup. Lets destroy and recreate the Redis service. Thanks to the auto-bootstrapping feature the value should be restored without any further input:

```
$ docker-compose stop redis && docker-compose rm -f redis
$ docker-compose up -d redis
$ until redis-cli -a password GET hello; do echo 'Waiting for redis to bootstrap'; sleep 1; done
Waiting for redis to bootstrap
Waiting for redis to bootstrap
Waiting for redis to bootstrap
Waiting for redis to bootstrap
"world"
```
So thanks to Shield and Habitat's binding feature we are very easily able to add arbitrary Redis services all with backups preconfigured.
