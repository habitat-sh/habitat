---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_security_policy resource
resource: windows_security_policy
aliases:
- "/resource_windows_security_policy.html"
menu:
  infra:
    title: windows_security_policy
    identifier: chef_infra/cookbook_reference/resources/windows_security_policy windows_security_policy
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_security_policy** resource to set a security policy
    on the Microsoft Windows platform.
resource_new_in: '16.0'
syntax_full_code_block: |-
  windows_security_policy 'name' do
    secoption      String # default value: 'name' unless specified
    secvalue       String
    action         Symbol # defaults to :set if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`windows_security_policy` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`secoption` and `secvalue` are the properties available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :set:
    markdown: Set the Windows security policy
properties_list:
- property: secoption
  ruby_type: String
  required: true
  default_value: The resource block's name
  allowed_values: '"ClearTextPassword", "EnableAdminAccount", "EnableGuestAccount",
    "ForceLogoffWhenHourExpire", "LSAAnonymousNameLookup", "LockoutBadCount", "LockoutDuration",
    "MaximumPasswordAge", "MinimumPasswordAge", "MinimumPasswordLength", "NewAdministratorName",
    "NewGuestName", "PasswordComplexity", "PasswordHistorySize", "RequireLogonToChangePassword",
    "ResetLockoutCount"'
  description_list:
  - markdown: The name of the policy to be set on windows platform to maintain its
      security.
- property: secvalue
  ruby_type: String
  required: true
  description_list:
  - markdown: Policy value to be set for policy name.
examples: |
  **Set Administrator Account to Enabled**:

  ```ruby
  windows_security_policy 'EnableAdminAccount' do
    secvalue       '1'
    action         :set
  end
  ```

  **Rename Administrator Account**:

  ```ruby
  windows_security_policy 'NewAdministratorName' do
    secvalue       'AwesomeChefGuy'
    action         :set
  end
  ```

  **Set Guest Account to Disabled**:

  ```ruby
  windows_security_policy 'EnableGuestAccount' do
    secvalue       '0'
    action         :set
  end
  ```
---
