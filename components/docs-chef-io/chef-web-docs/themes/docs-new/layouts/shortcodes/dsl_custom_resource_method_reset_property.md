Use the `reset_property` method to clear the value for a property as if
it had never been set, and then use the default value. For example, to
clear the value for a property named `password`:

``` ruby
reset_property(:password)
```