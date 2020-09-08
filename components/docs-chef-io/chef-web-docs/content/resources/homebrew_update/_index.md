---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: homebrew_update resource
resource: homebrew_update
aliases:
- "/resource_homebrew_update.html"
menu:
  infra:
    title: homebrew_update
    identifier: chef_infra/cookbook_reference/resources/homebrew_update homebrew_update
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **homebrew_update** resource to manage Homebrew repository updates
    on macOS.
resource_new_in: '16.2'
syntax_full_code_block: |-
  homebrew_update 'name' do
    frequency      Integer # default value: 86400
    action         Symbol # defaults to :periodic if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`homebrew_update` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`frequency` is the property available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :periodic:
    markdown:
  :update:
    markdown:
properties_list:
- property: frequency
  ruby_type: Integer
  required: false
  default_value: '86400'
  description_list:
  - markdown: Determines how frequently (in seconds) Homebrew updates are made. Use
      this property when the `:periodic` action is specified.
examples: |
  **Update the homebrew repository data at a specified interval**:
  ```ruby
  homebrew_update 'all platforms' do
    frequency 86400
    action :periodic
  end
  ```
  **Update the Homebrew repository at the start of a Chef Infra Client run**:
  ```ruby
  homebrew_update 'update'
  ```
---