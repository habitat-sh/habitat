---
resource_reference: true
common_resource_functionality_resources_common_windows_security: true
properties_shortcode: 
remote_directory_recursive_directories: true
resource_directory_recursive_directories: true
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: remote_directory resource
resource: remote_directory
aliases:
- "/resource_remote_directory.html"
menu:
  infra:
    title: remote_directory
    identifier: chef_infra/cookbook_reference/resources/remote_directory remote_directory
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **remote_directory** resource to incrementally transfer a

    directory from a cookbook to a node. The directory that is copied from

    the cookbook should be located under

    `COOKBOOK_NAME/files/default/REMOTE_DIRECTORY`. The

    **remote_directory** resource will obey file specificity.'
syntax_description: "A **remote_directory** resource block transfers a directory from\
  \ a\ncookbook to a node, and then assigns the permissions needed on that\ndirectory.\
  \ For example:\n\n``` ruby\nremote_directory '/etc/apache2' do\n  source 'apache2'\n\
  \  owner 'root'\n  group 'root'\n  mode '0755'\n  action :create\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`''/etc/apache2''` specifies the directory'
- '`source` specifies a directory in the current cookbook (use the `cookbook` property
  to specify a file that is in a different cookbook)'
- '`owner`, `group`, and `mode` define the permissions'
syntax_full_code_block: "remote_directory 'name' do\n  cookbook                  \
  \ String\n  files_backup               Integer, false # default value: 5\n  files_group\
  \                String, Integer\n  files_mode                 String, Integer #\
  \ default value: 0644 on *nix systems\n  files_owner                String, Integer\n\
  \  group                      String, Integer\n  inherits                   true,\
  \ false\n  mode                       String, Integer\n  overwrite             \
  \     true, false # default value: true\n  owner                      String, Integer\n\
  \  path                       String # default value: 'name' unless specified\n\
  \  purge                      true, false\n  recursive                  true, false\n\
  \  rights                     Hash\n  source                     String\n  action\
  \                     Symbol # defaults to :create if not specified\nend"
syntax_full_properties_list:
- "`remote_directory` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`cookbook`, `files_backup`, `files_group`, `files_mode`, `files_owner`, `group`,
  `mode`, `overwrite`, `owner`, `path`, `purge`, `recursive`, and `source` are the
  properties available to this resource."
actions_list:
  :create:
    markdown: Default. Create a directory and/or the contents of that directory. If
      a directory or its contents already exist (but does not match), update that
      directory or its contents to match.
  :create_if_missing:
    markdown: Create a directory and/or the contents of that directory, but only if
      it does not exist.
  :delete:
    markdown: Delete a directory, including the contents of that directory.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: cookbook
  ruby_type: String
  required: false
  description_list:
  - markdown: The cookbook in which a file is located (if it is not located in the
      current cookbook). The default value is the current cookbook.
- property: files_backup
  ruby_type: Integer, false
  required: false
  default_value: '5'
  description_list:
  - markdown: The number of backup copies to keep for files in the directory.
- property: files_group
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 'Configure group permissions for files. A string or ID that

      identifies the group owner by group name, including fully qualified

      group names such as `domain\group` or `group@domain`. If this value

      is not specified, existing groups remain unchanged and new group

      assignments use the default `POSIX` group (if available).'
- property: files_mode
  ruby_type: String, Integer
  required: false
  default_value: 0644 on *nix systems
  description_list:
  - markdown: 'The octal mode for a file.


      UNIX- and Linux-based systems: A quoted 3-5 character string that

      defines the octal mode that is passed to chmod. For example:

      `''755''`, `''0755''`, or `00755`. If the value is specified as a quoted

      string, it works exactly as if the `chmod` command was passed. If

      the value is specified as an integer, prepend a zero (`0`) to the

      value to ensure that it is interpreted as an octal number. For

      example, to assign read, write, and execute rights for all users,

      use `''0777''` or `''777''`; for the same rights, plus the sticky bit,

      use `01777` or `''1777''`.


      Microsoft Windows: A quoted 3-5 character string that defines the

      octal mode that is translated into rights for Microsoft Windows

      security. For example: `''755''`, `''0755''`, or `00755`. Values up to

      `''0777''` are allowed (no sticky bits) and mean the same in Microsoft

      Windows as they do in UNIX, where `4` equals `GENERIC_READ`, `2`

      equals `GENERIC_WRITE`, and `1` equals `GENERIC_EXECUTE`. This

      property cannot be used to set `:full_control`. This property has no

      effect if not specified, but when it and `rights` are both

      specified, the effects are cumulative.'
- property: files_owner
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 'Configure owner permissions for files. A string or ID that

      identifies the group owner by user name, including fully qualified

      user names such as `domain\user` or `user@domain`. If this value is

      not specified, existing owners remain unchanged and new owner

      assignments use the current user (when necessary).'
- property: group
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to configure permissions for directories. A string or ID that

      identifies the group owner by group name, including fully qualified

      group names such as `domain\group` or `group@domain`. If this value

      is not specified, existing groups remain unchanged and new group

      assignments use the default `POSIX` group (if available).'
- property: inherits
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: 'Microsoft Windows only. Whether a file inherits rights from its

      parent directory.'
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A quoted 3-5 character string that defines the octal mode. For

      example: `''755''`, `''0755''`, or `00755`. If `mode` is not specified

      and if the directory already exists, the existing mode on the

      directory is used. If `mode` is not specified, the directory does

      not exist, and the `:create` action is specified, Chef Infra Client

      assumes a mask value of `''0777''`, and then applies the umask for the

      system on which the directory is to be created to the `mask` value.

      For example, if the umask on a system is `''022''`, Chef Infra Client

      uses the default value of `''0755''`.


      The behavior is different depending on the platform.


      UNIX- and Linux-based systems: A quoted 3-5 character string that

      defines the octal mode that is passed to chmod. For example:

      `''755''`, `''0755''`, or `00755`. If the value is specified as a quoted

      string, it works exactly as if the `chmod` command was passed. If

      the value is specified as an integer, prepend a zero (`0`) to the

      value to ensure that it is interpreted as an octal number. For

      example, to assign read, write, and execute rights for all users,

      use `''0777''` or `''777''`; for the same rights, plus the sticky bit,

      use `01777` or `''1777''`.


      Microsoft Windows: A quoted 3-5 character string that defines the

      octal mode that is translated into rights for Microsoft Windows

      security. For example: `''755''`, `''0755''`, or `00755`. Values up to

      `''0777''` are allowed (no sticky bits) and mean the same in Microsoft

      Windows as they do in UNIX, where `4` equals `GENERIC_READ`, `2`

      equals `GENERIC_WRITE`, and `1` equals `GENERIC_EXECUTE`. This

      property cannot be used to set `:full_control`. This property has no

      effect if not specified, but when it and `rights` are both

      specified, the effects are cumulative.'
- property: overwrite
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: Overwrite a file when it is different.
- property: owner
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Use to configure permissions for directories. A string or ID that

      identifies the group owner by user name, including fully qualified

      user names such as `domain\user` or `user@domain`. If this value is

      not specified, existing owners remain unchanged and new owner

      assignments use the current user (when necessary).'
- property: path
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'The path to the directory. Using a fully qualified path is

      recommended, but is not always required. Default value: the `name`

      of the resource block. See "Syntax" section above for more

      information.'
- property: purge
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Purge extra files found in the target directory.
- property: recursive
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: 'Create or delete directories recursively. Chef Infra Client must be

      able to create the directory structure, including parent directories

      (if missing), as defined in

      `COOKBOOK_NAME/files/default/REMOTE_DIRECTORY`.'
- property: rights
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Microsoft Windows only. The permissions for users and groups in a

      Microsoft Windows environment. For example:

      `rights <permissions>, <principal>, <options>` where `<permissions>`

      specifies the rights granted to the principal, `<principal>` is the

      group or user name, and `<options>` is a Hash with one (or more)

      advanced rights options.'
- property: source
  ruby_type: String
  required: false
  default_value: The base portion of the 'path' property.
  new_in: null
  description_list:
  - markdown: 'The base name of the source file (and inferred from the `path`

      property). For example, in the default value, ''/some/path/'' would be

      ''path''.'
properties_shortcode: null
properties_multiple_packages: false
resource_directory_recursive_directories: true
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
remote_directory_recursive_directories: true
common_resource_functionality_resources_common_windows_security: true
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Recursively transfer a directory from a remote location\n\n  ```\
  \ ruby\n  # create up to 10 backups of the files\n  # set the files owner different\
  \ from the directory\n  remote_directory '/tmp/remote_something' do\n    source\
  \ 'something'\n    files_backup 10\n    files_owner 'root'\n    files_group 'root'\n\
  \    files_mode '0644'\n    owner 'nobody'\n    group 'nobody'\n    mode '0755'\n\
  \  end\n  ```\n\n  Use with the chef_handler resource\n\n  The following example\
  \ shows how to use the **remote_directory**\n  resource and the **chef_handler**\
  \ resource to reboot a handler named\n  `WindowsRebootHandler`:\n\n  ``` ruby\n\
  \  # the following code sample comes from the\n  # ``reboot_handler`` recipe in\
  \ the ``windows`` cookbook:\n  # https://github.com/chef-cookbooks/windows\n\n \
  \ remote_directory node['chef_handler']['handler_path'] do\n    source 'handlers'\n\
  \    recursive true\n    action :create\n  end\n\n  chef_handler 'WindowsRebootHandler'\
  \ do\n    source \"#{node['chef_handler']['handler_path']}/windows_reboot_handler.rb\"\
  \n    arguments node['windows']['allow_pending_reboots']\n    supports :report =>\
  \ true, :exception => false\n    action :enable\n  end\n  ```\n"

---
