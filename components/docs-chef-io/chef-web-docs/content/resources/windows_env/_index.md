---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_env resource
resource: windows_env
aliases:
- /resource_windows_env.html
- /resource_env.html
menu:
  infra:
    title: windows_env
    identifier: chef_infra/cookbook_reference/resources/windows_env windows_env
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **windows_env** resource to manage environment keys in

    Microsoft Windows. After an environment key is set, Microsoft Windows

    must be restarted before the environment key will be available to the

    Task Scheduler.


    This resource was previously called the **env** resource; its name was

    updated in Chef Client 14.0 to reflect the fact that only Windows is

    supported. Existing cookbooks using `env` will continue to function, but

    should be updated to use the new name.'
- note:
    markdown: 'On UNIX-based systems, the best way to manipulate environment keys
      is

      with the `ENV` variable in Ruby; however, this approach does not have

      the same permanent effect as using the **windows_env** resource.'
syntax_full_code_block: |-
  windows_env 'name' do
    delim         String, false
    key_name      String # default value: 'name' unless specified
    user          String # default value: "<System>"
    value         String
    action        Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_env` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`delim`, `key_name`, `user`, and `value` are the properties available to this resource."
actions_list:
  :create:
    markdown: Default. Create an environment variable. If an environment variable
      already exists (but does not match), update that environment variable to match.
  :delete:
    markdown: Delete an environment variable.
  :modify:
    markdown: Modify an existing environment variable. This prepends the new value
      to the existing value, using the delimiter specified by the `delim` property.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: delim
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The delimiter that is used to separate multiple values for a single
      key.
- property: key_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the name of the key that is to be created,
      deleted, or modified if it differs from the resource block's name.
- property: user
  ruby_type: String
  required: false
  default_value: "<System>"
  description_list:
  - markdown:
- property: value
  ruby_type: String
  required: true
  description_list:
  - markdown: The value of the environmental variable to set.
examples: |
  **Set an environment variable**:

  ```ruby
  windows_env 'ComSpec' do
    value 'C:\Windows\system32\cmd.exe'
  end
  ```
---