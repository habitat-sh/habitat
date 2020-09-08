---
title: openssl_dhparam resource
resource: openssl_dhparam
draft: false
aliases:
- /resource_openssl_dhparam.html
menu:
  infra:
    title: openssl_dhparam
    identifier: chef_infra/cookbook_reference/resources/openssl_dhparam openssl_dhparam
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **openssl_dhparam** resource to generate `dhparam.pem` files.

    If a valid `dhparam.pem` file is found at the specified location, no new

    file will be created. If a file is found at the specified location, but

    it is not a valid dhparam file, it will be overwritten.'
resource_new_in: '14.0'
handler_types: false
syntax_description: "The openssl_dhparam resource has the following syntax:\n\n```\
  \ ruby\nopenssl_dhparam 'name' do\n  generator       Integer # default value: 2\n\
  \  group           String, Integer\n  key_length      Integer # default value: 2048\n\
  \  mode            Integer, String # default value: \"0640\"\n  owner          \
  \ String, Integer\n  path            String # default value: 'name' unless specified\n\
  \  action          Symbol # defaults to :create if not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`openssl_dhparam` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`generator`, `group`, `key_length`, `mode`, `owner`, and `path` are the properties
  available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create the `dhparam.pem` file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: generator
  ruby_type: Integer
  required: false
  default_value: '2'
  new_in: null
  description_list:
  - markdown: 'The desired Diffie-Hellmann generator; available options are `2` and

      `5`.'
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The group ownership applied to all files created by the resource.
- property: key_length
  ruby_type: Integer
  required: false
  default_value: '2048'
  new_in: null
  description_list:
  - markdown: 'The desired bit length of the generated key; available options are

      `1024`, `2048`, `4096`, and `8192`.'
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: '0640'
  new_in: null
  description_list:
  - markdown: The permission mode applied to all files created by the resource.
- property: owner
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The owner applied to all files created by the resource.
- property: path
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'An optional property for specifying the path to write the file to if

      it differs from the resource block''s name.'
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
  Create a dhparam file\n\n  ``` ruby\n  openssl_dhparam '/etc/httpd/ssl/dhparam.pem'\n\
  \  ```\n\n  Create a dhparam file with a specific key length\n\n  ``` ruby\n  openssl_dhparam\
  \ '/etc/httpd/ssl/dhparam.pem' do\n    key_length 4096\n  end\n  ```\n\n  **Create\
  \ a dhparam file with specific user/group ownership**\n\n  ``` ruby\n  openssl_dhparam\
  \ '/etc/httpd/ssl/dhparam.pem' do\n    owner 'www-data'\n    group 'www-data'\n\
  \  end\n  ```\n\n  Manually specify the dhparam file path\n\n  ``` ruby\n  openssl_dhparam\
  \ 'httpd_dhparam' do\n    path '/etc/httpd/ssl/dhparam.pem'\n  end\n  ```\n"

---
