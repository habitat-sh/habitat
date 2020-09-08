---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: macos_userdefaults resource
resource: macos_userdefaults
aliases:
- "/resource_macos_userdefaults.html"
menu:
  infra:
    title: macos_userdefaults
    identifier: chef_infra/cookbook_reference/resources/macos_userdefaults macos_userdefaults
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **macos_userdefaults** resource to manage the macOS user defaults
    system. The properties of this resource are passed to the defaults command, and
    the parameters follow the convention of that command. See the defaults(1) man
    page for details on how the tool works.
resource_new_in: '14.0'
syntax_full_code_block: |-
  macos_userdefaults 'name' do
    domain      String # default value: NSGlobalDomain: the global domain.
    host        String, Symbol
    key         String
    sudo        true, false # default value: false
    type        String
    user        String
    value       Integer, Float, String, true, false, Hash, Array
    action      Symbol # defaults to :write if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`macos_userdefaults` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`domain`, `host`, `key`, `sudo`, `type`, `user`, and `value` are the properties
  available to this resource."
actions_list:
  :delete:
    markdown: Remove a setting key from the specified domain.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :write:
    markdown: Default. Writes the setting to the specified domain.
properties_list:
- property: domain
  ruby_type: String
  required: false
  default_value: 'NSGlobalDomain: the global domain.'
  description_list:
  - markdown: The domain that the user defaults belong to.
- property: host
  ruby_type: String, Symbol
  required: false
  new_in: '16.3'
  description_list:
  - markdown: Set either :current or a hostname to set the user default at the host
      level.
- property: key
  ruby_type: String
  required: true
  description_list:
  - markdown: The preference key.
- property: sudo
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Set to true if the setting you wish to modify requires privileged access.
      This requires passwordless sudo for the '/usr/bin/defaults' command to be setup
      for the user running Chef Infra Client.
- property: type
  ruby_type: String
  required: false
  allowed_values: '"array", "bool", "dict", "float", "int", "string"'
  description_list:
  - markdown: The value type of the preference key.
- property: user
  ruby_type: String
  required: false
  description_list:
  - markdown: The system user that the default will be applied to.
- property: value
  ruby_type: Integer, Float, String, true, false, Hash, Array
  required:
  - write
  description_list:
  - markdown: 'The value of the key. Note: With the `type` property set to `bool`,
      `String` forms of Boolean true/false values that Apple accepts in the defaults
      command will be coerced: 0/1, ''TRUE''/''FALSE,'' ''true''/false'', ''YES''/''NO'',
      or ''yes''/''no''.'
examples: |
  **Specify a global domain value**

  ```ruby
  macos_userdefaults 'Full keyboard access to all controls' do
    key 'AppleKeyboardUIMode'
    value 2
  end
  ```

  **Setting a value on a specific domain**

  ```ruby
  macos_userdefaults 'Enable macOS firewall' do
    domain '/Library/Preferences/com.apple.alf'
    key 'globalstate'
    value 1
  end
  ```

  **Specifying the type of a key to skip automatic type detection**

  ```ruby
  macos_userdefaults 'Finder expanded save dialogs' do
    key 'NSNavPanelExpandedStateForSaveMode'
    value 'TRUE'
    type 'bool'
  end
  ```
---