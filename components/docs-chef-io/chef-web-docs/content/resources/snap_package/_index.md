---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: snap_package resource
resource: snap_package
aliases:
- "/resource_snap_package.html"
menu:
  infra:
    title: snap_package
    identifier: chef_infra/cookbook_reference/resources/snap_package snap_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **snap_package** resource to manage snap packages on Debian and
    Ubuntu platforms.
resource_new_in: '15.0'
syntax_full_code_block: |-
  snap_package 'name' do
    channel           String # default value: "stable"
    options           String, Array
    package_name      String, Array
    source            String
    timeout           String, Integer
    version           String, Array
    action            Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`snap_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`channel`, `options`, `package_name`, `source`, `timeout`, and `version` are the
  properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :lock:
    markdown: Locks the apt package to a specific version.
  :purge:
    markdown: Purge a package. This action typically removes the configuration files
      as well as the package.
  :reconfig:
    markdown: Reconfigure a package. This action requires a response file.
  :remove:
    markdown: Remove a package.
  :unlock:
    markdown: Unlocks the apt package so that it can be upgraded to a newer version.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: channel
  ruby_type: String
  required: false
  default_value: stable
  allowed_values: '"beta", "candidate", "edge", "stable"'
  description_list:
  - markdown: 'The default channel. For example: stable.'
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
examples: 
---
