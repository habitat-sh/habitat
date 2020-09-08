+++
title = "Deprecation: Use of property_name inside of actions (CHEF-19)"
draft = false
robots = "noindex"

aliases = "/deprecations_namespace_collisions.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_namespace_collisions.md)

In Chef Client 12.5.1, the custom resources API allowed specifying
property names as the short form of `property_name` inside of actions,
instead of the long form of `new_resource.property_name` (as was
previously required in provider code in LWRPs/HWRPs/etc). That change
caused unsolvable namespace clashes and will be removed in Chef Client
14.0, and it will become mandatory to refer to properties as
`new_resource.property_name` in actions.

## Example

This code worked in Chef Client 12.5.1 and later revisions up to Chef
Client 13.0:

``` ruby
property :my_content, String

action :doit do
  file "/tmp/file.xy" do
    content my_content
  end
end
```

## Remediation

The `my_content` reference will no longer be wired up automatically to
the `new_resource` object and users will need to specify
`new_resource.my_content` explicitly:

``` ruby
property :my_content, String

action :doit do
  file "/tmp/file.xy" do
    content new_resource.my_content
  end
end
```

## Note

In some edge cases, this deprecation warning may mention that the
property should be referred to as `current_resource.property_name`
instead of `new_resource.property_name`, which is not a mistake; the
user should instead use the `current_resource.property_name` to preserve
prior behavior, or should modify their code to explicitly check the
`current_resource` if the `new_resource` is not set. There are several
possible remediations to this in the order of least complicated to the
most compatible with the old behavior, and the user will need to select
what works best for their use case:

``` ruby
content_to_set = new_resource.property_name || current_resource.property_name
content_to_set = new_resource.property_name.nil? ? current_resource.property_name : new_resource.property_name
content_to_set = new_resource.property_is_set?(:property_name) ? new_resource.property_name : current_resource.property_name
```

Unfortunately, if you were reliant upon the old code's automatic
switching between the `new_resource` and `current_resource` you will
need to be explicit. Most users, however, were not aware that this was
occurring and moving that uncommon logic explicitly into the action code
will produce more comprehensible code that is less reliant on subtle
tricks of the API.

It is also entirely possible that the access of the `current_resource`
was never intended by the user. If this behavior was undesired, the
correct remediation would be to simply access the property through the
`new_resource.property_name`. We cannot determine and accurately report
to the user when this deprecation message is incorrect, we can only
report on compatible behavior. The suggestion of the deprecation warning
to access the property through `current_resource.property_name` may be
incorrect, and it is up to the discretion of the user to choose the
appropriate remediation for their needs.

The fact that this is confusing behavior to explain is why it is being
removed.

## Rationale

The change in Chef Client 12.5.1 caused several insolvable problems. One
of the worst was that properties would override DSL commands so that
(for example) if a user had a `template` property they could no longer
use the <span class="title-ref">template</span> resource:

``` ruby
property :template, String

action :doit do
  template "/tmp/file.xy" do  # this would NOT create a template resource but would pass a string and a block to the template property
    source "file.xy.erb"
    variables({ stuff: "whatever" })
  end
end
```

The highly confusing workaround for this problem was to use
`declare_resource` to avoid the use of the resource DSL:

``` ruby
property :template, String

action :doit do
  declare_resource(:template, "/tmp/file.xy") do # now there is no ambiguity and we create a template resource
    source "file.xy.erb"
    variables({ stuff: "whatever" })
  end
end
```

This also caused issues when properties conflicted with properties on
subresources, where this example is ambiguous as to if the `content`
argument to `content` refers to the file subresource `content` property,
or if it refers to the parent custom resource `content` property.

``` ruby
property :content, String

action :doit do
  puts "content: #{content}"
  file "/tmp/file.xy" do
    content content
  end
end
```

In fact, the subprocess wins (because it has to) and this code will
result in the content always being nil and the file being empty. The
output of the `puts` debugging will be correct, however, since `content`
is being accessed outside of the file resource scope so it acquires it
from the `new_resource` implicitly (in Chef Client 12.5.1 and Chef
Client 13.x)

The way to remediate that is by specifying the `new_resource`:

``` ruby
property :content, String

action :doit do
  file "/tmp/file.xy" do
    content new_resource.content
  end
end
```

We are now enforcing this as the correct way to write resources.

Note that this namespace collision between custom resources and
subresources occurs with properties that are not also being immediately
used, and so this fails as well:

``` ruby
property :mode, String

action :doit do
  file "/tmp/file.xy" do
    content mode  # this accesses the mode property on the file resource rather than the mode property on the outer resource
  end
end
```

This will also cause namespace collisions if at some point in the future
a new property is introduced to a subresource.

``` ruby
property :spiffyness, String

action :doit do
  file "/tmp/file.xy" do
    content spiffyness
  end
end
```

This will work fine today, but if at some point in the future the file
resource grows a `spiffyness` property, then this will cause a namespace
collision with the custom resource and will result in the custom
resource failing. This is avoided by the explicit use of `new_resource`:

``` ruby
property :spiffyness, String

action :doit do
  file "/tmp/file.xy" do
    content new_resource.spiffyness # we are always referring to the outer custom resource's spiffiness property
  end
end
```
