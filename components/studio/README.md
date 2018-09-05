# Habitat Studio

This is the code that creates a habitat studio for linux and windows.

The key pieces here are:

* The studio hosting script - `bin/habitat-studio`. The `.sh` sets up a linux studio and the `.ps1` sets up a windows studio.
* The studio environment "types" in `libexec`. These tweak the environment of a studio and target it for specific purposes. Most will only be interested in `hab-studio-type-default.sh`.
* The habitat plans used to create the habitat studio package.

## What determines the studio type and version used?

Different studios will run depending on OS platform, hab client version and environmental overrides.

### Linux

If you are run in a Linux host or VM, the habitat client will install the `core/hab-studio/<version>` if it is not already installed. `<version>` will match the exact version of the `hab` binary you are using. However do note that if you have a later version already installed, it will use that one instead. It will then invoke `bin/hab-studio.sh <hab studio command>` passing whatever studio command you have entered (`enter`, `build`. etc) to the bash script.

The `-t` argument will determine which studio `type` is sourced. The default is `default`.

### Experimental Windows Studio

If you have the `$env:HAB_WINDOWS_STUDIO` variable set and are running on a Windows host, the hab client will pull down the appropriately versioned `core/hab-studio` package and invoke its `bin/hab-studio.ps1` script. Note that you must also be using a depot that hosts the windows habitat packages.

## Building a studio

You will need to build the studio and hab client packages for the same version.

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
