---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: kernel_module resource
resource: kernel_module
aliases:
- "/resource_kernel_module.html"
menu:
  infra:
    title: kernel_module
    identifier: chef_infra/cookbook_reference/resources/kernel_module kernel_module
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **kernel_module** resource to manage kernel modules on Linux systems.
    This resource can load, unload, blacklist, disable, install, and uninstall modules.
resource_new_in: '14.3'
syntax_full_code_block: |-
  kernel_module 'name' do
    load_dir        String # default value: "/etc/modules-load.d"
    modname         String # default value: 'name' unless specified
    options         Array
    unload_dir      String # default value: "/etc/modprobe.d"
    action          Symbol # defaults to :install if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`kernel_module` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`load_dir`, `modname`, `options`, and `unload_dir` are the properties available
  to this resource."
actions_list:
  :blacklist:
    markdown: Blacklist a kernel module.
  :disable:
    markdown: 'Disable a kernel module


      **New in Chef Client 15.2.**'
  :install:
    markdown: Default. Load kernel module, and ensure it loads on reboot.
  :load:
    markdown: Load a kernel module.
  :uninstall:
    markdown: Unload a kernel module and remove module config, so it doesn't load
      on reboot.
  :unload:
    markdown: Unload kernel module.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: load_dir
  ruby_type: String
  required: false
  default_value: "/etc/modules-load.d"
  description_list:
  - markdown: The directory to load modules from.
- property: modname
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the kernel module name if it differs from
      the resource block's name.
- property: options
  ruby_type: Array
  required: false
  new_in: '15.4'
  description_list:
  - markdown: An optional property to set options for the kernel module.
- property: unload_dir
  ruby_type: String
  required: false
  default_value: "/etc/modprobe.d"
  description_list:
  - markdown: The modprobe.d directory.
examples: |
  Install and load a kernel module, and ensure it loads on reboot.

  ```ruby
  kernel_module 'loop'
  ```

  Install and load a kernel with a specific set of options, and ensure it loads on reboot. Consult kernel module
  documentation for specific options that are supported.

  ```ruby
  kernel_module 'loop' do
    options [
      'max_loop=4',
      'max_part=8'
    ]
  end
  ```

  Load a kernel module.

  ```ruby
  kernel_module 'loop' do
    action :load
  end
  ```

  Unload a kernel module and remove module config, so it doesn't load on reboot.

  ```ruby
  kernel_module 'loop' do
    action :uninstall
  end
  ```

  Unload kernel module.

  ```ruby
  kernel_module 'loop' do
    action :unload
  end
  ```

  Blacklist a module from loading.

  ```ruby
  kernel_module 'loop' do
    action :blacklist
  end
  ```

  Disable a kernel module.

  ```ruby
  kernel_module 'loop' do
    action :disable
  end
  ```
---
