---
title: chef_data_bag_item resource
resource: chef_data_bag_item
draft: false
aliases:
- /resource_chef_data_bag_item.html
menu:
  infra:
    title: chef_data_bag_item
    identifier: chef_infra/cookbook_reference/resources/chef_data_bag_item chef_data_bag_item
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- shortcode: data_bag_item.md
- markdown: Use the **chef_data_bag_item** resource to manage data bag items.
resource_new_in: null
handler_types: false
syntax_description: "The syntax for using the **chef_data_bag_item** resource in a\
  \ recipe\nis as follows:\n\n``` ruby\nchef_data_bag_item 'name' do\n  attribute\
  \ 'value' # see properties section below\n  ...\n  action :action # see actions\
  \ section below\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`chef_data_bag_item` tells Chef Infra Client to use the `Chef::Provider::ChefDataBagItem`
  provider during a Chef Infra Client run'
- '`name` is the name of the resource block and also the name of the data bag item'
- '`attribute` is zero (or more) of the properties that are available for this resource'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Use to create a data bag item.
  :delete:
    markdown: Use to delete a data bag item.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: chef_server
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The URL for the Chef Infra Server.
- property: complete
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to specify if this resource defines a data bag item completely.

      When `true`, any property not specified by this resource will be

      reset to default property values.'
- property: encrypt
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Use to specify whether encryption is used for a data bag item.
- property: encryption_version
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The minimum required version of data bag encryption. Possible

      values: `0`, `1`, and `2`. When all of the machines in an

      organization are running chef-client version 11.6 (or higher), it is

      recommended that this value be set to `2`.'
- property: ignore_failure
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Continue running a recipe if a resource fails for any reason.
- property: notifies
  ruby_type: Symbol, Chef::Resource\[String\]
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: resources_common_notification_notifies.md
  - markdown: ''
  - shortcode: resources_common_notification_timers.md
  - markdown: ''
  - shortcode: resources_common_notification_notifies_syntax.md
- property: raw_data
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to create a data bag from a local file from

      `./data_bags/bag_name/file`.'
- property: raw_json
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: "The data bag item as JSON data. For example:\n\n``` javascript\n{\n\
      \  \"id\": \"adam\",\n  \"real_name\": \"Adam Brent Jacob\"\n}\n```"
- property: retries
  ruby_type: Integer
  required: false
  default_value: '0'
  new_in: null
  description_list:
  - markdown: The number of attempts to catch exceptions and retry the resource.
- property: retry_delay
  ruby_type: Integer
  required: false
  default_value: '2'
  new_in: null
  description_list:
  - markdown: The retry delay (in seconds).
- property: subscribes
  ruby_type: Symbol, Chef::Resource\[String\]
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: resources_common_notification_subscribes.md
  - markdown: ''
  - shortcode: resources_common_notification_timers.md
  - markdown: ''
  - shortcode: resources_common_notification_subscribes_syntax.md
properties_shortcode: null
properties_multiple_packages: false
resource_directory_recursive_directories: false
resources_common_atomic_update: false
properties_resources_common_windows_security: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
ps_credential_helper: false
ruby_style_basics_chef_log: false
debug_recipes_chef_shell: false
template_requirements: false
resources_common_properties: false
resources_common_notification: false
resources_common_guards: false
common_resource_functionality_multiple_packages: false
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples_list: null

---
