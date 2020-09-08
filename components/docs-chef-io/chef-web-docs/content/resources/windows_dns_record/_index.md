---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_dns_record resource
resource: windows_dns_record
aliases:
- "/resource_windows_dns_record.html"
menu:
  infra:
    title: windows_dns_record
    identifier: chef_infra/cookbook_reference/resources/windows_dns_record windows_dns_record
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: The windows_dns_record resource creates a DNS record for the given domain.
resource_new_in: '15.0'
syntax_full_code_block: |-
  windows_dns_record 'name' do
    dns_server       String # default value: "localhost"
    record_name      String # default value: 'name' unless specified
    record_type      String # default value: "ARecord"
    target           String
    zone             String
    action           Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_dns_record` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`dns_server`, `record_name`, `record_type`, `target`, and `zone` are the properties
  available to this resource."
actions_list:
  :create:
    markdown: Creates and updates the DNS entry.
  :delete:
    markdown: Deletes a DNS entry.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: dns_server
  ruby_type: String
  required: false
  default_value: localhost
  new_in: '16.3'
  description_list:
  - markdown: The name of the DNS server on which to create the record.
- property: record_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the dns record name if it differs from the
      resource block's name.
- property: record_type
  ruby_type: String
  required: false
  default_value: ARecord
  allowed_values: '"ARecord", "CNAME", "PTR"'
  description_list:
  - markdown: The type of record to create, can be either ARecord, CNAME or PTR.
- property: target
  ruby_type: String
  required: true
  description_list:
  - markdown: The target for the record.
- property: zone
  ruby_type: String
  required: true
  description_list:
  - markdown: The zone to create the record in.
examples: 
---
