This example uses the `:before` notification to restart the `php-fpm`
service before restarting `nginx`:

``` ruby
service 'nginx' do
  action :restart
  notifies :restart, 'service[php-fpm]', :before
end
```

With the `:before` notification, the action specified for the `nginx`
resource will not run until action has been taken on the notified
resource (`php-fpm`).