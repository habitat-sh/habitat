---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
notes_resource_based_on_package: true
title: windows_package resource
resource: windows_package
aliases:
- "/resource_windows_package.html"
menu:
  infra:
    title: windows_package
    identifier: chef_infra/cookbook_reference/resources/windows_package windows_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **windows_package** resource to manage Microsoft Installer

    Package (MSI) packages for the Microsoft Windows platform.'
resource_new_in: null
handler_types: false
syntax_description: 'A **windows_package** resource block manages a package on a node,

  typically by installing it. The simplest use of the **windows_package**

  resource is:


  ``` ruby

  windows_package ''package_name''

  ```


  which will install the named package using all of the default options

  and the default action (`:install`).'
syntax_full_code_block: |-
  windows_package 'name' do
    checksum                    String
    installer_type              Symbol
    options                     String
    package_name                String
    remote_file_attributes      Hash
    returns                     String, Integer, Array
    source                      String # default value: "The resource block's name"
    timeout                     String, Integer # default value: "600 (seconds)"
    version                     String
    action                      Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`checksum`, `installer_type`, `options`, `package_name`, `remote_file_attributes`,
  `returns`, `source`, `timeout`, and `version` are the properties available to this
  resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a package.
properties_list:
- property: checksum
  ruby_type: String
  required: false
  description_list:
  - markdown: The SHA-256 checksum of the file. Use to prevent a file from being re-downloaded.
      When the local file matches the checksum, Chef Infra Client does not download
      it. Use when a URL is specified by the `source` property.
- property: installer_type
  ruby_type: Symbol
  required: false
  allowed_values: ":custom, :inno, :installshield, :msi, :nsis, :wise"
  description_list:
  - markdown: 'A symbol that specifies the type of package. Possible values:

      `:custom` (such as installing a non-.msi file that embeds an

      .msi-based installer), `:inno` (Inno Setup), `:installshield`

      (InstallShield), `:msi` (Microsoft Installer Package (MSI)), `:nsis`

      (Nullsoft Scriptable Install System (NSIS)), `:wise` (Wise).'
- property: options
  ruby_type: String
  required: false
  description_list:
  - markdown: One (or more) additional options that are passed to the command.
- property: package_name
  ruby_type: String
  required: false
  description_list:
  - markdown: An optional property to set the package name if it differs from the
      resource block's name.
- property: remote_file_attributes
  ruby_type: Hash
  required: false
  description_list:
  - markdown: 'This property allows you to define a hash of properties and their

      value if the source package to be installed is at a remote location.

      This hash will be used by the underlying **remote_file** resource

      which will fetch the source package.'
- property: returns
  ruby_type: String, Integer, Array of integers
  required: false
  default_value: 0 (success) and 3010 (success where a reboot is necessary)
  description_list:
  - markdown: A comma-delimited list of return codes that indicate the success or
      failure of the package command that was run.
- property: source
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: 'The path to a package in the local file system. The location of the

      package may be at a URL.


      If the `source` property is not specified, the package name MUST be

      exactly the same as the display name found in **Add/Remove

      programs** or exactly the same as the `DisplayName` property in the

      appropriate registry key, which may be one of the following:


      ``` ruby

      HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Uninstall

      HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall

      HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall

      ```'
  - note:
    - markdown: 'If there are multiple versions of a package installed with the same

        display name, all of those packages will be removed unless a version

        is provided in the `version` property or unless it can be discovered

        in the installer file specified by the `source` property.'
- property: timeout
  ruby_type: String, Integer
  required: false
  default_value: 600 (seconds)
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: version
  ruby_type: String
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded.
examples: |
  **Install a package**:

  ```ruby
  windows_package '7zip' do
    action :install
    source 'C:\7z920.msi'
  end
  ```

  **Specify a URL for the source attribute**:

  ```ruby
  windows_package '7zip' do
    source 'http://www.7-zip.org/a/7z938-x64.msi'
  end
  ```

  **Specify path and checksum**:

  ```ruby
  windows_package '7zip' do
    source 'http://www.7-zip.org/a/7z938-x64.msi'
    checksum '7c8e873991c82ad9cfc123415254ea6101e9a645e12977dcd518979e50fdedf3'
  end
  ```

  **Modify remote_file resource attributes**:

  The windows_package resource may specify a package at a remote location using the remote_file_attributes property. This uses the remote_file resource to download the contents at the specified URL and passes in a Hash that modifies the properties of the remote_file resource.

  ```ruby
  windows_package '7zip' do
    source 'http://www.7-zip.org/a/7z938-x64.msi'
    remote_file_attributes ({
      :path => 'C:\7zip.msi',
      :checksum => '7c8e873991c82ad9cfc123415254ea6101e9a645e12977dcd518979e50fdedf3'
    })
  end
  ```

  **Download a nsis (Nullsoft) package resource**:

  ```ruby
  windows_package 'Mercurial 3.6.1 (64-bit)' do
    source 'http://mercurial.selenic.com/release/windows/Mercurial-3.6.1-x64.exe'
    checksum 'febd29578cb6736163d232708b834a2ddd119aa40abc536b2c313fc5e1b5831d'
  end
  ```

  **Download a custom package**:

  ```ruby
  windows_package 'Microsoft Visual C++ 2005 Redistributable' do
    source 'https://download.microsoft.com/download/6/B/B/6BB661D6-A8AE-4819-B79F-236472F6070C/vcredist_x86.exe'
    installer_type :custom
    options '/Q'
  end
  ```
---