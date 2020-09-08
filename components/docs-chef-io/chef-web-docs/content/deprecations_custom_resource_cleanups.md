+++
title = "Deprecation: Custom Resource Cleanups (CHEF-5)"
draft = false
robots = "noindex"


aliases = "/deprecations_custom_resource_cleanups.html"

+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_custom_resource_cleanups.md)

We are continuously improving and streamlining the way custom resources
work in Chef, to make it easier for cookbook authors and Chef developers
to build resources.

This page documents many deprecations over the course of many Chef
releases.

## Nil Properties

In current versions of Chef, `nil` was often used to mean that a
property had no good default, and needed to be set by the user. However,
it is often to useful to set a property to `nil`, meaning that it's not
set and should be ignored. In Chef Client 13, it is an error to set
`default: nil` on a property if that property doesn't allow `nil` as a
valid value.

### Remediation

If it is valid for the property to be set to nil, then update the
property to include that.

``` ruby
property :my_nillable_property, [ String, nil ], default: nil
```

Otherwise, remove the `default: nil` statement from the property.

## Invalid Defaults

Current versions of Chef emit a warning when a property's default value
is not valid. This is often because the type of the default value
doesn't match the specification of the property. For example:

``` ruby
property :my_property, [ String ], default: []
```

sets the type of the property to be a String, but then sets the default
to be an Array. In Chef Client 13, this will be an error.

### Remediation

Ensure that the default value of a property is correct.

## Property Getters

When writing a resource in Chef Client 12, calling `some_property nil`
behaves as a getter, returning the value of `some_property`. In Chef
Client 13, this will change to set `some_property` to `nil`.

### Remediation

Simply write `some_property` when retrieving the value of
`some_property`.

## Specifying both "default" and "name_property" on a resource

Current versions of Chef emit a warning if the property declaration has
both `default` and `name_property` set. In Chef Client 13, that will
become an error. For example:

``` ruby
property :my_property, [ String ], default: [], name_property: true
```

### Remediation

A property can either have a default, or it can be a "name" property
(meaning that it will take the value of the resource's name if not
otherwise specified), but not both.

## Overriding provides?

Some providers override the `provides?` method, used to check whether
they are a valid provider on the current platform. In Chef Client 13,
this will cause an error if the provider does not also register
themselves with the `provides` call.

### Example

``` ruby
def provides?
  true
end
```

### Remediation

``` ruby
provides :my_provider

def provides?
  true
end
```

## Don't use the updated method

The `updated=(true_or_false)` method is deprecated and will be removed
from Chef Client 13. This method never performed its intended job, as
notifications from the resource would not fire, and in general its use
has always been buggy. The Chef Infra Client notification code checks
`updated_by_last_action?` instead, so setting that is recommended as a
substitute. See the
[updated_by_last_action](/custom_resources_notes/#updated-by-last-action)
documentation for more information.

{{< note >}}

Setting `updated_by_last_action` is almost always unnecessary, and
correct use of `use_inline_resources` (which is the default in Chef
Client 13 and above) makes the `updated_by_last_action` setting
redundant. Simply deleting this code is very likely to be the correct
course of action in nearly all cases.

{{< /note >}}

### Example

``` ruby
action :foo do
  updated = true
end
```

### Remediation

``` ruby
action :foo do
  new_resource.updated_by_last_action true
end
```

## Don't use the dsl_name method

The `dsl_name` method is deprecated and will be removed from Chef Client
1.  It has been replaced by `resource_name`.

### Example

``` ruby
my_resource = MyResource.dsl_name
```

### Remediation

``` ruby
my_resource = MyResource.resource_name
```

## Don't use the provider_base method

The `Resource.provider_base` allows the developer to specify an
alternative module to load providers from, rather than `Chef::Provider`.
It is deprecated and will be removed in Chef Client 13. Instead, the
provider should call `provides` to register itself, or the resource
should call `provider` to specify the provider to use.
