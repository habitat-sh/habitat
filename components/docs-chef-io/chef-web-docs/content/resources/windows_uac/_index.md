---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_uac resource
resource: windows_uac
aliases:
- "/resource_windows_uac.html"
menu:
  infra:
    title: windows_uac
    identifier: chef_infra/cookbook_reference/resources/windows_uac windows_uac
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: The *windows_uac* resource configures UAC on Windows hosts by setting
    registry keys at `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System`
resource_new_in: '15.0'
syntax_full_code_block: |-
  windows_uac 'name' do
    consent_behavior_admins       Symbol # default value: :prompt_for_consent_non_windows_binaries
    consent_behavior_users        Symbol # default value: :prompt_for_creds
    detect_installers             true, false
    enable_uac                    true, false # default value: true
    prompt_on_secure_desktop      true, false # default value: true
    require_signed_binaries       true, false # default value: false
    action                        Symbol # defaults to :configure if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_uac` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`consent_behavior_admins`, `consent_behavior_users`, `detect_installers`, `enable_uac`,
  `prompt_on_secure_desktop`, and `require_signed_binaries` are the properties available
  to this resource."
actions_list:
  :configure:
    markdown: Configures UAC by setting registry keys at `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System`.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: consent_behavior_admins
  ruby_type: Symbol
  required: false
  default_value: ":prompt_for_consent_non_windows_binaries"
  allowed_values: ":no_prompt, :prompt_for_consent, :prompt_for_consent_non_windows_binaries,
    :prompt_for_creds, :secure_prompt_for_consent, :secure_prompt_for_creds"
  description_list:
  - markdown: Behavior of the elevation prompt for administrators in Admin Approval
      Mode. Sets HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA\ConsentPromptBehaviorAdmin.
- property: consent_behavior_users
  ruby_type: Symbol
  required: false
  default_value: ":prompt_for_creds"
  allowed_values: ":auto_deny, :prompt_for_creds, :secure_prompt_for_creds"
  description_list:
  - markdown: Behavior of the elevation prompt for standard users. Sets HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA\ConsentPromptBehaviorUser.
- property: detect_installers
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Detect application installations and prompt for elevation. Sets HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA\EnableInstallerDetection.
- property: enable_uac
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Enable or disable UAC Admin Approval Mode. If this is changed a system
      restart is required. Sets HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA.
- property: prompt_on_secure_desktop
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Switch to the secure desktop when prompting for elevation. Sets HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA\PromptOnSecureDesktop.
- property: require_signed_binaries
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Only elevate executables that are signed and validated. Sets HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA\ValidateAdminCodeSignatures.
examples: |
  **Disable UAC prompts for the admin**:

  ``` ruby
  windows_uac 'Disable UAC prompts for the admin' do
    enable_uac true
    prompt_on_secure_desktop false
    consent_behavior_admins :no_prompt
  end
  ```

  **Disable UAC entirely**:

  ``` ruby
  windows_uac 'Disable UAC entirely' do
    enable_uac false
  end
  ```
---
