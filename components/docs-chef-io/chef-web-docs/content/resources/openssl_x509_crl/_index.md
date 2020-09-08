---
title: openssl_x509_crl resource
resource: openssl_x509_crl
draft: false
aliases:
- /resource_openssl_x509_crl.html
menu:
  infra:
    title: openssl_x509_crl
    identifier: chef_infra/cookbook_reference/resources/openssl_x509_crl openssl_x509_crl
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **openssl_x509_crl** resource to generate PEM-formatted x509

    certificate revocation list (CRL) files.'
resource_new_in: '14.4'
handler_types: false
syntax_description: "The openssl_x509_crl resource has the following syntax:\n\n```\
  \ ruby\nopenssl_x509_crl 'name' do\n  ca_cert_file           String\n  ca_key_file\
  \            String\n  ca_key_pass            String\n  expire                 Integer\
  \ # default value: 8\n  group                  String, Integer\n  mode         \
  \          Integer, String\n  owner                  String, Integer\n  path   \
  \                String # default value: 'name' unless specified\n  renewal_threshold\
  \      Integer # default value: 1\n  revocation_reason      Integer # default value:\
  \ 0\n  serial_to_revoke       Integer, String\n  action                 Symbol #\
  \ defaults to :create if not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`openssl_x509_crl` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`ca_cert_file`, `ca_key_file`, `ca_key_pass`, `expire`, `group`, `mode`, `owner`,
  `path`, `renewal_threshold`, `revocation_reason`, and `serial_to_revoke` are the
  properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create the certificate revocation list file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: ca_cert_file
  ruby_type: String
  required: true
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to the CA X509 Certificate on the filesystem. If the

      ca_cert_file property is specified, the ca_key_file property

      must also be specified, the CRL will be signed with them.'
- property: ca_key_file
  ruby_type: String
  required: true
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to the CA private key on the filesystem. If the

      ca_key_file property is specified, the ca_cert_file property

      must also be specified, the CRL will be signed with them.'
- property: ca_key_pass
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The passphrase for CA private key's passphrase.
- property: expire
  ruby_type: Integer
  required: false
  default_value: '8'
  new_in: null
  description_list:
  - markdown: 'Value representing the number of days from now through which the

      issued CRL will remain valid. The CRL will expire after this period.'
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The group permission for the CRL file.
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The permission mode of the CRL file.
- property: owner
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The owner permission for the CRL file.
- property: path
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'An optional property for specifying the path to write the file to if

      it differs from the resource block''s name.'
- property: renewal_threshold
  ruby_type: Integer
  required: false
  default_value: '1'
  new_in: null
  description_list:
  - markdown: 'Number of days before the expiration. It this threshold is reached,

      the CRL will be renewed.'
- property: revocation_reason
  ruby_type: Integer
  required: false
  default_value: '0'
  new_in: null
  description_list:
  - markdown: Reason for the revocation.
- property: serial_to_revoke
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Serial of the X509 Certificate to revoke.
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
  Create a certificate revocation file\n\n  ``` ruby\n  openssl_x509_crl\
  \ '/etc/ssl_test/my_ca.crl' do\n    ca_cert_file '/etc/ssl_test/my_ca.crt'\n   \
  \ ca_key_file '/etc/ssl_test/my_ca.key'\n  end\n  ```\n\n  Create a certificate\
  \ revocation file for a particular serial\n\n  ``` ruby\n  openssl_x509_crl '/etc/ssl_test/my_ca.crl'\
  \ do\n    ca_cert_file '/etc/ssl_test/my_ca.crt'\n    ca_key_file '/etc/ssl_test/my_ca.key'\n\
  \    serial_to_revoke C7BCB6602A2E4251EF4E2827A228CB52BC0CEA2F\n  end\n  ```\n"

---
