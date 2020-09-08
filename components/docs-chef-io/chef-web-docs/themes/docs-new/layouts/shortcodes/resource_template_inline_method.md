A template helper method is always defined inline on a per-resource
basis. A simple example:

``` ruby
template '/path' do
  helper(:hello_world) { 'hello world' }
end
```

Another way to define an inline helper method is to reference a node
object so that repeated calls to one (or more) cookbook attributes can
be done efficiently:

``` ruby
template '/path' do
  helper(:app) { node['app'] }
end
```

An inline helper method can also take arguments:

``` ruby
template '/path' do
  helper(:app_conf) { |setting| node['app'][setting] }
end
```

Once declared, a template can then use the helper methods to build a
file. For example:

``` ruby
Say hello: <%= hello_world %>
```

or:

``` ruby
node['app']['listen_port'] is: <%= app['listen_port'] %>
```

or:

``` ruby
node['app']['log_location'] is: <%= app_conf('log_location') %>
```