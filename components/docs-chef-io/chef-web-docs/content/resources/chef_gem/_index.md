---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_gem resource
resource: chef_gem
aliases:
- "/resource_chef_gem.html"
menu:
  infra:
    title: chef_gem
    identifier: chef_infra/cookbook_reference/resources/chef_gem chef_gem
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: |-
    Use the **chef_gem** resource to install a gem only for the instance of Ruby that is dedicated to the chef-client.
    When a gem is installed from a local file, it must be added to the node using the **remote_file** or **cookbook_file** resources.

    The **chef_gem** resource works with all of the same properties and options as the **gem_package** resource, but does not
    accept the `gem_binary` property because it always uses the `CurrentGemEnvironment` under which the `chef-client` is
    running. In addition to performing actions similar to the **gem_package** resource, the **chef_gem** resource does the
    following:
    - Runs its actions immediately, before convergence, allowing a gem to be used in a recipe immediately after it is installed.
    - Runs `Gem.clear_paths` after the action, ensuring that gem is aware of changes so that it can be required immediately after it is installed.
- warning:
    markdown: |-
      The **chef_gem** and **gem_package** resources are both used to install Ruby gems. For any machine on which Chef Infra Client is
      installed, there are two instances of Ruby. One is the standard, system-wide instance of Ruby and the other is a dedicated instance that is
      available only to Chef Infra Client.
      Use the **chef_gem** resource to install gems into the instance of Ruby that is dedicated to Chef Infra Client.
      Use the **gem_package** resource to install all other gems (i.e. install gems system-wide).
syntax_full_code_block: |-
  chef_gem 'name' do
    clear_sources               true, false
    gem_binary                  String
    include_default_source      true, false
    options                     String, Hash, Array
    package_name                String
    source                      String, Array
    timeout                     String, Integer
    version                     String
    action                      Symbol # defaults to :install if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`chef_gem` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`clear_sources`, `gem_binary`, `include_default_source`, `options`, `package_name`,
  `source`, `timeout`, and `version` are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a gem. If a version is specified, install the specified
      version of the gem.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :purge:
    markdown: Purge a gem. This action typically removes the configuration files as
      well as the gem.
  :reconfig:
    markdown: Reconfigure a gem. This action requires a response file.
  :remove:
    markdown: Remove a gem.
  :upgrade:
    markdown: Install a gem and/or ensure that a gem is the latest version.
properties_list:
- property: clear_sources
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Set to `true` to download a gem from the path specified by the `source`
      property (and not from RubyGems).
- property: gem_binary
  ruby_type: String
  required: false
  default_value: The `gem` binary included with Chef Infra Client.
  description_list:
  - markdown: The path of a gem binary to use for the installation. By default, the
      same version of Ruby that is used by Chef Infra Client will be used.
- property: include_default_source
  ruby_type: true, false
  required: false
  new_in: '13.0'
  description_list:
  - markdown: Set to `false` to not include `Chef::Config[:rubygems_url]` in the sources.
- property: options
  ruby_type: String, Hash, Array
  required: false
  description_list:
  - markdown: Options for the gem install, either a Hash or a String. When a hash
      is given, the options are passed to `Gem::DependencyInstaller.new`, and the
      gem will be installed via the gems API. When a String is given, the gem will
      be installed by shelling out to the gem command. Using a Hash of options with
      an explicit gem_binary will result in undefined behavior.
- property: package_name
  ruby_type: String
  required: false
  description_list:
  - markdown: An optional property to set the package name if it differs from the
      resource block's name.
- property: source
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: Optional. The URL, or list of URLs, at which the gem package is located.
      This list is added to the source configured in `Chef::Config[:rubygems_url]`
      (see also include_default_source) to construct the complete list of rubygems
      sources. Users in an 'airgapped' environment should set Chef::Config[:rubygems_url]
      to their local RubyGems mirror.
- property: timeout
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: version
  ruby_type: String
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded.
examples: |
  **Compile time vs. converge time installation of gems**

  To install a gem while Chef Infra Client is configuring the node (the converge phase), set the `compile_time` property to `false`:
  ```ruby
  chef_gem 'right_aws' do
    compile_time false
    action :install
  end
  ```

  To install a gem while the resource collection is being built (the compile phase), set the `compile_time` property to `true`:
  ```ruby
  chef_gem 'right_aws' do
    compile_time true
    action :install
  end
  ```

  Install MySQL for Chef
  ```ruby
  apt_update

  build_essential 'install compilation tools' do
    compile_time true
  end

  chef_gem 'mysql'
  ```
---