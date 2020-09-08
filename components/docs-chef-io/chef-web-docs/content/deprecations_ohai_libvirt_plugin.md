+++
title = "Deprecation: Libvirt plugin attributes changes (OHAI-4)"
draft = false
robots = "noindex"


aliases = "/deprecations_ohai_libvirt_plugin.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_libvirt_plugin.md)

The Ohai libvirt plugin no longer places libvirt attributes under
`node['virtualization']` and instead uses the `node['libvirt']`
namespace to match other virtualization plugins.

## Remediation

Cookbooks utilizing attributes from the libvirt plugin under
`node['virtualization']` will need to be updated to instead use those
same attributes from `node['libvirt']`.
