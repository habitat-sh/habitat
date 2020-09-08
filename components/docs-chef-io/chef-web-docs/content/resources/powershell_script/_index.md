---
title: powershell_script resource
resource: powershell_script
draft: false
aliases:
- /resource_powershell_script.html
menu:
  infra:
    title: powershell_script
    identifier: chef_infra/cookbook_reference/resources/powershell_script powershell_script
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **powershell_script** resource to execute a script using the

    Windows PowerShell interpreter, much like how the **script** and

    **script**-based resources---**bash**, **csh**, **perl**, **python**,

    and **ruby**---are used. The **powershell_script** is specific to the

    Microsoft Windows platform and the Windows PowerShell interpreter.


    The **powershell_script** resource creates and executes a temporary

    file (similar to how the **script** resource behaves), rather than

    running the command inline. Commands that are executed with this

    resource are (by their nature) not idempotent, as they are typically

    unique to the environment in which they are run. Use `not_if` and

    `only_if` to guard this resource for idempotence.'
resource_new_in: null
handler_types: false
syntax_description: "A **powershell_script** resource block executes a batch script\
  \ using\nthe Windows PowerShell interpreter. For example, writing to an\ninterpolated\
  \ path:\n\n``` ruby\npowershell_script 'write-to-interpolated-path' do\n  code <<-EOH\n\
  \  $stream = [System.IO.StreamWriter] \"#{Chef::Config[:file_cache_path]}/powershell-test.txt\"\
  \n  $stream.WriteLine(\"In #{Chef::Config[:file_cache_path]}...word.\")\n  $stream.close()\n\
  \  EOH\nend\n```"
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "powershell_script 'name' do\n  architecture             \
  \  Symbol\n  code                       String\n  command                    String,\
  \ Array\n  convert_boolean_return     true, false\n  creates                   \
  \ String\n  cwd                        String\n  environment                Hash\n\
  \  flags                      String\n  group                      String, Integer\n\
  \  guard_interpreter          Symbol\n  interpreter                String\n  returns\
  \                    Integer, Array\n  timeout                    Integer, Float\n\
  \  user                       String\n  password                   String\n  domain\
  \                     String\n  action                     Symbol # defaults to\
  \ :run if not specified\n  elevated                   true, false\nend"
syntax_full_properties_list:
- '`powershell_script` is the resource.'
- '`name` is the name given to the resource block.'
- '`command` is the command to be run and `cwd` is the location from which the command
  is run.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`architecture`, `code`, `command`, `convert_boolean_return`, `creates`, `cwd`,
  `environment`, `flags`, `group`, `guard_interpreter`, `interpreter`, `returns`,
  `sensitive`, `timeout`, `user`, `password`, `domain` and `elevated` are properties
  of this resource, with the Ruby type shown. See "Properties" section below for more
  information about all of the properties that may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :nothing:
    markdown: Inherited from **execute** resource. Prevent a command from running.
      This action is used to specify that a command is run only when another resource
      notifies it.
  :run:
    markdown: Default. Run the script.
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
  default_value: null
  new_in: null
  description_list:
  - markdown: A quoted (" ") string of code to be executed.
- property: command
  ruby_type: String, Array
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'An optional property to set the command to be executed if it differs

      from the resource block''s name.'
- property: convert_boolean_return
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: "Return `0` if the last line of a command is evaluated to be true or\n\
      to return `1` if the last line is evaluated to be false.\n\nWhen the `guard_interpreter`\
      \ common attribute is set to\n`:powershell_script`, a string command will be\
      \ evaluated as if this\nvalue were set to `true`. This is because the behavior\
      \ of this\nattribute is similar to the value of the `\"$?\"` expression common\
      \ in\nUNIX interpreters. For example, this:\n\n``` ruby\npowershell_script 'make_safe_backup'\
      \ do\n  guard_interpreter :powershell_script\n  code 'cp ~/data/nodes.json ~/data/nodes.bak'\n\
      \  not_if 'test-path ~/data/nodes.bak'\nend\n```\n\nis similar to:\n\n``` ruby\n\
      bash 'make_safe_backup' do\n  code 'cp ~/data/nodes.json ~/data/nodes.bak'\n\
      \  not_if 'test -e ~/data/nodes.bak'\nend\n```"
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
  - markdown: 'A Hash of environment variables in the form of ({''ENV_VARIABLE'' =\>

      ''VALUE''}).'
- property: flags
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A string that is passed to the Windows PowerShell command. Default

      value (Windows PowerShell 3.0+):

      `-NoLogo, -NonInteractive, -NoProfile, -ExecutionPolicy Bypass, -InputFormat
      None`.'
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The group name or group ID that must be changed before running a

      command.'
- property: guard_interpreter
  ruby_type: Symbol
  required: false
  default_value: :powershell_script
  new_in: null
  description_list:
  - markdown: 'When this property is set to `:powershell_script`, the 64-bit

      version of the Windows PowerShell shell will be used to evaluate

      strings values for the `not_if` and `only_if` properties. Set this

      value to `:default` to use the 32-bit version of the cmd.exe shell.'
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
  - markdown: 'Inherited from **execute** resource. The return value for a command.

      This may be an array of accepted values. An exception is raised when

      the return value(s) do not match.'
- property: timeout
  ruby_type: Integer, Float
  required: false
  default_value: null
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

      i.e. <span class="title-ref">domain\\user</span> or <span

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

      class="title-ref">user</span> property. Default value: <span

      class="title-ref">nil</span>. This property is mandatory if <span

      class="title-ref">user</span> is specified on Windows and may only

      be specified if <span class="title-ref">user</span> is specified.

      The <span class="title-ref">sensitive</span> property for this

      resource will automatically be set to true if password is specified.'
- property: domain
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '*Windows only*: The domain of the user specified by the <span

      class="title-ref">user</span> property. Default value: <span

      class="title-ref">nil</span>. If not specified, the user name and

      password specified by the <span class="title-ref">user</span> and

      <span class="title-ref">password</span> properties will be used to

      resolve that user against the domain in which the system running

      Chef Infra Client is joined, or if that system is not joined to a

      domain it will resolve the user as a local account on that system.

      An alternative way to specify the domain is to leave this property

      unspecified and specify the domain as part of the <span

      class="title-ref">user</span> property.'
- property: elevated
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Determines whether the script will run with elevated permissions to

      circumvent User Access Control (UAC) interactively blocking the

      process.


      This will cause the process to be run under a batch login instead of

      an interactive login. The user running Chef needs the "Replace a

      process level token" and "Adjust Memory Quotas for a process"

      permissions. The user that is running the command needs the "Log on

      as a batch job" permission.


      Because this requires a login, the `user` and `password` properties

      are required.'
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
  Write to an interpolated path\n\n  ``` ruby\n  powershell_script\
  \ 'write-to-interpolated-path' do\n    code <<-EOH\n    $stream = [System.IO.StreamWriter]\
  \ \"#{Chef::Config[:file_cache_path]}/powershell-test.txt\"\n    $stream.WriteLine(\"\
  In #{Chef::Config[:file_cache_path]}...word.\")\n    $stream.close()\n    EOH\n\
  \  end\n  ```\n\n  Change the working directory\n\n  ``` ruby\n  powershell_script\
  \ 'cwd-then-write' do\n    cwd Chef::Config[:file_cache_path]\n    code <<-EOH\n\
  \    $stream = [System.IO.StreamWriter] \"C:/powershell-test2.txt\"\n    $pwd =\
  \ pwd\n    $stream.WriteLine(\"This is the contents of: $pwd\")\n    $dirs = dir\n\
  \    foreach ($dir in $dirs) {\n      $stream.WriteLine($dir.fullname)\n    }\n\
  \    $stream.close()\n    EOH\n  end\n  ```\n\n  Change the working directory in\
  \ Microsoft Windows\n\n  ``` ruby\n  powershell_script 'cwd-to-win-env-var' do\n\
  \    cwd '%TEMP%'\n    code <<-EOH\n    $stream = [System.IO.StreamWriter] \"./temp-write-from-chef.txt\"\
  \n    $stream.WriteLine(\"chef on windows rox yo!\")\n    $stream.close()\n    EOH\n\
  \  end\n  ```\n\n  Pass an environment variable to a script\n\n  ``` ruby\n  powershell_script\
  \ 'read-env-var' do\n    cwd Chef::Config[:file_cache_path]\n    environment ({'foo'\
  \ => 'BAZ'})\n    code <<-EOH\n    $stream = [System.IO.StreamWriter] \"./test-read-env-var.txt\"\
  \n    $stream.WriteLine(\"FOO is $env:foo\")\n    $stream.close()\n    EOH\n  end\n\
  \  ```\n\n  **Evaluate for true and/or false**\n\n  Use the `convert_boolean_return`\
  \ attribute to raise an exception when\n  certain conditions are met. For example,\
  \ the following fragments will\n  run successfully without error:\n\n  ``` ruby\n\
  \  powershell_script 'false' do\n    code '$false'\n  end\n  ```\n\n  and:\n\n \
  \ ``` ruby\n  powershell_script 'true' do\n    code '$true'\n  end\n  ```\n\n  whereas\
  \ the following will raise an exception:\n\n  ``` ruby\n  powershell_script 'false'\
  \ do\n    convert_boolean_return true\n    code '$false'\n  end\n  ```\n\n  Use\
  \ the flags attribute\n\n  ``` ruby\n  powershell_script 'Install IIS' do\n    code\
  \ <<-EOH\n    Import-Module ServerManager\n    Add-WindowsFeature Web-Server\n \
  \   EOH\n    flags '-NoLogo, -NonInteractive, -NoProfile, -ExecutionPolicy Unrestricted,\
  \ -InputFormat None, -File'\n    guard_interpreter :powershell_script\n    not_if\
  \ '(Get-WindowsFeature -Name Web-Server).Installed'\n  end\n  ```\n\n  Rename computer,\
  \ join domain, reboot\n\n  The following example shows how to rename a computer,\
  \ join a domain, and\n  then reboot the computer:\n\n  ``` ruby\n  reboot 'Restart\
  \ Computer' do\n    action :nothing\n  end\n\n  powershell_script 'Rename and Join\
  \ Domain' do\n    code <<-EOH\n      ...your rename and domain join logic here...\n\
  \    EOH\n    not_if <<-EOH\n      $ComputerSystem = gwmi win32_computersystem\n\
  \      ($ComputerSystem.Name -like '#{node['some_attribute_that_has_the_new_name']}')\
  \ -and\n        $ComputerSystem.partofdomain)\n    EOH\n    notifies :reboot_now,\
  \ 'reboot[Restart Computer]', :immediately\n  end\n  ```\n\n  where:\n\n  -   The\
  \ **powershell_script** resource block renames a computer, and\n      then joins\
  \ a domain\n  -   The **reboot** resource restarts the computer\n  -   The `not_if`\
  \ guard prevents the Windows PowerShell script from\n      running when the settings\
  \ in the `not_if` guard match the desired\n      state\n  -   The `notifies` statement\
  \ tells the **reboot** resource block to run\n      if the **powershell_script**\
  \ block was executed during a Chef Infra\n      Client run\n\n  Run a command as\
  \ an alternate user\n\n  *Note*: When Chef is running as a service, this feature\
  \ requires that\n  the user that Chef runs as has 'SeAssignPrimaryTokenPrivilege'\
  \ (aka\n  'SE_ASSIGNPRIMARYTOKEN_NAME') user right. By default only LocalSystem\n\
  \  and NetworkService have this right when running as a service. This is\n  necessary\
  \ even if the user is an Administrator.\n\n  This right can be added and checked\
  \ in a recipe using this example:\n\n  ``` ruby\n  # Add 'SeAssignPrimaryTokenPrivilege'\
  \ for the user\n  Chef::ReservedNames::Win32::Security.add_account_right('<user>',\
  \ 'SeAssignPrimaryTokenPrivilege')\n\n  # Check if the user has 'SeAssignPrimaryTokenPrivilege'\
  \ rights\n  Chef::ReservedNames::Win32::Security.get_account_right('<user>').include?('SeAssignPrimaryTokenPrivilege')\n\
  \  ```\n\n  The following example shows how to run `mkdir test_dir` from a Chef\n\
  \  Infra Client run as an alternate user.\n\n  ``` ruby\n  # Passing only username\
  \ and password\n  powershell_script 'mkdir test_dir' do\n   code \"mkdir test_dir\"\
  \n   cwd Chef::Config[:file_cache_path]\n   user \"username\"\n   password \"password\"\
  \n  end\n\n  # Passing username and domain\n  powershell_script 'mkdir test_dir'\
  \ do\n   code \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n   domain\
  \ \"domain\"\n   user \"username\"\n   password \"password\"\n  end\n\n  # Passing\
  \ username = 'domain-name\\\\username'. No domain is passed\n  powershell_script\
  \ 'mkdir test_dir' do\n   code \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n\
  \   user \"domain-name\\\\username\"\n   password \"password\"\n  end\n\n  # Passing\
  \ username = 'username@domain-name'. No domain is passed\n  powershell_script 'mkdir\
  \ test_dir' do\n   code \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n\
  \   user \"username@domain-name\"\n   password \"password\"\n  end\n\n  # Work around\
  \ User Access Control (UAC)\n  powershell_script 'mkdir test_dir' do\n   code \"\
  mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n   user \"username\"\n\
  \   password \"password\"\n   elevated true\n  end\n  ```\n"

---
