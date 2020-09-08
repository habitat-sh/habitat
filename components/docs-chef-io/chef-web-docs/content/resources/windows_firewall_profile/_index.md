---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_firewall_profile resource
resource: windows_firewall_profile
aliases:
- "/resource_windows_firewall_profile.html"
menu:
  infra:
    title: windows_firewall_profile
    identifier: chef_infra/cookbook_reference/resources/windows_firewall_profile windows_firewall_profile
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_firewall_profile** resource to enable, disable, and
    configure the Windows firewall.
resource_new_in: '16.3'
syntax_full_code_block: |-
  windows_firewall_profile 'name' do
    allow_inbound_rules             true, false, String
    allow_local_firewall_rules      true, false, String
    allow_local_ipsec_rules         true, false, String
    allow_unicast_response          true, false, String
    allow_user_apps                 true, false, String
    allow_user_ports                true, false, String
    default_inbound_action          String
    default_outbound_action         String
    display_notification            true, false, String
    profile                         String # default value: 'name' unless specified
    action                          Symbol # defaults to :enable if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`windows_firewall_profile` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`allow_inbound_rules`, `allow_local_firewall_rules`, `allow_local_ipsec_rules`,
  `allow_unicast_response`, `allow_user_apps`, `allow_user_ports`, `default_inbound_action`,
  `default_outbound_action`, `display_notification`, and `profile` are the properties
  available to this resource."
actions_list:
  :disable:
    markdown: 'Disable a Windows Firewall profile'
  :enable:
    markdown: 'Enable and optionally configure a Windows Firewall profile'
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: allow_inbound_rules
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Allow users to set inbound firewall rules
- property: allow_local_firewall_rules
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Merges inbound firewall rules into the policy
- property: allow_local_ipsec_rules
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Allow users to manage local connection security rules
- property: allow_unicast_response
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Allow unicast responses to multicast and broadcast messages
- property: allow_user_apps
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Allow user applications to manage firewall
- property: allow_user_ports
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Allow users to manage firewall port rules
- property: default_inbound_action
  ruby_type: String
  required: false
  allowed_values: '"Allow", "Block", "NotConfigured"'
  description_list:
  - markdown: Set the default policy for inbound network traffic
- property: default_outbound_action
  ruby_type: String
  required: false
  allowed_values: '"Allow", "Block", "NotConfigured"'
  description_list:
  - markdown: Set the default policy for outbound network traffic
- property: display_notification
  ruby_type: true, false, String
  required: false
  allowed_values: true, false, "NotConfigured"
  description_list:
  - markdown: Display a notification when firewall blocks certain activity
- property: profile
  ruby_type: String
  required: false
  default_value: The resource block's name
  allowed_values: '"Domain", "Private", "Public"'
  description_list:
  - markdown: Set the Windows Profile being configured
examples: |
  **Enable and Configure the Private Profile of the Windows Profile**:

  ```ruby
  windows_firewall_profile 'Private' do
    default_inbound_action 'Block'
    default_outbound_action 'Allow'
    allow_inbound_rules true
    display_notification false
    action :enable
  end
  ```

  **Enable and Configure the Public Profile of the Windows Firewall**:

  ```ruby
  windows_firewall_profile 'Public' do
    default_inbound_action 'Block'
    default_outbound_action 'Allow'
    allow_inbound_rules false
    display_notification false
    action :enable
  end
  ```

  **Disable the Domain Profile of the Windows Firewall**:

  ```ruby
  windows_firewall_profile 'Disable the Domain Profile of the Windows Firewall' do
    profile 'Domain'
    action :disable
  end
  ```
---