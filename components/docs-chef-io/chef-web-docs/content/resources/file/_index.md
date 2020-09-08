---
title: file resource
resource: file
draft: false
aliases:
- /resource_file.html
menu:
  infra:
    title: file
    identifier: chef_infra/cookbook_reference/resources/file file
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: Use the **file** resource to manage files directly on a node.
- note:
    markdown: 'Use the **cookbook_file** resource to copy a file from a cookbook''s

      `/files` directory. Use the **template** resource to create a file based

      on a template in a cookbook''s `/templates` directory. And use the

      **remote_file** resource to transfer a file to a node from a remote

      location.'
resource_new_in: null
handler_types: false
syntax_description: "A **file** resource block manages files that exist on nodes.\
  \ For\nexample, to write the home page for an Apache website:\n\n``` ruby\nfile\
  \ '/var/www/customers/public_html/index.php' do\n  content '<html>This is a placeholder\
  \ for the home page.</html>'\n  mode '0755'\n  owner 'web_admin'\n  group 'web_admin'\n\
  end\n```"
syntax_code_block: null
syntax_properties_list:
- '`''/var/www/customers/public_html/index.php''` is path to the file and also the
  filename to be managed'
- '`content` defines the contents of the file'
syntax_full_code_block: "file 'name' do\n  atomic_update              true, false\n\
  \  backup                     false, Integer\n  checksum                   String\n\
  \  content                    String\n  force_unlink               true, false\n\
  \  group                      String, Integer\n  inherits                   true,\
  \ false\n  manage_symlink_source      true, false\n  mode                      \
  \ String, Integer\n  owner                      String, Integer\n  path        \
  \               String # defaults to 'name' if not specified\n  rights         \
  \            Hash\n  verify                     String, Block\n  action        \
  \             Symbol # defaults to :create if not specified\nend"
syntax_full_properties_list:
- '`file` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`atomic_update`, `backup`, `checksum`, `content`, `force_unlink`, `group`, `inherits`,
  `manage_symlink_source`, `mode`, `owner`, `path`, `rights`, `sensitive`, and `verify`
  are properties of this resource, with the Ruby type shown. See "Properties" section
  below for more information about all of the properties that may be used with this
  resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create a file. If a file already exists (but does not match),
      update that file to match.
  :create_if_missing:
    markdown: Create a file only if the file does not exist. When the file exists,
      nothing happens.
  :delete:
    markdown: Delete a file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :touch:
    markdown: Touch a file. This updates the access (atime) and file modification
      (mtime) times for a file.
properties_list:
- property: atomic_update
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Perform atomic file updates on a per-resource basis. Set to `true`

      for atomic file updates. Set to `false` for non-atomic file updates.

      This setting overrides `file_atomic_update`, which is a global

      setting found in the client.rb file.'
- property: backup
  ruby_type: Integer, false
  required: false
  default_value: '5'
  new_in: null
  description_list:
  - markdown: 'The number of backups to be kept in `/var/chef/backup` (for UNIX-

      and Linux-based platforms) or `C:/chef/backup` (for the Microsoft

      Windows platform). Set to `false` to prevent backups from being

      kept.'
- property: checksum
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The SHA-256 checksum of the file. Use to ensure that a specific file

      is used. If the checksum does not match, the file is not used.

      Default value: no checksum required.'
- property: content
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A string that is written to the file. The contents of this property

      replace any previous content when this property has something other

      than the default value. The default behavior will not modify

      content.'
- property: force_unlink
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: 'How Chef Infra Client handles certain situations when the target

      file turns out not to be a file. For example, when a target file is

      actually a symlink. Set to `true` for Chef Infra Client delete the

      non-file target and replace it with the specified file. Set to

      `false` for Chef Infra Client to raise an error.'
- property: group
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A string or ID that identifies the group owner by group name,

      including fully qualified group names such as `domain\group` or

      `group@domain`. If this value is not specified, existing groups

      remain unchanged and new group assignments use the default `POSIX`

      group (if available).'
- property: inherits
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: 'Microsoft Windows only. Whether a file inherits rights from its

      parent directory.'
- property: manage_symlink_source
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: '(with

      warning)


      Change the behavior of the file resource if it is pointed at a

      symlink. When this value is set to `false`, Chef Infra Client will

      manage the symlink''s permissions or will replace the symlink with a

      normal file if the resource has content. When this value is set to

      `true`, Chef will follow the symlink and will manage the permissions

      and content of symlink''s target file.


      The default behavior is `true` but emits a warning that the default

      value will be changed to `false` in a future version; setting this

      explicitly to `true` or `false` suppresses this warning.'
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A quoted 3-5 character string that defines the octal mode. For

      example: `''755''`, `''0755''`, or `00755`. If `mode` is not specified

      and if the file already exists, the existing mode on the file is

      used. If `mode` is not specified, the file does not exist, and the

      `:create` action is specified, Chef Infra Client assumes a mask

      value of `''0777''` and then applies the umask for the system on which

      the file is to be created to the `mask` value. For example, if the

      umask on a system is `''022''`, Chef Infra Client uses the default

      value of `''0755''`.


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
- property: owner
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A string or ID that identifies the group owner by user name,

      including fully qualified user names such as `domain\user` or

      `user@domain`. If this value is not specified, existing owners

      remain unchanged and new owner assignments use the current user

      (when necessary).'
- property: path
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The full path to the file, including the file name and its

      extension. For example: `/files/file.txt`. Default value: the `name`

      of the resource block. See "Syntax" section above for more

      information.


      Microsoft Windows: A path that begins with a forward slash (`/`)

      will point to the root of the current working directory of Chef

      Infra Client process. This path can vary from system to system.

      Therefore, using a path that begins with a forward slash (`/`) is

      not recommended.'
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
- property: verify
  ruby_type: String, Block
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Allows verification of a file''s contents before it is created.

      Creates a temporary file and then allows execution of commands or

      Ruby code. If this code evaluates to true, the file is created. If

      the code evaluates to false, an error is raised.


      The types for this property are a block or a string. When specified

      as a block, it returns `true` or `false`. When specified as a

      string, it is executed as a system command. It evaluates to `true`

      if the command returns 0 as its exit status code and `false` if the

      command returns a non-zero exit status code.'
  - note:
    - markdown: 'A block is arbitrary Ruby defined within the resource block by using

        the `verify` property. When a block returns `true`, Chef Infra

        Client will continue to update the file as appropriate.'
  - markdown: "For example, this should return `true`:\n\n``` ruby\nfile '/tmp/baz'\
      \ do\n  verify { 1 == 1 }\nend\n```\n\nThis should also return `true`:\n\n```\
      \ ruby\nfile '/etc/nginx.conf' do\n  verify 'nginx -t -c %{path}'\nend\n```\n\
      \nIn this example, the `%{path}` portion of this command is expanded\nto the\
      \ temporary location where a copy of the file to be created\nexists. This will\
      \ use Nginx's syntax checking feature to ensure the\nfile is a valid Nginx configuration\
      \ file before writing the file. An\nerror will be raised if the executed command\
      \ returns a non-zero exit\nstatus code.\n\nThis should return `true`:\n\n```\
      \ ruby\nfile '/tmp/foo' do\n  content \"hello\"\n  verify do |path|\n    open(path).read.include?\
      \ \"hello\"\n  end\nend\n```\n\nWhereas, this should return `false`:\n\n```\
      \ ruby\nfile '/tmp/foo' do\n  content \"goodbye\"\n  verify do |path|\n    open(path).read.include?\
      \ \"hello\"\n  end\nend\n```\n\nIf a string or a block return `false`, the Chef\
      \ Infra Client run\nwill stop and an error is raised."
properties_shortcode: null
properties_multiple_packages: false
resource_directory_recursive_directories: false
resources_common_atomic_update: true
properties_resources_common_windows_security: true
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
  Create a file\n\n  ``` ruby\n  file '/tmp/something' do\n    owner\
  \ 'root'\n    group 'root'\n    mode '0755'\n    action :create\n  end\n  ```\n\n\
  \  Create a file in Microsoft Windows\n\n  To create a file in Microsoft Windows,\
  \ be sure to add an escape\n  character---`\\`---before the backslashes in the paths:\n\
  \n  ``` ruby\n  file 'C:\\\\tmp\\\\something.txt' do\n    rights :read, 'Everyone'\n\
  \    rights :full_control, 'DOMAIN\\\\User'\n    action :create\n  end\n  ```\n\n\
  \  Remove a file\n\n  ``` ruby\n  file '/tmp/something' do\n    action :delete\n\
  \  end\n  ```\n\n  Set file modes\n\n  ``` ruby\n  file '/tmp/something' do\n  \
  \  mode '0755'\n  end\n  ```\n\n  Delete a repository using yum to scrub the cache\n\
  \n  ``` ruby\n  # the following code sample thanks to gaffneyc @ https://gist.github.com/918711\n\
  \n  execute 'clean-yum-cache' do\n    command 'yum clean all'\n    action :nothing\n\
  \  end\n\n  file '/etc/yum.repos.d/bad.repo' do\n    action :delete\n    notifies\
  \ :run, 'execute[clean-yum-cache]', :immediately\n    notifies :create, 'ruby_block[reload-internal-yum-cache]',\
  \ :immediately\n  end\n  ```\n\n  Add the value of a data bag item to a file\n\n\
  \  The following example shows how to get the contents of a data bag item\n  named\
  \ `impossible_things`, create a .pem file located at\n  `some/directory/path/`,\
  \ and then use the `content` attribute to update\n  the contents of that file with\
  \ the value of the `impossible_things` data\n  bag item:\n\n  ``` ruby\n  private_key\
  \ = data_bag_item('impossible_things', private_key_name)['private_key']\n\n  file\
  \ \"some/directory/path/#{private_key_name}.pem\" do\n    content private_key\n\
  \    owner 'root'\n    group 'group'\n    mode '0755'\n  end\n  ```\n\n  Write a\
  \ YAML file\n\n  The following example shows how to use the `content` property to\
  \ write a\n  YAML file:\n\n  ``` ruby\n  file \"#{app['deploy_to']}/shared/config/settings.yml\"\
  \ do\n    owner \"app['owner']\"\n    group \"app['group']\"\n    mode '0755'\n\
  \    content app.to_yaml\n  end\n  ```\n\n  Write a string to a file\n\n  The following\
  \ example specifies a directory, and then uses the `content`\n  property to add\
  \ a string to the file created in that directory:\n\n  ``` ruby\n  status_file =\
  \ '/path/to/file/status_file'\n\n  file status_file do\n    owner 'root'\n    group\
  \ 'root'\n    mode '0755'\n    content 'My favourite foremost coastal Antarctic\
  \ shelf, oh Larsen B!'\n  end\n  ```\n\n  Create a file from a copy\n\n  The following\
  \ example shows how to copy a file from one directory to\n  another, locally on\
  \ a node:\n\n  ``` ruby\n  file '/root/1.txt' do\n    content IO.read('/tmp/1.txt')\n\
  \    action :create\n  end\n  ```\n\n  where the `content` attribute uses the Ruby\
  \ `IO.read` method to get the\n  contents of the `/tmp/1.txt` file.\n"

---
