---
title: package resource
resource: package
draft: false
aliases:
- /resource_package.html
menu:
  infra:
    title: package
    identifier: chef_infra/cookbook_reference/resources/package package
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **package** resource to manage packages. When the package is

    installed from a local file (such as with RubyGems, dpkg, or RPM Package

    Manager), the file must be added to the node using the **remote_file**

    or **cookbook_file** resources.


    This resource is the base resource for several other resources used for

    package management on specific platforms. While it is possible to use

    each of these specific resources, it is recommended to use the

    **package** resource as often as possible.


    For more information about specific resources for specific platforms,

    see the following topics:


    -   [apt_package](/resources/apt_package/)

    -   [bff_package](/resources/bff_package/)

    -   [cab_package](/resources/cab_package/)

    -   [chef_gem](/resources/chef_gem/)

    -   [chocolatey_package](/resources/chocolatey_package/)

    -   [dmg_package](/resources/dmg_package/)

    -   [dnf_package](/resources/dnf_package/)

    -   [dpkg_package](/resources/dpkg_package/)

    -   [freebsd_package](/resources/freebsd_package/)

    -   [gem_package](/resources/gem_package/)

    -   [homebrew_package](/resources/homebrew_package/)

    -   [ips_package](/resources/ips_package/)

    -   [macports_package](/resources/macports_package/)

    -   [msu_package](/resources/msu_package/)

    -   [openbsd_package](/resources/openbsd_package/)

    -   [pacman_package](/resources/pacman_package/)

    -   [paludis_package](/resources/paludis_package/)

    -   [portage_package](/resources/portage_package/)

    -   [rpm_package](/resources/rpm_package/)

    -   [smartos_package](/resources/smartos_package/)

    -   [snap_package](/resources/snap_package/)

    -   [solaris_package](/resources/solaris_package/)

    -   [windows_package](/resources/windows_package/)

    -   [yum_package](/resources/yum_package/)

    -   [zypper_package](/resources/zypper_package/)'
resource_new_in: null
handler_types: false
syntax_description: "A **package** resource block manages a package on a node, typically\
  \ by\ninstalling it. The simplest use of the **package** resource is:\n\n``` ruby\n\
  package 'httpd'\n```\n\nwhich will install Apache using all of the default options\
  \ and the\ndefault action (`:install`).\n\nFor a package that has different package\
  \ names, depending on the\nplatform, use a `case` statement within the **package**:\n\
  \n``` ruby\npackage 'Install Apache' do\n  case node[:platform]\n  when 'redhat',\
  \ 'centos'\n    package_name 'httpd'\n  when 'ubuntu', 'debian'\n    package_name\
  \ 'apache2'\n  end\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`''redhat'', ''centos''` will install Apache using the `httpd` package

  and `''ubuntu'', ''debian''` will install it using the `apache2` package'
syntax_full_code_block: "package 'name' do\n  allow_downgrade            true, false\
  \ # Yum, RPM packages only\n  arch                       String, Array # Yum packages\
  \ only\n  default_release            String # Apt packages only\n  flush_cache \
  \               Array\n  gem_binary                 String\n  homebrew_user    \
  \          String, Integer # Homebrew packages only\n  notifies                \
  \   # see description\n  options                    String\n  package_name     \
  \          String, Array # defaults to 'name' if not specified\n  response_file\
  \              String # Apt packages only\n  response_file_variables    Hash # Apt\
  \ packages only\n  source                     String\n  subscribes             \
  \    # see description\n  timeout                    String, Integer\n  version\
  \                    String, Array\n  action                     Symbol # defaults\
  \ to :install if not specified\nend"
syntax_full_properties_list:
- '`package` tells Chef Infra Client to manage a package; Chef Infra Client will determine
  the correct package provider to use based on the platform running on the node'
- '`''name''` is the name of the package'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state'
- '`allow_downgrade`, `arch`, `default_release`, `flush_cache`, `gem_binary`, `homebrew_user`,
  `options`, `package_name`, `response_file`, `response_file_variables`, `source`,
  `recursive`, `timeout`, and `version` are properties of this resource, with the
  Ruby type shown. See "Properties" section below for more information about all of
  the properties that may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: true
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :purge:
    markdown: Purge a package. This action typically removes the configuration files
      as well as the package. (Debian platform only; for other platforms, use the
      `:remove` action.)
  :reconfig:
    markdown: Reconfigure a package. This action requires a response file.
  :remove:
    markdown: Remove a package.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.
properties_list:
- property: allow_downgrade
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: '**yum_package** resource only. Downgrade a package to satisfy

      requested version requirements.'
- property: arch
  ruby_type: String, Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '**yum_package** resource only. The architecture of the package to

      be installed or upgraded. This value can also be passed as part of

      the package name.'
- property: default_release
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '**apt_package** resource only. The default release. For example:

      `stable`.'
- property: flush_cache
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Flush the in-memory cache before or after a Yum operation that

      installs, upgrades, or removes a package. Default value:

      `[ :before, :after ]`. The value may also be a Hash:

      `( { :before => true/false, :after => true/false } )`.'
  - shortcode: resources_common_package_yum_cache.md
  - markdown: "As an array:\n\n``` ruby\nyum_package 'some-package' do\n  #...\n \
      \ flush_cache [ :before ]\n  #...\nend\n```\n\nand as a Hash:\n\n``` ruby\n\
      yum_package 'some-package' do\n  #...\n  flush_cache( { :after => true } )\n\
      \  #...\nend\n```"
  - note:
    - markdown: 'The `flush_cache` property does not flush the local Yum cache! Use

        Yum tools---`yum clean headers`, `yum clean packages`,

        `yum clean all`---to clean the local Yum cache.'
- property: gem_binary
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A property for the `gem_package` provider that is used to specify a

      gems binary.'
- property: homebrew_user
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '**homebrew_package** resource only. The name of the Homebrew owner

      to be used by Chef Infra Client when executing a command.'
- property: ignore_failure
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Continue running a recipe if a resource fails for any reason.
- property: notifies
  ruby_type: Symbol, Chef::Resource\[String\]
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: resources_common_notification_notifies.md
  - markdown: ''
  - shortcode: resources_common_notification_timers.md
  - markdown: ''
  - shortcode: resources_common_notification_notifies_syntax.md
- property: options
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: One (or more) additional options that are passed to the command.
- property: package_name
  ruby_type: String, Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The name of the package. Default value: the `name` of the resource

      block. See "Syntax" section above for more information.'
- property: response_file
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '**apt_package** and **dpkg_package** resources only. The direct

      path to the file used to pre-seed a package.'
- property: response_file_variables
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '**apt_package** and **dpkg_package** resources only. A Hash of

      response file variables in the form of `{"VARIABLE" => "VALUE"}`.'
- property: retries
  ruby_type: Integer
  required: false
  default_value: '0'
  new_in: null
  description_list:
  - markdown: The number of attempts to catch exceptions and retry the resource.
- property: retry_delay
  ruby_type: Integer
  required: false
  default_value: '2'
  new_in: null
  description_list:
  - markdown: The retry delay (in seconds).
- property: source
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Optional. The path to a package in the local file system.
  - note:
    - markdown: 'The AIX platform requires `source` to be a local file system path

        because `installp` does not retrieve packages using HTTP or FTP.'
- property: subscribes
  ruby_type: Symbol, Chef::Resource\[String\]
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: resources_common_notification_subscribes.md
  - markdown: ''
  - shortcode: resources_common_notification_timers.md
  - markdown: ''
  - shortcode: resources_common_notification_subscribes_syntax.md
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
properties_multiple_packages: true
resource_directory_recursive_directories: false
resources_common_atomic_update: false
properties_resources_common_windows_security: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
ps_credential_helper: false
ruby_style_basics_chef_log: false
debug_recipes_chef_shell: false
template_requirements: false
resources_common_properties: false
resources_common_notification: false
resources_common_guards: false
common_resource_functionality_multiple_packages: null
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Install a gems file for use in recipes\n\n  ``` ruby\n  chef_gem\
  \ 'right_aws' do\n    action :install\n  end\n\n  require 'right_aws'\n  ```\n\n\
  \  Install a gems file from the local file system\n\n  ``` ruby\n  gem_package 'right_aws'\
  \ do\n    source '/tmp/right_aws-1.11.0.gem'\n    action :install\n  end\n  ```\n\
  \n  Install a package\n\n  ``` ruby\n  package 'tar' do\n    action :install\n \
  \ end\n  ```\n\n  Install a package version\n\n  ``` ruby\n  package 'tar' do\n\
  \    version '1.16.1-1'\n    action :install\n  end\n  ```\n\n  Install a package\
  \ with options\n\n  ``` ruby\n  package 'debian-archive-keyring' do\n    action\
  \ :install\n    options '--force-yes'\n  end\n  ```\n\n  Install a package with\
  \ a response_file\n\n  Use of a `response_file` is only supported on Debian and\
  \ Ubuntu at this\n  time. Custom resources must be written to support the use of\
  \ a\n  `response_file`, which contains debconf answers to questions normally\n \
  \ asked by the package manager on installation. Put the file in\n  `/files/default`\
  \ of the cookbook where the package is specified and Chef\n  Infra Client will use\
  \ the **cookbook_file** resource to retrieve it.\n\n  To install a package with\
  \ a `response_file`:\n\n  ``` ruby\n  package 'sun-java6-jdk' do\n    response_file\
  \ 'java.seed'\n  end\n  ```\n\n  Install a specified architecture using a named\
  \ provider\n\n  ``` ruby\n  yum_package 'glibc-devel' do\n    arch 'i386'\n  end\n\
  \  ```\n\n  Purge a package\n\n  ``` ruby\n  package 'tar' do\n    action :purge\n\
  \  end\n  ```\n\n  Remove a package\n\n  ``` ruby\n  package 'tar' do\n    action\
  \ :remove\n  end\n  ```\n\n  Upgrade a package\n\n  ``` ruby\n  package 'tar' do\n\
  \    action :upgrade\n  end\n  ```\n\n  Use the ignore_failure common attribute\n\
  \n  ``` ruby\n  gem_package 'syntax' do\n    action :install\n    ignore_failure\
  \ true\n  end\n  ```\n\n  Avoid unnecessary string interpolation\n\n  Do this:\n\
  \n  ``` ruby\n  package 'mysql-server' do\n    version node['mysql']['version']\n\
  \    action :install\n  end\n  ```\n\n  and not this:\n\n  ``` ruby\n  package 'mysql-server'\
  \ do\n    version \"#{node['mysql']['version']}\"\n    action :install\n  end\n\
  \  ```\n\n  Install a package in a platform\n\n  The following example shows how\
  \ to use the **package** resource to\n  install an application named `app` and ensure\
  \ that the correct packages\n  are installed for the correct platform:\n\n  ```\
  \ ruby\n  package 'app_name' do\n    action :install\n  end\n\n  case node[:platform]\n\
  \  when 'ubuntu','debian'\n    package 'app_name-doc' do\n      action :install\n\
  \    end\n  when 'centos'\n    package 'app_name-html' do\n      action :install\n\
  \    end\n  end\n  ```\n\n  **Install sudo, then configure /etc/sudoers/ file**\n\
  \n  The following example shows how to install sudo and then configure the\n  `/etc/sudoers`\
  \ file:\n\n  ``` ruby\n  #  the following code sample comes from the ``default``\
  \ recipe in the ``sudo`` cookbook: https://github.com/chef-cookbooks/sudo\n\n  package\
  \ 'sudo' do\n    action :install\n  end\n\n  if node['authorization']['sudo']['include_sudoers_d']\n\
  \    directory '/etc/sudoers.d' do\n      mode        '0755'\n      owner      \
  \ 'root'\n      group       'root'\n      action      :create\n    end\n\n    cookbook_file\
  \ '/etc/sudoers.d/README' do\n      source      'README'\n      mode        '0440'\n\
  \      owner       'root'\n      group       'root'\n      action      :create\n\
  \    end\n  end\n\n  template '/etc/sudoers' do\n    source 'sudoers.erb'\n    mode\
  \ '0440'\n    owner 'root'\n    group platform?('freebsd') ? 'wheel' : 'root'\n\
  \    variables(\n      :sudoers_groups => node['authorization']['sudo']['groups'],\n\
  \      :sudoers_users => node['authorization']['sudo']['users'],\n      :passwordless\
  \ => node['authorization']['sudo']['passwordless']\n    )\n  end\n  ```\n\n  where\n\
  \n  -   the **package** resource is used to install sudo\n  -   the `if` statement\
  \ is used to ensure availability of the\n      `/etc/sudoers.d` directory\n  - \
  \  the **template** resource tells Chef Infra Client where to find the\n      `sudoers`\
  \ template\n  -   the `variables` property is a hash that passes values to template\n\
  \      files (that are located in the `templates/` directory for the\n      cookbook\n\
  \n  Use a case statement to specify the platform\n\n  The following example shows\
  \ how to use a case statement to tell Chef\n  Infra Client which platforms and packages\
  \ to install using cURL.\n\n  ``` ruby\n  package 'curl'\n    case node[:platform]\n\
  \    when 'redhat', 'centos'\n      package 'package_1'\n      package 'package_2'\n\
  \      package 'package_3'\n    when 'ubuntu', 'debian'\n      package 'package_a'\n\
  \      package 'package_b'\n      package 'package_c'\n    end\n  end\n  ```\n\n\
  \  where `node[:platform]` for each node is identified by Ohai during every\n  Chef\
  \ Infra Client run. For example:\n\n  ``` ruby\n  package 'curl'\n    case node[:platform]\n\
  \    when 'redhat', 'centos'\n      package 'zlib-devel'\n      package 'openssl-devel'\n\
  \      package 'libc6-dev'\n    when 'ubuntu', 'debian'\n      package 'openssl'\n\
  \      package 'pkg-config'\n      package 'subversion'\n    end\n  end\n  ```\n\
  \n  Use symbols to reference attributes\n\n  Symbols may be used to reference attributes:\n\
  \n  ``` ruby\n  package 'mysql-server' do\n    version node[:mysql][:version]\n\
  \    action :install\n  end\n  ```\n\n  instead of strings:\n\n  ``` ruby\n  package\
  \ 'mysql-server' do\n    version node['mysql']['version']\n    action :install\n\
  \  end\n  ```\n\n  Use a whitespace array to simplify a recipe\n\n  The following\
  \ examples show different ways of doing the same thing. The\n  first shows a series\
  \ of packages that will be upgraded:\n\n  ``` ruby\n  package 'package-a' do\n \
  \   action :upgrade\n  end\n\n  package 'package-b' do\n    action :upgrade\n  end\n\
  \n  package 'package-c' do\n    action :upgrade\n  end\n\n  package 'package-d'\
  \ do\n    action :upgrade\n  end\n  ```\n\n  and the next uses a single **package**\
  \ resource and a whitespace array\n  (`%w`):\n\n  ``` ruby\n  package %w{package-a\
  \ package-b package-c package-d} do\n    action :upgrade\n  end\n  ```\n\n  Specify\
  \ the Homebrew user with a UUID\n\n  ``` ruby\n  homebrew_package 'emacs' do\n \
  \   homebrew_user 1001\n  end\n  ```\n\n  Specify the Homebrew user with a string\n\
  \n  ``` ruby\n  homebrew_package 'vim' do\n    homebrew_user 'user1'\n  end\n  ```\n"

---
