---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_client_trusted_certificate resource
resource: chef_client_trusted_certificate
aliases:
- "/resource_chef_client_trusted_certificate.html"
menu:
  infra:
    title: chef_client_trusted_certificate
    identifier: chef_infra/cookbook_reference/resources/chef_client_trusted_certificate
      chef_client_trusted_certificate
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chef_client_trusted_certificate** resource to add certificates
    to Chef Infra Client's trusted certificate directory. This allows the Chef Infra
    Client to communicate with internal encrypted resources without errors.
resource_new_in: '16.5'
syntax_full_code_block: |-
  chef_client_trusted_certificate 'name' do
    cert_name        String # default value: 'name' unless specified
    certificate      String
    action           Symbol # defaults to :add if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`chef_client_trusted_certificate` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`cert_name` and `certificate` are the properties available to this resource."
actions_list:
  :add:
    markdown:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown:
properties_list:
- property: cert_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name to use for the certificate file on disk. If not provided the
      name of the resource block will be used instead.
- property: certificate
  ruby_type: String
  required:
  - add
  description_list:
  - markdown: The text of the certificate file including the BEGIN/END comment lines.
examples: |
  **Trust a self signed certificate**:

  ```ruby
  chef_client_trusted_certificate 'self-signed.badssl.com' do
    certificate <<~CERT
    -----BEGIN CERTIFICATE-----
    MIIDeTCCAmGgAwIBAgIJAPziuikCTox4MA0GCSqGSIb3DQEBCwUAMGIxCzAJBgNV
    BAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRYwFAYDVQQHDA1TYW4gRnJhbmNp
    c2NvMQ8wDQYDVQQKDAZCYWRTU0wxFTATBgNVBAMMDCouYmFkc3NsLmNvbTAeFw0x
    OTEwMDkyMzQxNTJaFw0yMTEwMDgyMzQxNTJaMGIxCzAJBgNVBAYTAlVTMRMwEQYD
    VQQIDApDYWxpZm9ybmlhMRYwFAYDVQQHDA1TYW4gRnJhbmNpc2NvMQ8wDQYDVQQK
    DAZCYWRTU0wxFTATBgNVBAMMDCouYmFkc3NsLmNvbTCCASIwDQYJKoZIhvcNAQEB
    BQADggEPADCCAQoCggEBAMIE7PiM7gTCs9hQ1XBYzJMY61yoaEmwIrX5lZ6xKyx2
    PmzAS2BMTOqytMAPgLaw+XLJhgL5XEFdEyt/ccRLvOmULlA3pmccYYz2QULFRtMW
    hyefdOsKnRFSJiFzbIRMeVXk0WvoBj1IFVKtsyjbqv9u/2CVSndrOfEk0TG23U3A
    xPxTuW1CrbV8/q71FdIzSOciccfCFHpsKOo3St/qbLVytH5aohbcabFXRNsKEqve
    ww9HdFxBIuGa+RuT5q0iBikusbpJHAwnnqP7i/dAcgCskgjZjFeEU4EFy+b+a1SY
    QCeFxxC7c3DvaRhBB0VVfPlkPz0sw6l865MaTIbRyoUCAwEAAaMyMDAwCQYDVR0T
    BAIwADAjBgNVHREEHDAaggwqLmJhZHNzbC5jb22CCmJhZHNzbC5jb20wDQYJKoZI
    hvcNAQELBQADggEBAGlwCdbPxflZfYOaukZGCaxYK6gpincX4Lla4Ui2WdeQxE95
    w7fChXvP3YkE3UYUE7mupZ0eg4ZILr/A0e7JQDsgIu/SRTUE0domCKgPZ8v99k3A
    vka4LpLK51jHJJK7EFgo3ca2nldd97GM0MU41xHFk8qaK1tWJkfrrfcGwDJ4GQPI
    iLlm6i0yHq1Qg1RypAXJy5dTlRXlCLd8ufWhhiwW0W75Va5AEnJuqpQrKwl3KQVe
    wGj67WWRgLfSr+4QG1mNvCZb2CkjZWmxkGPuoP40/y7Yu5OFqxP5tAjj4YixCYTW
    EVA0pmzIzgBg+JIe3PdRy27T0asgQW/F4TY61Yk=
    -----END CERTIFICATE-----
    CERT
  end
  ```
---