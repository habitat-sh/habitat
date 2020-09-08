+++
title = "Custom Resources Notes"
draft = false

aliases = ["/custom_resources_notes.html"]

[menu]
  [menu.infra]
    title = "Custom Resource Guide"
    identifier = "chef_infra/cookbook_reference/resources/custom_resources_notes.md Custom Resource Guide"
    parent = "chef_infra/cookbook_reference/resources"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/custom_resources_notes.md)

{{< warning >}}

This page mentions multiple ways of building custom resources. Chef
Software recommends you try the approach outlined in the [Custom
Resource documentation](/custom_resources/) first, before trying the
resource/provider pair (older approach) or library type (pure Ruby)
approaches. If you run into issues while designing 12.5-style custom
resources, please ask for help in the [Chef Mailing
List](https://discourse.chef.io) or [file a
bug](https://github.com/chef/chef/issues/new) for Chef Infra Client.

{{< /warning >}}

## Custom Resources

This is the recommended way of writing resources for all users. There
are two gotchas which we're working through:

1.  For helper functions that you used to write in your provider code or
    used to mixin to your provider code, you have to use an
    `action_class.class_eval do ... end` block.

You cannot subclass, and must use mixins for code-sharing (which is
really a best practice anyway -- e.g. see languages like rust which do
not support subclassing).

in `resources/whatever.rb`:

``` ruby
resource_name :my_resource
provides :my_resource

property :foo, String, name_property: true
extend MyResourceHelperFunctions  # probably only used for common properties which is why you extend with class methods

action :run do
  # helpers must be defined inside the action_class block
  a_helper()
  # you will save yourself some pain by referring to properties with `new_resource.foo` and not `foo`
  # since the latter works most of the time, but will troll you with odd scoping problems, while the
  # former just works.
  puts new_resource.foo
end

action_class.class_eval do
  include MyProviderHelperFunctions

  def a_helper
  end
end
```

## "Old school" LWRPS

This method is not recommended, but is preferable to writing library
resources/providers (as described below). It has the same functionality
as library providers, only you cannot subclass and must use mixins for
code sharing (which is good).

in `resources/my_resource.rb`:

``` ruby
resource_name :my_resource
provides :my_resource

property :foo, String, name_property: true
extend MyResourceHelperFunctions  # probably only used for common properties which is why you extend with class methods
```

in `providers/my_resource.rb`:

``` ruby
# you have to worry about this
def whyrun_supported?
  true
end

include MyProviderHelperFunctions

def a_helper
end

action :run do
  a_helper()
  # here you have to use new_resource.foo
  puts new_resource.foo
end
```

## Library Resources/Providers

Library resources are discouraged since you can more easily shoot
yourself in the foot. They used to be encouraged back before Chef Client
12.0 `provides` was introduced since it allowed for renaming the
resource so that it didn't have to be prefixed by the cookbook name.

There are many ways to go wrong writing library providers. One of the
biggest issues is that internal Chef Infra Client code superficially
looks like a library provider, but it is not. Chef internal resources do
not inherit from `LWRPBase` and we've had to manually create resources
directly through `Chef::Resource::File.new()`, we also have not been
able to `use_inline_resources` and not had access to other niceties that
cookbook authors have had access to for years now. We've got some
modernization of internal Chef cookbook code now and resources like
`apt_update` and `apt_repository` in core have started to be written
more like cookbook code should be written, but core resources are
actually behind the curve and are bad code examples.

in `libraries/resource_my_resource.rb`:

``` ruby
class MyBaseClass
  class Resource
    class MyResource < Chef::Resource::LWRPBase  # it is very important to inherit from LWRPBase
      resource_name :my_resource
      provides :my_resource

      property :foo, String, name_property: true
      extend MyResourceHelperFunctions  # probably only used for common properties which is why you extend with class methods
    end
  end
end
```

in `libraries/resource_my_resource.rb`:

``` ruby
class MyBaseClass
  class Resource
    class MyProvider < Chef::Provider::LWRPBase  # it is very important to inherit from LWRPBase

      # you have to worry about this
      def whyrun_supported?
        true
      end

      include MyProviderHelperFunctions

      def a_helper
      end

      # NEVER use `def action_run` here -- you defeat use_inline_resources and will break notifications if you do
      # If you don't understand how use_inline_resources is built and why you have to use the `action` method, and what the implications are and how resource notifications
      # break if use_inline_resources is not used and/or is broken, then you should really not be using library providers+resources.  You might feel "closer to the metal",
      # but you're now using a chainsaw without any guard...
      action :run do
        a_helper()
          # here you have to use new_resource.foo
          puts new_resource.foo
      end
    end
  end
end
```

## updated_by_last_action

Modern Chef Infra Client code (since Chef Client version 11.0.0) should
never have provider code which directly sets `updated_by_last_action`
itself.

THIS CODE IS WRONG:

``` ruby
action :run do
  t = file "/tmp/foo" do
    content "foo"
  end
  t.run_action(:install)
  # This is Chef Client 10 code which fell through a timewarp into 2016 -- never use updated_by_last_action in modern Chef Client 11.x/12.x code
  t.new_resource.updated_by_last_action(true) if t.updated_by_last_action?
end
```

That used to be kinda-correct-code-with-awful-edge-cases back in Chef
Client version 10. If you're not using that version of Chef Client,
please stop writing actions this way.

THIS IS CORRECT:

``` ruby
def whyrun_supported?
  true
end

action :run do
  file "/tmp/foo" do
    content "foo"
  end
end
```

That is the magic of `use_inline_resources` (and why
`use_inline_resources` is turned on by default in Chef Client 12.5
resources) The sub-resources are defined in a sub-resource collection
which is compiled and converged as part of the provider executing. Any
resources that update in the sub-resource collection cause the resource
itself to be updated automatically. Notifications then fire normally off
the resource. It also works to arbitrary levels of nesting of
sub-sub-sub-resources being updating causing the wrapping resources to
update and fire notifications.

This also gets the why-run case correct. If all the work that you do in
your resource is done by calling sub-resources, then why-run should work
automatically. All your sub-resources will be NO-OP'd and will report
what they would have done instead of doing it.

If you do need to write code which mutates the system through pure-Ruby
then you should do so like this:

``` ruby
def whyrun_supported?
  true
end

action :run do
  unless File.exist?("/tmp/foo")
    converge_by("touch /tmp/foo") do
      ::FileUtils.touch "/tmp/foo"
    end
  end
end
```

When the `converge_by` block is run in why-run mode, it will only log
`touch "/tmp/foo"` and will not run the code inside the block.

A `converge_by` block that is not wrapped in an idempotency check will
always cause the resource to be updated, and will always cause
notifications to fire. To prevent this, a properly written resource
should wrap all `converge_by` checks with an idempotency check. The
[`converge_if_changed`](/custom_resources/#converge_if_changed)
block may be used instead which will wrap a `converge_by` block with an
idempotency check for you.

``` ruby
action :run do
  # This code is bad, it lacks an idempotency check here.
  # It will always be updated
  # Chef Infra Client runs will always report a resource being updated
  # It will run the code in the block on every run
  converge_by("touch /tmp/foo") do
    ::FileUtils.touch "/tmp/foo"
  end
end
```

Of course it is vastly simpler to just use Chef Infra Client resources
when you can. Compare the equivalent implementations:

``` ruby
action :run do
  file "/tmp/foo"
end
```

is basically the same as this:

``` ruby
action :run do
  unless File.exist?("/tmp/foo")
    converge_by("touch /tmp/foo") do
      ::FileUtils.touch "/tmp/foo"
    end
  end
end
```

You may see a lot of `converge_by` and `updated_by_last_action` in the
core chef resources. This is sometimes due to the fact that Chef is
written as a declarative language with an imperative language, which
means someone has to take the first step and write the declarative file
resources in imperative Ruby. As such, core Chef resources may not
represent ideal code examples with regard to what custom resources
should look like.
