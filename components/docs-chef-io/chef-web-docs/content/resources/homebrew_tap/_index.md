---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: homebrew_tap resource
resource: homebrew_tap
aliases:
- "/resource_homebrew_tap.html"
menu:
  infra:
    title: homebrew_tap
    identifier: chef_infra/cookbook_reference/resources/homebrew_tap homebrew_tap
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **homebrew_tap** resource to add additional formula repositories
    to the Homebrew package manager.
resource_new_in: '14.0'
syntax_full_code_block: |-
  homebrew_tap 'name' do
    full               true, false # default value: false
    homebrew_path      String # default value: "/usr/local/bin/brew"
    owner              String
    tap_name           String # default value: 'name' unless specified
    url                String
    action             Symbol # defaults to :tap if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`homebrew_tap` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`full`, `homebrew_path`, `owner`, `tap_name`, and `url` are the properties available
  to this resource."
actions_list:
  :tap:
    markdown: Default. Add a Homebrew tap.
  :untap:
    markdown: Remove a Homebrew tap.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: full
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Perform a full clone on the tap, as opposed to a shallow clone.
- property: homebrew_path
  ruby_type: String
  required: false
  default_value: "/usr/local/bin/brew"
  description_list:
  - markdown: The path to the Homebrew binary.
- property: owner
  ruby_type: String
  required: false
  default_value: Calculated default username
  description_list:
  - markdown: The owner of the Homebrew installation.
- property: tap_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the tap name if it differs from the resource
      block's name.
- property: url
  ruby_type: String
  required: false
  description_list:
  - markdown: The URL of the tap.
examples: 
---