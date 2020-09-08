---
title: chef_environment resource
resource: chef_environment
draft: false
aliases:
- /resource_chef_environment.html
menu:
  infra:
    title: chef_environment
    identifier: chef_infra/cookbook_reference/resources/chef_environment chef_environment
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- shortcode: environment.md
- markdown: Use the **chef_environment** resource to manage environments.
resource_new_in: null
handler_types: false
syntax_description: "The syntax for using the **chef_environment** resource in a recipe\
  \ is\nas follows:\n\n``` ruby\nchef_environment 'name' do\n  attribute 'value' #\
  \ see properties section below\n  ...\n  action :action # see actions section below\n\
  end\n```"
syntax_code_block: null
syntax_properties_list:
- '`chef_environment` tells Chef Infra Client to use the `Chef::Provider::ChefEnvironment`
  provider during a Chef Infra Client run'
- '`name` is the name of the resource block; when the `name` property is not specified
  as part of a recipe, `name` is also the name of the environment'
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
    markdown: Default. Use to create an environment.
  :delete:
    markdown: Use to delete an environment.
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
  - markdown: 'Use to specify if this resource defines an environment completely.

      When `true`, any property not specified by this resource will be

      reset to default property values.'
- property: cookbook_versions
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The cookbook versions used with the environment. Default value:

      `{}`.'
- property: default_attributes
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: node_attribute_type_default.md
  - markdown: 'Default value: `{}`.'
- property: description
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The description of the environment. This value populates the

      description field for the environment on the Chef Infra Server.'
- property: ignore_failure
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Continue running a recipe if a resource fails for any reason.
- property: name
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The name of the environment.
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
- property: override_attributes
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: node_attribute_type_override.md
  - markdown: 'Default value: `{}`.'
- property: raw_json
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: "The environment as JSON data. For example:\n\n``` javascript\n{\n \
      \ \"name\":\"backend\",\n  \"description\":\"\",\n  \"cookbook_versions\":{},\n\
      \  \"json_class\":\"Chef::Environment\",\n  \"chef_type\":\"environment\",\n\
      \  \"default_attributes\":{},\n  \"override_attributes\":{}\n}\n```"
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
