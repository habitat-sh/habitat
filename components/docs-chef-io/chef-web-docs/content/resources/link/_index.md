---
title: link resource
resource: link
draft: false
aliases:
- /resource_link.html
menu:
  infra:
    title: link
    identifier: chef_infra/cookbook_reference/resources/link link
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **link** resource to create symbolic or hard links.


    A symbolic link---sometimes referred to as a soft link---is a directory

    entry that associates a file name with a string that contains an

    absolute or relative path to a file on any file system. In other words,

    "a file that contains a path that points to another file." A symbolic

    link creates a new file with a new inode that points to the inode

    location of the original file.


    A hard link is a directory entry that associates a file with another

    file in the same file system. In other words, "multiple directory

    entries to the same file." A hard link creates a new file that points to

    the same inode as the original file. On Windows, this resource can be

    used to create directory junction/reparse points.'
resource_new_in: null
handler_types: false
syntax_description: "A **link** resource block creates symbolic or hard links. For\
  \ example,\nto create a hard link from `/tmp/file` to `/etc/file`:\n\n``` ruby\n\
  link '/tmp/file' do\n  to '/etc/file'\n  link_type :hard\nend\n```\n\nBecause the\
  \ default value for `link_type` is symbolic, and because\nproperties that are not\
  \ specified in the resource block will be assigned\ntheir default values, the following\
  \ example creates a symbolic link:\n\n``` ruby\nlink '/tmp/file' do\n  to '/etc/file'\n\
  end\n```"
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "link 'name' do\n  group            String, Integer\n  link_type\
  \        String, Symbol # default value: :symbolic\n  mode             Integer,\
  \ String\n  owner            String, Integer\n  target_file      String # default\
  \ value: 'name' unless specified\n  to               String\n  action          \
  \ Symbol # defaults to :create if not specified\nend"
syntax_full_properties_list:
- '`link` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`group`, `link_type`, `mode`, `owner`, `target_file`, and `to` are properties of
  this resource, with the Ruby type shown. See "Properties" section below for more
  information about all of the properties that may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create a link. If a link already exists (but does not match),
      update that link to match.
  :delete:
    markdown: Delete a link.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A group name or ID number that identifies the group associated with

      a symbolic link.'
- property: link_type
  ruby_type: String, Symbol
  required: false
  default_value: :symbolic
  new_in: null
  allowed_values: ':symbolic :hard'
  description_list:
  - markdown: 'The type of link: `:symbolic` or `:hard`. On Windows, `:symbolic` will create a junction point if the target is a directory.'
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: '777'
  new_in: null
  description_list:
  - markdown: 'If `mode` is not specified and if the file already exists, the

      existing mode on the file is used. If `mode` is not specified, the

      file does not exist, and the `:create` action is specified, Chef

      Infra Client assumes a mask value of `''0777''` and then applies the

      umask for the system on which the file is to be created to the

      `mask` value. For example, if the umask on a system is `''022''`, Chef

      Infra Client uses the default value of `''0755''`.


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
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The owner associated with a symbolic link.
- property: target_file
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'An optional property to set the target file if it differs from the

      resource block''s name.'
- property: to
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The actual file to which the link is to be created.
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
  Create symbolic links\n\n  The following example will create a symbolic\
  \ link from `/tmp/file` to\n  `/etc/file`:\n\n  ``` ruby\n  link '/tmp/file' do\n\
  \    to '/etc/file'\n  end\n  ```\n\n  Create hard links\n\n  The following example\
  \ will create a hard link from `/tmp/file` to\n  `/etc/file`:\n\n  ``` ruby\n  link\
  \ '/tmp/file' do\n    to '/etc/file'\n    link_type :hard\n  end\n  ```\n\n  Delete\
  \ links\n\n  The following example will delete the `/tmp/file` symbolic link and\
  \ uses\n  the `only_if` guard to run the `test -L` command, which verifies that\n\
  \  `/tmp/file` is a symbolic link, and then only deletes `/tmp/file` if the\n  test\
  \ passes:\n\n  ``` ruby\n  link '/tmp/file' do\n    action :delete\n    only_if\
  \ 'test -L /tmp/file'\n  end\n  ```\n\n  Create multiple symbolic links\n\n  The\
  \ following example creates symbolic links from two files in the\n  `/vol/webserver/cert/`\
  \ directory to files located in the\n  `/etc/ssl/certs/` directory:\n\n  ``` ruby\n\
  \  link '/vol/webserver/cert/server.crt' do\n    to '/etc/ssl/certs/ssl-cert-name.pem'\n\
  \  end\n\n  link '/vol/webserver/cert/server.key' do\n    to '/etc/ssl/certs/ssl-cert-name.key'\n\
  \  end\n  ```\n\n  Create platform-specific symbolic links\n\n  The following example\
  \ shows installing a filter module on Apache. The\n  package name is different for\
  \ different platforms, and for the Red Hat\n  Enterprise Linux family, a symbolic\
  \ link is required:\n\n  ``` ruby\n  include_recipe 'apache2::default'\n\n  case\
  \ node['platform_family']\n  when 'debian'\n    ...\n  when 'suse'\n    ...\n  when\
  \ 'rhel', 'fedora'\n    ...\n\n    link '/usr/lib64/httpd/modules/mod_apreq.so'\
  \ do\n      to      '/usr/lib64/httpd/modules/mod_apreq2.so'\n      only_if 'test\
  \ -f /usr/lib64/httpd/modules/mod_apreq2.so'\n    end\n\n    link '/usr/lib/httpd/modules/mod_apreq.so'\
  \ do\n      to      '/usr/lib/httpd/modules/mod_apreq2.so'\n      only_if 'test\
  \ -f /usr/lib/httpd/modules/mod_apreq2.so'\n    end\n  end\n\n  ...\n  ```\n\n \
  \ For the complete recipe, see\n  <https://github.com/onehealth-cookbooks/apache2/blob/68bdfba4680e70b3e90f77e40223dd535bf22c17/recipes/mod_apreq2.rb>.\n\
  \n  **Create Windows junction/reparse points**\n\n  This example demonstrates how\
  \ to create a directory junction/reparse\n  point. In this example, `C:\\destination`\
  \ will be a junction/reparse\n  point to the `C:\\source` directory.\n\n  ``` ruby\n\
  \  directory 'C:/source'\n\n  link 'C:/destination' do\n      link_type :symbolic\n\
  \      to 'C:/source'\n  end\n  ```\n"

---
