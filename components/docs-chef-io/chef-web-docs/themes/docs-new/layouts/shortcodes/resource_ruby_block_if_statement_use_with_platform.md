The following example shows how an if statement can be used with the
`platform?` method in the Recipe DSL to run code specific to Microsoft
Windows. The code is defined using the **ruby_block** resource:

``` ruby
# the following code sample comes from the ``client`` recipe
# in the following cookbook: https://github.com/chef-cookbooks/mysql

if platform?('windows')
  ruby_block 'copy libmysql.dll into ruby path' do
    block do
      require 'fileutils'
      FileUtils.cp "#{node['mysql']['client']['lib_dir']}\\libmysql.dll",
        node['mysql']['client']['ruby_dir']
    end
    not_if { File.exist?("#{node['mysql']['client']['ruby_dir']}\\libmysql.dll") }
  end
end
```