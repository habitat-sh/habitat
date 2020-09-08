---
resource_reference: true
common_resource_functionality_multiple_packages: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
notes_resource_based_on_package: true
title: homebrew_package resource
resource: homebrew_package
aliases:
- "/resource_homebrew_package.html"
menu:
  infra:
    title: homebrew_package
    identifier: chef_infra/cookbook_reference/resources/homebrew_package homebrew_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **homebrew_package** resource to manage packages for the macOS
    platform.
- note:
    markdown: Starting with Chef Infra Client 16 the homebrew resource now accepts
      an array of packages for installing multiple packages at once.
resource_new_in: '12.0'
syntax_full_code_block: |-
  homebrew_package 'name' do
    homebrew_user      String, Integer
    options            String, Array
    package_name       String, Array
    source             String
    timeout            String, Integer
    version            String, Array
    action             Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`homebrew_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`homebrew_user`, `options`, `package_name`, `source`, `timeout`, and `version`
  are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :purge:
    markdown: Purge a package. This action typically removes the configuration files
      as well as the package.
  :remove:
    markdown: Remove a package.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.
properties_list:
- property: homebrew_user
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 'The name or uid of the Homebrew owner to be used by Chef Infra Client
      when executing a command.


      Chef Infra Client, by default, will attempt to execute a Homebrew

      command as the owner of `/usr/local/bin/brew`. If that executable

      does not exist, Chef Infra Client will attempt to find the user by

      executing `which brew`. If that executable cannot be found, Chef

      Infra Client will print an error message:

      `Could not find the "brew" executable in /usr/local/bin or anywhere on the path.`.

      Use the `homebrew_user` attribute to specify the Homebrew owner for

      situations where Chef Infra Client cannot automatically detect the

      correct owner.'
- property: options
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: One (or more) additional command options that are passed to the command.
- property: package_name
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: An optional property to set the package name if it differs from the
      resource block's name.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: The optional path to a package on the local file system.
- property: timeout
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: version
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded.
examples: |
  **Install a package**:

  ```ruby
  homebrew_package 'git'
  ```

  **Install multiple packages at once**:

  ```ruby
  homebrew_package %w(git fish ruby)
  ```

  **Specify the Homebrew user with a UUID**

  ```ruby
  homebrew_package 'git' do
    homebrew_user 1001
  end
  ```

  **Specify the Homebrew user with a string**:

  ```ruby
  homebrew_package 'vim' do
    homebrew_user 'user1'
  end
  ```
---