---
title: powershell_package_source resource
resource: powershell_package_source
draft: false
aliases:
- /resource_powershell_package_source.html
menu:
  infra:
    title: powershell_package_source
    identifier: chef_infra/cookbook_reference/resources/powershell_package_source
      powershell_package_source
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **powershell_package_source** resource to register a

    PowerShell package repository.'
resource_new_in: '14.3'
handler_types: false
syntax_description: "The powershell_package_source resource has the following syntax:\n\
  \n``` ruby\npowershell_package_source 'name' do\n  provider_name               \
  \ String # default value: \"NuGet\"\n  publish_location             String\n  script_publish_location\
  \      String\n  script_source_location       String\n  source_name            \
  \      String # default value: 'name' unless specified\n  trusted              \
  \        true, false # default value: false\n  url                          String\n\
  \  action                       Symbol # defaults to :register if not specified\n\
  end\n```"
syntax_code_block: null
syntax_properties_list:
- '`powershell_package_source` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`provider_name`, `publish_location`, `script_publish_location`, `script_source_location`,
  `source_name`, `trusted`, and `url` are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :register:
    markdown: Default. Registers and updates the PowerShell package source.
  :unregister:
    markdown: Unregisters the PowerShell package source.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: provider_name
  ruby_type: String
  required: false
  default_value: '"NuGet"'
  new_in: null
  description_list:
  - markdown: 'The package management provider for the source. It supports the

      following providers: ''Programs'', ''msi'', ''NuGet'', ''msu'',

      ''PowerShellGet'', ''psl'' and ''chocolatey''.'
- property: publish_location
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The url where modules will be published to for this source. Only

      valid if the provider is ''PowerShellGet''.'
- property: script_publish_location
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The location where scripts will be published to for this source.

      Only valid if the provider is ''PowerShellGet''.'
- property: script_source_location
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The url where scripts are located for this source. Only valid if the

      provider is ''PowerShellGet''.'
- property: source_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: The name of the package source.
- property: trusted
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Whether or not to trust packages from this source.
- property: url
  ruby_type: String
  required: true
  default_value: null
  new_in: null
  description_list:
  - markdown: The url to the package source.
properties_shortcode: null
properties_multiple_packages: false
resource_directory_recursive_directories: false
resources_common_atomic_update: false
properties_resources_common_windows_security: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
ps_credential_helper: false
ruby_style_basics_chef_log: false
debug_recipes_chef_shell: false
template_requirements: false
resources_common_properties: true
resources_common_notification: true
resources_common_guards: true
common_resource_functionality_multiple_packages: false
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples_list: null

---
