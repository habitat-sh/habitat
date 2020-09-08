The
[error_report](https://github.com/chef/chef/blob/master/lib/chef/handler/error_report.rb)
handler is built into Chef Infra Client and can be used for both
exceptions and reports. It serializes error report data to a JSON file.
This handler may be enabled in one of the following ways.

By adding the following lines of Ruby code to either the client.rb file
or the solo.rb file, depending on how Chef Infra Client is being run:

``` ruby
require 'chef/handler/error_report'
report_handlers << Chef::Handler::ErrorReport.new()
exception_handlers << Chef::Handler::ErrorReport.new()
```

By using the [chef_handler](/resources/chef_handler/) resource in a
recipe, similar to the following:

``` ruby
chef_handler 'Chef::Handler::ErrorReport' do
  source 'chef/handler/error_report'
  action :enable
end
```