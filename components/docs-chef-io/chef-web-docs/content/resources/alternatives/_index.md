---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: alternatives resource
resource: alternatives
aliases:
- "/resource_alternatives.html"
menu:
  infra:
    title: alternatives
    identifier: chef_infra/cookbook_reference/resources/alternatives alternatives
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: The alternatives resource allows for configuration of command alternatives
    in Linux using the alternatives or update-alternatives packages.
resource_new_in: '16.0'
syntax_full_code_block: |-
  alternatives 'name' do
    link           String # default value: "/usr/bin/LINK_NAME"
    link_name      String # default value: 'name' unless specified
    path           String
    priority       String, Integer
    action         Symbol # defaults to :install if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`alternatives` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`link`, `link_name`, `path`, and `priority` are the properties available to this
  resource."
actions_list:
  :auto:
    markdown:
  :install:
    markdown:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :refresh:
    markdown:
  :remove:
    markdown:
  :set:
    markdown:
properties_list:
- property: link
  ruby_type: String
  required: false
  default_value: "/usr/bin/LINK_NAME"
  description_list:
  - markdown: The path to the alternatives link.
- property: link_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the link to create. This will be the command you type on
      the command line such as `ruby` or `gcc`.
- property: path
  ruby_type: String
  required: false
  description_list:
  - markdown: The absolute path to the original application binary such as `/usr/bin/ruby27`.
- property: priority
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The priority of the alternative.
examples: |
  **Install an alternative**:

  ```ruby
  alternatives 'python install 2' do
    link_name 'python'
    path '/usr/bin/python2.7'
    priority 100
    action :install
  end
  ```

  **Set an alternative**:

  ```ruby
  alternatives 'python set version 3' do
    link_name 'python'
    path '/usr/bin/python3'
    action :set
  end
  ```

  **Set the automatic alternative state**:

  ```ruby
  alternatives 'python auto' do
    link_name 'python'
    action :auto
  end
  ```

  **Refresh an alternative**:

  ```ruby
  alternatives 'python refresh' do
    link_name 'python'
    action :refresh
  end
  ```

  **Remove an alternative**:

  ```ruby
  alternatives 'python remove' do
    link_name 'python'
    path '/usr/bin/python3'
    action :remove
  end
  ```
---