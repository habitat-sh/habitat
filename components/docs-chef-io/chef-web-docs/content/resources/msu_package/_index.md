---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: msu_package resource
resource: msu_package
aliases:
- "/resource_msu_package.html"
menu:
  infra:
    title: msu_package
    identifier: chef_infra/cookbook_reference/resources/msu_package msu_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **msu_package** resource to install Microsoft Update(MSU) packages
    on Microsoft Windows machines.
resource_new_in: '12.17'
syntax_full_code_block: |-
  msu_package 'name' do
    checksum          String
    options           String
    package_name      String
    source            String
    timeout           String, Integer # default value: 3600
    action            Symbol # defaults to :install if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`msu_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`checksum`, `options`, `package_name`, `source`, and `timeout` are the properties
  available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: checksum
  ruby_type: String
  required: false
  description_list:
  - markdown: SHA-256 digest used to verify the checksum of the downloaded MSU package.
- property: options
  ruby_type: String
  required: false
  description_list:
  - markdown: One (or more) additional command options that are passed to the command.
- property: package_name
  ruby_type: String
  required: false
  description_list:
  - markdown: An optional property to set the package name if it differs from the
      resource block's name.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: The local file path or URL for the MSU package.
- property: timeout
  ruby_type: String, Integer
  required: false
  default_value: '3600'
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
examples: "
  Using local path in source\n\n  ``` ruby\n  msu_package 'Install\
  \ Windows 2012R2 Update KB2959977' do\n    source 'C:\\Users\\xyz\\AppData\\Local\\\
  Temp\\Windows8.1-KB2959977-x64.msu'\n    action :install\n  end\n  ```\n\n  ```\
  \ ruby\n  msu_package 'Remove Windows 2012R2 Update KB2959977' do\n    source 'C:\\\
  Users\\xyz\\AppData\\Local\\Temp\\Windows8.1-KB2959977-x64.msu'\n    action :remove\n\
  \  end\n  ```\n\n  Using URL in source\n\n  ``` ruby\n  msu_package 'Install Windows\
  \ 2012R2 Update KB2959977' do\n    source 'https://s3.amazonaws.com/my_bucket/Windows8.1-KB2959977-x64.msu'\n\
  \    action :install\n  end\n  ```\n\n  ``` ruby\n  msu_package 'Remove Windows\
  \ 2012R2 Update KB2959977' do\n    source 'https://s3.amazonaws.com/my_bucket/Windows8.1-KB2959977-x64.msu'\n\
  \    action :remove\n  end\n  ```\n"

---
