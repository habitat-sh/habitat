---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: rhsm_errata_level resource
resource: rhsm_errata_level
aliases:
- "/resource_rhsm_errata_level.html"
menu:
  infra:
    title: rhsm_errata_level
    identifier: chef_infra/cookbook_reference/resources/rhsm_errata_level rhsm_errata_level
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **rhsm_errata_level** resource to install all packages of a specified
    errata level from the Red Hat Subscription Manager. For example, you can ensure
    that all packages associated with errata marked at a 'Critical' security level
    are installed.
resource_new_in: '14.0'
syntax_full_code_block: |-
  rhsm_errata_level 'name' do
    errata_level      String # default value: 'name' unless specified
    action            Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`rhsm_errata_level` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`errata_level` is the property available to this resource."
actions_list:
  :install:
    markdown: Default. Install all packages of the specified errata level.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: errata_level
  ruby_type: String
  required: false
  default_value: The resource block's name
  allowed_values: '"critical", "important", "low", "moderate"'
  description_list:
  - markdown: An optional property for specifying the errata level of packages to
      install if it differs from the resource block's name.
examples: "
  Specify an errata level that differs from the resource name\n\n \
  \ ``` ruby\n  rhsm_errata_level 'example_install_moderate' do\n    errata_level\
  \ 'moderate'\n  end\n  ```\n"

---
