An environment is defined using JSON or the Ruby DSL. chef-solo will
look for environments in `/var/chef/environments`, but this location can
be modified by changing the setting for `environment_path` in solo.rb.
For example, the following setting in solo.rb:

``` ruby
environment_path '/var/chef-solo/environments'
```

Environment data looks like the following in JSON:

``` javascript
{
  "name": "dev",
  "default_attributes": {
    "apache2": {
      "listen_ports": [
        "80",
        "443"
      ]
    }
  },
  "json_class": "Chef::Environment",
    "description": "",
    "cookbook_versions": {
    "couchdb": "= 11.0.0"
  },
  "chef_type": "environment"
  }
```

and like the following in the Ruby DSL:

``` ruby
name 'environment_name'
description 'environment_description'
cookbook OR cookbook_versions  'cookbook' OR 'cookbook' => 'cookbook_version'
default_attributes 'node' => { 'attribute' => %w(value value etc.) }
override_attributes 'node' => { 'attribute' => %w(value value etc.) }
```