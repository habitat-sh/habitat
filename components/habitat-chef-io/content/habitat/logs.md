+++
title = "Chef Habitat Builder on-prem logging"
description = "Logs and Logging for Chef Habitat Builder on-prem"

[menu]
  [menu.habitat]
    title = "Logs"
    identifier = "habitat/builder-on-prem/logs"
    parent = "habitat"

+++

The recognized values for logging are: `error`, `warn`, `info`, `debug`, and `trace`.
For a more detailed explanation of logging in Chef Habitat, see the [Supervisor Log Configuration Reference](https://www.habitat.sh/docs/reference/#supervisor-log-configuration-reference) and the [Supervisor Log Key](https://www.habitat.sh/docs/reference/#supervisor-log-key) documentation.

### Basic Logging

To turn on and examine the services debug logging in your Habitat installation:

1. Edit the `sudo /hab/svc/builder-api/user.toml` file
1. On the first line, change the log_level from **error** to **debug**

    ```toml
    log_level="debug,tokio_core=error,tokio_reactor=error,zmq=error,hyper=error"
    ```

1. Save and close the file
1. Restart Habitat with `sudo systemctl restart hab-sup`.
1. Use `journalctl -fu hab-sup` to view the logs.
1. Reset `/hab/svc/builder-api/user.toml` file to the default `log_level=error` and restart the services with `sudo systemctl restart hab-sup` to restore error-level logging.

### RUST_LOG

1. Use `RUST_LOG=debug RUST_BACKTRACE=1` to see an individual command's debug and backtrace.

    ```bash
    # Linux/MacOS
    # replace "hab sup run" with your command
    env RUST_LOG=debug RUST_BACKTRACE=1 hab sup run
    ```

1. Edit the `sudo /hab/svc/builder-api/user.toml` file
1. On the second line, change:

    ```toml
    RUST_LOG=debug RUST_BACKTRACE=1
    ```

### Log Rotation

The `builder-api-proxy` service will log (via Nginx) all access and errors to log files in your service directory. Since these files may get large, you may want to add a log rotation script. Below is a sample logrotate file that you can use as an example for your needs:

    ```bash
    /hab/svc/builder-api-proxy/logs/host.access.log
    /hab/svc/builder-api-proxy/logs/host.error.log
    {
            rotate 7
            daily
            missingok
            notifempty
            delaycompress
            compress
            postrotate
                    /bin/kill -USR1 `cat /hab/svc/builder-api-proxy/var/pid 2>/dev/null` 2>/dev/null || true
            endscript
    }
    ```
