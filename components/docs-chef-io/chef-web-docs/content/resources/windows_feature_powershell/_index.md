---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_feature_powershell resource
resource: windows_feature_powershell
aliases:
- "/resource_windows_feature_powershell.html"
menu:
  infra:
    title: windows_feature_powershell
    identifier: chef_infra/cookbook_reference/resources/windows_feature_powershell
      windows_feature_powershell
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **windows_feature_powershell** resource to add, remove, or

    entirely delete Windows features and roles using PowerShell. This

    resource offers significant speed benefits over the

    [windows_feature_dism](/resources/windows_feature_dism/) resource,

    but requires installation of the Remote Server Administration Tools on

    non-server releases of Windows.'
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_feature_powershell 'name' do
    all                   true, false # default value: false
    feature_name          Array, String # default value: 'name' unless specified
    management_tools      true, false # default value: false
    source                String
    timeout               Integer # default value: 600
    action                Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_feature_powershell` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`all`, `feature_name`, `management_tools`, `source`, and `timeout` are the properties
  available to this resource."
actions_list:
  :delete:
    markdown: Delete a Windows role / feature from the image using PowerShell.
  :install:
    markdown: Default. Install a Windows role / feature using PowerShell.
  :remove:
    markdown: Remove a Windows role / feature using PowerShell.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: all
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Install all subfeatures. When set to `true`, this is the equivalent
      of specifying the `-InstallAllSubFeatures` switch with `Add-WindowsFeature`.
- property: feature_name
  ruby_type: Array, String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the feature(s) or role(s) to install if they differ from
      the resource block's name.
- property: management_tools
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Install all applicable management tools for the roles, role services,
      or features.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: Specify a local repository for the feature install.
- property: timeout
  ruby_type: Integer
  required: false
  default_value: '600'
  description_list:
  - markdown: Specifies a timeout (in seconds) for the feature installation.
examples: |
  **Add the SMTP Server feature**:

  ```ruby
  windows_feature_powershell "smtp-server" do
    action :install
    all true
  end
  ```

  **Install multiple features using one resource**:

  ```ruby
  windows_feature_powershell ['Web-Asp-Net45', 'Web-Net-Ext45'] do
    action :install
  end
  ```

  **Install the Network Policy and Access Service feature**:

  ```ruby
  windows_feature_powershell 'NPAS' do
    action :install
    management_tools true
  end
  ```
---