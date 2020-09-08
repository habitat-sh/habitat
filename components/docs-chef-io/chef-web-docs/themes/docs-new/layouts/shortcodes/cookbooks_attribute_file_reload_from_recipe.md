Attributes sometimes depend on actions taken from within recipes, so it
may be necessary to reload a given attribute from within a recipe. For
example:

``` ruby
ruby_block 'some_code' do
  block do
    node.from_file(run_context.resolve_attribute('COOKBOOK_NAME', 'ATTR_FILE'))
  end
  action :nothing
end
```