---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: dpkg_package resource
resource: dpkg_package
aliases:
- "/resource_dpkg_package.html"
menu:
  infra:
    title: dpkg_package
    identifier: chef_infra/cookbook_reference/resources/dpkg_package dpkg_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **dpkg_package** resource to manage packages for the dpkg platform.
    When a package is installed from a local file, it must be added to the node using
    the **remote_file** or **cookbook_file** resources.
resource_new_in:
syntax_description: 'A **dpkg_package** resource block manages a package on a node,

  typically by installing it. The simplest use of the **dpkg_package**

  resource is:


  ``` ruby

  dpkg_package ''package_name''

  ```


  which will install the named package using all of the default options

  and the default action (`:install`).'
syntax_properties_list: 
syntax_full_code_block: "dpkg_package 'name' do\n  options                      String,\
  \ Array\n  package_name                 String, Array\n  response_file         \
  \       String\n  response_file_variables      Hash\n  source                  \
  \     String, Array\n  timeout                      String, Integer\n  version \
  \                     String, Array\n  action                       Symbol # defaults\
  \ to :install if not specified\nend"
syntax_full_properties_list:
- "`dpkg_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`options`, `package_name`, `response_file`, `response_file_variables`, `source`,
  `timeout`, and `version` are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :purge:
    markdown: Purge a package. This action typically removes the configuration files
      as well as the package.
  :remove:
    markdown: Remove a package.
properties_list:
- property: options
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: One (or more) additional command options that are passed to the command.
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
- property: source
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The path to a package in the local file system.
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
examples: "
  Install a package\n\n  ``` ruby\n  dpkg_package 'wget_1.13.4-2ubuntu1.4_amd64.deb'\
  \ do\n    source '/foo/bar/wget_1.13.4-2ubuntu1.4_amd64.deb'\n    action :install\n\
  \  end\n  ```\n"

---