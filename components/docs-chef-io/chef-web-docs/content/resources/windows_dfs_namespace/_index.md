---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_dfs_namespace resource
resource: windows_dfs_namespace
aliases:
- "/resource_windows_dfs_namespace.html"
menu:
  infra:
    title: windows_dfs_namespace
    identifier: chef_infra/cookbook_reference/resources/windows_dfs_namespace windows_dfs_namespace
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_dfs_namespace** resource to creates a share and DFS
    namespace on a Windows server.
resource_new_in: '15.0'
syntax_full_code_block: |-
  windows_dfs_namespace 'name' do
    change_users        Array # default value: []
    description         String
    full_users          Array # default value: ["BUILTIN\\administrators"]
    namespace_name      String # default value: 'name' unless specified
    read_users          Array # default value: []
    root                String # default value: "C:\\DFSRoots"
    action              Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_dfs_namespace` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`change_users`, `description`, `full_users`, `namespace_name`, `read_users`, and
  `root` are the properties available to this resource."
actions_list:
  :delete:
    markdown: Deletes a DFS Namespace including the directory on disk.
  :create:
    markdown: Creates the dfs namespace on the server. Default.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: change_users
  ruby_type: Array
  required: false
  default_value: "[]"
  description_list:
  - markdown: Determines which users should have change access to the share.
- property: description
  ruby_type: String
  required: true
  description_list:
  - markdown: Description of the share.
- property: full_users
  ruby_type: Array
  required: false
  default_value: '["BUILTIN\\administrators"]'
  description_list:
  - markdown: Determines which users should have full access to the share.
- property: namespace_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the dfs namespace if it differs from the
      resource block's name.
- property: read_users
  ruby_type: Array
  required: false
  default_value: "[]"
  description_list:
  - markdown: Determines which users should have read access to the share.
- property: root
  ruby_type: String
  required: false
  default_value: C:\DFSRoots
  description_list:
  - markdown: The root from which to create the DFS tree. Defaults to C:\DFSRoots.
examples: 
---
