---
title: openssl_rsa_private_key resource
resource: openssl_rsa_private_key
draft: false
aliases:
- /resource_openssl_rsa_private_key.html
menu:
  infra:
    title: openssl_rsa_private_key
    identifier: chef_infra/cookbook_reference/resources/openssl_rsa_private_key openssl_rsa_private_key
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **openssl_rsa_private_key** resource to generate RSA private

    key files. If a valid RSA key file can be opened at the specified

    location, no new file will be created. If the RSA key file cannot be

    opened or does not exist, it will be overwritten.'
- note:
    markdown: 'If the password to your RSA key file does not match the password in
      the

      recipe, it cannot be opened, and will be overwritten.'
resource_new_in: '14.0'
handler_types: false
syntax_description: "The openssl_rsa_private_key resource has the following syntax:\n\
  \n``` ruby\nopenssl_rsa_private_key 'name' do\n  force           true, false # default\
  \ value: false\n  group           String, Integer\n  key_cipher      String # default\
  \ value: \"des3\"\n  key_length      Integer # default value: 2048\n  key_pass \
  \       String\n  mode            Integer, String # default value: \"0600\"\n  owner\
  \           String, Integer\n  path            String # default value: 'name' unless\
  \ specified\n  action          Symbol # defaults to :create if not specified\nend\n\
  ```"
syntax_code_block: null
syntax_properties_list:
- '`openssl_rsa_private_key` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`force`, `group`, `key_cipher`, `key_length`, `key_pass`, `mode`, `owner`, and
  `path` are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create the RSA private key file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: force
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: 'Force creation of the key even if the same key already exists on the

      node.'
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The group ownership applied to all files created by the resource.
- property: key_cipher
  ruby_type: String
  required: false
  default_value: '"des3"'
  new_in: null
  description_list:
  - markdown: 'The designed cipher to use when generating your key. Run

      `openssl list-cipher-algorithms` to see available options.'
- property: key_length
  ruby_type: Integer
  required: false
  default_value: '2048'
  new_in: null
  description_list:
  - markdown: 'The desired bit length of the generated key; available options are

      `1024`, `2048`, `4096`, and `8192`.'
- property: key_pass
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The desired passphrase for the key.
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: '"0600"'
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
  Create a new 2048bit key with the default des3 cipher\n\n  ``` ruby\n\
  \  openssl_rsa_private_key '/etc/ssl_files/rsakey_des3.pem' do\n     key_length\
  \ 2048\n     action :create\n  end\n  ```\n\n  Create a new 1024 bit key with the\
  \ aes-128-cbc cipher\n\n  ``` ruby\n  openssl_rsa_key '/etc/ssl_files/rsakey_aes128cbc.pem'\
  \ do\n     key_length 1024\n     key_cipher 'aes-128-cbc'\n     action :create\n\
  \  end\n  ```\n"

---
