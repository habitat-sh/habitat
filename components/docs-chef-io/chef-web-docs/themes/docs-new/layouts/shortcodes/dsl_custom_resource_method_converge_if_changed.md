Use the `converge_if_changed` method inside an `action` block in a
custom resource to compare the desired property values against the
current property values (as loaded by the `load_current_value` method).
Use the `converge_if_changed` method to ensure that updates only occur
when property values on the system are not the desired property values
and to otherwise prevent a resource from being converged.

To use the `converge_if_changed` method, wrap it around the part of a
recipe or custom resource that should only be converged when the current
state is not the desired state:

``` ruby
action :some_action do

  converge_if_changed do
    # some property
  end

end
```

For example, a custom resource defines two properties (`content` and
`path`) and a single action (`:create`). Use the `load_current_value`
method to load the property value to be compared, and then use the
`converge_if_changed` method to tell Chef Infra Client what to do if
that value is not the desired value:

``` ruby
property :content, String
property :path, String, name_property: true

load_current_value do
  if ::File.exist?(path)
    content IO.read(path)
  end
end

action :create do
  converge_if_changed do
    IO.write(new_resource.path, new_resource.content)
  end
end
```

When the file does not exist, the
`IO.write(new_resource.path, new_resource.content)` code is executed and
the Chef Infra Client output will print something similar to:

``` bash
Recipe: recipe_name::block
  * resource_name[blah] action create
    - update my_file[blah]
    -   set content to "hola mundo" (was "hello world")
```