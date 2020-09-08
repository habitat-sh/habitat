---
title: openssl_ec_private_key resource
resource: openssl_ec_private_key
draft: false
aliases:
- /resource_openssl_ec_private_key.html
menu:
  infra:
    title: openssl_ec_private_key
    identifier: chef_infra/cookbook_reference/resources/openssl_ec_private_key openssl_ec_private_key
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **openssl_ec_private_key** resource to generate an elliptic

    curve (EC) private key file. If a valid EC key file can be opened at the

    specified location, no new file will be created. If the EC key file

    cannot be opened -- either because it does not exist or because the

    password to the EC key file does not match the password in the recipe --

    then it will be overwritten.'
resource_new_in: '14.4'
handler_types: false
syntax_description: "The openssl_ec_private_key resource has the following syntax:\n\
  \n``` ruby\nopenssl_ec_private_key 'name' do\n  force           true, false # default\
  \ value: false\n  group           String, Integer\n  key_cipher      String # default\
  \ value: \"des3\"\n  key_curve       String # default value: \"prime256v1\"\n  key_pass\
  \        String\n  mode            Integer, String # default value: \"0600\"\n \
  \ owner           String, Integer\n  path            String # default value: 'name'\
  \ unless specified\n  action          Symbol # defaults to :create if not specified\n\
  end\n```"
syntax_code_block: null
syntax_properties_list:
- '`openssl_ec_private_key` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`force`, `group`, `key_cipher`, `key_curve`, `key_pass`, `mode`, `owner`, and `path`
  are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create the EC private key file.
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
- property: key_curve
  ruby_type: String
  required: false
  default_value: '"prime256v1"'
  new_in: null
  description_list:
  - markdown: 'The desired curve of the generated key (if key_type is equal to

      ''ec''). Run `openssl ecparam -list_curves` to see available options.'
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
  Create a new ec private key with a prime256v1 key curve and the\n\
  \  default des3 cipher\n\n  ``` ruby\n  openssl_ec_private_key '/etc/ssl_files/eckey_prime256v1_des3.pem'\
  \ do\n    key_curve 'prime256v1'\n    key_pass 'something'\n    action :create\n\
  \  end\n  ```\n\n  Create a new ec private key with a prime256v1 key curve and a\n\
  \  aes-128-cbc cipher\n\n  ``` ruby\n  openssl_ec_private_key '/etc/ssl_files/eckey_prime256v1_des3.pem'\
  \ do\n    key_curve 'prime256v1'\n    key_cipher 'aes-128-cbc'\n    key_pass 'something'\n\
  \    action :create\n  end\n  ```\n"

---
