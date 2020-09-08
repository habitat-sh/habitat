---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_printer_port resource
resource: windows_printer_port
aliases:
- "/resource_windows_printer_port.html"
menu:
  infra:
    title: windows_printer_port
    identifier: chef_infra/cookbook_reference/resources/windows_printer_port windows_printer_port
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_printer_port** resource to create and delete TCP/IPv4
    printer ports on Windows.
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_printer_port 'name' do
    ipv4_address          String # default value: 'name' unless specified
    port_description      String
    port_name             String
    port_number           Integer # default value: 9100
    port_protocol         Integer # default value: 1
    snmp_enabled          true, false # default value: false
    action                Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_printer_port` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`ipv4_address`, `port_description`, `port_name`, `port_number`, `port_protocol`,
  and `snmp_enabled` are the properties available to this resource."
actions_list:
  :create:
    markdown: Default. Create the printer port, if one doesn't already exist.
  :delete:
    markdown: Delete an existing printer port.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: ipv4_address
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property for the IPv4 address of the printer if it differs
      from the resource block's name.
- property: port_description
  ruby_type: String
  required: false
  description_list:
  - markdown: The description of the port.
- property: port_name
  ruby_type: String
  required: false
  description_list:
  - markdown: The port name.
- property: port_number
  ruby_type: Integer
  required: false
  default_value: '9100'
  description_list:
  - markdown: The port number.
- property: port_protocol
  ruby_type: Integer
  required: false
  default_value: '1'
  allowed_values: 1, 2
  description_list:
  - markdown: 'The printer port protocol: 1 (RAW) or 2 (LPR).'
- property: snmp_enabled
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines if SNMP is enabled on the port.
examples: |
  **Delete a printer port**

  ```ruby
  windows_printer_port '10.4.64.37' do
    action :delete
  end
  ```

  **Delete a port with a custom port_name**

  ```ruby
  windows_printer_port '10.4.64.38' do
    port_name 'My awesome port'
    action :delete
  end
  ```

  **Create a port with more options**

  ```ruby
  windows_printer_port '10.4.64.39' do
    port_name 'My awesome port'
    snmp_enabled true
    port_protocol 2
  end
  ```
---