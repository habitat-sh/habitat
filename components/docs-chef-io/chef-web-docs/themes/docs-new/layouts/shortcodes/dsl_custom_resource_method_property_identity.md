Add `identity:` to set a resource to a particular set of properties.
This value may be `true` or `false`.

-   When `true`, data for that property is returned as part of the
    resource data set and may be available to external applications,
    such as reporting
-   When `false`, no data for that property is returned.

If no properties are marked `true`, the property that defaults to the
`name` of the resource is marked `true`.

For example, the following properties define `username` and `password`
properties with no default values specified, but with `identity` set to
`true` for the user name:

``` ruby
property :username, String, identity: true
property :password, String
```