+++
title = "Deprecation: Removal of IpScopes Plugin (OHAI-13)"
draft = false
robots = "noindex"


aliases = "/deprecations_ohai_ipscopes.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_ipscopes.md)



Chef/Ohai 14 (April 2018) will remove the IpScopes plugin. The data
returned by this plugin is nearly identical to information already
returned by individual network plugins, and this plugin required the
inclusion of an additional gem into the Chef installation. We believe
that few users were installing the gem, and users would be better served
by the data returned from network plugins.
