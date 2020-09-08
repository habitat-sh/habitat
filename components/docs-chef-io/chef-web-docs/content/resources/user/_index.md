---
title: user resource
resource: user
draft: false
aliases:
- /resource_user.html
menu:
  infra:
    title: user
    identifier: chef_infra/cookbook_reference/resources/user user
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **user** resource to add users, update existing users, remove

    users, and to lock/unlock user passwords.'
- note:
    markdown: 'System attributes are collected by Ohai at the start of every Chef
      Infra

      Client run. By design, the actions available to the **user** resource

      are processed **after** the start of a Chef Infra Client run. This means

      that system attributes added or modified by the **user** resource during

      a Chef Infra Client run must be reloaded before they can be available to

      Chef Infra Client. These system attributes can be reloaded in two ways:

      by picking up the values at the start of the (next) Chef Infra Client

      run or by using the [ohai resource](/resources/ohai/) to reload the

      system attributes during the current Chef Infra Client run.'
resource_new_in: null
handler_types: false
syntax_description: "A **user** resource block manages users on a node:\n\n``` ruby\n\
  user 'a user' do\n  comment 'A random user'\n  uid 1234\n  gid 'groupname'\n  home\
  \ '/home/random'\n  shell '/bin/bash'\n  password '$1$JJsvHslasdfjVEroftprNn4JHtDi'\n\
  end\n```"
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "user 'name' do\n  comment                    String\n  force\
  \                      true, false # see description\n  gid                    \
  \    String, Integer\n  home                       String\n  iterations        \
  \         Integer\n  manage_home                true, false\n  non_unique      \
  \           true, false\n  password                   String\n  salt           \
  \            String\n  shell                      String\n  system             \
  \        true, false\n  uid                        String, Integer\n  username \
  \                  String # defaults to 'name' if not specified\n  action      \
  \               Symbol # defaults to :create if not specified\nend"
syntax_full_properties_list:
- '`user` is the resource'
- '`name` is the name of the resource block'
- '`action` identifies the steps Chef Infra Client will take to bring the node into
  the desired state'
- '`comment`, `force`, `gid`, `home`, `iterations`, `manage_home`, `non_unique`, `password`,
  `salt`, `shell`, `system`, `uid`, and `username` are properties of this resource,
  with the Ruby type shown. See "Properties" section below for more information about
  all of the properties that may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create a user with given properties. If a user already exists
      (but does not match), update that user to match.
  :lock:
    markdown: Lock a user's password.
  :manage:
    markdown: Manage an existing user. This action does nothing if the user does not
      exist.
  :modify:
    markdown: Modify an existing user. This action raises an exception if the user
      does not exist.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a user.
  :unlock:
    markdown: Unlock a user's password.
properties_list:
- property: comment
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: One (or more) comments about the user.
- property: force
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Force the removal of a user. May be used only with the `:remove`

      action.'
  - warning:
    - markdown: 'Using this property may leave the system in an inconsistent state.

        For example, a user account will be removed even if the user is

        logged in. A user''s home directory will be removed, even if that

        directory is shared by multiple users.'
- property: gid
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The identifier for the group. This property was previously named

      `group` and both continue to function.'
- property: home
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The location of the home directory.
- property: iterations
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'macOS platform only. The number of iterations for a password with a

      SALTED-SHA512-PBKDF2 shadow hash.'
- property: manage_home
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Manage a user''s home directory.


      When used with the `:create` action, a user''s home directory is

      created based on `HOME_DIR`. If the home directory is missing, it is

      created unless `CREATE_HOME` in `/etc/login.defs` is set to `no`.

      When created, a skeleton set of files and subdirectories are

      included within the home directory.


      When used with the `:modify` action, a user''s home directory is

      moved to `HOME_DIR`. If the home directory is missing, it is created

      unless `CREATE_HOME` in `/etc/login.defs` is set to `no`. The

      contents of the user''s home directory are moved to the new location.'
- property: non_unique
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Create a duplicate (non-unique) user account.
- property: password
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The password shadow hash
- property: salt
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A SALTED-SHA512-PBKDF2 hash.
- property: shell
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The login shell.
- property: system
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Create a system user. This property may be used with `useradd` as

      the provider to create a system user which passes the `-r` flag to

      `useradd`.'
- property: uid
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The numeric user identifier.
- property: username
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The name of the user. Default value: the `name` of the resource

      block. See "Syntax" section above for more information.'
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
  Create a system user\n\n  ``` ruby\n  user 'systemguy' do\n    comment\
  \ 'system guy'\n    system true\n    shell '/bin/false'\n  end\n  ```\n\n  Create\
  \ a system user with a variable\n\n  The following example shows how to create a\
  \ system user. In this\n  instance, the `home` value is calculated and stored in\
  \ a variable called\n  `user_home` which sets the user's `home` attribute.\n\n \
  \ ``` ruby\n  user_home = \"/home/#{node['cookbook_name']['user']}\"\n\n  user node['cookbook_name']['user']\
  \ do\n    gid node['cookbook_name']['group']\n    shell '/bin/bash'\n    home user_home\n\
  \    system true\n    action :create\n  end\n  ```\n\n  Use SALTED-SHA512-PBKDF2\
  \ passwords\n\n  macOS 10.8 (and higher) calculates the password shadow hash using\n\
  \  SALTED-SHA512-PBKDF2. The length of the shadow hash value is 128 bytes,\n  the\
  \ salt value is 32 bytes, and an integer specifies the number of\n  iterations.\
  \ The following code will calculate password shadow hashes for\n  macOS 10.8 (and\
  \ higher):\n\n  ``` ruby\n  password = 'my_awesome_password'\n  salt = OpenSSL::Random.random_bytes(32)\n\
  \  iterations = 25000 # Any value above 20k should be fine.\n\n  shadow_hash = OpenSSL::PKCS5::pbkdf2_hmac(\n\
  \    password,\n    salt,\n    iterations,\n    128,\n    OpenSSL::Digest::SHA512.new\n\
  \  ).unpack('H*').first\n  salt_value = salt.unpack('H*').first\n  ```\n\n  Use\
  \ the calculated password shadow hash with the **user** resource:\n\n  ``` ruby\n\
  \  user 'my_awesome_user' do\n    password 'cbd1a....fc843'  # Length: 256\n   \
  \ salt 'bd1a....fc83'        # Length: 64\n    iterations 25000\n  end\n  ```\n"

---
