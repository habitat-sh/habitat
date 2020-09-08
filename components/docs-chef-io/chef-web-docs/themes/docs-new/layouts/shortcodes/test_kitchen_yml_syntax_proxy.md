The environment variables `http_proxy`, `https_proxy`, and `ftp_proxy`
are honored by Test Kitchen for proxies. The client.rb file is read to
look for proxy configuration settings. If `http_proxy`, `https_proxy`,
and `ftp_proxy` are specified in the client.rb file, Chef Infra Client
will configure the `ENV` variable based on these (and related) settings.
For example:

``` ruby
http_proxy 'http://proxy.example.org:8080'
http_proxy_user 'myself'
http_proxy_pass 'Password1'
```

will be set to:

``` ruby
ENV['http_proxy'] = 'http://myself:Password1@proxy.example.org:8080'
```

Test Kitchen also supports `http_proxy` and `https_proxy` in the
`kitchen.yml` file. You can set them manually or have them read from
your local environment variables:

``` yaml
driver:
  name: vagrant

provisioner:
  name: chef_zero
  # Set proxy settings manually, or
  http_proxy: 'http://user:password@server:port'
  https_proxy: 'http://user:password@server:port'

  # Read from local environment variables
  http_proxy: <%= ENV['http_proxy'] %>
  https_proxy: <%= ENV['https_proxy'] %>
```

This will not set the proxy environment variables for applications other
than Chef. The Vagrant plugin,
[vagrant-proxyconf](http://tmatilai.github.io/vagrant-proxyconf/), can
be used to set the proxy environment variables for applications inside
the VM.