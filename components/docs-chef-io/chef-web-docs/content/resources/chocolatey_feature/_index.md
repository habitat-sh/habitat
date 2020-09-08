---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chocolatey_feature resource
resource: chocolatey_feature
aliases:
- "/resource_chocolatey_feature.html"
menu:
  infra:
    title: chocolatey_feature
    identifier: chef_infra/cookbook_reference/resources/chocolatey_feature chocolatey_feature
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chocolatey_feature** resource to enable and disable Chocolatey
    features.
resource_new_in: '15.1'
syntax_full_code_block: |-
  chocolatey_feature 'name' do
    feature_name      String # default value: 'name' unless specified
    action            Symbol # defaults to :enable if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`chocolatey_feature` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`feature_name` is the property available to this resource."
actions_list:
  :disable:
    markdown: Disable a Chocolatey Feature.
  :enable:
    markdown: Enable a Chocolatey Feature.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: feature_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the Chocolatey feature to enable or disable.
examples: |
  **Enable the checksumFiles Chocolatey feature**

  ```ruby
  chocolatey_feature 'checksumFiles' do
    action :enable
  end
  ```

  **Disable the checksumFiles Chocolatey feature**

  ```ruby
  chocolatey_feature 'checksumFiles' do
    action :disable
  end
  ```
---