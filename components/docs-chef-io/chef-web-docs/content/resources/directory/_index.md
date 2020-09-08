---
resource_reference: true
properties_resources_common_windows_security: true
properties_shortcode: 
resource_directory_recursive_directories: true
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: directory resource
resource: directory
aliases:
- "/resource_directory.html"
menu:
  infra:
    title: directory
    identifier: chef_infra/cookbook_reference/resources/directory directory
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **directory** resource to manage a directory, which is a

    hierarchy of folders that comprises all of the information stored on a

    computer. The root directory is the top-level, under which the rest of

    the directory is organized. The **directory** resource uses the `name`

    property to specify the path to a location in a directory. Typically,

    permission to access that location in the directory is required.'
syntax_description: "A **directory** resource block declares a directory and the permissions\n\
  needed on that directory. For example:\n\n``` ruby\ndirectory '/etc/apache2' do\n\
  \  owner 'root'\n  group 'root'\n  mode '0755'\n  action :create\nend\n```"
syntax_properties_list:
- '`''/etc/apache2''` specifies the directory'
- '`owner`, `group`, and `mode` define the permissions'
syntax_full_code_block: "directory 'name' do\n  group                      String,\
  \ Integer\n  inherits                   true, false\n  mode                    \
  \   String, Integer\n  owner                      String, Integer\n  path      \
  \                 String # defaults to 'name' if not specified\n  recursive    \
  \              true, false\n  rights                     Hash\n  action        \
  \             Symbol # defaults to :create if not specified\nend"
syntax_full_properties_list:
- '`directory` is the resource.'
- '`name` is the name of the resource block; when the `path` property is not specified,
  `name` is also the path to the directory, from the root'
- '`action` identifies the steps Chef Infra Client will take to bring the node into
  the desired state'
- '`group`, `inherits`, `mode`, `owner`, `path`, `recursive`, and `rights` are properties
  of this resource, with the Ruby type shown. See "Properties" section below for more
  information about all of the properties that may be used with this resource.'
actions_list:
  :create:
    markdown: Default. Create a directory. If a directory already exists (but does
      not match), update that directory to match.
  :delete:
    markdown: Delete a directory.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
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
  default_value: The resource block's name
  description_list:
  - markdown: 'The path to the directory. Using a fully qualified path is

      recommended, but is not always required. Default value: the `name`

      of the resource block. See "Syntax" section above for more

      information.'
- property: recursive
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: 'Create parent directories recursively and delete directories,

      subdirectories, and files recursively. The `owner`, `group`, and

      `mode` properties apply only to the leaf directory.'
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
examples: "
  Create a directory\n\n  ``` ruby\n  directory '/tmp/something' do\n\
  \    owner 'root'\n    group 'root'\n    mode '0755'\n    action :create\n  end\n\
  \  ```\n\n  Create a directory in Microsoft Windows\n\n  ``` ruby\n  directory \"\
  C:\\\\tmp\\\\something\" do\n    rights :full_control, \"DOMAIN\\\\User\"\n    inherits\
  \ false\n    action :create\n  end\n  ```\n\n  or:\n\n  ``` ruby\n  directory 'C:\\\
  tmp\\something' do\n    rights :full_control, 'DOMAIN\\User'\n    inherits false\n\
  \    action :create\n  end\n  ```\n\n  <div class=\"admonition-note\">\n    <p class=\"\
  admonition-note-title\">Note</p>\n      <div class=\"admonition-note-text\">\n \
  \       <p>The difference between the two previous examples is the single- versus\
  \ double-quoted strings, where if the double quotes are used, the backslash character\
  \ (<code>\\</code>) must be escaped using the Ruby escape character (which is a\
  \ backslash).</p>\n\n      </div>\n    </div>\n\n  Create a directory recursively\n\
  \n  ``` ruby\n  %w{dir1 dir2 dir3}.each do |dir|\n    directory \"/tmp/mydirs/#{dir}\"\
  \ do\n      mode '0755'\n      owner 'root'\n      group 'root'\n      action :create\n\
  \      recursive true\n    end\n  end\n  ```\n\n  Delete a directory\n\n  ``` ruby\n\
  \  directory '/tmp/something' do\n    recursive true\n    action :delete\n  end\n\
  \  ```\n\n  Set directory permissions using a variable\n\n  The following example\
  \ shows how read/write/execute permissions can be\n  set using a variable named\
  \ `user_home`, and then for owners and groups\n  on any matching node:\n\n  ```\
  \ ruby\n  user_home = \"/#{node[:matching_node][:user]}\"\n\n  directory user_home\
  \ do\n    owner 'node[:matching_node][:user]'\n    group 'node[:matching_node][:group]'\n\
  \    mode '0755'\n    action :create\n  end\n  ```\n\n  where `matching_node` represents\
  \ a type of node. For example, if the\n  `user_home` variable specified `{node[:nginx]...}`,\
  \ a recipe might look\n  similar to:\n\n  ``` ruby\n  user_home = \"/#{node[:nginx][:user]}\"\
  \n\n  directory user_home do\n    owner 'node[:nginx][:user]'\n    group 'node[:nginx][:group]'\n\
  \    mode '0755'\n    action :create\n  end\n  ```\n\n  Set directory permissions\
  \ for a specific type of node\n\n  The following example shows how permissions can\
  \ be set for the\n  `/certificates` directory on any node that is running Nginx.\
  \ In this\n  example, permissions are being set for the `owner` and `group`\n  properties\
  \ as `root`, and then read/write permissions are granted to the\n  root.\n\n  ```\
  \ ruby\n  directory \"#{node[:nginx][:dir]}/shared/certificates\" do\n    owner\
  \ 'root'\n    group 'root'\n    mode '0755'\n    recursive true\n  end\n  ```\n\n\
  \  Reload the configuration\n\n  The following example shows how to reload the configuration\
  \ of a\n  chef-client using the **remote_file** resource to:\n\n  -   using an if\
  \ statement to check whether the plugins on a node are the\n      latest versions\n\
  \  -   identify the location from which Ohai plugins are stored\n  -   using the\
  \ `notifies` property and a **ruby_block** resource to\n      trigger an update\
  \ (if required) and to then reload the client.rb\n      file.\n\n  <!-- -->\n\n\
  \  ``` ruby\n  directory 'node[:ohai][:plugin_path]' do\n    owner 'chef'\n    recursive\
  \ true\n  end\n\n  ruby_block 'reload_config' do\n    block do\n      Chef::Config.from_file('/etc/chef/client.rb')\n\
  \    end\n    action :nothing\n  end\n\n  if node[:ohai].key?(:plugins)\n    node[:ohai][:plugins].each\
  \ do |plugin|\n      remote_file node[:ohai][:plugin_path] +\"/#{plugin}\" do\n\
  \        source plugin\n        owner 'chef'\n        notifies :run, 'ruby_block[reload_config]',\
  \ :immediately\n      end\n    end\n  end\n  ```\n\n  Manage dotfiles\n\n  The following\
  \ example shows using the **directory** and\n  **cookbook_file** resources to manage\
  \ dotfiles. The dotfiles are\n  defined by a JSON data structure similar to:\n\n\
  \  ``` javascript\n  \"files\": {\n    \".zshrc\": {\n      \"mode\": '0755',\n\
  \      \"source\": \"dot-zshrc\"\n      },\n    \".bashrc\": {\n      \"mode\":\
  \ '0755',\n      \"source\": \"dot-bashrc\"\n       },\n    \".bash_profile\": {\n\
  \      \"mode\": '0755',\n      \"source\": \"dot-bash_profile\"\n      },\n   \
  \ }\n  ```\n\n  and then the following resources manage the dotfiles:\n\n  ``` ruby\n\
  \  if u.has_key?('files')\n    u['files'].each do |filename, file_data|\n\n    directory\
  \ \"#{home_dir}/#{File.dirname(filename)}\" do\n      recursive true\n      mode\
  \ '0755'\n    end if file_data['subdir']\n\n    cookbook_file \"#{home_dir}/#{filename}\"\
  \ do\n      source \"#{u['id']}/#{file_data['source']}\"\n      owner 'u['id']'\n\
  \      group 'group_id'\n      mode 'file_data['mode']'\n      ignore_failure true\n\
  \      backup 0\n    end\n  end\n  ```\n"

---
