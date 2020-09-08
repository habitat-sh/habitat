---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_feature resource
resource: windows_feature
aliases:
- "/resource_windows_feature.html"
menu:
  infra:
    title: windows_feature
    identifier: chef_infra/cookbook_reference/resources/windows_feature windows_feature
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **windows_feature** resource to add, remove or entirely delete

    Windows features and roles. This resource calls the

    [windows_feature_dism](/resources/windows_feature_dism/) or

    [windows_feature_powershell](/resources/windows_feature_powershell/)

    resources depending on the specified installation method, and defaults

    to DISM, which is available on both Workstation and Server editions of

    Windows.'
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_feature 'name' do
    all                   true, false # default value: false
    feature_name          Array, String # default value: 'name' unless specified
    install_method        Symbol # default value: :windows_feature_dism
    management_tools      true, false # default value: false
    source                String
    timeout               Integer # default value: 600
    action                Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_feature` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`all`, `feature_name`, `install_method`, `management_tools`, `source`, and `timeout`
  are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a Windows role / feature using PowerShell.
  :remove:
    markdown: Remove a Windows role / feature using PowerShell.
  :delete:
    markdown: Delete a Windows role / feature from the image using PowerShell.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: all
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Install all sub-features.
- property: feature_name
  ruby_type: Array, String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the feature(s) or role(s) to install if they differ from
      the resource block's name. The same feature may have different names depending
      on the underlying installation method being used (ie DHCPServer vs DHCP; DNS-Server-Full-Role
      vs DNS).
- property: install_method
  ruby_type: Symbol
  required: false
  default_value: ":windows_feature_dism"
  allowed_values: ":windows_feature_dism, :windows_feature_powershell, :windows_feature_servermanagercmd"
  description_list:
  - markdown: The underlying installation method to use for feature installation.
      Specify `:windows_feature_dism` for DISM or `:windows_feature_powershell` for
      PowerShell.
- property: management_tools
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Install all applicable management tools for the roles, role services,
      or features (PowerShell-only).
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
  **Install the DHCP Server feature**:

  ```ruby
  windows_feature 'DHCPServer' do
    action :install
  end
  ```

  **Install the .Net 3.5.1 feature using repository files on DVD**:

  ```ruby
  windows_feature "NetFx3" do
    action :install
    source 'd:\sources\sxs'
  end
  ```

  **Remove Telnet Server and Client features**:

  ```ruby
  windows_feature %w(TelnetServer TelnetClient) do
    action :remove
  end
  ```

  **Add the SMTP Server feature using the PowerShell provider**:

  ```ruby
  windows_feature 'smtp-server' do
    action :install
    all true
    install_method :windows_feature_powershell
  end
  ```

  **Install multiple features using one resource with the PowerShell provider**:

  ```ruby
  windows_feature %w(Web-Asp-Net45 Web-Net-Ext45) do
    action :install
    install_method :windows_feature_powershell
  end
  ```

  **Install the Network Policy and Access Service feature, including the management tools**:

  ```ruby
  windows_feature 'NPAS' do
    action :install
    management_tools true
    install_method :windows_feature_powershell
  end
  ```
---