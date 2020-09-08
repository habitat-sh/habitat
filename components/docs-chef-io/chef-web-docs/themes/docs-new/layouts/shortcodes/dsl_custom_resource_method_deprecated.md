
## Deprecating entire resources

Chef Infra Client 14+ provides new primitives that allow you to deprecate resources or properties with the same
functionality used for deprecations in Chef Infra Client resources. This allows you make breaking changes
to enterprise or community cookbooks with friendly notifications to downstream cookbook consumers
directly in the Chef Infra Client run.

Deprecate the foo_bar resource in a cookbook

``` ruby
deprecated "The foo_bar resource has been deprecated and will be removed in the next major release of this cookbook scheduled for 12/25/2021!"

property :thing, String, name_property: true

action :create do
 # Chef resource code
end
```

## Deprecating a property

Deprecate the `badly_named` property in a resource:

```ruby
property :badly_named, String, deprecated: 'The badly_named property has been deprecated and will be removed in the next major release of this cookbook scheduled for 12/25/2021!'
```

## Deprecate and alias

Rename a property with a deprecation warning for users of the old property name:

```ruby
deprecated_property_alias 'badly_named', 'really_well_named', 'The badly_named property was renamed really_well_named in the 2.0 release of this cookbook. Please update your cookbooks to use the new property name.'
```
