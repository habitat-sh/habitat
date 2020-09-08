---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_certificate resource
resource: windows_certificate
aliases:
- "/resource_windows_certificate.html"
menu:
  infra:
    title: windows_certificate
    identifier: chef_infra/cookbook_reference/resources/windows_certificate windows_certificate
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_certificate** resource to install a certificate into
    the Windows certificate store from a file. The resource grants read-only access
    to the private key for designated accounts. Due to current limitations in WinRM,
    installing certificates remotely may not work if the operation requires a user
    profile. Operations on the local machine store should still work.
resource_new_in: '14.7'
syntax_full_code_block: |-
  windows_certificate 'name' do
    cert_path            String
    pfx_password         String
    private_key_acl      Array
    source               String # default value: 'name' unless specified
    store_name           String # default value: "MY"
    user_store           true, false # default value: false
    action               Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_certificate` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`cert_path`, `pfx_password`, `private_key_acl`, `source`, `store_name`, and `user_store`
  are the properties available to this resource."
actions_list:
  :acl_add:
    markdown: Adds read-only entries to a certificate's private key ACL.
  :create:
    markdown: Creates or updates a certificate.
  :delete:
    markdown: Deletes a certificate.
  :fetch:
    markdown: Fetches a certificate.
  :verify:
    markdown: Verifies a certificate.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: cert_path
  ruby_type: String
  required: false
  description_list: []
- property: pfx_password
  ruby_type: String
  required: false
  description_list:
  - markdown: The password to access the source if it is a pfx file.
- property: private_key_acl
  ruby_type: Array
  required: false
  description_list:
  - markdown: "An array of 'domain\account' entries to be granted read-only access
      to the certificate's private key. Not idempotent."
- property: source
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The source file (for create and acl_add), thumbprint (for delete and
      acl_add) or subject (for delete) if it differs from the resource block's name.
- property: store_name
  ruby_type: String
  required: false
  default_value: MY
  allowed_values: '"AUTHROOT", "CA", "CLIENTAUTHISSUER", "DISALLOWED", "MY", "REMOTE
    DESKTOP", "ROOT", "SMARTCARDROOT", "TRUST", "TRUSTEDDEVICES", "TRUSTEDPEOPLE",
    "TRUSTEDPUBLISHER", "TrustedPublisher", "WEBHOSTING"'
  description_list:
  - markdown: The certificate store to manipulate.
- property: user_store
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Use the user store of the local machine store if set to false.
examples: |
  **Add PFX cert to local machine personal store and grant accounts read-only access to private key**

  ```ruby
  windows_certificate 'c:/test/mycert.pfx' do
    pfx_password 'password'
    private_key_acl ["acme\fred", "pc\jane"]
  end
  ```

  **Add cert to trusted intermediate store**

  ```ruby
  windows_certificate 'c:/test/mycert.cer' do
    store_name 'CA'
  end
  ```

  **Remove all certificates matching the subject**

  ```ruby
  windows_certificate 'me.acme.com' do
    action :delete
  end
  ```
---
