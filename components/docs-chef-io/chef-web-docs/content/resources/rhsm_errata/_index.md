---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: rhsm_errata resource
resource: rhsm_errata
aliases:
- "/resource_rhsm_errata.html"
menu:
  infra:
    title: rhsm_errata
    identifier: chef_infra/cookbook_reference/resources/rhsm_errata rhsm_errata
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **rhsm_errata** resource to install packages associated with a
    given Red Hat Subscription Manager Errata ID. This is helpful if packages to mitigate
    a single vulnerability must be installed on your hosts.
resource_new_in: '14.0'
syntax_full_code_block: |-
  rhsm_errata 'name' do
    errata_id      String # default value: 'name' unless specified
    action         Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`rhsm_errata` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`errata_id` is the property available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package for a specific errata ID.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: errata_id
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property for specifying the errata ID if it differs from
      the resource block's name.
examples: "
  Install a package from an Errata ID\n\n  ``` ruby\n  rhsm_errata\
  \ 'RHSA:2018-1234'\n  ```\n\n  Specify an Errata ID that differs from the resource\
  \ name\n\n  ``` ruby\n  rhsm_errata 'errata-install'\n    errata_id 'RHSA:2018-1234'\n\
  \  end\n  ```\n"

---
