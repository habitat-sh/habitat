Use the `property` method to define properties for the custom resource.
The syntax is:

``` ruby
property :property_name, ruby_type, default: 'value', parameter: 'value'
```

where

-   `:property_name` is the name of the property
-   `ruby_type` is the optional Ruby type or array of types, such as
    `String`, `Integer`, `true`, or `false`
-   `default: 'value'` is the optional default value loaded into the
    resource
-   `parameter: 'value'` optional parameters

For example, the following properties define `username` and `password`
properties with no default values specified:

``` ruby
property :username, String
property :password, String
```