---
title: chef_organization resource
resource: chef_organization
draft: false
aliases:
- /resource_chef_organization.html
menu:
  infra:
    title: chef_organization
    identifier: chef_infra/cookbook_reference/resources/chef_organization chef_organization
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **chef_organization** resource to interact with organization

    objects that exist on the Chef Infra Server.'
resource_new_in: null
handler_types: false
syntax_description: "The syntax for using the **chef_organization** resource in a\
  \ recipe is\nas follows:\n\n``` ruby\nchef_organization 'name' do\n  attribute 'value'\
  \ # see attributes section below\n  ...\n  action :action # see actions section\
  \ below\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`chef_organization` tells Chef Infra Client to use the `Chef::Provider::ChefOrganization`
  provider during a Chef Infra Client run'
- '`name` is the name of the resource block'
- '`attribute` is zero (or more) of the attributes that are available for this resource'
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
  :delete:
    markdown: ''
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
  - markdown: 'Use to specify if this resource defines an organization completely.

      When `true`, any property not specified by this resource will be

      reset to default property values.'
- property: full_name
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The full name must begin with a non-white space character and must

      be between 1 and 1023 characters. For example:

      `Chef Software, Inc.`.'
- property: ignore_failure
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Continue running a recipe if a resource fails for any reason.
- property: invites
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to specify a list of users to be invited to the organization. An

      invitation is sent to any user in this list who is not already a

      member of the organization.'
- property: members
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to specify a list of users who MUST be members of the

      organization. These users will be added directly to the

      organization. The user who initiates this operation MUST also have

      permission to add users to the specified organization.'
- property: members_specified
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to discover if a user is a member of an organization. Will

      return `true` if the user is a member.'
- property: name
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The name must begin with a lower-case letter or digit, may only

      contain lower-case letters, digits, hyphens, and underscores, and

      must be between 1 and 255 characters. For example: `chef`.'
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
- property: raw_json
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: "The organization as JSON data. For example:\n\n``` none\n{\n  \"name\"\
      : \"chef\",\n  \"full_name\": \"Chef Software, Inc\",\n  \"guid\": \"f980d1asdfda0331235s00ff36862\n\
      \  ...\n}\n```"
- property: remove_members
  ruby_type: null
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to remove the specified users from an organization. Invitations

      that have not been accepted will be cancelled.'
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
