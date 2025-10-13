+++
title = "Environment Variables"
description = "Customize and configure your Chef Habitat Studio and Supervisor environments"
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Environment Variables"
    identifier = "habitat/reference/environment-variables"
    parent = "habitat/reference"

+++

This is a list of all environment variables that can be used to modify the operation of the Chef Habitat Studio and Supervisor.

| Variable | Context | Default | Description |
|----------|----------|----------|-----------|
| `HAB_AUTH_TOKEN` | build system | no default | Authorization token used to perform privileged operations against the depot, e.g. uploading packages or keys. Can also be configured in `~/.hab/etc/cli.toml` as `auth_token`.
| `HAB_BINLINK_DIR` | build system | `/hab/bin` | Allows you to change the target directory for the symlink created when you run `hab pkg binlink`. The default value is already included in the `$PATH` variable inside the Studio. |
| `HAB_CACHE_KEY_PATH` | build system, Supervisor | `/hab/cache/keys` if running as root; `$HOME/.hab/cache/keys` if running as non-root | Cache directory for origin signing keys |
| `HAB_CTL_SECRET` | Supervisor | no default | Shared secret used for [communicating with a Supervisor]({{< relref "sup_remote_control" >}}). |
| `HAB_BLDR_CHANNEL` | build system, Supervisor | `stable` | Set the Chef Habitat Builder channel you are subscribing to, to a specific channel. Defaults to `stable`.
| `HAB_BLDR_URL` | build system, Supervisor | `https://bldr.habitat.sh` | Sets an alternate default endpoint for communicating with Builder. Used by the Chef Habitat build system and the Supervisor |
| `HAB_DOCKER_OPTS` | build system | no default | When running a Studio on a platform that uses Docker (macOS), additional command line options to pass to the `docker` command. |
| `HAB_INTERNAL_BLDR_CHANNEL` | build system, Supervisor, exporters | `stable` | Channel from which Chef Habitat-specific packages (e.g., `chef/hab-sup`, `chef/hab-launcher`, etc.) are downloaded on-demand when first called. Generally of use only for those developing Chef Habitat. Only applies to Chef Habitat-specific packages, and nothing else. |
| `HAB_LICENSE` | build system, Supervisor, exporters | no default | Used to accept the [Chef EULA]({{< relref "chef_license#chef-eula" >}}). See [Accepting the Chef License]({{< relref "chef_license_accept#habitat" >}}) for valid values. |
| `HAB_LISTEN_CTL` | Supervisor | 127.0.0.1:9632 | The listen address for the Control Gateway. This also affects `hab` commands that interact with the Supervisor via the Control Gateway, for example: `hab sup status`. |
| `HAB_LISTEN_GOSSIP` | Supervisor | 0.0.0.0:9638 | The listen address for the Gossip System Gateway |
| `HAB_LISTEN_HTTP` | Supervisor | 0.0.0.0:9631 | The listen address for the HTTP Gateway |
| `HAB_NOCOLORING` | build system | no default | If set to the lowercase string `"true"` this environment variable will unconditionally disable text coloring where possible |
| `HAB_NONINTERACTIVE` | build system | no default | If set to the lowercase string `"true"` this environment variable will unconditionally disable interactive progress bars (i.e. "spinners") where possible |
| `HAB_ORG` | Supervisor | no default | Organization to use when running with [service group encryption]({{< relref "sup_secure" >}})
| `HAB_ORIGIN` | build system | no default | Origin used to build packages. The signing key for this origin is passed to the build system. |
| `HAB_ORIGIN_KEYS` | build system | no default | Comma-separated list of origin keys to automatically share with the build system |
| `HAB_REFRESH_CHANNEL` | build system | `stable` | Channel used to retrieve plan dependencies for Chef supported origins. Can also be configured in `~/.hab/etc/cli.toml` as `refresh_channel`. |
| `HAB_RING` | Supervisor | no default | The name of the ring used by the Supervisor when running with [wire encryption]({{< relref "sup_secure" >}}) |
| `HAB_RING_KEY` | Supervisor | no default | The contents of the ring key when running with [wire encryption]({{< relref "sup_secure" >}}). Useful when running in a container. |
| `HAB_STUDIO_SECRET_<VARIABLE>` | build system | no default | Prefix to allow environment variables into the Studio. The prefix will be removed and your variable will be passed into the Studio at build time. |
| `HAB_STUDIOS_HOME` | build system | `/hab/studios` | Directory in which to create build Studios |
| `HAB_STUDIO_BACKLINE_PKG` | build system | `chef/hab-backline/{{studio_version}}` | Overrides the default package identifier for the "backline" package which installs the Studio baseline package set. |
| `HAB_STUDIO_ROOT` | build system | no default | Root of the current Studio under `$HAB_STUDIOS_HOME`. Infrequently overridden. |
| `HAB_STUDIO_NOSTUDIORC` | build system | no default | When set to a non-empty value, a `.studiorc` will not be sourced when entering an interactive Studio via `hab studio enter`. |
| `HAB_STUDIO_SUP` | build system | no default | Used to customize the arguments passed to an automatically launched Supervisor, or to disable the automatic launching by setting it to `false`, `no`, or `0`. |
| `HAB_GLYPH_STYLE` | build system | `full` (`limited` on Windows) | Used to customize the rendering of unicode glyphs in UI messages. Valid values are `full`, `limited`, or `ascii`. |
| `HAB_SUP_UPDATE_MS` | Supervisor | 60000 | Interval in milliseconds governing how often to check for Supervisor updates when running with the [--auto-update]({{< relref "habitat_cli/#hab-sup-run" >}}) flag. Note: This variable has been deprecated. Users should instead use the [--auto-update-period]({{< relref "habitat_cli/#hab-sup-run" >}}) flag. |
| `HAB_UPDATE_STRATEGY_FREQUENCY_MS` | Supervisor | 60000 | Interval in milliseconds governing how often to check for service updates when running with an [update strategy]({{< relref "service_updates" >}}). Note: This variable has been deprecated. Users should instead use the [--service-update-period]({{< relref "habitat_cli/#hab-sup-run" >}}) flag. |
| `HAB_USER` | Supervisor | no default | User key to use when running with [service group encryption]({{< relref "sup_secure" >}}) |
| `http_proxy` | build system, Supervisor | no default | A URL for a local HTTP proxy server optionally supporting basic authentication |
| `https_proxy` | build system, Supervisor | no default | A URL for a local HTTPS proxy server optionally supporting basic authentication |
| `NO_INSTALL_DEPS` | build system | no default | Set this variable to prevent dependencies install during build |
| `no_proxy` | build system, Supervisor | no default | A comma-separated list of domain exclusions for the `http_proxy` and `https_proxy` environment variables |
| `SSL_CERT_FILE` | system | no default | Standard OpenSSL environment variable to override the system certificate file. This is particularly important for the secure HTTPS connection with a Builder instance. Can be used to help you navigate corporate firewalls. |
