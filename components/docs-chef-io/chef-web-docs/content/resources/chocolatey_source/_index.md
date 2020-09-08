---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chocolatey_source resource
resource: chocolatey_source
aliases:
- "/resource_chocolatey_source.html"
menu:
  infra:
    title: chocolatey_source
    identifier: chef_infra/cookbook_reference/resources/chocolatey_source chocolatey_source
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chocolatey_source** resource to add, remove, enable, or disable
    Chocolatey sources.
resource_new_in: '14.3'
syntax_full_code_block: |-
  chocolatey_source 'name' do
    admin_only              true, false # default value: false
    allow_self_service      true, false # default value: false
    bypass_proxy            true, false # default value: false
    priority                Integer # default value: 0
    source                  String
    source_name             String # default value: 'name' unless specified
    action                  Symbol # defaults to :add if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`chocolatey_source` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`admin_only`, `allow_self_service`, `bypass_proxy`, `priority`, `source`, and `source_name`
  are the properties available to this resource."
actions_list:
  :add:
    markdown: Default. Adds a Chocolatey source.
  :disable:
    markdown: "Disables a Chocolatey source.\n **New in Chef Infra Client 15.1.**"
  :enable:
    markdown: "Enables a Chocolatey source.\n **New in Chef Infra Client 15.1.**"
  :remove:
    markdown: Removes a Chocolatey source.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: admin_only
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '15.1'
  description_list:
  - markdown: Whether or not to set the source to be accessible to only admins.
- property: allow_self_service
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '15.1'
  description_list:
  - markdown: Whether or not to set the source to be used for self service.
- property: bypass_proxy
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Whether or not to bypass the system's proxy settings to access the source.
- property: priority
  ruby_type: Integer
  required: false
  default_value: '0'
  description_list:
  - markdown: The priority level of the source.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: The source URL.
- property: source_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the source name if it differs from the resource
      block's name.
examples: |
  **Add a Chocolatey source**

  ```ruby
  chocolatey_source 'MySource' do
    source 'http://example.com/something'
    action :add
  end
  ```

  **Remove a Chocolatey source**

  ```ruby
  chocolatey_source 'MySource' do
    action :remove
  end
  ```
---