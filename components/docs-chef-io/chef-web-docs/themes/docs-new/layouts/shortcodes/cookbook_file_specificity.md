A cookbook is frequently designed to work across many platforms and is
often required to distribute a specific file to a specific platform. A
cookbook can be designed to support the distribution of files across
platforms, while ensuring that the correct file ends up on each system.

The pattern for file specificity depends on two things: the lookup path
and the source attribute. The first pattern that matches is used:

1.  /host-\$fqdn/\$source
2.  /\$platform-\$platform_version/\$source
3.  /\$platform/\$source
4.  /default/\$source
5.  /\$source

Use an array with the `source` attribute to define an explicit lookup
path. For example:

``` ruby
file '/conf.py' do
  source ['#{node.chef_environment}.py', 'conf.py']
end
```

The following example emulates the entire file specificity pattern by
defining it as an explicit path:

``` ruby
file '/conf.py' do
  source %W{
    host-#{node['fqdn']}/conf.py
    #{node['platform']}-#{node['platform_version']}/conf.py
    #{node['platform']}/conf.py
    default/conf.py
  }
end
```

A cookbook may have a `/files` directory structure like this:

    files/
       host-foo.example.com
       ubuntu-20.04
       ubuntu-20
       ubuntu
       redhat-8.2
       redhat-7.8
       ...
       default

and a resource that looks something like the following:

``` ruby
cookbook_file '/usr/local/bin/apache2_module_conf_generate.pl' do
  source 'apache2_module_conf_generate.pl'
  mode '0755'
  owner 'root'
  group 'root'
end
```

This resource is matched in the same order as the `/files` directory
structure. For a node that is running Ubuntu 16.04, the second item
would be the matching item and the location to which the file identified
in the **cookbook_file** resource would be distributed:

``` ruby
host-foo.example.com/apache2_module_conf_generate.pl
ubuntu-20.04/apache2_module_conf_generate.pl
ubuntu-20/apache2_module_conf_generate.pl
ubuntu/apache2_module_conf_generate.pl
default/apache2_module_conf_generate.pl
```

If the `apache2_module_conf_generate.pl` file was located in the
cookbook directory under `files/host-foo.example.com/`, the specified
file(s) would only be copied to the machine with the domain name
foo.example.com.

**Host Notation**

The naming of folders within cookbook directories must literally match
the host notation used for file specificity matching. For example, if a
host is named `foo.example.com`, the folder must be named
`host-foo.example.com`.
