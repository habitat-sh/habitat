+++
title = "Deprecation: Ohai::Config removal (OHAI-1)"
draft = false
robots = "noindex"


aliases = "/deprecations_ohai_legacy_config.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_legacy_config.md)

Ohai 8.8.0 (Chef Client 12.6.0) introduced a new Ohai configuration
system as defined in
[RFC-053](https://github.com/chef/chef-rfc/blob/master/rfc053-ohai-config.md).
This system replaced the existing usage of `Ohai::Config` config system,
which will be removed in Chef Client 13.

## Remediation

Previous Ohai configuration values in the `client.rb` file need to be
updated for the new configuration system format. For example, to
configure the `plugin_path` value previously you would set
`Ohai::Config[:plugin_path] = "/etc/chef/ohai/plugins.local"`, where as
you would now use `ohai.plugin_path = "/etc/chef/ohai/plugins.local"`.
See the [Ohai Configuration
Documentation](/ohai/#ohai-settings-in-client-rb) for additional
usage information.
