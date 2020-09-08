---
title: openssl_x509_request resource
resource: openssl_x509_request
draft: false
aliases:
- /resource_openssl_x509_request.html
menu:
  infra:
    title: openssl_x509_request
    identifier: chef_infra/cookbook_reference/resources/openssl_x509_request openssl_x509_request
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **openssl_x509_request** resource to generate PEM-formatted

    x509 certificates requests. If no existing key is specified, the

    resource will automatically generate a passwordless key with the

    certificate.'
resource_new_in: '14.4'
handler_types: false
syntax_description: "The openssl_x509_request resource has the following syntax:\n\
  \n``` ruby\nopenssl_x509_request 'name' do\n  city             String\n  common_name\
  \      String\n  country          String\n  email            String\n  group   \
  \         String, Integer\n  key_curve        String # default value: \"prime256v1\"\
  \n  key_file         String\n  key_length       Integer # default value: 2048\n\
  \  key_pass         String\n  key_type         String # default value: \"ec\"\n\
  \  mode             Integer, String\n  org              String\n  org_unit     \
  \    String\n  owner            String, Integer\n  path             String # default\
  \ value: 'name' unless specified\n  state            String\n  action          \
  \ Symbol # defaults to :create if not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`openssl_x509_request` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`city`, `common_name`, `country`, `email`, `group`, `key_curve`, `key_file`, `key_length`,
  `key_pass`, `key_type`, `mode`, `org`, `org_unit`, `owner`, `path`, and `state`
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
    markdown: Default. Create the certificate request file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: city
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `L` certificate field.
- property: common_name
  ruby_type: String
  required: true
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `CN` certificate field.
- property: country
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `C` certificate field.
- property: email
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `email` certificate field.
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The group ownership applied to all files created by the resource.
- property: key_curve
  ruby_type: String
  required: false
  default_value: '"prime256v1"'
  new_in: null
  description_list:
  - markdown: 'The desired curve of the generated key (if key_type is equal to

      ''ec''). Run `openssl ecparam -list_curves` to see available options.'
- property: key_file
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to a certificate key file on the filesystem. If the

      key_file property is specified, the resource will attempt to source

      a key from this location. If no key file is found, the resource will

      generate a new key file at this location. If the key_file property

      is not specified, the resource will generate a key file in the same

      directory as the generated certificate, with the same name as the

      generated certificate.'
- property: key_length
  ruby_type: Integer
  required: false
  default_value: '2048'
  new_in: null
  description_list:
  - markdown: 'The desired bit length of the generated key (if key_type is equal

      to ''rsa''). Available options are `1024`, `2048`, `4096`, and `8192`.'
- property: key_pass
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The passphrase for an existing key's passphrase.
- property: key_type
  ruby_type: String
  required: false
  default_value: '"ec"'
  new_in: null
  description_list:
  - markdown: The desired type of the generated key (rsa or ec).
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The permission mode applied to all files created by the resource.
- property: org
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `O` certificate field.
- property: org_unit
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `OU` certificate field.
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
- property: state
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `ST` certificate field.
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
  Create a certificate request file\n\n  ``` ruby\n  openssl_x509_request\
  \ '/etc/ssl_files/my_ec_request.csr' do\n    common_name 'myecrequest.example.com'\n\
  \    org 'Test Kitchen Example'\n    org_unit 'Kitchens'\n    country 'UK'\n  end\n\
  \  ```\n\n  Create a new certificate request file from an existing ec key\n\n  ```\
  \ ruby\n  openssl_x509_request '/etc/ssl_files/my_ec_request2.csr' do\n     common_name\
  \ 'myecrequest2.example.com'\n     org 'Test Kitchen Example'\n     org_unit 'Kitchens'\n\
  \     country 'UK'\n     key_file '/etc/ssl_files/my_ec_request.key'\n  end\n  ```\n\
  \n  Create both a new rsa key and certificate request file\n\n  ``` ruby\n  openssl_x509_request\
  \ '/etc/ssl_files/my_rsa_request.csr' do\n     common_name 'myrsarequest.example.com'\n\
  \     org 'Test Kitchen Example'\n     org_unit 'Kitchens'\n     country 'UK'\n\
  \     key_type 'rsa'\n  end\n  ```\n"

---
