+++
title = "Deprecation: Filesystem2 attribute removal (OHAI-12)"
draft = false
robots = "noindex"


aliases = "/deprecations_ohai_filesystem_v2.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_filesystem_v2.md)

In Ohai/Chef Client 13 we replaced the existing Filesystem plugin with
the Filesystem V2 plugin. That was done by having Ohai populate both
`node['filesystem']` and `node['filesystem_v2']` with the data
previously found at `node['filesystem2']`. In Chef Client 14 we will no
longer populate `node['filesystem2']`.

## Remediation

If you have a cookbook that relies on data from `node['filesystem2']`
you will need to update the code to instead use `node['filesystem']`.
Keep in mind that if you're attempting to support releases earlier than
Chef Client 13 the data structure of node\['filesystem'\] will be
different. Foodcritic's FC094 rule will detect any usage of the
`node['filesystem_v2']` attributes.
