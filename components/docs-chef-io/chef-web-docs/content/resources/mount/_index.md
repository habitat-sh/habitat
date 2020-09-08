---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: mount resource
resource: mount
aliases:
- "/resource_mount.html"
menu:
  infra:
    title: mount
    identifier: chef_infra/cookbook_reference/resources/mount mount
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **mount** resource to manage a mounted file system.
syntax_full_code_block: |-
  mount 'name' do
    device           String
    device_type      String, Symbol # default value: :device
    domain           String
    dump             Integer, false # default value: 0
    enabled          true, false # default value: false
    fsck_device      String # default value: "-"
    fstype           String # default value: "auto"
    mount_point      String # default value: 'name' unless specified
    options          Array, String # default value: ["defaults"]
    pass             Integer, false # default value: 2
    password         String
    supports         Array, Hash
    username         String
    action           Symbol # defaults to :mount if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`mount` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`device`, `device_type`, `domain`, `dump`, `enabled`, `fsck_device`, `fstype`,
  `mount_point`, `options`, `pass`, `password`, `supports`, and `username` are the
  properties available to this resource."
actions_list:
  :disable:
    markdown: Remove an entry from the file systems table (`fstab`).
  :enable:
    markdown: Add an entry to the file systems table (`fstab`).
  :mount:
    markdown: Default. Mount a device.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remount:
    markdown: Remount a device.
  :umount:
    markdown: Unmount a device.
  :unmount:
    markdown: Alias for `:umount` action.
properties_list:
- property: device
  ruby_type: String
  required: false
  description_list:
  - markdown: 'Required for `:umount` and `:remount` actions (for the purpose of

      checking the mount command output for presence). The special block

      device or remote node, a label, or a uuid to be mounted.'
- property: device_type
  ruby_type: String, Symbol
  required: false
  default_value: ":device"
  allowed_values: ":device, :label, :uuid"
  description_list:
  - markdown: 'The type of device: :device, :label, or :uuid'
- property: domain
  ruby_type: String
  required: false
  description_list:
  - markdown: 'Windows only: Use to specify the domain in which the `username` and

      `password` are located.'
- property: dump
  ruby_type: Integer, false
  required: false
  default_value: '0'
  description_list:
  - markdown: The dump frequency (in days) used while creating a file systems table
      (fstab) entry.
- property: enabled
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Use to specify if a mounted file system is enabled.
- property: fsck_device
  ruby_type: String
  required: false
  default_value: "-"
  description_list:
  - markdown: 'Solaris only: The fsck device.'
- property: fstype
  ruby_type: String
  required: false
  default_value: auto
  description_list:
  - markdown: The file system type (fstype) of the device.
- property: mount_point
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The directory (or path) in which the device is to be mounted. Defaults
      to the name of the resource block if not provided.
- property: options
  ruby_type: Array, String
  required: false
  default_value: '["defaults"]'
  description_list:
  - markdown: An array or comma separated list of options for the mount.
- property: pass
  ruby_type: Integer, false
  required: false
  default_value: '2'
  description_list:
  - markdown: The pass number used by the file system check (fsck) command while creating
      a file systems table (fstab) entry.
- property: password
  ruby_type: String
  required: false
  description_list:
  - markdown: Windows only:. Use to specify the password for username.
- property: supports
  ruby_type: Array, Hash
  required: false
  description_list:
  - markdown: 'Specify a Hash of supported mount features. Default value:

      `remount: false` (preferred). Array defaults to `remount: true`

      (non-preferred).'
- property: username
  ruby_type: String
  required: false
  description_list:
  - markdown: 'Windows only: Use to specify the user name.'
examples: "
  Mount a labeled file system\n\n  ``` ruby\n  mount '/mnt/volume1'\
  \ do\n    device 'volume1'\n    device_type :label\n    fstype 'xfs'\n    options\
  \ 'rw'\n  end\n  ```\n\n  Mount a local block drive\n\n  ``` ruby\n  mount '/mnt/local'\
  \ do\n    device '/dev/sdb1'\n    fstype 'ext3'\n  end\n  ```\n\n  Mount a non-block\
  \ file system\n\n  ``` ruby\n  mount '/mount/tmp' do\n    pass     0\n    fstype\
  \   'tmpfs'\n    device   '/dev/null'\n    options  'nr_inodes=999k,mode=755,size=500m'\n\
  \    action   [:mount, :enable]\n  end\n  ```\n\n  Mount and add to the file systems\
  \ table\n\n  ``` ruby\n  mount '/export/www' do\n    device 'nas1prod:/export/web_sites'\n\
  \    fstype 'nfs'\n    options 'rw'\n    action [:mount, :enable]\n  end\n  ```\n\
  \n  Mount a remote file system\n\n  ``` ruby\n  mount '/export/www' do\n    device\
  \ 'nas1prod:/export/web_sites'\n    fstype 'nfs'\n    options 'rw'\n  end\n  ```\n\
  \n  Mount a remote folder in Microsoft Windows\n\n  ``` ruby\n  mount 'T:' do\n\
  \    action :mount\n    device '\\\\\\\\hostname.example.com\\\\folder'\n  end\n\
  \  ```\n\n  Unmount a remote folder in Microsoft Windows\n\n  ``` ruby\n  mount\
  \ 'T:' do\n    action :umount\n    device '\\\\\\\\hostname.example.com\\\\D$'\n\
  \  end\n  ```\n\n  Stop a service, do stuff, and then restart it\n\n  The following\
  \ example shows how to use the **execute**, **service**, and\n  **mount** resources\
  \ together to ensure that a node running on Amazon EC2\n  is running MySQL. This\
  \ example does the following:\n\n  -   Checks to see if the Amazon EC2 node has\
  \ MySQL\n  -   If the node has MySQL, stops MySQL\n  -   Installs MySQL\n  -   Mounts\
  \ the node\n  -   Restarts MySQL\n\n  <!-- -->\n\n  ``` ruby\n  # the following\
  \ code sample comes from the ``server_ec2``\n  # recipe in the following cookbook:\n\
  \  # https://github.com/chef-cookbooks/mysql\n\n  if (node.attribute?('ec2') &&\
  \ ! FileTest.directory?(node['mysql']['ec2_path']))\n\n    service 'mysql' do\n\
  \      action :stop\n    end\n\n    execute 'install-mysql' do\n      command \"\
  mv #{node['mysql']['data_dir']} #{node['mysql']['ec2_path']}\"\n      not_if do\
  \ FileTest.directory?(node['mysql']['ec2_path']) end\n    end\n\n    [node['mysql']['ec2_path'],\
  \ node['mysql']['data_dir']].each do |dir|\n      directory dir do\n        owner\
  \ 'mysql'\n        group 'mysql'\n      end\n    end\n\n    mount node['mysql']['data_dir']\
  \ do\n      device node['mysql']['ec2_path']\n      fstype 'none'\n      options\
  \ 'bind,rw'\n      action [:mount, :enable]\n    end\n\n    service 'mysql' do\n\
  \      action :start\n    end\n\n  end\n  ```\n\n  where\n\n  -   the two **service**\
  \ resources are used to stop, and then restart the\n      MySQL service\n  -   the\
  \ **execute** resource is used to install MySQL\n  -   the **mount** resource is\
  \ used to mount the node and enable MySQL\n"

---
