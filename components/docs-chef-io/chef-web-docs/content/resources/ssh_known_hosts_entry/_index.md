---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: ssh_known_hosts_entry resource
resource: ssh_known_hosts_entry
aliases:
- "/resource_ssh_known_hosts_entry.html"
menu:
  infra:
    title: ssh_known_hosts_entry
    identifier: chef_infra/cookbook_reference/resources/ssh_known_hosts_entry ssh_known_hosts_entry
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **ssh_known_hosts_entry** resource to add an entry for the specified
    host in /etc/ssh/ssh_known_hosts or a user's known hosts file if specified.
resource_new_in: '14.3'
syntax_full_code_block: |-
  ssh_known_hosts_entry 'name' do
    file_location      String # default value: "/etc/ssh/ssh_known_hosts"
    group              String, Integer
    hash_entries       true, false # default value: false
    host               String # default value: 'name' unless specified
    key                String
    key_type           String # default value: "rsa"
    mode               String # default value: "0644"
    owner              String, Integer # default value: "root"
    port               Integer # default value: 22
    timeout            Integer # default value: 30
    action             Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`ssh_known_hosts_entry` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`file_location`, `group`, `hash_entries`, `host`, `key`, `key_type`, `mode`, `owner`,
  `port`, and `timeout` are the properties available to this resource."
actions_list:
  :create:
    markdown: Default. Create an entry in the ssh_known_hosts file.
  :flush:
    markdown: Immediately flush the entries to the config file. Without this the actual
      writing of the file is delayed in the Chef run so all entries can be accumulated
      before writing the file out.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: file_location
  ruby_type: String
  required: false
  default_value: "/etc/ssh/ssh_known_hosts"
  description_list:
  - markdown: The location of the ssh known hosts file. Change this to set a known
      host file for a particular user.
- property: group
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The file group for the ssh_known_hosts file.
- property: hash_entries
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Hash the hostname and addresses in the ssh_known_hosts file for privacy.
- property: host
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The host to add to the known hosts file.
- property: key
  ruby_type: String
  required: false
  description_list:
  - markdown: An optional key for the host. If not provided this will be automatically
      determined.
- property: key_type
  ruby_type: String
  required: false
  default_value: rsa
  description_list:
  - markdown: The type of key to store.
- property: mode
  ruby_type: String
  required: false
  default_value: '"0644"'
  description_list:
  - markdown: The file mode for the ssh_known_hosts file.
- property: owner
  ruby_type: String, Integer
  required: false
  default_value: root
  description_list:
  - markdown: The file owner for the ssh_known_hosts file.
- property: port
  ruby_type: Integer
  required: false
  default_value: '22'
  description_list:
  - markdown: The server port that the ssh-keyscan command will use to gather the
      public key.
- property: timeout
  ruby_type: Integer
  required: false
  default_value: '30'
  description_list:
  - markdown: The timeout in seconds for ssh-keyscan.
examples: |
  **Add a single entry for github.com with the key auto detected**

  ```ruby
  ssh_known_hosts_entry 'github.com'
  ```

  **Add a single entry with your own provided key**

  ```ruby
  ssh_known_hosts_entry 'github.com' do
    key 'node.example.com ssh-rsa ...'
  end
  ```
---