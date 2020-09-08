A resource may listen to another resource, and then take action if the
state of the resource being listened to changes. Specify a
`'resource[name]'`, the `:action` to be taken, and then the `:timer` for
that action.

Note that `subscribes` does not apply the specified action to the
resource that it listens to - for example:

``` ruby
file '/etc/nginx/ssl/example.crt' do
  mode '0600'
  owner 'root'
end

service 'nginx' do
  subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
end
```

In this case the `subscribes` property reloads the `nginx` service
whenever its certificate file, located under
`/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make any
changes to the certificate file itself, it merely listens for a change
to the file, and executes the `:reload` action for its resource (in this
example `nginx`) when a change is detected.