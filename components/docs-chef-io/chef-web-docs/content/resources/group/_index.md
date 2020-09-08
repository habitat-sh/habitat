---
title: group resource
resource: group
draft: false
aliases:
- /resource_group.html
menu:
  infra:
    title: group
    identifier: chef_infra/cookbook_reference/resources/group group
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: Use the **group** resource to manage a local group.
resource_new_in: null
handler_types: false
syntax_description: "The group resource has the following syntax:\n\n``` ruby\ngroup\
  \ 'name' do\n  append                true, false # default value: false\n  comment\
  \               String\n  excluded_members      String, Array\n  gid           \
  \        String, Integer\n  group_name            String # default value: 'name'\
  \ unless specified\n  members               String, Array\n  non_unique        \
  \    true, false # default value: false\n  system                true, false # default\
  \ value: false\n  action                Symbol # defaults to :create if not specified\n\
  end\n```"
syntax_code_block: null
syntax_properties_list:
- '`group` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`append`, `comment`, `excluded_members`, `gid`, `group_name`, `members`, `non_unique`,
  and `system` are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create a group. If a group already exists (but does not match),
      update that group to match.
  :manage:
    markdown: Manage an existing group. This action does nothing if the group does
      not exist.
  :modify:
    markdown: Modify an existing group. This action raises an exception if the group
      does not exist.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a group.
properties_list:
- property: append
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: 'How members should be appended and/or removed from a group. When

      `true`, `members` are appended and `excluded_members` are removed.

      When `false`, group members are reset to the value of the `members`

      property.'
- property: comment
  ruby_type: String
  required: false
  default_value: null
  new_in: '14.9'
  description_list:
  - markdown: Specifies a comment to associate with the local group.
- property: excluded_members
  ruby_type: String, Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Remove users from a group. May only be used when `append` is set to

      `true`.'
- property: gid
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The identifier for the group.
- property: group_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'The name of the group. Default value: the `name` of the resource

      block. See "Syntax" section above for more information.'
- property: members
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Which users should be set or appended to a group. When more than one

      group member is identified, the list of members should be an array:

      `members [''user1'', ''user2'']`.'
- property: non_unique
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: 'Allow `gid` duplication. May only be used with the `Groupadd`

      provider.'
- property: system
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: 'Set if a group belongs to a system group. Set to `true` if the group

      belongs to a system group.'
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
resources_common_properties: true
resources_common_notification: true
resources_common_guards: true
common_resource_functionality_multiple_packages: false
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Append users to groups\n\n  ``` ruby\n  group 'www-data' do\n   \
  \ action :modify\n    members 'maintenance'\n    append true\n  end\n  ```\n\n \
  \ Add a user to group on the Windows platform\n\n  ``` ruby\n  group 'Administrators'\
  \ do\n    members ['domain\\foo']\n    append true\n    action :modify\n  end\n\
  \  ```\n"

---
