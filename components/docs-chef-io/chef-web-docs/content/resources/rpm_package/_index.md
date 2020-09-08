---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: rpm_package resource
resource: rpm_package
aliases:
- "/resource_rpm_package.html"
menu:
  infra:
    title: rpm_package
    identifier: chef_infra/cookbook_reference/resources/rpm_package rpm_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **rpm_package** resource to manage packages using the RPM Package
    Manager.
syntax_full_code_block: |-
  rpm_package 'name' do
    allow_downgrade      true, false # default value: true
    options              String, Array
    package_name         String
    source               String
    timeout              String, Integer
    version              String
    action               Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`rpm_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`allow_downgrade`, `options`, `package_name`, `source`, `timeout`, and `version`
  are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a package.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.
properties_list:
- property: allow_downgrade
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Allow downgrading a package to satisfy requested version requirements.
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
  description_list:
  - markdown: The optional path to a package on the local file system.
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
  Install a package\n\n  ``` ruby\n  rpm_package 'name of package'\
  \ do\n    action :install\n  end\n  ```\n"

---
