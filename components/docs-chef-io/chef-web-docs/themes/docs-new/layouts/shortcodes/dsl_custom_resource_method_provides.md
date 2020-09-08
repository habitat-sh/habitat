Use the `provides` method to associate a custom resource with the Recipe
DSL on different operating systems. When multiple custom resources use
the same DSL, specificity rules are applied to determine the priority,
from highest to lowest:

1.  provides :my_custom_resource, platform_version: '0.1.2'
2.  provides :my_custom_resource, platform: 'platform_name'
3.  provides :my_custom_resource, platform_family: 'platform_family'
4.  provides :my_custom_resource, os: 'operating_system'
5.  provides :my_custom_resource

For example:

``` ruby
provides :my_custom_resource, platform: 'redhat' do |node|
  node['platform_version'].to_i >= 7
end

provides :my_custom_resource, platform: 'redhat'

provides :my_custom_resource, platform_family: 'rhel'

provides :my_custom_resource, os: 'linux'

provides :my_custom_resource
```

This allows you to use multiple custom resources files that provide the
same resource to the user, but for different operating systems or
operation system versions. With this you can eliminate the need for
platform or platform version logic within your resources.
