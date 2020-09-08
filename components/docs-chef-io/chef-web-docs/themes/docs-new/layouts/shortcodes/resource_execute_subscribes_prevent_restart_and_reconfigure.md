Use the `:nothing` action (common to all resources) to prevent the test
from starting automatically, and then use the `subscribes` notification
to run a configuration test when a change to the template is detected:

``` ruby
execute 'test-nagios-config' do
  command 'nagios3 --verify-config'
  action :nothing
  subscribes :run, 'template[/etc/nagios3/configures-nagios.conf]', :immediately
end
```