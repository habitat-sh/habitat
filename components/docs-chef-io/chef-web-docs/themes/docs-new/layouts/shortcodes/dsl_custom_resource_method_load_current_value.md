Use the `load_current_value` method to load the specified property
values from the node, and then use those values when the resource is
converged. This method may take a block argument.

``` ruby
property :path, String
property :content, String
property :mode, String

load_current_value do |new_resource|
  if ::File.exist?(new_resource.path)
    content IO.read(new_resource.path)
    mode ::File.stat(new_resource.path).mode
  end
end
```

Use the `load_current_value` method to guard against property values
being replaced. For example:

``` ruby
property :homepage, String
property :page_not_found, String

load_current_value do
  if ::File.exist?('/var/www/html/index.html')
    homepage IO.read('/var/www/html/index.html')
  end

  if ::File.exist?('/var/www/html/404.html')
    page_not_found IO.read('/var/www/html/404.html')
  end
end
```

This ensures the values for `homepage` and `page_not_found` are not
changed to the default values when Chef Infra Client configures the
node.