The following example shows using multiple `Chef::Log` entry types:

``` ruby
...

begin
  aws = Chef::DataBagItem.load(:aws, :main)
  Chef::Log.info("Loaded AWS information from DataBagItem aws[#{aws['id']}]")
rescue
  Chef::Log.fatal("Could not find the 'main' item in the 'aws' data bag")
  raise
end

...
```

The full recipe is in the `ebs_volume.rb` recipe of the [database
cookbook](https://github.com/chef-cookbooks/database/) that is
maintained by Chef.