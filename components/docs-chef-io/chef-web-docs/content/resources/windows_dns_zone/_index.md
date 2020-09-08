---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_dns_zone resource
resource: windows_dns_zone
aliases:
- "/resource_windows_dns_zone.html"
menu:
  infra:
    title: windows_dns_zone
    identifier: chef_infra/cookbook_reference/resources/windows_dns_zone windows_dns_zone
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: The windows_dns_zone resource creates an Active Directory Integrated DNS
    Zone on the local server.
resource_new_in: '15.0'
syntax_full_code_block: |-
  windows_dns_zone 'name' do
    replication_scope      String # default value: "Domain"
    server_type            String # default value: "Domain"
    zone_name              String # default value: 'name' unless specified
    action                 Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_dns_zone` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`replication_scope`, `server_type`, and `zone_name` are the properties available
  to this resource."
actions_list:
  :create:
    markdown: Creates and updates a DNS Zone. Default.
  :delete:
    markdown: Deletes a DNS Zone.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: replication_scope
  ruby_type: String
  required: false
  default_value: Domain
  description_list:
  - markdown: The replication scope for the zone, required if server_type set to 'Domain'.
- property: server_type
  ruby_type: String
  required: false
  default_value: Domain
  allowed_values: '"Domain", "Standalone"'
  description_list:
  - markdown: The type of DNS server, Domain or Standalone.
- property: zone_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the dns zone name if it differs from the
      resource block's name.
examples: 
---
