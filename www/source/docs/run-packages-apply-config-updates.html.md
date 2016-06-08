---
title: Apply config updates to a service group
---

# Apply configuration updates to a service group
Similar to specifying updates to individual settings at runtime, you can apply multiple configuration changes to an entire service group at runtime using stdin from your shell or through a TOML file. These configuration updates can be sent in the clear, or encrypted so only service group members can see the changes.

## Usage
When submitting a configuration update to a service group, you must specify a peer in the service group to connect to, the version number of the configuration update, and the new configuration itself. Configuration updates can be either TOML passed into stdin, or passed in a TOML file that is referenced in `hab config apply`.

Configuration updates for service groups must be versioned and the version number must be incremented with every subsequent update to the same service group. If the version number is less than or equal to the current version number, the change(s) will not be applied.

Here are some examples of how to apply configuration changes through both the shell and through a TOML file.

**Stdin**

       echo 'buffersize = 16384' | hab config apply 1 myapp.prod

**TOML file**

      hab config apply --peer 172.17.0.3 1 /tmp/newconfig.toml

  > Note: The filename of the configuration file is not important.

    Your output would look something like this:

       » Applying configuration
       ↑ Applying configuration for myapp.prod into ring via ["172.17.0.3:9634"]
       Joining peer: 172.17.0.3:9634
       Configuration applied to: 172.17.0.3:9634
       ★ Applied configuration.

    The services in the myapp.prod service group will restart with the new configuration changes.

      Writing new file from gossip: /hab/svc/myapp/gossip.toml
      hab-sup(SC): Updated config.json
      myapp(SV): Stopping
      hab-sup(SV): myapp - process 981 died with signal 15
      hab-sup(SV): myapp - Service exited
      myapp(SV): Starting
      ...

## Applying changes securely
Configuration updates can optionally be encrypted to securely pass updates to a service group. This involves defining service and user keys to encrypt, verify, and decrypt the updates. See [Service Group Encryption](/docs/run-packages-security#service-group-encryption) for more information.
