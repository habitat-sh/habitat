---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: hostname resource
resource: hostname
aliases:
- "/resource_hostname.html"
menu:
  infra:
    title: hostname
    identifier: chef_infra/cookbook_reference/resources/hostname hostname
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **hostname** resource to set the system's hostname, configure
    hostname and hosts config file, and re-run the Ohai hostname plugin so the hostname
    will be available in subsequent cookbooks.
resource_new_in: '14.0'
syntax_full_code_block: |-
  hostname 'name' do
    aliases             Array
    compile_time        true, false # default value: true
    hostname            String # default value: 'name' unless specified
    ipaddress           String # default value: The node's IP address as determined by Ohai.
    windows_reboot      true, false # default value: true
    action              Symbol # defaults to :set if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`hostname` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`aliases`, `compile_time`, `hostname`, `ipaddress`, and `windows_reboot` are the
  properties available to this resource."
actions_list:
  :set:
    markdown: Default action. Set the node's hostname.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: aliases
  ruby_type: Array
  required: false
  description_list:
  - markdown: An array of hostname aliases to use when configuring the hosts file.
- property: compile_time
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether or not the resource should be run at compile time.
- property: hostname
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the hostname if it differs from the resource
      block's name.
- property: ipaddress
  ruby_type: String
  required: false
  default_value: The node's IP address as determined by Ohai.
  description_list:
  - markdown: The IP address to use when configuring the hosts file.
- property: windows_reboot
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether or not Windows should be reboot after changing the
      hostname, as this is required for the change to take effect.
examples: |
  **Set the hostname using the IP address, as detected by Ohai**:

  ```ruby
  hostname 'example'
  ```

  **Manually specify the hostname and IP address**:

  ```ruby
  hostname 'statically_configured_host' do
    hostname 'example'
    ipaddress '198.51.100.2'
  end
  ```
---