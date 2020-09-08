---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: cab_package resource
resource: cab_package
aliases:
- "/resource_cab_package.html"
menu:
  infra:
    title: cab_package
    identifier: chef_infra/cookbook_reference/resources/cab_package cab_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **cab_package** resource to install or remove Microsoft Windows
    cabinet (.cab) packages.
resource_new_in: '12.15'
syntax_full_code_block: |-
  cab_package 'name' do
    options           String, Array
    package_name      String
    source            String # default value: The package name.
    timeout           String, Integer
    version           String
    action            Symbol # defaults to :install if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`cab_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`options`, `package_name`, `source`, `timeout`, and `version` are the properties
  available to this resource."
actions_list:
  :install:
    markdown: Installs the cabinet package.
  :remove:
    markdown: Removes the cabinet package.
properties_list:
- property: options
  ruby_type: String, Array
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
  default_value: The package name.
  description_list:
  - markdown: The local file path or URL for the CAB package.
- property: timeout
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: version
  ruby_type: String
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded.
examples: "
  Using local path in source\n\n  ``` ruby\n  cab_package 'Install\
  \ .NET 3.5 sp1 via KB958488' do\n    source 'C:\\Users\\xyz\\AppData\\Local\\Temp\\\
  Windows6.1-KB958488-x64.cab'\n    action :install\n  end\n  ```\n\n  ``` ruby\n\
  \  cab_package 'Remove .NET 3.5 sp1 via KB958488' do\n    source 'C:\\Users\\xyz\\\
  AppData\\Local\\Temp\\Windows6.1-KB958488-x64.cab'\n    action :remove\n  end\n\
  \  ```\n\n  Using URL in source\n\n  ``` ruby\n  cab_package 'Install .NET 3.5 sp1\
  \ via KB958488' do\n    source 'https://s3.amazonaws.com/my_bucket/Windows6.1-KB958488-x64.cab'\n\
  \    action :install\n  end\n  ```\n\n  ``` ruby\n  cab_package 'Remove .NET 3.5\
  \ sp1 via KB958488' do\n    source 'https://s3.amazonaws.com/my_bucket/Temp\\Windows6.1-KB958488-x64.cab'\n\
  \    action :remove\n  end\n  ```\n"

---
