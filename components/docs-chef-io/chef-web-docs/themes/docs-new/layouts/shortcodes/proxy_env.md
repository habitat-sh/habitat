If `http_proxy`, `https_proxy`, `ftp_proxy`, or `no_proxy` is set in the
client.rb file but not set in the `ENV`, Chef Infra Client will
configure the `ENV` variable based on these (and related) settings. For
example:

``` ruby
http_proxy 'http://proxy.example.org:8080'
http_proxy_user 'myself'
http_proxy_pass 'Password1'
```

Or an alternative way to define the proxy (if the previous version does
not work):

``` ruby
http_proxy 'http://myself:Password1@proxy.example.org:8080'
```

will be set to:

``` ruby
ENV['http_proxy'] = 'http://myself:Password1@proxy.example.org:8080'
```