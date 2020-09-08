Use the `property_is_set?` method to check if the value for a property
is set. The syntax is:

``` ruby
property_is_set?(:property_name)
```

The `property_is_set?` method will return `true` if the property is set.

For example, the following custom resource creates and/or updates user
properties, but not their password. The `property_is_set?` method checks
if the user has specified a password and then tells Chef Infra Client
what to do if the password is not identical:

``` ruby
action :create do
  converge_if_changed do
    shell_out!("rabbitmqctl create_or_update_user #{username} --prop1 #{prop1} ... ")
  end

  if property_is_set?(:password)
    if shell_out("rabbitmqctl authenticate_user #{username} #{password}").error?
      converge_by "Updating password for user #{username} ..." do
        shell_out!("rabbitmqctl update_user #{username} --password #{password}")
      end
    end
  end
end
```