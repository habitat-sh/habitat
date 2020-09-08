---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
notes_resource_based_on_package: true
title: smartos_package resource
resource: smartos_package
aliases:
- "/resource_smartos_package.html"
menu:
  infra:
    title: smartos_package
    identifier: chef_infra/cookbook_reference/resources/smartos_package smartos_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **smartos_package** resource to manage packages for the SmartOS
    platform.
syntax_full_code_block: |-
  smartos_package 'name' do
    options           String, Array
    package_name      String
    source            String
    timeout           String, Integer
    version           String
    action            Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`smartos_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`options`, `package_name`, `source`, `timeout`, and `version` are the properties
  available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a package.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.
properties_list:
- property: options
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: One (or more) additional command options that are passed to the command.
- property: package_name
  ruby_type: String
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
  ruby_type: String
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded.
examples: 
---
