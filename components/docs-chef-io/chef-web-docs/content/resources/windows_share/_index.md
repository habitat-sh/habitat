---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_share resource
resource: windows_share
aliases:
- "/resource_windows_share.html"
menu:
  infra:
    title: windows_share
    identifier: chef_infra/cookbook_reference/resources/windows_share windows_share
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_share** resource to create, modify and remove Windows
    shares.
resource_new_in: '14.7'
syntax_full_code_block: |-
  windows_share 'name' do
    ca_timeout                  Integer # default value: 0
    change_users                Array
    concurrent_user_limit       Integer # default value: 0
    continuously_available      true, false # default value: false
    description                 String
    encrypt_data                true, false # default value: false
    full_users                  Array
    path                        String
    read_users                  Array
    scope_name                  String # default value: "*"
    share_name                  String # default value: 'name' unless specified
    temporary                   true, false # default value: false
    action                      Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_share` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`ca_timeout`, `change_users`, `concurrent_user_limit`, `continuously_available`,
  `description`, `encrypt_data`, `full_users`, `path`, `read_users`, `scope_name`,
  `share_name`, and `temporary` are the properties available to this resource."
actions_list:
  :create:
    markdown: Create and modify Windows shares.
  :delete:
    markdown: Delete an existing Windows share.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: ca_timeout
  ruby_type: Integer
  required: false
  default_value: '0'
  description_list:
  - markdown: The continuous availability time-out for the share.
- property: change_users
  ruby_type: Array
  required: false
  default_value: null
  description_list:
  - markdown: The users that should have 'modify' permission on the share in domain\username
      format.
- property: concurrent_user_limit
  ruby_type: Integer
  required: false
  default_value: '0'
  description_list:
  - markdown: The maximum number of concurrently connected users the share can accommodate.
- property: continuously_available
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Indicates that the share is continuously available.
- property: description
  ruby_type: String
  required: false
  description_list:
  - markdown: The description to be applied to the share.
- property: encrypt_data
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Indicates that the share is encrypted.
- property: full_users
  ruby_type: Array
  required: false
  default_value: null
  description_list:
  - markdown: The users that should have 'Full control' permissions on the share in
      domain\username format.
- property: path
  ruby_type: String
  required: false
  description_list:
  - markdown: The path of the folder to share. Required when creating. If the share
      already exists on a different path then it is deleted and re-created.
- property: read_users
  ruby_type: Array
  required: false
  default_value: null
  description_list:
  - markdown: The users that should have 'read' permission on the share in domain\username
      format.
- property: scope_name
  ruby_type: String
  required: false
  default_value: "*"
  description_list:
  - markdown: The scope name of the share.
- property: share_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the share name if it differs from the resource
      block's name.
- property: temporary
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: The lifetime of the new SMB share. A temporary share does not persist
      beyond the next restart of the computer.
examples: |
  **Create a share**:

  ```ruby
  windows_share 'foo' do
    action :create
    path 'C:\foo'
    full_users ['DOMAIN_A\some_user', 'DOMAIN_B\some_other_user']
    read_users ['DOMAIN_C\Domain users']
  end
  ```

  **Delete a share**:

  ```ruby
  windows_share 'foo' do
    action :delete
  end
  ```
---