---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: rhsm_repo resource
resource: rhsm_repo
aliases:
- "/resource_rhsm_repo.html"
menu:
  infra:
    title: rhsm_repo
    identifier: chef_infra/cookbook_reference/resources/rhsm_repo rhsm_repo
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **rhsm_repo** resource to enable or disable Red Hat Subscription
    Manager repositories that are made available via attached subscriptions.
resource_new_in: '14.0'
syntax_full_code_block: |-
  rhsm_repo 'name' do
    repo_name      String # default value: 'name' unless specified
    action         Symbol # defaults to :enable if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`rhsm_repo` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`repo_name` is the property available to this resource."
actions_list:
  :enable:
    markdown: Default. Enable an RHSM repository.
  :disable:
    markdown: Disable an RHSM repository.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: repo_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property for specifying the repository name if it differs
      from the resource block's name.
examples: "
  Enable an RHSM repository\n\n  ``` ruby\n  rhsm_repo 'rhel-7-server-extras-rpms'\n\
  \  ```\n\n  Disable an RHSM repository\n\n  ``` ruby\n  rhsm_repo 'rhel-7-server-extras-rpms'\
  \ do\n    action :disable\n  end\n  ```\n"

---
