---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: rhsm_subscription resource
resource: rhsm_subscription
aliases:
- "/resource_rhsm_subscription.html"
menu:
  infra:
    title: rhsm_subscription
    identifier: chef_infra/cookbook_reference/resources/rhsm_subscription rhsm_subscription
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **rhsm_subscription** resource to add or remove Red Hat Subscription
    Manager subscriptions from your host. This can be used when a host's activation_key
    does not attach all necessary subscriptions to your host.
resource_new_in: '14.0'
syntax_full_code_block: |-
  rhsm_subscription 'name' do
    pool_id      String # default value: 'name' unless specified
    action       Symbol # defaults to :attach if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`rhsm_subscription` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`pool_id` is the property available to this resource."
actions_list:
  :attach:
    markdown: Default. Attach the node to a subscription pool.
  :remove:
    markdown: Remove the node from a subscription pool.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: pool_id
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property for specifying the Pool ID if it differs from the
      resource block's name.
examples: 
---
