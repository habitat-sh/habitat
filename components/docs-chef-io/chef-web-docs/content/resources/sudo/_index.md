---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: sudo resource
resource: sudo
aliases:
- "/resource_sudo.html"
menu:
  infra:
    title: sudo
    identifier: chef_infra/cookbook_reference/resources/sudo sudo
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **sudo** resource to add or remove individual sudo entries using

    `sudoers.d` files. Sudo version 1.7.2 or newer is required to use the

    sudo resource, as it relies on the `#includedir` directive introduced in

    version 1.7.2. This resource does not enforce installation of the

    required sudo version. Chef-supported releases of Ubuntu, SuSE, Debian,

    and RHEL (6+) all support this feature.'
resource_new_in: '14.0'
syntax_full_code_block: |-
  sudo 'name' do
    command_aliases        Array
    commands               Array # default value: ["ALL"]
    config_prefix          String # default value: "Prefix values based on the node's platform"
    defaults               Array
    env_keep_add           Array
    env_keep_subtract      Array
    filename               String # default value: 'name' unless specified
    groups                 String, Array
    host                   String # default value: "ALL"
    noexec                 true, false # default value: false
    nopasswd               true, false # default value: false
    runas                  String # default value: "ALL"
    setenv                 true, false # default value: false
    template               String
    users                  String, Array
    variables              Hash
    visudo_binary          String # default value: "/usr/sbin/visudo"
    action                 Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`sudo` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`command_aliases`, `commands`, `config_prefix`, `defaults`, `env_keep_add`, `env_keep_subtract`,
  `filename`, `groups`, `host`, `noexec`, `nopasswd`, `runas`, `setenv`, `template`,
  `users`, `variables`, and `visudo_binary` are the properties available to this resource."
actions_list:
  :create:
    markdown: Default. Create a single sudoers configuration file in the `sudoers.d`
      directory.
  :delete:
    markdown: Removed a sudoers configuration file from the `sudoers.d` directory.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: command_aliases
  ruby_type: Array
  required: false
  description_list:
  - markdown: Command aliases that can be used as allowed commands later in the configuration.
- property: commands
  ruby_type: Array
  required: false
  default_value: '["ALL"]'
  description_list:
  - markdown: An array of full paths to commands this sudoer can execute.
- property: config_prefix
  ruby_type: String
  required: false
  default_value: Prefix values based on the node's platform
  description_list:
  - markdown: The directory that contains the sudoers configuration file.
- property: defaults
  ruby_type: Array
  required: false
  default_value: null
  description_list:
  - markdown: An array of defaults for the user/group.
- property: env_keep_add
  ruby_type: Array
  required: false
  default_value: null
  description_list:
  - markdown: An array of strings to add to `env_keep`.
- property: env_keep_subtract
  ruby_type: Array
  required: false
  default_value: null
  description_list:
  - markdown: An array of strings to remove from `env_keep`.
- property: filename
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the sudoers.d file if it differs from the name of the resource
      block
- property: groups
  ruby_type: String, Array
  required: false
  default_value: null
  description_list:
  - markdown: Group(s) to provide sudo privileges to. This property accepts either
      an array or a comma separated list. Leading % on group names is optional.
- property: host
  ruby_type: String
  required: false
  default_value: ALL
  description_list:
  - markdown: The host to set in the sudo configuration.
- property: noexec
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Prevent commands from shelling out.
- property: nopasswd
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Allow sudo to be run without specifying a password.
- property: runas
  ruby_type: String
  required: false
  default_value: ALL
  description_list:
  - markdown: User that the command(s) can be run as.
- property: setenv
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines whether or not to permit preservation of the environment
      with `sudo -E`.
- property: template
  ruby_type: String
  required: false
  description_list:
  - markdown: The name of the erb template in your cookbook, if you wish to supply
      your own template.
- property: users
  ruby_type: String, Array
  required: false
  default_value: null
  description_list:
  - markdown: User(s) to provide sudo privileges to. This property accepts either
      an array or a comma separated list.
- property: variables
  ruby_type: Hash
  required: false
  description_list:
  - markdown: The variables to pass to the custom template. This property is ignored
      if not using a custom template.
- property: visudo_binary
  ruby_type: String
  required: false
  default_value: "/usr/sbin/visudo"
  description_list:
  - markdown: The path to visudo for configuration verification.
examples: |
  **Grant a user sudo privileges for any command**

  ```ruby
  sudo 'admin' do
    user 'admin'
  end
  ```

  **Grant a user and groups sudo privileges for any command**

  ```ruby
  sudo 'admins' do
    users 'bob'
    groups 'sysadmins, superusers'
  end
  ```

  **Grant passwordless sudo privileges for specific commands**

  ```ruby
  sudo 'passwordless-access' do
    commands ['/bin/systemctl restart httpd', '/bin/systemctl restart mysql']
    nopasswd true
  end
  ```
---