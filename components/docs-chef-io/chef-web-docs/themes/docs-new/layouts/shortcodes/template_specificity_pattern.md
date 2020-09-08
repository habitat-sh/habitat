The pattern for template specificity depends on two things: the lookup
path and the source. The first pattern that matches is used:

1.  `/host-$fqdn/$source`
2.  `/$platform-$platform_version/$source`
3.  `/$platform/$source`
4.  `/default/$source`
5.  `/$source`

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

To specify a particular Windows version, use the [operating system
version
number](https://docs.microsoft.com/en-us/windows/win32/sysinfo/operating-system-version).
For example, a template in `templates/windows-6.3` will be deployed on
systems installed with Windows 8.1.



</div>

</div>

Use an array with the `source` property to define an explicit lookup
path. For example:

``` ruby
template '/test' do
  source ["#{node.chef_environment}.erb", 'default.erb']
end
```

The following example emulates the entire file specificity pattern by
defining it as an explicit path:

``` ruby
template '/test' do
  source %W(
    host-#{node['fqdn']}/test.erb
    #{node['platform']}-#{node['platform_version']}/test.erb
    #{node['platform']}/test.erb
    default/test.erb
  )
end
```