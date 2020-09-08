+++
title = "Deprecation: System Profile plugin (OHAI-14)"
draft = false
robots = "noindex"


aliases = "/depreciations_ohai_system_profile.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_system_profile.md)

The system_profile plugin will be removed from Chef/Ohai 15 in April
2019. This plugin does not correctly return data on modern Mac systems.
Additionally the same data is provided by the hardware plugin, which has
a format that is simpler to consume. Removing this plugin will reduce
Ohai return by \~3 seconds and greatly reduce the size of the node
object on the Chef Infra Server.

## Remediation

If you relied on data in node\['system_profile'\], you should look at
the format in node\['hardware'\] and migrate to that new data format.
