---
title: Apply config updates to a service or service group
---

# Configuration updates

One of the key features of Habitat is the ability to define an immutable package with a default configuration which can then be updated dynamically at runtime. You can update service configuration on two levels: individual services (for testing purposes), or a service group.


## Apply configuration updates to an individual service
When starting a single service, you can provide alternate configuration values to those specified in `default.toml` through the use of an environment variable 
with the following format: `HAB_PACKAGENAME='keyname1=newvalue1 keyname2=newvalue2'`. 

    HAB_MYTUTORIALAPP='message = "Habitat rocks!"' hab start <origin>/<packagename>

> Note: The package name in the environment variable must be uppercase, any dashes must be replaced with underscores, and if you are overriding values in a TOML table, you must override all values in the table.

For multiline environment variables, such as those in a TOML table, it's preferrable to place your changes in a .toml
file and pass it in using `HAB_PACKAGENAME="$(cat foo.toml)"`.

    HAB_MYTUTORIALAPP="$(cat my-env-stuff.toml)" hab start <origin>/<packagename>

The main advantage of applying configuration updates to an individual service through an environment variable is that you can quickly test configuration settings to see how your service behaves at runtime. The disadvantages of this method are that configuration changes have to be applied to one service at a time, and you have to manually interrupt (Ctrl+C) a running service before changing its configuration settings again. 

For an example of how to use an environment variable to update default configuration values, see [Run your service](/tutorials/getting-started/linux/process-build) in the Getting Started tutorial.

## Apply configuration updates to a service group
Similar to specifying updates to individual settings at runtime, you can apply multiple configuration changes to an entire service group at runtime using stdin from your shell or through a TOML file. These configuration updates can be sent in the clear or encrypted in gossip messages through [wire encryption](/docs/run-packages-security#wire-encryption). Configuration updates to a service group will trigger a restart of the services as new changes are applied throughout the group.

> Note: Wire encryption secures all traffic between supervisors in a ring that possess a ring key; however, if a supervisor has the ring key, it can read any configuration content passed around the ring.

### Usage
When submitting a configuration update to a service group, you must specify a peer in the ring to connect to, the version number of the configuration update, and the new configuration itself. Configuration updates can be either TOML passed into stdin, or passed in a TOML file that is referenced in `hab config apply`.

Configuration updates for service groups must be versioned. The version number must be an integer that starts at one and must be incremented with every subsequent update to the same service group. *If the version number is less than or equal to the current version number, the change(s) will not be applied.*

Here are some examples of how to apply configuration changes through both the shell and through a TOML file.

**Stdin**

       echo 'buffersize = 16384' | hab config apply --peer 172.17.0.3 myapp.prod 1

**TOML file**

      hab config apply --peer 172.17.0.3 myapp.prod 1 /tmp/newconfig.toml

  > Note: The filename of the configuration file is not important.

  > Note: 1 is the version number. Increment this for
  additional configuration updates.

    Your output would look something like this:

       » Applying configuration
       ↑ Applying configuration for myapp.prod into ring via ["172.17.0.3:9634"]
       Joining peer: 172.17.0.3:9634
       Configuration applied to: 172.17.0.3:9634
       ★ Applied configuration.

  The services in the myapp.prod service group will restart according to the service group's [update strategy](/docs/run-packages-update-strategy).

      Writing new file from gossip: /hab/svc/myapp/gossip.toml
      hab-sup(SC): Updated config.json
      myapp(SV): Stopping
      hab-sup(SV): myapp - process 981 died with signal 15
      hab-sup(SV): myapp - Service exited
      myapp(SV): Starting
      ...

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-upload-files">Upload files</a></li>
</ul>
