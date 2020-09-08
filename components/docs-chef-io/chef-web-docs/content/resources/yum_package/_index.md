---
resource_reference: true
common_resource_functionality_multiple_packages: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
notes_resource_based_on_package: true
title: yum_package resource
resource: yum_package
aliases:
- "/resource_yum_package.html"
- /resource_yum.html
menu:
  infra:
    title: yum_package
    identifier: chef_infra/cookbook_reference/resources/yum_package yum_package
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **yum_package** resource to install, upgrade, and remove packages
    with Yum for the Red Hat and CentOS platforms. The yum_package resource is able
    to resolve `provides` data for packages much like Yum can do when it is run from
    the command line. This allows a variety of options for installing packages, like
    minimum versions, virtual provides, and library names.
- note:
    markdown: 'Support for using file names to install packages (as in

      `yum_package "/bin/sh"`) is not available because the volume of data

      required to parse for this is excessive.'
syntax_full_code_block: |-
  yum_package 'name' do
    allow_downgrade      true, false # default value: true
    arch                 String, Array
    flush_cache          Hash # default value: {"before"=>false, "after"=>false}
    options              String, Array
    package_name         String, Array
    source               String
    timeout              String, Integer
    version              String, Array
    yum_binary           String
    action               Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`yum_package` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`allow_downgrade`, `arch`, `flush_cache`, `options`, `package_name`, `source`,
  `timeout`, `version`, and `yum_binary` are the properties available to this resource."
actions_list:
  :install:
    markdown: Default. Install a package. If a version is specified, install the specified
      version of the package.
  :lock:
    markdown: Locks the yum package to a specific version.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :purge:
    markdown: Purge a package. This action typically removes the configuration files
      as well as the package.
  :remove:
    markdown: Remove a package.
  :unlock:
    markdown: Unlocks the yum package so that it can be upgraded to a newer version.
  :upgrade:
    markdown: Install a package and/or ensure that a package is the latest version.
      This action will ignore the `version` attribute.
properties_list:
- property: allow_downgrade
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Allow downgrading a package to satisfy requested version requirements.
- property: arch
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The architecture of the package to be installed or upgraded. This value
      can also be passed as part of the package name.
- property: flush_cache
  ruby_type: Hash
  required: false
  default_value: '{"before"=>false, "after"=>false}'
  description_list:
  - markdown: 'Flush the in-memory cache before or after a Yum operation that

      installs, upgrades, or removes a package. Accepts a Hash in the

      form: { :before =\> true/false, :after =\> true/false } or an Array

      in the form \[ :before, :after \].'
  - shortcode: resources_common_package_yum_cache.md
  - markdown: "As an array:\n\n``` ruby\nyum_package 'some-package' do\n  #...\n \
      \ flush_cache [ :before ]\n  #...\nend\n```\n\nand as a Hash:\n\n``` ruby\n\
      yum_package 'some-package' do\n  #...\n  flush_cache( { :after => true } )\n\
      \  #...\nend\n```"
  - note:
    - markdown: 'The `flush_cache` property does not flush the local Yum cache! Use

        Yum tools---`yum clean headers`, `yum clean packages`,

        `yum clean all`---to clean the local Yum cache.'
- property: options
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: One (or more) additional command options that are passed to the command.
- property: package_name
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: 'One of the following: the name of a package, the name of a package

      and its architecture, the name of a dependency. Default value: the

      `name` of the resource block. See "Syntax" section above for more

      information.'
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
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The version of a package to be installed or upgraded. This property
      is ignored when using the `:upgrade` action.
- property: yum_binary
  ruby_type: String
  required: false
  description_list:
  - markdown: The path to the yum binary.
examples: |
  **Install an exact version**:

  ``` ruby
  yum_package 'netpbm = 10.35.58-8.el8'
  ```

  **Install a minimum version**:

  ``` ruby
  yum_package 'netpbm >= 10.35.58-8.el8'
  ```

  **Install a minimum version using the default action**:

  ``` ruby
  yum_package 'netpbm'
  ```

  **Install a version without worrying about the exact release**:

  ``` ruby
  yum_package 'netpbm-10.35*'
  ```


  **To install a package**:

  ``` ruby
  yum_package 'netpbm' do
    action :install
  end
  ```

  **To install a partial minimum version**:

  ``` ruby
  yum_package 'netpbm >= 10'
  ```

  **To install a specific architecture**:

  ``` ruby
  yum_package 'netpbm' do
    arch 'i386'
  end
  ```

  or:

  ``` ruby
  yum_package 'netpbm.x86_64'
  ```

  **To install a specific version-release**

  ``` ruby
  yum_package 'netpbm' do
    version '10.35.58-8.el8'
  end
  ```

  **Handle cookbook_file and yum_package resources in the same recipe**:

  When a **cookbook_file** resource and a **yum_package** resource are
  both called from within the same recipe, use the `flush_cache` attribute
  to dump the in-memory Yum cache, and then use the repository immediately
  to ensure that the correct package is installed:

  ``` ruby
  cookbook_file '/etc/yum.repos.d/custom.repo' do
    source 'custom'
    mode '0755'
  end

  yum_package 'pkg-that-is-only-in-custom-repo' do
    action :install
    flush_cache [ :before ]
  end
  ```
---
