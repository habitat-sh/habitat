---
title: mdadm resource
resource: mdadm
aliases:
- "/resource_mdadm.html"
menu:
  infra:
    title: mdadm
    identifier: chef_infra/cookbook_reference/resources/mdadm mdadm
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **mdadm** resource to manage RAID devices in a Linux environment

    using the mdadm utility. The **mdadm** resource will create and assemble

    an array, but it will not create the config file that is used to persist

    the array upon reboot. If the config file is required, it must be done

    by specifying a template with the correct array layout, and then by

    using the **mount** resource to create a file systems table (fstab)

    entry.'
resource_new_in: null
handler_types: false
syntax_description: "The mdadm resource has the following syntax:\n\n``` ruby\nmdadm\
  \ 'name' do\n  bitmap           String\n  chunk            Integer # default value:\
  \ 16\n  devices          Array\n  layout           String\n  level            Integer\
  \ # default value: 1\n  metadata         String # default value: \"0.90\"\n  raid_device\
  \      String # default value: 'name' unless specified\n  action           Symbol\
  \ # defaults to :create if not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`mdadm` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`bitmap`, `chunk`, `devices`, `layout`, `level`, `metadata`, and `raid_device`
  are the properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :assemble:
    markdown: Assemble a previously created array into an active array.
  :create:
    markdown: Default. Create an array with per-device superblocks. If an array already
      exists (but does not match), update that array to match.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :stop:
    markdown: Stop an active array.
properties_list:
- property: bitmap
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The path to a file in which a write-intent bitmap is stored.
- property: chunk
  ruby_type: Integer
  required: false
  default_value: '16'
  new_in: null
  description_list:
  - markdown: 'The chunk size. This property should not be used for a RAID 1

      mirrored pair (i.e. when the `level` property is set to `1`).'
- property: devices
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The devices to be part of a RAID array.
- property: layout
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The RAID5 parity algorithm. Possible values: `left-asymmetric` (or

      `la`), `left-symmetric` (or `ls`), `right-asymmetric` (or `ra`), or

      `right-symmetric` (or `rs`).'
- property: level
  ruby_type: Integer
  required: false
  default_value: '1'
  new_in: null
  description_list:
  - markdown: The RAID level.
- property: metadata
  ruby_type: String
  required: false
  default_value: '"0.90"'
  new_in: null
  description_list:
  - markdown: The superblock type for RAID metadata.
- property: raid_device
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'An optional property to specify the name of the RAID device if it

      differs from the resource block''s name.'
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
  Create and assemble a RAID 0 array\n\n  The mdadm command can be\
  \ used to create RAID arrays. For example, a RAID\n  0 array named `/dev/md0` with\
  \ 10 devices would have a command similar to\n  the following:\n\n  ``` bash\n \
  \ mdadm --create /dev/md0 --level=0 --raid-devices=10 /dev/s01.../dev/s10\n  ```\n\
  \n  where `/dev/s01 .. /dev/s10` represents 10 devices (01, 02, 03, and so\n  on).\
  \ This same command, when expressed as a recipe using the **mdadm**\n  resource,\
  \ would be similar to:\n\n  ``` ruby\n  mdadm '/dev/md0' do\n    devices [ '/dev/s01',\
  \ ... '/dev/s10' ]\n    level 0\n    action :create\n  end\n  ```\n\n  (again, where\
  \ `/dev/s01 .. /dev/s10` represents devices /dev/s01,\n  /dev/s02, /dev/s03, and\
  \ so on).\n\n  Create and assemble a RAID 1 array\n\n  ``` ruby\n  mdadm '/dev/md0'\
  \ do\n    devices [ '/dev/sda', '/dev/sdb' ]\n    level 1\n    action [ :create,\
  \ :assemble ]\n  end\n  ```\n\n  Create and assemble a RAID 5 array\n\n  The mdadm\
  \ command can be used to create RAID arrays. For example, a RAID\n  5 array named\
  \ `/dev/sd0` with 4, and a superblock type of `0.90` would\n  be similar to:\n\n\
  \  ``` ruby\n  mdadm '/dev/sd0' do\n    devices [ '/dev/s1', '/dev/s2', '/dev/s3',\
  \ '/dev/s4' ]\n    level 5\n    metadata '0.90'\n    chunk 32\n    action :create\n\
  \  end\n  ```\n"

---
