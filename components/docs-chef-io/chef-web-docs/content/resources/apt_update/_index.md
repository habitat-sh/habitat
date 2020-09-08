---
resource_reference: true
nameless_apt_update: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: apt_update resource
resource: apt_update
aliases:
- "/resource_apt_update.html"
menu:
  infra:
    title: apt_update
    identifier: chef_infra/cookbook_reference/resources/apt_update apt_update
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **apt_update** resource to manage APT repository updates on Debian
    and Ubuntu platforms.
resource_new_in: '12.7'
syntax_full_code_block: |-
  apt_update 'name' do
    frequency      Integer # default value: 86400
    action         Symbol # defaults to :periodic if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`apt_update` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`frequency` is the property available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :periodic:
    markdown: Update the Apt repository at the interval specified by the `frequency`
      property.
  :update:
    markdown: Update the Apt repository at the start of a Chef Infra Client run.
properties_list:
- property: frequency
  ruby_type: Integer
  required: false
  default_value: '86400'
  description_list:
  - markdown: Determines how frequently (in seconds) APT repository updates are made.
      Use this property when the `:periodic` action is specified.
examples: |
  **Update the Apt repository at a specified interval**:

  ```ruby
  apt_update 'all platforms' do
  frequency 86400
  action :periodic
  end
  ```

  **Update the Apt repository at the start of a Chef Infra Client run**:

  ```ruby
  apt_update 'update'
  ```
---