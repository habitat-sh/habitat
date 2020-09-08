<div class="admonition-warning"><p class="admonition-warning-title">Warning</p><div class="admonition-warning-text">

This is an example of something that should NOT be done. Use the
**service** resource to control a service, not the **execute** resource.

</div></div>

Do something like this:

``` ruby
service 'tomcat' do
  action :start
end
```

and NOT something like this:

``` ruby
execute 'start-tomcat' do
  command '/etc/init.d/tomcat6 start'
  action :run
end
```

There is no reason to use the **execute** resource to control a service
because the **service** resource exposes the `start_command` property
directly, which gives a recipe full control over the command issued in a
much cleaner, more direct manner.