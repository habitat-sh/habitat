---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_auto_run resource
resource: windows_auto_run
aliases:
- "/resource_windows_auto_run.html"
menu:
  infra:
    title: windows_auto_run
    identifier: chef_infra/cookbook_reference/resources/windows_auto_run windows_auto_run
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_auto_run** resource to set applications to run at login.
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_auto_run 'name' do
    args              String
    path              String
    program_name      String # default value: 'name' unless specified
    root              Symbol # default value: :machine
    action            Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_auto_run` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`args`, `path`, `program_name`, and `root` are the properties available to this
  resource."
actions_list:
  :create:
    markdown: Create an item to be run at login.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remover an item that was previously configured to run at login.
properties_list:
- property: args
  ruby_type: String
  required: false
  description_list:
  - markdown: Any arguments to be used with the program.
- property: path
  ruby_type: String
  required: false
  description_list:
  - markdown: The path to the program that will run at login.
- property: program_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the program to run at login if it differs from the resource
      block's name.
- property: root
  ruby_type: Symbol
  required: false
  default_value: ":machine"
  allowed_values: ":machine, :user"
  description_list:
  - markdown: The registry root key to put the entry under.
examples: |
  **Run BGInfo at login**

  ```ruby
  windows_auto_run 'BGINFO' do
    program 'C:/Sysinternals/bginfo.exe'
    args    ''C:/Sysinternals/Config.bgi' /NOLICPROMPT /TIMER:0'
    action  :create
  end
  ```
---