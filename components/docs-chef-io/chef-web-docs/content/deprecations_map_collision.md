+++
title = "Deprecation: Map Collision (CHEF-25)"
draft = false
robots = "noindex"


aliases = "/deprecations_map_collision.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_map_collision.md)

The resource(s) referenced in the error message has been loaded from a
cookbook. This resource is now included in Chef Infra Client and will
take precedence over the existing cookbook resource in the next major
release of Chef Infra Client (15.0, April 2019). Alternatively, there
may be a newer version of this cookbook without this resource.

## Remediation

Please upgrade your cookbook to the latest version, which may fix your
issue, or ignore this error message.
