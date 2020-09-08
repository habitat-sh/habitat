---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: homebrew_cask resource
resource: homebrew_cask
aliases:
- "/resource_homebrew_cask.html"
menu:
  infra:
    title: homebrew_cask
    identifier: chef_infra/cookbook_reference/resources/homebrew_cask homebrew_cask
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **homebrew_cask** resource to install binaries distributed via
    the Homebrew package manager.
resource_new_in: '14.0'
syntax_full_code_block: |-
  homebrew_cask 'name' do
    cask_name          String # default value: 'name' unless specified
    homebrew_path      String # default value: "/usr/local/bin/brew"
    install_cask       true, false # default value: true
    options            String
    owner              String, Integer
    action             Symbol # defaults to :install if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`homebrew_cask` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`cask_name`, `homebrew_path`, `install_cask`, `options`, and `owner` are the properties
  available to this resource."
actions_list:
  :install:
    markdown: Default. Install an application that is packaged as a Homebrew cask.
  :remove:
    markdown: Remove an application that is packaged as a Homebrew cask.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: cask_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the cask name if it differs from the resource
      block's name.
- property: homebrew_path
  ruby_type: String
  required: false
  default_value: "/usr/local/bin/brew"
  description_list:
  - markdown: The path to the homebrew binary.
- property: install_cask
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Automatically install the Homebrew cask tap, if necessary.
- property: options
  ruby_type: String
  required: false
  description_list:
  - markdown: Options to pass to the brew command during installation.
- property: owner
  ruby_type: String, Integer
  required: false
  default_value: lazy default
  description_list:
  - markdown: The owner of the Homebrew installation.
examples:
---