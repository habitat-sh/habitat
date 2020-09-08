The `converge_if_changed` method may be used multiple times. The
following example shows how to use the `converge_if_changed` method to
compare the multiple desired property values against the current
property values (as loaded by the `load_current_value` method).

``` ruby
property :path, String
property :content, String
property :mode, String

load_current_value do |desired|
  if ::File.exist?(desired.path)
    content IO.read(desired.path)
    mode ::File.stat(desired.path).mode
  end
end

action :create do
  converge_if_changed :content do
    IO.write(new_resource.path, new_resource.content)
  end
  converge_if_changed :mode do
    ::File.chmod(new_resource.mode, new_resource.path)
  end
end
```

where

-   `load_current_value` loads the property values for both `content`
    and `mode`
-   A `converge_if_changed` block tests only `content`
-   A `converge_if_changed` block tests only `mode`

Chef Infra Client will only update the property values that require
updates and will not make changes when the property values are already
in the desired state