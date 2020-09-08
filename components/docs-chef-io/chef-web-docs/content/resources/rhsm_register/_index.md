---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: rhsm_register resource
resource: rhsm_register
aliases:
- "/resource_rhsm_register.html"
menu:
  infra:
    title: rhsm_register
    identifier: chef_infra/cookbook_reference/resources/rhsm_register rhsm_register
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **rhsm_register** resource to register a node with the Red Hat
    Subscription Manager or a local Red Hat Satellite server.
resource_new_in: '14.0'
syntax_full_code_block: |-
  rhsm_register 'name' do
    activation_key             String, Array
    auto_attach                true, false # default value: false
    environment                String
    force                      true, false # default value: false
    https_for_ca_consumer      true, false # default value: false
    install_katello_agent      true, false # default value: true
    organization               String
    password                   String
    satellite_host             String
    username                   String
    action                     Symbol # defaults to :register if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`rhsm_register` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`activation_key`, `auto_attach`, `environment`, `force`, `https_for_ca_consumer`,
  `install_katello_agent`, `organization`, `password`, `satellite_host`, and `username`
  are the properties available to this resource."
actions_list:
  :register:
    markdown: Default. Register the node with RHSM.
  :unregister:
    markdown: Unregister the node from RHSM.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: activation_key
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: A string or array of activation keys to use when registering; you must
      also specify the 'organization' property when using this property.
- property: auto_attach
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: If true, RHSM will attempt to automatically attach the host to applicable
      subscriptions. It is generally better to use an activation key with the subscriptions
      pre-defined.
- property: environment
  ruby_type: String
  required: false
  description_list:
  - markdown: The environment to use when registering; required when using the username
      and password properties.
- property: force
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: If true, the system will be registered even if it is already registered.
      Normally, any register operations will fail if the machine has already been
      registered.
- property: https_for_ca_consumer
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '15.9'
  description_list:
  - markdown: If true, Chef Infra Client will fetch the katello-ca-consumer-latest.noarch.rpm
      from the satellite_host using HTTPS.
- property: install_katello_agent
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: If true, the 'katello-agent' RPM will be installed.
- property: organization
  ruby_type: String
  required: false
  description_list:
  - markdown: The organization to use when registering; required when using the 'activation_key'
      property.
- property: password
  ruby_type: String
  required: false
  description_list:
  - markdown: The password to use when registering. This property is not applicable
      if using an activation key. If specified, username and environment are also
      required.
- property: satellite_host
  ruby_type: String
  required: false
  description_list:
  - markdown: The FQDN of the Satellite host to register with. If this property is
      not specified, the host will register with Red Hat's public RHSM service.
- property: username
  ruby_type: String
  required: false
  description_list:
  - markdown: The username to use when registering. This property is not applicable
      if using an activation key. If specified, password and environment properties
      are also required.
examples: "
  Register a node with RHSM\n\n  ``` ruby\n  rhsm_register 'myhost'\
  \ do\n    activation_key 'ABCD1234'\n    organization 'my_org'\n  end\n  ```\n"

---
