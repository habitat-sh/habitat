---
title: chef_acl resource
resource: chef_acl
draft: false
aliases:
- /resource_chef_acl.html
menu:
  infra:
    title: chef_acl
    identifier: chef_infra/cookbook_reference/resources/chef_acl chef_acl
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **chef_acl** resource to interact with access control lists

    (ACLs) that exist on the Chef Infra Server.'
resource_new_in: null
handler_types: false
syntax_description: "The syntax for using the **chef_acl** resource in a recipe is\
  \ as\nfollows:\n\n``` ruby\nchef_acl 'name' do\n  attribute 'value' # see properties\
  \ section below\n  ...\n  action :action # see actions section below\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`chef_acl` tells Chef Infra Client to use the `Chef::Provider::ChefAcl` provider
  during a Chef Infra Client run'
- '`name` is the name of the resource block; when the `path` property is not specified
  as part of a recipe, `name` is also the name of the Chef Infra Client.'
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
    markdown: Default.
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
  - markdown: 'Use to specify if this resource defines a chef-client completely.

      When `true`, any property not specified by this resource will be

      reset to default property values.'
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
- property: path
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A path to a directory in the chef-repo against which the ACL is

      applied. For example: `nodes`, `nodes/*`, `nodes/my_node`, `*/*`,

      `**`, `roles/base`, `data/secrets`, `cookbooks/apache2`, `/users/*`,

      and so on.'
- property: raw_json
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: "Chef Infra Client as JSON data. For example:\n\n``` javascript\n{\n\
      \  \"clientname\": \"client_name\",\n  \"orgname\": \"org_name\",\n  \"validator\"\
      : false,\n  \"certificate\": \"-----BEGIN CERTIFICATE-----\\n\n            \
      \      ...\n                  1234567890abcdefghijklmnopq\\n\n             \
      \     ...\n                  -----END CERTIFICATE-----\\n\",\n  \"name\": \"\
      node_name\"\n}\n```"
- property: recursive
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to apply changes to child objects. Use `:on_change` to apply

      changes to child objects only if the parent object changes. Set to

      `true` to apply changes even if the parent object does not change.

      Set to `false` to prevent any changes. Default value: `:on_change`.'
- property: remove_rights
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to remove rights. For example:


      ``` ruby

      remove_rights :read, :users => ''jkeiser'', :groups => [ ''admins'', ''users''
      ]

      ```


      or:


      ``` ruby

      remove_rights [ :create, :read ], :users => [ ''jkeiser'', ''adam'' ]

      ```


      or:


      ``` ruby

      remove_rights :all, :users => [ ''jkeiser'', ''adam'' ]

      ```'
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
- property: rights
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to add rights. Syntax:

      `:right, :right => ''user'', :groups => [ ''group'', ''group'']`. For

      example:


      ``` ruby

      rights :read, :users => ''jkeiser'', :groups => [ ''admins'', ''users'' ]

      ```


      or:


      ``` ruby

      rights [ :create, :read ], :users => [ ''jkeiser'', ''adam'' ]

      ```


      or:


      ``` ruby

      rights :all, :users => ''jkeiser''

      ```'
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
