A **breakpoint** resource block creates a breakpoint in a recipe:

``` ruby
breakpoint 'name' do
  action :break
end
```

where

-   `:break` will tell Chef Infra Client to stop running a recipe; can
    only be used when Chef Infra Client is being run in chef-shell mode