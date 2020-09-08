---
title: ohai_hint resource
resource: ohai_hint
draft: false
aliases:
- /resource_ohai_hint.html
menu:
  infra:
    title: ohai_hint
    identifier: chef_infra/cookbook_reference/resources/ohai_hint ohai_hint
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **ohai_hint** resource to aid in configuration detection by

    passing hint data to Ohai.'
resource_new_in: '14.0'
handler_types: false
syntax_description: "The ohai_hint resource has the following syntax:\n\n``` ruby\n\
  ohai_hint 'name' do\n  compile_time      true, false # default value: true\n  content\
  \           Hash\n  hint_name         String # default value: 'name' unless specified\n\
  \  action            Symbol # defaults to :create if not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`ohai_hint` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`compile_time`, `content`, and `hint_name` are the properties available to this
  resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create an Ohai hint file.
  :delete:
    markdown: Delete an Ohai hint file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: compile_time
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: 'Determines whether or not the resource is executed during the

      compile time phase.'
- property: content
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Values to include in the hint file.
- property: hint_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'An optional property to set the hint name if it differs from the

      resource block''s name.'
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
examples: "
  Create a hint file\n\n  ``` ruby\n  ohai_hint 'example' do\n    content\
  \ Hash[:a, 'test_content']\n  end\n  ```\n\n  Create a hint file with a name that\
  \ does not match the resource name\n\n  ``` ruby\n  ohai_hint 'example' do\n   \
  \ hint_name 'custom'\n  end\n  ```\n\n  Create a hint file that is not loaded at\
  \ compile time\n\n  ``` ruby\n  ohai_hint 'example' do\n    compile_time false\n\
  \  end\n  ```\n\n  Delete a hint file\n\n  ``` ruby\n  ohai-hint 'example' do\n\
  \    action :delete\n  end\n  ```\n"

---
