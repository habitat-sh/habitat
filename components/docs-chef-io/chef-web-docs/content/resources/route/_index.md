---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: route resource
resource: route
aliases:
- "/resource_route.html"
menu:
  infra:
    title: route
    identifier: chef_infra/cookbook_reference/resources/route route
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **route** resource to manage the system routing table in a Linux
    environment.
syntax_description: "A **route** resource block manages the system routing table in\
  \ a Linux\nenvironment:\n\n``` ruby\nroute '10.0.1.10/32' do\n  gateway '10.0.0.20'\n\
  \  device 'eth1'\nend\n```"
syntax_full_code_block: |-
  route 'name' do
    comment         String
    device          String
    gateway         String
    metric          Integer
    netmask         String
    route_type      Symbol, String # default value: :host
    target          String # default value: 'name' unless specified
    action          Symbol # defaults to :add if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`route` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`comment`, `device`, `gateway`, `metric`, `netmask`, `route_type`, and `target`
  are the properties available to this resource."
actions_list:
  :add:
    markdown: Default. Add a route.
  :delete:
    markdown: Delete a route.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: comment
  ruby_type: String
  required: false
  new_in: '14.0'
  description_list:
  - markdown: Add a comment for the route.
- property: device
  ruby_type: String
  required: false
  description_list:
  - markdown: The network interface to which the route applies.
- property: gateway
  ruby_type: String
  required: false
  description_list:
  - markdown: The gateway for the route.
- property: metric
  ruby_type: Integer
  required: false
  description_list:
  - markdown: The route metric value.
- property: netmask
  ruby_type: String
  required: false
  description_list:
  - markdown: 'The decimal representation of the network mask. For example:

      `255.255.255.0`.'
- property: route_type
  ruby_type: Symbol, String
  required: false
  default_value: ":host"
  allowed_values: ":host, :net"
  description_list: []
- property: target
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The IP address of the target route.
examples: 
---
