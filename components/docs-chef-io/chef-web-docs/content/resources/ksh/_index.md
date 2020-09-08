---
title: ksh resource
resource: ksh
draft: false
aliases:
- /resource_ksh.html
menu:
  infra:
    title: ksh
    identifier: chef_infra/cookbook_reference/resources/ksh ksh
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **ksh** resource to execute scripts using the Korn shell (ksh)

    interpreter. This resource may also use any of the actions and

    properties that are available to the **execute** resource. Commands that

    are executed with this resource are (by their nature) not idempotent, as

    they are typically unique to the environment in which they are run. Use

    `not_if` and `only_if` to guard this resource for idempotence.'
resource_new_in: null
handler_types: false
syntax_description: "A **ksh** resource block executes scripts using ksh:\n\n``` ruby\n\
  ksh 'hello world' do\n  code <<-EOH\n    echo \"Hello world!\"\n    echo \"Current\
  \ directory: \" $cwd\n  EOH\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`code` specifies the command to run'
syntax_full_code_block: "ksh 'name' do\n  code                       String\n  creates\
  \                    String\n  cwd                        String\n  environment\
  \                Hash\n  flags                      String\n  group            \
  \          String, Integer\n  path                       Array\n  returns      \
  \              Integer, Array\n  timeout                    Integer, Float\n  user\
  \                       String, Integer\n  umask                      String, Integer\n\
  \  action                     Symbol # defaults to :run if not specified\nend"
syntax_full_properties_list:
- '`ksh` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`code`, `creates`, `cwd`, `environment`, `flags`, `group`, `path`, `returns`, `timeout`,
  `user`, and `umask` are properties of this resource, with the Ruby type shown. See
  "Properties" section below for more information about all of the properties that
  may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :nothing:
    markdown: Prevent a command from running. This action is used to specify that
      a command is run only when another resource notifies it.
  :run:
    markdown: Default. Run a script.
properties_list:
- property: code
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A quoted (" ") string of code to be executed.
- property: creates
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Prevent a command from creating a file when that file already

      exists.'
- property: cwd
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The current working directory from which the command will be run.
- property: environment
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A Hash of environment variables in the form of

      `({"ENV_VARIABLE" => "VALUE"})`. (These variables must exist for a

      command to be run successfully.)'
- property: flags
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'One or more command line flags that are passed to the interpreter

      when a command is invoked.'
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The group name or group ID that must be changed before running a

      command.'
- property: returns
  ruby_type: Integer, Array
  required: false
  default_value: '0'
  new_in: null
  description_list:
  - markdown: 'The return value for a command. This may be an array of accepted

      values. An exception is raised when the return value(s) do not

      match.'
- property: timeout
  ruby_type: Integer, Float
  required: false
  default_value: '3600'
  new_in: null
  description_list:
  - markdown: 'The amount of time (in seconds) a command is to wait before timing

      out.'
- property: user
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The user name or user ID that should be changed before running a

      command.'
- property: umask
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The file mode creation mask, or umask.
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
examples_list: null

---
