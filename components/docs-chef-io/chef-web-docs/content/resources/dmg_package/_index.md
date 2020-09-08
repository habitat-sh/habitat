---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: dmg_package resource
resource: dmg_package
aliases:
- "/resource_dmg_package.html"
menu:
  infra:
    title: dmg_package
    identifier: chef_infra/cookbook_reference/resources/dmg_package dmg_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **dmg_package** resource to install a package from a .dmg file.
    The resource will retrieve the dmg file from a remote URL, mount it using macOS'
    `hdidutil`, copy the application (.app directory) to the specified destination
    (`/Applications`), and detach the image using `hdiutil`. The dmg file will be
    stored in the `Chef::Config[:file_cache_path]`.
resource_new_in: '14.0'
syntax_full_code_block: |-
  dmg_package 'name' do
    accept_eula          true, false # default value: false
    allow_untrusted      true, false # default value: false
    app                  String # default value: 'name' unless specified
    checksum             String
    destination          String # default value: "/Applications"
    dmg_name             String # default value: The value passed for the application name.
    dmg_passphrase       String
    file                 String
    headers              Hash
    owner                String, Integer
    package_id           String
    source               String
    type                 String # default value: "app"
    volumes_dir          String # default value: The value passed for the application name.
    action               Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`dmg_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`accept_eula`, `allow_untrusted`, `app`, `checksum`, `destination`, `dmg_name`,
  `dmg_passphrase`, `file`, `headers`, `owner`, `package_id`, `source`, `type`, and
  `volumes_dir` are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Installs the application.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: accept_eula
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Specify whether to accept the EULA. Certain dmg files require acceptance
      of EULA before mounting.
- property: allow_untrusted
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Allow installation of packages that do not have trusted certificates.
- property: app
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the application as it appears in the `/Volumes` directory
      if it differs from the resource block's name.
- property: checksum
  ruby_type: String
  required: false
  description_list:
  - markdown: The sha256 checksum of the `.dmg` file to download.
- property: destination
  ruby_type: String
  required: false
  default_value: "/Applications"
  description_list:
  - markdown: The directory to copy the `.app` into.
- property: dmg_name
  ruby_type: String
  required: false
  default_value: The value passed for the application name.
  description_list:
  - markdown: The name of the `.dmg` file if it differs from that of the app, or if
      the name has spaces.
- property: dmg_passphrase
  ruby_type: String
  required: false
  description_list:
  - markdown: Specify a passphrase to be used to decrypt the `.dmg` file during the
      mount process.
- property: file
  ruby_type: String
  required: false
  description_list:
  - markdown: The absolute path to the `.dmg` file on the local system.
- property: headers
  ruby_type: Hash
  required: false
  description_list:
  - markdown: Allows custom HTTP headers (like cookies) to be set on the `remote_file`
      resource.
- property: owner
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The user that should own the package installation.
- property: package_id
  ruby_type: String
  required: false
  description_list:
  - markdown: The package ID that is registered with `pkgutil` when a `pkg` or `mpkg`
      is installed.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: The remote URL that is used to download the `.dmg` file, if specified.
- property: type
  ruby_type: String
  required: false
  default_value: app
  allowed_values: '"app", "mpkg", "pkg"'
  description_list:
  - markdown: The type of package.
- property: volumes_dir
  ruby_type: String
  required: false
  default_value: The value passed for the application name.
  description_list:
  - markdown: The directory under `/Volumes` where the `dmg` is mounted if it differs
      from the name of the `.dmg` file.
examples: |
  **Install Google Chrome via the DMG package**:

  ```ruby
  dmg_package 'Google Chrome' do
    dmg_name 'googlechrome'
    source   'https://dl-ssl.google.com/chrome/mac/stable/GGRM/googlechrome.dmg'
    checksum '7daa2dc5c46d9bfb14f1d7ff4b33884325e5e63e694810adc58f14795165c91a'
    action   :install
  end
  ```

  **Install Virtualbox from the .mpkg**:

  ```ruby
  dmg_package 'Virtualbox' do
    source 'http://dlc.sun.com.edgesuite.net/virtualbox/4.0.8/VirtualBox-4.0.8-71778-OSX.dmg'
    type   'mpkg'
  end
  ```

  **Install pgAdmin and automatically accept the EULA**:

  ```ruby
  dmg_package 'pgAdmin3' do
    source   'http://wwwmaster.postgresql.org/redir/198/h/pgadmin3/release/v1.12.3/osx/pgadmin3-1.12.3.dmg'
    checksum '9435f79d5b52d0febeddfad392adf82db9df159196f496c1ab139a6957242ce9'
    accept_eula true
  end
  ```
---