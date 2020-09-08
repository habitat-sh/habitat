Custom resources are designed to use core resources that are built into
Chef. In some cases, it may be necessary to specify a property in the
custom resource that is the same as a property in a core resource, for
the purpose of overriding that property when used with the custom
resource. For example:

``` ruby
property :command, String, name_property: true
property :version, String

# Useful properties from the `execute` resource
property :cwd, String
property :environment, Hash, default: {}
property :user, [String, Integer]
property :sensitive, [true, false], default: false

prefix = '/opt/languages/node'

load_current_value do
  current_value_does_not_exist! if node.run_state['nodejs'].nil?
  version node.run_state['nodejs'][:version]
end

action :run do
  execute 'execute-node' do
    cwd cwd
    environment environment
    user user
    sensitive sensitive
    # gsub replaces 10+ spaces at the beginning of the line with nothing
    command <<-CODE.gsub(/^ {10}/, '')
      #{prefix}/#{new_resource.version}/#{command}
    CODE
  end
end
```

where the `property :cwd`, `property :environment`, `property :user`,
and `property :sensitive` are identical to properties in the **execute**
resource, embedded as part of the `action :run` action. Because both the
custom properties and the **execute** properties are identical, this
will result in an error message similar to:

``` ruby
ArgumentError
-------------
wrong number of arguments (0 for 1)
```

To prevent this behavior, use `new_resource.` to tell Chef Infra Client
to process the properties from the core resource instead of the
properties in the custom resource. For example:

``` ruby
property :command, String, name_property: true
property :version, String

# Useful properties from the `execute` resource
property :cwd, String
property :environment, Hash, default: {}
property :user, [String, Integer]
property :sensitive, [true, false], default: false

prefix = '/opt/languages/node'

load_current_value do
  current_value_does_not_exist! if node.run_state['nodejs'].nil?
  version node.run_state['nodejs'][:version]
end

action :run do
  execute 'execute-node' do
    cwd new_resource.cwd
    environment new_resource.environment
    user new_resource.user
    sensitive new_resource.sensitive
    # gsub replaces 10+ spaces at the beginning of the line with nothing
    command <<-CODE.gsub(/^ {10}/, '')
      #{prefix}/#{new_resource.version}/#{new_resource.command}
    CODE
  end
end
```

where `cwd new_resource.cwd`, `environment new_resource.environment`,
`user new_resource.user`, and `sensitive new_resource.sensitive`
correctly use the properties of the **execute** resource and not the
identically-named override properties of the custom resource.
