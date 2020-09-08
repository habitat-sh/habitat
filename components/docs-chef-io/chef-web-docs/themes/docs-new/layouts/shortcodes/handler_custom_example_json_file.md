The
[json_file](https://github.com/chef/chef/blob/master/lib/chef/handler/json_file.rb)
handler is available from the **chef_handler** cookbook and can be used
with exceptions and reports. It serializes run status data to a JSON
file. This handler may be enabled in one of the following ways.

By adding the following lines of Ruby code to either the client.rb file
or the solo.rb file, depending on how Chef Infra Client is being run:

``` ruby
require 'chef/handler/json_file'
report_handlers << Chef::Handler::JsonFile.new(:path => '/var/chef/reports')
exception_handlers << Chef::Handler::JsonFile.new(:path => '/var/chef/reports')
```

By using the **chef_handler** resource in a recipe, similar to the
following:

``` ruby
chef_handler 'Chef::Handler::JsonFile' do
  source 'chef/handler/json_file'
  arguments :path => '/var/chef/reports'
  action :enable
end
```

After it has run, the run status data can be loaded and inspected via
Interactive Ruby (IRb):

``` ruby
irb(main):002:0> require 'json' => true
irb(main):003:0> require 'chef' => true
irb(main):004:0> r = JSON.parse(IO.read('/var/chef/reports/chef-run-report-20110322060731.json')) => ... output truncated
irb(main):005:0> r.keys => ['end_time', 'node', 'updated_resources', 'exception', 'all_resources', 'success', 'elapsed_time', 'start_time', 'backtrace']
irb(main):006:0> r['elapsed_time'] => 0.00246
```