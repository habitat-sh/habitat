The following custom handler defines how cookbooks and cookbook versions
that are used during a Chef Infra Client run will be compiled into a
report using the `Chef::Log` class in Chef Infra Client:

``` ruby
require 'chef/log'

module Opscode
  class CookbookVersionsHandler < Chef::Handler

    def report
      cookbooks = run_context.cookbook_collection
      Chef::Log.info('Cookbooks and versions run: #{cookbooks.keys.map {|x| cookbooks[x].name.to_s + ' ' + cookbooks[x].version} }')
    end
  end
end
```