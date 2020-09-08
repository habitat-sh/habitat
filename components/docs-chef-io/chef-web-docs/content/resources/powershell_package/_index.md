---
title: powershell_package resource
resource: powershell_package
draft: false
aliases:
- /resource_powershell_package.html
menu:
  infra:
    title: powershell_package
    identifier: chef_infra/cookbook_reference/resources/powershell_package powershell_package
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **powershell_package** resource to install and manage packages

    via the PowerShell Package Manager for the Microsoft Windows platform.

    The **powershell_package** resource requires administrative access, and

    a source must be configured in the PowerShell Package Manager via the

    [Register-PackageSource](https://docs.microsoft.com/en-us/powershell/module/packagemanagement/register-packagesource?view=powershell-5.1)

    command or the

    [powershell_package_source](/resources/powershell_package_source/)

    resource.'
resource_new_in: '12.16'
handler_types: false
syntax_description: "A **powershell_package** resource block manages a package on\
  \ a node,\ntypically by installing it. The simplest use of the\n**powershell_package**\
  \ resource is:\n\n``` ruby\npowershell_package 'package_name'\n```\n\nwhich will\
  \ install the named package using all of the default options\nand the default action\
  \ (`:install`).\n\nThe powershell_package resource has the following syntax:\n\n\
  ``` ruby\npowershell_package 'name' do\n  options                   String, Array\n\
  \  package_name              String, Array\n  skip_publisher_check      true, false\
  \ # default value: false\n  source                    String\n  timeout        \
  \           String, Integer\n  version                   String, Array\n  action\
  \                    Symbol # defaults to :install if not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`powershell_package` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`options`, `package_name`, `skip_publisher_check`, `source`, `timeout`, and `version`
  are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :remove:
    markdown: Remove a package.
properties_list:
- property: options
  ruby_type: String, Array
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
  - markdown: 'The name of the package. Default value: the name of the resource

      block.'
- property: skip_publisher_check
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '14.3'
  description_list:
  - markdown: Skip validating module author.
- property: source
  ruby_type: String
  required: false
  default_value: null
  new_in: '14.0'
  description_list:
  - markdown: Specify the source of the package.
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
  Install a specific version of a package:\n\n  ``` ruby\n  powershell_package\
  \ 'xCertificate' do\n    action :install\n    version '1.1.0.0'\n  end\n  ```\n\n\
  \  Install multiple packages:\n\n  ``` ruby\n  powershell_package 'Install Multiple\
  \ Packages' do\n    action :install\n    package_name %w(xCertificate xNetworking)\n\
  \  end\n  ```\n\n  Install a package from a custom source:\n\n  ``` ruby\n  powershell_package\
  \ 'xCertificate' do\n    action :install\n    source 'MyGallery'\n  end\n  ```\n\
  \n  Install multiple packages, and specify package versions:\n\n  ``` ruby\n  powershell_package\
  \ 'Install Multiple Packages' do\n    action :install\n    package_name %w(xCertificate\
  \ xNetworking)\n    version ['2.0.0.0', '2.12.0.0']\n  end\n  ```\n\n  Install multiple\
  \ packages, specifying the package version for one\n  package but not the other:\n\
  \n  ``` ruby\n  powershell_package 'Install Multiple Packages' do\n     action :install\n\
  \     package_name %w(xCertificate xNetworking)\n     version [nil, '2.12.0.0']\n\
  \   end\n  ```\n\n  In this example, the `nil` tells `powershell_package` to install\
  \ the\n  most up to date version of `xCertificate` that is available, while\n  pinning\
  \ `xNetworking` to version 2.12.0.0.\n\n  Remove a package:\n\n  ``` ruby\n  powershell_package\
  \ 'xCertificate' do\n    action :remove\n  end\n  ```\n"

---
