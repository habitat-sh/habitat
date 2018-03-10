---
title: Exporting Docker Images for Cloud Foundry with Habitat
date: 2018-03-09
author: Garrett Amini
tags: exporter, cloud foundry, docker
category: product
classes: body-article
---

With Habitat, it's easy to package and export your applications to any environment, and Cloud Foundry is no exception. Our friends over at [Stark & Wayne](https://starkandwayne.com/) set up a quick demo to just how easy it can be!

## Set up Habitat

First, [clone the example Node repo here](https://github.com/habitat-sh/cloudfoundry-example). Initialize Habitat with the Node Scaffolding by entering the following on the command line at the project root:

```shell
$ hab plan init -s node
```
_If you don't have Habitat installed, [get it from here](https://www.habitat.sh/docs/install-habitat/#install-habitat)_

Next, configure the `plan.sh` file to have a configurable port:

```shell
pkg_origin=myoriginname
pkg_name=expresso
pkg_version=0.1.0
pkg_scaffolding=core/scaffolding-node

pkg_exports=(
    [port]=app.port
)
pkg_exposes(port)
```

Edit `default.toml` to include a default value for the port variable:

```toml
[app]
port = "3000"
```

All set!

## Package up the app

Enter the Habitat studio, and build the initial Habitat artifact:

```shell
$ hab studio enter
[1][default:/src:0]# build .
(...)
[2][default:/src:0]# ls results/*.hart
myoriginname-expresso-0.1.0-20170927132309-x86_64-linux.hart
```

Cloud Foundry configures applications via injected environment variables. One way to get those variables picked up by the Habitat CLI is via a `.toml` file that overrides the existing `default.toml`:

```shell
[3][default:/src:0]# cat <<EOF >mapping.toml
> [app]
> port = "${PORT}"
> EOF
[4][default:/src:0]# hab pkg export cf myoriginname/expresso ./mapping.toml
(...)
[5][default:/src:0]# exit
```

You should be able to see the resulting Docker images like so:

```shell
$ docker images
REPOSITORY                          TAG
myoriginname/expresso               cf-0.1.0-20170927132309
myoriginname/expresso               0.1.0-20170927132309
myoriginname/expresso               latest
```

## Deploy to Cloud Foundry

If you are logged in with the cf-CLI to a Cloud Foundry instance that supports Docker containers, deployment is a simple matter of pushing to DockerHub and running via Cloud Foundry.

```shell
$ docker push myoriginname/expresso:cf-0.1.0-20170927132309
$ cf app expresso
Showing health and status for app expresso in org test / space habitat as admin...

name:              expresso
requested state:   started
instances:         1/1
usage:             256M x 1 instances
routes:            expresso.local.pcfdev.io
last uploaded:     Sat 10 Mar 12:23:52 CEST 2018
stack:             cflinuxfs2
docker image:      myoriginname/expresso:cf-0.1.0-20170927132309

     state     since                  cpu    memory         disk            details
#0   running   2018-10-03T11:32:48Z   0.2%   44.7 of 256M   556K of 512M
```

You should now be able to see the page at http://expresso.local.pcfdev.io ! 

To learn more about using Habitat with Cloud Foundry, check out the [Getting Started page here](https://www.habitat.sh/get-started/).