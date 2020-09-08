+++
title = "Deprecation: resource_name declaration without provides (CHEF-31)"
draft = false
robots = "noindex"

+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_resource_name_without_provides.md)

In Chef Client 12.5.1 through 15, resources could be addressed from
recipe code using only the name of the resource, provided that no other
`provides` declaration was used in the same file.

The intent was to fulfill the demands that "users should just be
able to name a resource and it should work" with the demands that, for
more complicated use cases, users should be able to use `provides`
lines to arbitrarily wire up resources to the Chef recipe language.

This failed because it attempted to overly simplify user intent while
generating two separate constructs with confusing interactions. Most
users were entirely unaware that the `resource_name` statement implicitly
issued a `provides` statement which wired up a specially designated
`canonical` DSL entry, which was then removed behind the scenes if any
subsequent `provides` declaration followed. When this worked it was
easy to use, but when it failed the edge conditions were confusing and
required too much background knowledge to debug.

An attempt was made to preserve more complete backwards compatibility between
Chef Infra Client 16.0 and earlier versions by retaining some automatic
wiring of the `provides` statement with the `resource_name`. This failed
due to complicated interactions between cookbooks that used multiple
resources with the same name wired up via `provides` lines to different
resource implementations on different operating systems. This was a silent
error and dependent upon the parse order of the resources in the cookbook
for it to become apparent, and could not be easily detected or remedied.

The solution eventually adopted in Chef Infra Client 16.2 was to require
all resources to declare a `provides` lines, and to make the `resource_name`
setting only affect the display output. As a result, any cookbook which
declares a resource with only a `resource_name` needs to add a `provides`
line for Chef Infra Client 16. While this is more disruptive to users it
is simple, it can be autocorrected via static analysis, and it results in
a much simpler end state where the `resource_name` is just a display name
and the `provides` statement is solely responsible for how the resource
is addressed in recipe mode.

There is also the very old standard that existed before resources could
declare what they provided. In that standard, the resource was addressed
by prepending the `cookbook_name` to the filename that the resource was declared in.
That has remained unchanged and is not affected by this change.

## Remediation

A resource with only a `resource_name` property:

```ruby
resource_name :my_custom_resource

property :my_property, String

action :run do
  [ ...implementation of the action... ]
end
```

Should have a `provides` statement added:

```ruby
resource_name :my_custom_resource
provides :my_custom_resource

property :my_property, String

action :run do
  [ ...implementation of the action... ]
end
```

It also works to have the `provides` line come prior to the `resource_name`,
the order does not matter.

For cookbooks which do not have to support Chef Infra Client 15 or before, the
`resource_name` can also be entirely omitted:

```ruby
provides :my_custom_resource

property :my_property, String

action :run do
  [ ...implementation of the action... ]
end
```
