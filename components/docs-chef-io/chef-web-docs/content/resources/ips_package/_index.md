---
title: ips_package resource
resource: ips_package
draft: false
aliases:
- /resource_ips_package.html
menu:
  infra:
    title: ips_package
    identifier: chef_infra/cookbook_reference/resources/ips_package ips_package
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **ips_package** resource to manage packages (using Image

    Packaging System (IPS)) on the Solaris 11 platform.'
- notes_resource_based_on_package: true
resource_new_in: null
handler_types: false
syntax_description: 'A **ips_package** resource block manages a package on a node,
  typically

  by installing it. The simplest use of the **ips_package** resource is:


  ``` ruby

  ips_package ''package_name''

  ```


  which will install the named package using all of the default options

  and the default action (`:install`).'
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "ips_package 'name' do\n  accept_license             true,\
  \ false # default value: false\n  options                    String\n  package_name\
  \               String, Array\n  source                     String\n  timeout  \
  \                  String, Integer\n  version                    String, Array\n\
  \  action                     Symbol # defaults to :install if not specified\nend"
syntax_full_properties_list:
- '`ips_package` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`accept_license`, `options`, `package_name`, `source`, `timeout`, and `version`
  are the properties available to this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
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
- property: accept_license
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Accept an end-user license agreement, automatically.
- property: options
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'One (or more) additional command options that are passed to the

      command.'
- property: package_name
  ruby_type: String, Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'An optional property to set the package name if it differs from the

      resource block''s name.'
- property: source
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The optional path to a package on the local file system.
- property: timeout
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: version
  ruby_type: String, Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The version of a package to be installed or upgraded.
properties_shortcode: null
properties_multiple_packages: false
resource_directory_recursive_directories: false
resources_common_atomic_update: false
properties_resources_common_windows_security: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
ps_credential_helper: false
ruby_style_basics_chef_log: false
debug_recipes_chef_shell: false
template_requirements: false
resources_common_properties: true
resources_common_notification: true
resources_common_guards: true
common_resource_functionality_multiple_packages: false
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Install a package\n\n  ``` ruby\n  ips_package 'name of package'\
  \ do\n    action :install\n  end\n  ```\n"

---
