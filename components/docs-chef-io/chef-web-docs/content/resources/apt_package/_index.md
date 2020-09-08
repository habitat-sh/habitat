---

resource_reference: true
common_resource_functionality_multiple_packages: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
notes_resource_based_on_package: true
title: apt_package resource
resource: apt_package
aliases:
- "/resource_apt_package.html"
menu:
  infra:
    title: apt_package
    identifier: chef_infra/cookbook_reference/resources/apt_package apt_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **apt_package** resource to manage packages on Debian and Ubuntu
    platforms.
syntax_description: 'A **apt_package** resource block manages a package on a node,
  typically

  by installing it. The simplest use of the **apt_package** resource is:

```
apt_package ''package_name''
```

  which will install the named package using all of the default options and the default action of `:install`.'
syntax_full_code_block: |-
                    apt_package 'name' do
                      default_release              String
                      options                      String, Array
                      overwrite_config_files       true, false # default value: false
                      package_name                 String, Array
                      response_file                String
                      response_file_variables      Hash
                      timeout                      String, Integer
                      version                      String, Array
                      action                       Symbol # defaults to :install if not specified
                    end
syntax_properties_list:
syntax_full_properties_list:
- "`apt_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`default_release`, `options`, `overwrite_config_files`, `package_name`, `response_file`,
  `response_file_variables`, `timeout`, and `version` are the properties
  available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :lock:
    markdown: Locks the apt package to a specific version.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :purge:
    markdown: Purge a package. This action typically removes the configuration files
      as well as the package.
  :reconfig:
    markdown: Reconfigure a package. This action requires a response file.
  :remove:
    markdown: Remove a package.
  :unlock:
    markdown: Unlocks the apt package so that it can be upgraded to a newer version.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.

properties_list:
- property: default_release
  ruby_type: String
  required: false
  description_list:
  - markdown: 'The default release. For example: `stable`.'
- property: options
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: 'One (or more) additional options that are passed to the command. For
      example, common apt-get directives, such as `--no-install-recommends`. See the [apt-get man page](http://manpages.ubuntu.com/manpages/zesty/man8/apt-get.8.html)
      for the full list.'
- property: overwrite_config_files
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '14.0'
  description_list:
  - markdown: Overwrite existing configuration files with those supplied by the package,
      if prompted by APT.
- property: package_name
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: An optional property to set the package name if it differs from the
      resource block's name.
- property: response_file
  ruby_type: String
  required: false
  description_list:
  - markdown: The direct path to the file used to pre-seed a package.
- property: response_file_variables
  ruby_type: Hash
  required: false
  description_list:
  - markdown: A Hash of response file variables in the form of {'VARIABLE' => 'VALUE'}.
- property: timeout
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: version
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded.
examples: |
  **Install a package using package manager**:

  ```ruby
  apt_package 'name of package' do
    action :install
  end
  ```

  **Install a package without specifying the default action**:

  ```ruby
  apt_package 'name of package'
  ```

  **Install multiple packages at once**:

  ```ruby
  apt_package %(package1 package2 package3)
  ```

  **Install without using recommend packages as a dependency**:

  ```ruby
  package 'apache2' do
    options '--no-install-recommends'
  end
  ```
---
