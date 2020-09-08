---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_firewall_rule resource
resource: windows_firewall_rule
aliases:
- "/resource_windows_firewall_rule.html"
menu:
  infra:
    title: windows_firewall_rule
    identifier: chef_infra/cookbook_reference/resources/windows_firewall_rule windows_firewall_rule
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_firewall_rule** resource to create, change or remove
    Windows firewall rules.
resource_new_in: '14.7'
syntax_full_code_block: |-
  windows_firewall_rule 'name' do
    description          String
    direction            Symbol, String # default value: :inbound
    displayname          String # default value: The rule_name property value.
    enabled              true, false # default value: true
    firewall_action      Symbol, String # default value: :allow
    group                String
    icmp_type            String, Integer # default value: "Any"
    interface_type       Symbol, String # default value: :any
    local_address        String
    local_port           String, Integer, Array
    profile              Symbol, String, Array # default value: :any
    program              String
    protocol             String # default value: "TCP"
    remote_address       String
    remote_port          String, Integer, Array
    rule_name            String # default value: 'name' unless specified
    service              String
    action               Symbol # defaults to :create if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`windows_firewall_rule` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`description`, `direction`, `displayname`, `enabled`, `firewall_action`, `group`,
  `icmp_type`, `interface_type`, `local_address`, `local_port`, `profile`, `program`,
  `protocol`, `remote_address`, `remote_port`, `rule_name`, and `service` are the
  properties available to this resource."
actions_list:
  :create:
    markdown: Create a Windows firewall entry.
  :delete:
    markdown: Delete an existing Windows firewall entry.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: description
  ruby_type: String
  required: false
  description_list:
  - markdown: The description to assign to the firewall rule.
- property: direction
  ruby_type: Symbol, String
  required: false
  default_value: ":inbound"
  allowed_values: ":inbound, :outbound"
  description_list:
  - markdown: The direction of the firewall rule. Direction means either inbound or
      outbound traffic.
- property: displayname
  ruby_type: String
  required: false
  default_value: The rule_name property value.
  new_in: '16.0'
  description_list:
  - markdown: The displayname to assign to the firewall rule.
- property: enabled
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Whether or not to enable the firewall rule.
- property: firewall_action
  ruby_type: Symbol, String
  required: false
  default_value: ":allow"
  allowed_values: ":allow, :block, :notconfigured"
  description_list:
  - markdown: The action of the firewall rule.
- property: group
  ruby_type: String
  required: false
  new_in: '16.0'
  description_list:
  - markdown: Specifies that only matching firewall rules of the indicated group association
      are copied.
- property: icmp_type
  ruby_type: String, Integer
  required: false
  default_value: Any
  new_in: '16.0'
  description_list:
  - markdown: Specifies the ICMP Type parameter for using a protocol starting with
      ICMP
- property: interface_type
  ruby_type: Symbol, String
  required: false
  default_value: ":any"
  allowed_values: ":any, :remoteaccess, :wired, :wireless"
  description_list:
  - markdown: The interface type the firewall rule applies to.
- property: local_address
  ruby_type: String
  required: false
  description_list:
  - markdown: The local address the firewall rule applies to.
- property: local_port
  ruby_type: String, Integer, Array
  required: false
  description_list:
  - markdown: The local port the firewall rule applies to.
- property: profile
  ruby_type: Symbol, String, Array
  required: false
  default_value: ":any"
  description_list:
  - markdown: The profile the firewall rule applies to.
- property: program
  ruby_type: String
  required: false
  description_list:
  - markdown: The program the firewall rule applies to.
- property: protocol
  ruby_type: String
  required: false
  default_value: TCP
  description_list:
  - markdown: The protocol the firewall rule applies to.
- property: remote_address
  ruby_type: String
  required: false
  description_list:
  - markdown: The remote address the firewall rule applies to.
- property: remote_port
  ruby_type: String, Integer, Array
  required: false
  description_list:
  - markdown: The remote port the firewall rule applies to.
- property: rule_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the name of the firewall rule to assign
      if it differs from the resource block's name.
- property: service
  ruby_type: String
  required: false
  description_list:
  - markdown: The service the firewall rule applies to.
examples: |
  **Allowing port 80 access**:

  ```ruby
  windows_firewall_rule 'IIS' do
    local_port '80'
    protocol 'TCP'
    firewall_action :allow
  end
  ```

  **Allow protocol ICMPv6 with ICMP Type**:

  ```ruby
  windows_firewall_rule 'CoreNet-Rule' do
    rule_name 'CoreNet-ICMP6-LR2-In'
    display_name 'Core Networking - Multicast Listener Report v2 (ICMPv6-In)'
    local_port 'RPC'
    protocol 'ICMPv6'
    icmp_type '8'
  end
  ```

  **Blocking WinRM over HTTP on a particular IP**:

  ```ruby
  windows_firewall_rule 'Disable WinRM over HTTP' do
    local_port '5985'
    protocol 'TCP'
    firewall_action :block
    local_address '192.168.1.1'
  end
  ```

  **Deleting an existing rule**

  ```ruby
  windows_firewall_rule 'Remove the SSH rule' do
    rule_name 'ssh'
    action :delete
  end
  ```
---