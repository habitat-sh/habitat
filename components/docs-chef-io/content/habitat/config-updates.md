+++
title = "Service Updates"
description = "Update services at runtime or dynamically"

[menu]
  [menu.habitat]
    title = "Service Updates"
    identifier = "habitat/services/service-updates Configuration Updates*?"
    parent = "habitat/services"

+++

One of the key features of Chef Habitat is the ability to define an immutable package with a default configuration which can then be updated dynamically at runtime. You can update service configuration on two levels: individual services (for testing purposes), or a service group.

## Apply Configuration Updates to an Individual Service

When starting a single service, you can provide alternate configuration values to those specified in `default.toml`.

### Using a _user.toml_ File

You can supply a `user.toml` containing any configuration data that you want to override default values. This file should be placed in the Chef Habitat `user` directory under the `config` subdirectory of the specific service directory that owns the configuration data. For example, to override the default configuration of the `myservice` service, this `user.toml` would be located at `/hab/user/myservice/config/user.toml`.

### Using an Environment Variable

Override default configuration data through the use of an environment variable with the following format: `HAB_PACKAGENAME='{"keyname1":"newvalue1", "tablename1":{"keyname2":"newvalue2"}}'`.

```bash
$ HAB_MYTUTORIALAPP='{"message":"Chef Habitat rocks!"}' hab run <origin>/<packagename>
```

> Note: The syntax used for applying configuration through environment variables can be either JSON or TOML, but TOML is preferred. The package name in the environment variable must be uppercase, any dashes must be replaced with underscores.

> Note: The way that environment variable configuration is currently processed means that variables must be set when the Supervisor process starts, not when the service is loaded, which may require a bit of planning on the part of the Chef Habitat operator. This may change in the future.

For multiline environment variables, such as those in a TOML table or nested key value pairs, it can be easier to place your changes in a file and pass it in using something like `HAB_PACKAGENAME="$(cat foo.toml)"` or `HAB_PACKAGENAME="$(cat foo.json)"`.

```bash
$ HAB_MYTUTORIALAPP="$(cat my-env-stuff.toml)" hab run
$ hab svc load <origin>/mytutorialapp
```
(or `HAB_MYTUTORIALAPP="$(cat my-env-stuff.toml)" hab run <origin>/mytutorialapp` for testing scenarios and containerized workflows; see [here](#using-packages)).

The main advantage of applying configuration updates to an individual service through an environment variable is that you can quickly test configuration settings to see how your service behaves at runtime. The disadvantages of this method are that configuration changes have to be applied when the Supervisor itself starts up, and you have to restart a running Supervisor (and thus, all services it may be running) in order to change these settings again.

## Apply Configuration Updates to all Services in a Service Group
Similar to specifying updates to individual settings at runtime, you can apply multiple configuration changes to an entire service group at runtime. These configuration updates can be sent in the clear or encrypted in gossip messages through [wire encryption](/using-habitat/using-encryption). Configuration updates to a service group will trigger a restart of the services as new changes are applied throughout the group.

### Usage

When submitting a configuration update to a service group, you must specify a Supervisor to connect to, the version number of the configuration update, and the new configuration itself. Configuration updates can be either TOML passed into stdin, or passed in a TOML file that is referenced in `hab config apply`.

Configuration updates for service groups must be versioned. The version number must be an integer that starts at one and must be incremented with every subsequent update to the same service group. *If the version number is less than or equal to the current version number, the change(s) will not be applied.*

Here are some examples of how to apply configuration changes through both the shell and through a TOML file.

**Stdin**

```bash
$ echo 'buffersize = 16384' | hab config apply --remote-sup=hab1.mycompany.com myapp.prod 1
```

**TOML file**

```bash
$ hab config apply --remote-sup=hab1.mycompany.com myapp.prod 1 /tmp/newconfig.toml
```

  > Note: The filename of the configuration file is not important.

  > Note: 1 is the version number. Increment this for
  additional configuration updates.

    Your output would look something like this:

       » Setting new configuration version 1 for myapp.prod
       Ω Creating service configuration
       ↑ Applying via peer 172.18.0.2:9632
       ★ Applied configuration

  The services in the myapp.prod service group will restart.

       myapp.prod(SR): Service configuration updated from butterfly: acd2c21580748d38f64a014f964f19a0c1547955e4c86e63bf641a4e142b2200
       hab-sup(SC): Updated myapp.conf a85c2ed271620f895abd3f8065f265e41f198973317cc548a016f3eb60c7e13c
       myapp.prod(SV): Stopping
       ...
       myapp.prod(SV): Starting

> Note: As with all Supervisor interaction commands, if you do not specify `--remote-sup`, `hab config apply` will attempt to connect to a Supervisor running on the same host.

### Encryption

Configuration updates can be encrypted for the service group they are intended. To do so, pass the `--user` option with the name of your user key, and the `--org` option with the organization of the service group. If you have the public key for the service group, the data will be encrypted for that key, signed with your user key, and sent to the ring.

It will then be stored encrypted in memory, and decrypted on disk.

