---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_vault_secret resource
resource: chef_vault_secret
aliases:
- "/resource_chef_vault_secret.html"
menu:
  infra:
    title: chef_vault_secret
    identifier: chef_infra/cookbook_reference/resources/chef_vault_secret chef_vault_secret
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chef_vault_secret** resource to store secrets in Chef Vault
    items. Where possible and relevant, this resource attempts to map behavior and
    functionality to the knife vault sub-commands.
resource_new_in: '16.0'
syntax_full_code_block: |-
  chef_vault_secret 'name' do
    admins           String, Array
    clients          String, Array
    data_bag         String
    environment      String
    id               String # default value: 'name' unless specified
    raw_data         Hash, ChefUtils::Mash # default value: {}
    search           String # default value: "*:*"
    action           Symbol # defaults to :create if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`chef_vault_secret` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`admins`, `clients`, `data_bag`, `environment`, `id`, `raw_data`, and `search`
  are the properties available to this resource."
actions_list:
  :create:
    markdown: Create a Chef Vault data bag.
  :create_if_missing:
    markdown: Create a Chef Vault data bag unless it already exists.
  :delete:
    markdown: Delete a Chef Vault data bag if present.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: admins
  ruby_type: String, Array
  required: true
  description_list:
  - markdown: A list of admin users who should have access to the item. Corresponds
      to the 'admin' option when using the chef-vault knife plugin. Can be specified
      as a comma separated string or an array.
- property: clients
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: A search query for the nodes' API clients that should have access to
      the item.
- property: data_bag
  ruby_type: String
  required: true
  description_list:
  - markdown: The data bag that contains the item.
- property: environment
  ruby_type: String
  required: false
  description_list:
  - markdown: The Chef environment of the data if storing per environment values.
- property: id
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the data bag item if it differs from the name of the resource
      block
- property: raw_data
  ruby_type: Hash, ChefUtils::Mash
  required: false
  default_value: "{}"
  description_list:
  - markdown: The raw data, as a Ruby Hash, that will be stored in the item.
- property: search
  ruby_type: String
  required: false
  default_value: "*:*"
  description_list:
  - markdown: Search query that would match the same used for the clients, gets stored
      as a field in the item.
examples: |
  **To create a 'foo' item in an existing 'bar' data bag**:

  ```ruby
  chef_vault_secret 'foo' do
    data_bag 'bar'
    raw_data({'auth' => 'baz'})
    admins 'jtimberman'
    search '*:*'
  end
  ```

  **To allow multiple admins access to an item**:

  ```ruby
  chef_vault_secret 'root-password' do
    admins 'jtimberman,paulmooring'
    data_bag 'secrets'
    raw_data({'auth' => 'DoNotUseThisPasswordForRoot'})
    search '*:*'
  end
  ```
---