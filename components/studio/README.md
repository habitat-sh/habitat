# Habitat Studio

This is the code that creates a habitat studio for linux, windows and docker.

The key pieces here are:

* The studio hosting script - `bin/habitat-studio`. The `.sh` sets up a linux studio (the same script is used in the docker studios) and the `.ps1` sets up a windows studio.
* The studio environment "types" in `libexec`. These tweak the environment of a studio and target it for specific purposes. Most will only be interested in `hab-studio-type-defaulf.sh`.
* The docker image builder `build-docker-image.sh`.
* The habitat plans used to create the habitat studio package.

## What determines the studio type and version used?

Different studios will run depending on OS platform, hab client version and environmental overrides.

### Linux

If you are run in a Linux host or VM, the habitat client will install the `core/hab-studio/<version>` if it is not already installed. `<version>` will match the exact version of the `hab` binary you are using. However do note that if you have a later version already installed, it will use that one instead. It will then invoke `bin/hab-studio.sh <hab studio command>` passing whatever studio command you have entered (`enter`, `build`. etc) to the bash script.

The `-t` argument will determine which studio `type` is sourced. The default is `default`.

### Mac and Windows

Both Mac and Windows platforms will enter a docker based studio. By default, habitat pulls the image from its bintray registry: `habitat-docker-registry.bintray.io/studio:<version>`. Like the linus studio, `<version>` will align with the `hab` binary you are using.

The `hab` client will run:

```
docker run --rm --tty --interactive --privileged  --env HAB_ORIGIN=<your origin> --volume /var/run/docker.sock:/var/run/docker.sock --volume /hab /cache/keys:/hab/cache/keys --volume <current directory>:/src <docker image> <studio command>
```

The `<studio command>` simply forwards the studio commands you entered in the `hab studio` command (`enter`, `build`, etc). This invokes the exact same `bin/hab-studio.sh` script used in the linux studio.

### Experimental Windows Studio

If you have the `$env:HAB_WINDOWS_STUDIO` variable set and are running on a Windows host, the hab client will pull down the appropriately versioned `core/hab-studio` package and invoke its `bin/hab-studio.ps1` script. Note that you must also be using a depot that hosts the windows habitat packages.

## Building a studio

Whether you want to build a plain linux studio or the docker image, you will need to build the studio and hab client packages for the same version.

### Build the hab and hab-studio packages

Navigate to the root of the habitat repo and build the hab client

```
cd /habitat
hab pkg build components/hab
```

Next build the studio

```
hab pkg build components/studio
```

The most important thing here is that you emd up with `hart` files at the same version for both `core/hab` and `core/hab-studio`.

On a linux OS, you can `hab pkg install` the `hart`s and then run the hab binary you installed with the studio command. For example:

```
/hab/pkgs/core/hab/0.22.0-dev/20170407021836/bin/hab studio enter
```

### Building a docker image

Use the `build-docker-image.sh` script to build and install a docker image containing the above packages. You must have a running docker service installed in order to build the image.

The script should be passed the two built hart files with the hab and studio packages.

```
./build-docker-image.sh ./results/core-hab-*.hart
```

This will build the docker image and also install the docker image locally. Now you can enter this Docker Studio with:

```
docker run --rm --tty --interactive --privileged  --env HAB_ORIGIN=<your origin> --volume /var/run/docker.sock:/var/run/docker.sock --volume /hab /cache/keys:/hab/cache/keys --volume <current directory>:/src <docker image sha> enter
```
