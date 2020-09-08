Use the `resource_name` method at the top of a custom resource to
declare a custom name for that resource. For example:

``` ruby
resource_name :my_resource_name
```

The `resource_name` is only used as a fallback name for display purposes.
Even for display purposes, the name used in recipe code which matches a
`provides` statement is favored over the `resource_name` setting, so the
`resource_name` has little effect. In Chef Infra Client 16 and later,
the first `provides` in a resource declaration also sets the fallback
`resource_name`, so we do not recommend that users set the `resource_name` at all.
The `resource_name` setting is necessary for backwards compatibility with Chef
Infra Client 12 through 15.
