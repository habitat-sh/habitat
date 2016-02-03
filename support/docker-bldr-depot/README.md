# Docker Bldr Depot Service

This is a place-holder project which automates the creation and destruction of a host that can support deployment of one or more Bldr Depot services for the purposes of development and acceptance. It uses the [Docker Machine](https://docs.docker.com/machine/) and [Docker Compose](https://docs.docker.com/compose/) projects with a super lofi mechanism of sharing Docker Machine configuration between a small team.

## depotman

The main program is `depotman` and has an embedded help which can be accessed by running `./depotman help` the requirements to run this program are as follows:

* [Docker Toolbox](https://www.docker.com/products/docker-toolbox) - As of 2016-02-01, the latest version of Docker Toolbox is required for the latest Docker Machine release.
* Set `$AWS_ACCESS_KEY_ID` environment variable - required for creating and destroying the Docker Machine instance.
* Set `$AWS_SECRET_ACCESS_KEY` environment variable - required for creating and destroying the Docker Machine instance.
* Set `$AWS_VPC_ID` environment variable - required for creating and destroying the Docker Machine instance.

## Quickstart

```sh
# For a Docker configured to an Engine with bldr/base:latest present
docker save -o bldr-base-latest.tar bldr/base:latest

# Launch a Docker Machine named 'bldrdepot' with a running service 'bldrdepot'
./depotman launch

# Get the $BLDR_REPO value
./depotman repo-url

# Stream the service's logs
./depotman logs

# Destroy the machine instance with all running services
./depotman destroy-machine
```

## Preparing The bldr/base Docker Image Tarball

The newly created Docker Machine will need a `bldr/base:latest` image to be loaded, which can be done using the `./depotman load-image <TARBALL>` subcommand. To prepare this tarball, you need a shell wired up to a Docker instance containing this image, then run:

```
docker save -o bldr-base-latest.tar bldr/base:latest
```

Then place the tarball in this directory for the `load-image` subcommand's defaults to work.

## Creating A Docker Machine Instance

Run:

```sh
./depotman create-machine
```

If you require a custom Docker Machine other than the default of `"bldrdepot"`, then provide your name as an argument:

```sh
./depotman create-machine my-depot-machine
```

## Loading The bldr/base Docker Image

Assuming you've followed the directions above (which produce a `bldr-base-latest.tar` file), run:

```sh
./depotman load-image
```

## Deploying An Instance Of The Bldr Depot Service

Run:

```sh
./depotman deploy-service
```

If you wish to customize the project name of the service, then provide your name as an argument:

```sh
./depotman deploy-service my-depot
```

## Launching A Docker Machine with First Running Bldr Depot Service

You can combine the creation, loading, and deployment into one command by running:

```
./depotman launch
```

Note that if you need to launch a second or to customize the name of the Docker Machine and the initial Bldr Depot service, then provide the name as an argument:

```
./depotman launch lindas-bldrdepot
```

## Killing An Instance Of The Bldr Depot Service

For the default-named service, run:

```sh
./depotman kill-service
```

For a custom-named service, run:

```sh
./depotman kill-service my-depot
```

## Destroying A Docker Machine Instance

For the default-named Docker Machine, run:

```sh
./depotman destroy-machine
```

For a custom-named machine, run:

```sh
./depotman destroy-machine my-depot-machine
```

## Sharing Docker Machine Configuration

While this is experimental (and crazy lofi), it may be just enough to be useful. To export your Docker Machine configuration, run:

```sh
./depotman export-config
```

This will create a fille similar to `docker-machine-bldrdepot-2016-02-03-050728-config.tar` which can be given to another team member. They place the tarball in this directory and run:

```sh
./depotman import-config <CONFIG_TARBALL>
```

No guarentees made, but it seems to work ;)
