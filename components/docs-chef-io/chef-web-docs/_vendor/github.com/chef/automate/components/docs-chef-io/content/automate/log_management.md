+++
title = "Log Management"

draft = false

[menu]
  [menu.automate]
    title = "Log Management"
    parent = "automate/configuring_automate"
    identifier = "automate/configuring_automate/log_management.md Log Management"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/log_management.md)

Chef Automate 2.0 uses `systemd`. Log management is performed according to the configuration defined for the system service `journald`.

### Viewing Logs

To view the logs you can run:

```shell
journalctl -u chef-automate
```

To follow the logs in realtime:

```shell
journalctl -u chef-automate -f
```

For information on changing the output, please refer to the man page or run:

```shell
journalctl --help
```

### Configuring Log Level

You can configure Chef Automate log level for all services by creating a TOML file and configuring the log level. By default each service will initialize at the "info" level but can be any of 'debug, 'info', 'warning', 'panic', or 'fatal'.

```shell
[global.v1.log]
level = "debug"
```

Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

### Configuring Log Rotation and Retention

Log rotation and retention settings are managed at a system level using `journald`. At this point, `journald` does not support log retention policies at a granular level for units within itself. See the [man page](https://www.freedesktop.org/software/systemd/man/journald.conf.html) for more configuration options in `/etc/systemd/journald.conf`.
