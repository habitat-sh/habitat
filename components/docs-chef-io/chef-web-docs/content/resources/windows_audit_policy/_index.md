---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_audit_policy resource
resource: windows_audit_policy
aliases:
- "/resource_windows_audit_policy.html"
menu:
  infra:
    title: windows_audit_policy
    identifier: chef_infra/cookbook_reference/resources/windows_audit_policy windows_audit_policy
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_audit_policy** resource to configure system level and
    per-user Windows advanced audit policy settings.
resource_new_in: '16.2'
syntax_full_code_block: |-
  windows_audit_policy 'name' do
    audit_base_directories       true, false
    audit_base_objects           true, false
    crash_on_audit_fail          true, false
    exclude_user                 String
    failure                      true, false
    full_privilege_auditing      true, false
    include_user                 String
    subcategory                  String, Array
    success                      true, false
    action                       Symbol # defaults to :set if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`windows_audit_policy` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`audit_base_directories`, `audit_base_objects`, `crash_on_audit_fail`, `exclude_user`,
  `failure`, `full_privilege_auditing`, `include_user`, `subcategory`, and `success`
  are the properties available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :set:
    markdown: Configure an audit policy.
properties_list:
- property: audit_base_directories
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Setting this audit policy option to true will force the system to assign
      a System Access Control List to named objects to enable auditing of container
      objects such as directories.
- property: audit_base_objects
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Setting this audit policy option to true will force the system to assign
      a System Access Control List to named objects to enable auditing of base objects
      such as mutexes.
- property: crash_on_audit_fail
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Setting this audit policy option to true will cause the system to crash
      if the auditing system is unable to log events.
- property: exclude_user
  ruby_type: String
  required: false
  description_list:
  - markdown: The audit policy specified by the category or subcategory is applied
      per-user if specified. When a user is specified, exclude user. Include and exclude
      cannot be used at the same time.
- property: failure
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Specify failure auditing. By setting this property to true the resource
      will enable failure for the category or sub category. Success is the default
      and is applied if neither success nor failure are specified.
- property: full_privilege_auditing
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Setting this audit policy option to true will force the audit of all
      privilege changes except SeAuditPrivilege. Setting this property may cause the
      logs to fill up more quickly.
- property: include_user
  ruby_type: String
  required: false
  description_list:
  - markdown: The audit policy specified by the category or subcategory is applied
      per-user if specified. When a user is specified, include user. Include and exclude
      cannot be used at the same time.
- property: subcategory
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The audit policy subcategory, specified by GUID or name. Applied system-wide
      if no user is specified.
- property: success
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Specify success auditing. By setting this property to true the resource
      will enable success for the category or sub category. Success is the default
      and is applied if neither success nor failure are specified.
examples: |
  **Set Logon and Logoff policy to "Success and Failure"**:

  ```ruby
  windows_audit_policy "Set Audit Policy for 'Logon and Logoff' actions to 'Success and Failure'" do
    subcategory %w(Logon Logoff)
    success true
    failure true
    action :set
  end
  ```

  **Set Credential Validation policy to "Success"**:

  ```ruby
  windows_audit_policy "Set Audit Policy for 'Credential Validation' actions to 'Success'" do
    subcategory  'Credential Validation'
    success true
    failure false
    action :set
  end
  ```

  **Enable CrashOnAuditFail option**:

  ```ruby
  windows_audit_policy 'Enable CrashOnAuditFail option' do
    crash_on_audit_fail true
    action :set
  end
  ```
---