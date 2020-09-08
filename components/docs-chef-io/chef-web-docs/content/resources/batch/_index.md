---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: batch resource
resource: batch
aliases:
- "/resource_batch.html"
menu:
  infra:
    title: batch
    identifier: chef_infra/cookbook_reference/resources/batch batch
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **batch** resource to execute a batch script using the cmd.exe

    interpreter on Windows. The **batch** resource creates and executes a

    temporary file (similar to how the **script** resource behaves), rather

    than running the command inline. Commands that are executed with this

    resource are (by their nature) not idempotent, as they are typically

    unique to the environment in which they are run. Use `not_if` and

    `only_if` to guard this resource for idempotence.'
syntax_description: "A **batch** resource block executes a batch script using the\
  \ cmd.exe\ninterpreter:\n\n``` ruby\nbatch 'echo some env vars' do\n  code <<-EOH\n\
  \    echo %TEMP%\n    echo %SYSTEMDRIVE%\n    echo %PATH%\n    echo %WINDIR%\n \
  \ EOH\nend\n```"
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "batch 'name' do\n  architecture               Symbol\n  code\
  \                       String\n  command                    String, Array\n  creates\
  \                    String\n  cwd                        String\n  flags      \
  \                String\n  group                      String, Integer\n  guard_interpreter\
  \          Symbol\n  interpreter                String\n  returns              \
  \      Integer, Array\n  timeout                    Integer, Float\n  user     \
  \                  String\n  password                   String\n  domain       \
  \              String\n  action                     Symbol # defaults to :run if\
  \ not specified\nend"
syntax_full_properties_list:
- '`batch` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`architecture`, `code`, `command`, `creates`, `cwd`, `flags`, `group`, `guard_interpreter`,
  `interpreter`, `returns`, `timeout`, `user`, `password` and `domain` are properties
  of this resource, with the Ruby type shown. See "Properties" section below for more
  information about all of the properties that may be used with this resource.'
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :run:
    markdown: Run a batch file.
properties_list:
- property: architecture
  ruby_type: Symbol
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The architecture of the process under which a script is executed. If

      a value is not provided, Chef Infra Client defaults to the correct

      value for the architecture, as determined by Ohai. An exception is

      raised when anything other than `:i386` is specified for a 32-bit

      process. Possible values: `:i386` (for 32-bit processes) and

      `:x86_64` (for 64-bit processes).'
- property: code
  ruby_type: String
  required: true
  description_list:
  - markdown: A quoted string of code to be executed.
- property: command
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: The name of the command to be executed.
- property: creates
  ruby_type: String
  required: false
  description_list:
  - markdown: Prevent a command from creating a file when that file already exists.
- property: cwd
  ruby_type: String
  required: false
  description_list:
  - markdown: The current working directory from which the command will be run.
- property: flags
  ruby_type: String
  required: false
  description_list:
  - markdown: One or more command line flags that are passed to the interpreter when
      a command is invoked.
- property: group
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 'The group name or group ID that must be changed before running a

      command.'
- property: guard_interpreter
  ruby_type: Symbol
  required: false
  default_value: :batch
  description_list:
  - markdown: 'When this property is set to `:batch`, the 64-bit version of the

      cmd.exe shell will be used to evaluate strings values for the

      `not_if` and `only_if` properties. Set this value to `:default` to

      use the 32-bit version of the cmd.exe shell.'
- property: interpreter
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The script interpreter to use during code execution. Changing the

      default value of this property is not supported.'
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
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The user name of the user identity with which to launch the new

      process. The user name may optionally be specified with a domain,

      i.e. <span class="title-ref">domainuser</span> or <span

      class="title-ref">user@my.dns.domain.com</span> via Universal

      Principal Name (UPN)format. It can also be specified without a

      domain simply as user if the domain is instead specified using the

      <span class="title-ref">domain</span> attribute. On Windows only, if

      this property is specified, the <span

      class="title-ref">password</span> property must be specified.'
- property: password
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '*Windows only*: The password of the user specified by the <span

      class="title-ref">user</span> property. This property is mandatory

      if <span class="title-ref">user</span> is specified on Windows and

      may only be specified if <span class="title-ref">user</span> is

      specified. The <span class="title-ref">sensitive</span> property for

      this resource will automatically be set to true if password is

      specified.'
- property: domain
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '*Windows only*: The domain of the user user specified by the <span

      class="title-ref">user</span> property. If not specified, the user

      name and password specified by the <span

      class="title-ref">user</span> and <span

      class="title-ref">password</span> properties will be used to resolve

      that user against the domain in which the system running Chef Infra

      Client is joined, or if that system is not joined to a domain it

      will resolve the user as a local account on that system. An

      alternative way to specify the domain is to leave this property

      unspecified and specify the domain as part of the <span

      class="title-ref">user</span> property.'
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
  Unzip a file, and then move it\n\n  To run a batch file that unzips\
  \ and then moves Ruby, do something like:\n\n  ``` ruby\n  batch 'unzip_and_move_ruby'\
  \ do\n    code <<-EOH\n      7z.exe x #{Chef::Config[:file_cache_path]}/ruby-1.8.7-p352-i386-mingw32.7z\n\
  \        -oC:\\\\source -r -y\n      xcopy C:\\\\source\\\\ruby-1.8.7-p352-i386-mingw32\
  \ C:\\\\ruby /e /y\n    EOH\n  end\n\n  batch 'echo some env vars' do\n    code\
  \ <<-EOH\n      echo %TEMP%\n      echo %SYSTEMDRIVE%\n      echo %PATH%\n     \
  \ echo %WINDIR%\n    EOH\n  end\n  ```\n\n  or:\n\n  ``` ruby\n  batch 'unzip_and_move_ruby'\
  \ do\n    code <<-EOH\n      7z.exe x #{Chef::Config[:file_cache_path]}/ruby-1.8.7-p352-i386-mingw32.7z\n\
  \        -oC:\\\\source -r -y\n      xcopy C:\\\\source\\\\ruby-1.8.7-p352-i386-mingw32\
  \ C:\\\\ruby /e /y\n    EOH\n  end\n\n  batch 'echo some env vars' do\n    code\
  \ 'echo %TEMP%\\\\necho %SYSTEMDRIVE%\\\\necho %PATH%\\\\necho %WINDIR%'\n  end\n\
  \  ```\n\n  Run a command as an alternate user\n\n  *Note*: When Chef is running\
  \ as a service, this feature requires that\n  the user that Chef runs as has 'SeAssignPrimaryTokenPrivilege'\
  \ (aka\n  'SE_ASSIGNPRIMARYTOKEN_NAME') user right. By default only LocalSystem\n\
  \  and NetworkService have this right when running as a service. This is\n  necessary\
  \ even if the user is an Administrator.\n\n  This right can be added and checked\
  \ in a recipe using this example:\n\n  ``` ruby\n  # Add 'SeAssignPrimaryTokenPrivilege'\
  \ for the user\n  Chef::ReservedNames::Win32::Security.add_account_right('<user>',\
  \ 'SeAssignPrimaryTokenPrivilege')\n\n  # Check if the user has 'SeAssignPrimaryTokenPrivilege'\
  \ rights\n  Chef::ReservedNames::Win32::Security.get_account_right('<user>').include?('SeAssignPrimaryTokenPrivilege')\n\
  \  ```\n\n  The following example shows how to run `mkdir test_dir` from a Chef\n\
  \  Infra Client run as an alternate user.\n\n  ``` ruby\n  # Passing only username\
  \ and password\n  batch 'mkdir test_dir' do\n   code \"mkdir test_dir\"\n   cwd\
  \ Chef::Config[:file_cache_path]\n   user \"username\"\n   password \"password\"\
  \n  end\n\n  # Passing username and domain\n  batch 'mkdir test_dir' do\n   code\
  \ \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n   domain \"domain\"\
  \n   user \"username\"\n   password \"password\"\n  end\n\n  # Passing username\
  \ = 'domain-name\\\\username'. No domain is passed\n  batch 'mkdir test_dir' do\n\
  \   code \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n   user \"domain-name\\\
  \\username\"\n   password \"password\"\n  end\n\n  # Passing username = 'username@domain-name'.\
  \ No domain is passed\n  batch 'mkdir test_dir' do\n   code \"mkdir test_dir\"\n\
  \   cwd Chef::Config[:file_cache_path]\n   user \"username@domain-name\"\n   password\
  \ \"password\"\n  end\n  ```\n"

---
