---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: swap_file resource
resource: swap_file
aliases:
- "/resource_swap_file.html"
menu:
  infra:
    title: swap_file
    identifier: chef_infra/cookbook_reference/resources/swap_file swap_file
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **swap_file** resource to create or delete swap files on Linux
    systems, and optionally to manage the swappiness configuration for a host.
resource_new_in: '14.0'
syntax_full_code_block: |-
  swap_file 'name' do
    path            String # default value: 'name' unless specified
    persist         true, false # default value: false
    size            Integer
    swappiness      Integer
    timeout         Integer # default value: 600
    action          Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`swap_file` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`path`, `persist`, `size`, `swappiness`, and `timeout` are the properties available
  to this resource."
actions_list:
  :create:
    markdown: Default. Create a swap file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a swap file and disable swap.
properties_list:
- property: path
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The path where the swap file will be created on the system if it differs
      from the resource block's name.
- property: persist
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Persist the swapon.
- property: size
  ruby_type: Integer
  required: false
  description_list:
  - markdown: The size (in MBs) of the swap file.
- property: swappiness
  ruby_type: Integer
  required: false
  description_list:
  - markdown: The swappiness value to set on the system.
- property: timeout
  ruby_type: Integer
  required: false
  default_value: '600'
  description_list:
  - markdown: Timeout for `dd` / `fallocate` commands.
examples: |
  **Create a swap file**

  ```ruby
  swap_file '/dev/sda1' do
    size 1024
  end
  ```

  **Remove a swap file**

  ```ruby
  swap_file '/dev/sda1' do
    action :remove
  end
  ```
---