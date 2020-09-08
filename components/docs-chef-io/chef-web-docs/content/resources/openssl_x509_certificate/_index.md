---
title: openssl_x509_certificate resource
resource: openssl_x509_certificate
draft: false
aliases:
- /resource_openssl_x509_certificate.html
menu:
  infra:
    title: openssl_x509_certificate
    identifier: chef_infra/cookbook_reference/resources/openssl_x509_certificate openssl_x509_certificate
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **openssl_x509_certificate** resource to generate signed or

    self-signed, PEM-formatted x509 certificates. If no existing key is

    specified, the resource will automatically generate a passwordless key

    with the certificate. If a CA private key and certificate are provided,

    the certificate will be signed with them. Note: This resource was

    renamed from openssl_x509 to openssl_x509_certificate. The legacy

    name will continue to function, but cookbook code should be updated for

    the new resource name.'
resource_new_in: '14.4'
handler_types: false
syntax_description: "The openssl_x509_certificate resource has the following syntax:\n\
  \n``` ruby\nopenssl_x509_certificate 'name' do\n  ca_cert_file             String\n\
  \  ca_key_file              String\n  ca_key_pass              String\n  city  \
  \                   String\n  common_name              String\n  country       \
  \           String\n  csr_file                 String\n  email                 \
  \   String\n  expire                   Integer # default value: 365\n  extensions\
  \               Hash\n  group                    String, Integer\n  key_curve  \
  \              String # default value: \"prime256v1\"\n  key_file              \
  \   String\n  key_length               Integer # default value: 2048\n  key_pass\
  \                 String\n  key_type                 String # default value: \"\
  rsa\"\n  mode                     Integer, String\n  org                      String\n\
  \  org_unit                 String\n  owner                    String, Integer\n\
  \  path                     String # default value: 'name' unless specified\n  renew_before_expiry\
  \      Integer\n  state                    String\n  subject_alt_name         Array\n\
  \  action                   Symbol # defaults to :create if not specified\nend\n\
  ```"
syntax_code_block: null
syntax_properties_list:
- '`openssl_x509_certificate` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`ca_cert_file`, `ca_key_file`, `ca_key_pass`, `city`, `common_name`, `country`,
  `csr_file`, `email`, `expire`, `extensions`, `group`, `key_curve`, `key_file`, `key_length`,
  `key_pass`, `key_type`, `mode`, `org`, `org_unit`, `owner`, `path`, `renew_before_expiry`,
  `state`, and `subject_alt_name` are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create the certificate file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: ca_cert_file
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to the CA X509 Certificate on the filesystem. If the

      ca_cert_file property is specified, the `ca_key_file` property

      must also be specified, the certificate will be signed with them.'
- property: ca_key_file
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to the CA private key on the filesystem. If the

      ca_key_file property is specified, the `ca_cert_file` property

      must also be specified, the certificate will be signed with them.'
- property: ca_key_pass
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The passphrase for CA private key's passphrase.
- property: city
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `L` certificate field.
- property: common_name
  ruby_type: String
  required: false
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
- property: csr_file
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to a X509 Certificate Request (CSR) on the filesystem. If

      the csr_file property is specified, the resource will attempt to

      source a CSR from this location. If no CSR file is found, the

      resource will generate a Self-Signed Certificate and the certificate

      fields must be specified (common_name at last).'
- property: email
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `email` certificate field.
- property: expire
  ruby_type: Integer
  required: false
  default_value: '365'
  new_in: null
  description_list:
  - markdown: 'Value representing the number of days from now through which the

      issued certificate cert will remain valid. The certificate will

      expire after this period.'
- property: extensions
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Hash of X509 Extensions entries, in format

      `{ ''keyUsage'' => { ''values'' => %w( keyEncipherment digitalSignature), ''critical''
      => true } }`.'
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
  default_value: '"rsa"'
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
- property: renew_before_expiry
  ruby_type: Integer
  required: false
  default_value: null
  new_in: '15.7'
  description_list:
  - markdown: 'The number of days before the expiry. The certificate will be

      automaticaly renewed when the value is reached.'
- property: state
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Value for the `ST` certificate field.
- property: subject_alt_name
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Array of Subject Alternative Name entries, in format

      <DNS:example.com> or IP:1.2.3.4.'
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
  Create a simple self-signed certificate file\n\n  ``` ruby\n  openssl_x509_certificate\
  \ '/etc/httpd/ssl/mycert.pem' do\n    common_name 'www.f00bar.com'\n    org 'Foo\
  \ Bar'\n    org_unit 'Lab'\n    country 'US'\n  end\n  ```\n\n  Create a certificate\
  \ using additional options\n\n  ``` ruby\n  openssl_x509_certificate '/etc/ssl_test/my_signed_cert.crt'\
  \ do\n    common_name 'www.f00bar.com'\n    ca_key_file '/etc/ssl_test/my_ca.key'\n\
  \    ca_cert_file '/etc/ssl_test/my_ca.crt'\n    expire 365\n    extensions(\n \
  \     'keyUsage' => {\n        'values' => %w(\n          keyEncipherment\n    \
  \      digitalSignature),\n        'critical' => true,\n      },\n      'extendedKeyUsage'\
  \ => {\n        'values' => %w(serverAuth),\n        'critical' => false,\n    \
  \  }\n    )\n    subject_alt_name ['IP:127.0.0.1', 'DNS:localhost.localdomain']\n\
  \  end\n  ```\n"

---
