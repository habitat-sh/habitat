For example, the `httpd.rb` file in the `website` cookbook could be
assigned a custom resource name like this:

``` ruby
resource_name :httpd

property :homepage, String, default: '<h1>Hello world!</h1>'

action :create do
  package 'httpd'

  service 'httpd' do
    action [:enable, :start]
  end

  file '/var/www/html/index.html' do
    content new_resource.homepage
  end
end
```

and is then usable in a recipe like this:

``` ruby
httpd 'build website' do
  homepage '<h1>Welcome to the Example Co. website!</h1>'
  action :create
end
```