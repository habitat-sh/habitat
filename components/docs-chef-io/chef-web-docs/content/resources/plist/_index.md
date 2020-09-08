---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: plist resource
resource: plist
aliases:
- "/resource_plist.html"
menu:
  infra:
    title: plist
    identifier: chef_infra/cookbook_reference/resources/plist plist
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **plist** resource to set config values in plist files on macOS
    systems.
resource_new_in: '16.0'
syntax_full_code_block: |-
  plist 'name' do
    encoding      String # default value: "binary"
    entry         String
    group         String # default value: "wheel"
    mode          String, Integer
    owner         String # default value: "root"
    path          String # default value: 'name' unless specified
    value         true, false, String, Integer, Float, Hash
    action        Symbol # defaults to :set if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`plist` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`encoding`, `entry`, `group`, `mode`, `owner`, `path`, and `value` are the properties
  available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :set:
    markdown: 
properties_list:
- property: encoding
  ruby_type: String
  required: false
  default_value: binary
  description_list:
  - markdown: 
- property: entry
  ruby_type: String
  required: false
  description_list:
  - markdown: 
- property: group
  ruby_type: String
  required: false
  default_value: wheel
  description_list:
  - markdown: The group of the plist file.
- property: mode
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 'The file mode of the plist file. Ex: ''644'''
- property: owner
  ruby_type: String
  required: false
  default_value: root
  description_list:
  - markdown: The owner of the plist file.
- property: path
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The path on disk to the plist file.
- property: value
  ruby_type: true, false, String, Integer, Float, Hash
  required: false
  description_list:
  - markdown: 
examples: |
  **Show hidden files in finder**:

  ```ruby
  plist 'show hidden files' do
    path '/Users/vagrant/Library/Preferences/com.apple.finder.plist'
    entry 'AppleShowAllFiles'
    value true
  end
  ```
---