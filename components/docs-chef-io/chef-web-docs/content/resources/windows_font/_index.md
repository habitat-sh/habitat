---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_font resource
resource: windows_font
aliases:
- "/resource_windows_font.html"
menu:
  infra:
    title: windows_font
    identifier: chef_infra/cookbook_reference/resources/windows_font windows_font
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_font** resource to install font files on Windows. By
    default, the font is sourced from the cookbook using the resource, but a URI source
    can be specified as well.
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_font 'name' do
    font_name      String # default value: 'name' unless specified
    source         String
    action         Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_font` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`font_name` and `source` are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install the font to the system fonts directory.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: font_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the name of the font to install if it differs
      from the resource block's name.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: A local filesystem path or URI that is used to source the font file.
examples: |
  **Install a font from a https source**:

  ```ruby
  windows_font 'Custom.otf' do
    source "https://example.com/Custom.otf"
  end
  ```
---
