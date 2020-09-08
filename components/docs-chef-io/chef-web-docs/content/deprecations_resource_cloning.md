+++
title = "Deprecation: Resource Cloning (CHEF-3694)"
draft = false
robots = "noindex"


aliases = "/deprecations_resource_cloning.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_resource_cloning.md)

Chef allows resources to be created with duplicate names, rather than
treating that as an error. This means that several cookbooks can request
the same package be installed, without needing to carefully create
unique names. This is problematic because having multiple resources with
the same name makes it impossible to safely deliver notifications to the
right resource.

In Chef Client 13, resources with the same name will be treated as
entirely separate, without any cloning of properties.

The behavior in Chef Client 12 and earlier, which is now deprecated, is
that we will try to clone the existing resource, and then apply any
properties from the new resource. For example:

``` ruby
file "/etc/my_file" do
  owner "ken"
end

file "/etc/my_file" do
  mode "0755"
end
```

will result in the second instance having the following properties:

``` ruby
file "/etc/my_file" do
  owner "ken"
  mode "0755"
end
```

Resource cloning was deprecated in Chef Client 10.18.0 and will be
removed in Chef Client 13.

{{< note >}}

Chef will only emit a deprecation warning in the situation that a cloned
resource is significantly different from the existing one.

{{< /note >}}

## Remediation

Ensure that resources you intend to notify are given unique names.
