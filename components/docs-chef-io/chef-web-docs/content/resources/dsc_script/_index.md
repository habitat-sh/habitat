---
resource_reference: true
properties_shortcode:
ps_credential_helper: true
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: dsc_script resource
resource: dsc_script
aliases:
- "/resource_dsc_script.html"
menu:
  infra:
    title: dsc_script
    identifier: chef_infra/cookbook_reference/resources/dsc_script dsc_script
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- shortcode: resources_common_powershell.md
- shortcode: resources_common_powershell_dsc.md
- markdown: 'Many DSC resources are comparable to built-in Chef Infra resources. For

    example, both DSC and Chef have **file**, **package**, and **service**

    resources. The **dsc_script** resource is most useful for those DSC

    resources that do not have a direct comparison to a resource in Chef,

    such as the `Archive` resource, a custom DSC resource, an existing DSC

    script that performs an important task, and so on. Use the

    **dsc_script** resource to embed the code that defines a DSC

    configuration directly within a Chef Infra recipe.'
- note:
    markdown: 'Windows PowerShell 4.0 is required for using the **dsc_script**

      resource with Chef Infra.'
- note:
    markdown: 'The WinRM service must be enabled. (Use `winrm quickconfig` to enable

      the service.)'
- warning:
    markdown: 'The **dsc_script** resource may not be used in the same run-list with

      the **dsc_resource**. This is because the **dsc_script** resource

      requires that `RefreshMode` in the Local Configuration Manager be set to

      `Push`, whereas the **dsc_resource** resource requires it to be set to

      `Disabled`.'
resource_new_in: null
handler_types: false
syntax_description: "A **dsc_script** resource block embeds the code that defines\
  \ a DSC\nconfiguration directly within a Chef recipe:\n\n``` ruby\ndsc_script 'get-dsc-resource-kit'\
  \ do\n  code <<-EOH\n    Archive reskit\n    {\n      ensure = 'Present'\n     \
  \ path = \"#{Chef::Config[:file_cache_path]}\\\\DSCResourceKit620082014.zip\"\n\
  \      destination = \"#{ENV['PROGRAMW6432']}\\\\WindowsPowerShell\\\\Modules\"\n\
  \    }\n  EOH\nend\n```"
syntax_code_block: null
syntax_properties_list:
- 'the **remote_file** resource is first used to download the

  `DSCResourceKit620082014.zip` file.'
syntax_full_code_block: "dsc_script 'name' do\n  code                       String\n\
  \  command                    String\n  configuration_data         String\n  configuration_data_script\
  \  String\n  configuration_name         String\n  cwd                        String\n\
  \  environment                Hash\n  flags                      Hash\n  imports\
  \                    Array\n  timeout                    Integer\n  action     \
  \                Symbol # defaults to :run if not specified\nend"
syntax_full_properties_list:
- '`dsc_script` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`code`, `command`, `configuration_data`, `configuration_data_script`, `configuration_name`,
  `cwd`, `environment`, `flags`, `imports`, and `timeout` are properties of this resource,
  with the Ruby type shown. See "Properties" section below for more information about
  all of the properties that may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :run:
    markdown: Default. Use to run the DSC configuration defined as defined in this
      resource.
properties_list:
- property: code
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The code for the DSC configuration script. This property may not be

      used in conjunction with the `command` property.'
- property: command
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to a valid Windows PowerShell data file that contains the

      DSC configuration script. This data file must be capable of running

      independently of Chef and must generate a valid DSC configuration.

      This property may not be used in conjunction with the `code`

      property.'
- property: configuration_data
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The configuration data for the DSC script. The configuration data

      must be [a valid Windows PowerShell data

      file](https://docs.microsoft.com/en-us/powershell/developer/windows-powershell).

      This property may not be used in conjunction with the

      `configuration_data_script` property.'
- property: configuration_data_script
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to a valid Windows PowerShell data file that also contains

      a node called `localhost`. This property may not be used in

      conjunction with the `configuration_data` property.'
- property: configuration_name
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The name of a valid Windows PowerShell cmdlet. The name may only

      contain letter (a-z, A-Z), number (0-9), and underscore (_)

      characters and should start with a letter. The name may not be null

      or empty. This property may not be used in conjunction with the

      `code` property.'
- property: cwd
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The current working directory.
- property: environment
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A Hash of environment variables in the form of

      `({''ENV_VARIABLE'' => ''VALUE''})`. (These variables must exist for a

      command to be run successfully.)'
- property: flags
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Pass parameters to the DSC script that is specified by the `command`

      property. Parameters are defined as key-value pairs, where the value

      of each key is the parameter to pass. This property may not be used

      in the same recipe as the `code` property. For example:

      `flags ({ :EditorChoice => ''emacs'', :EditorFlags => ''--maximized'' })`.'
- property: imports
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - warning:
    - markdown: This property **MUST** be used with the `code` attribute.
  - markdown: 'Use to import DSC resources from a module.


      To import all resources from a module, specify only the module name:


      ``` ruby

      imports ''module_name''

      ```


      To import specific resources, specify the module name, and then

      specify the name for each resource in that module to import:


      ``` ruby

      imports ''module_name'', ''resource_name_a'', ''resource_name_b'', ...

      ```


      For example, to import all resources from a module named

      `cRDPEnabled`:


      ``` ruby

      imports ''cRDPEnabled''

      ```


      To import only the `PSHOrg_cRDPEnabled` resource:


      ``` ruby

      imports ''cRDPEnabled'', ''PSHOrg_cRDPEnabled''

      ```'
- property: timeout
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The amount of time (in seconds) a command is to wait before timing

      out.'
properties_shortcode: null
properties_multiple_packages: false
resource_directory_recursive_directories: false
resources_common_atomic_update: false
properties_resources_common_windows_security: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
ps_credential_helper: true
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
  Specify DSC code directly\n\n  DSC data can be specified directly\
  \ in a recipe:\n\n  ``` ruby\n  dsc_script 'emacs' do\n    code <<-EOH\n    Environment\
  \ 'texteditor'\n    {\n      Name = 'EDITOR'\n      Value = 'c:\\\\emacs\\\\bin\\\
  \\emacs.exe'\n    }\n    EOH\n  end\n  ```\n\n  Specify DSC code using a Windows\
  \ PowerShell data file\n\n  Use the `command` property to specify the path to a\
  \ Windows PowerShell\n  data file. For example, the following Windows PowerShell\
  \ script defines\n  the `DefaultEditor`:\n\n  ``` powershell\n  Configuration 'DefaultEditor'\n\
  \  {\n    Environment 'texteditor'\n      {\n        Name = 'EDITOR'\n        Value\
  \ = 'c:\\emacs\\bin\\emacs.exe'\n      }\n  }\n  ```\n\n  Use the following recipe\
  \ to specify the location of that data file:\n\n  ``` ruby\n  dsc_script 'DefaultEditor'\
  \ do\n    command 'c:\\dsc_scripts\\emacs.ps1'\n  end\n  ```\n\n  Pass parameters\
  \ to DSC configurations\n\n  If a DSC script contains configuration data that takes\
  \ parameters, those\n  parameters may be passed using the `flags` property. For\
  \ example, the\n  following Windows PowerShell script takes parameters for the\n\
  \  `EditorChoice` and `EditorFlags` settings:\n\n  ``` powershell\n  $choices =\
  \ @{'emacs' = 'c:\\emacs\\bin\\emacs';'vi' = 'c:\\vim\\vim.exe';'powershell' = 'powershell_ise.exe'}\n\
  \    Configuration 'DefaultEditor'\n      {\n        [CmdletBinding()]\n       \
  \ param\n          (\n            $EditorChoice,\n            $EditorFlags = ''\n\
  \          )\n        Environment 'TextEditor'\n        {\n          Name = 'EDITOR'\n\
  \          Value =  \"$($choices[$EditorChoice]) $EditorFlags\"\n        }\n   \
  \   }\n  ```\n\n  Use the following recipe to set those parameters:\n\n  ``` ruby\n\
  \  dsc_script 'DefaultEditor' do\n    flags ({ :EditorChoice => 'emacs', :EditorFlags\
  \ => '--maximized' })\n    command 'c:\\dsc_scripts\\editors.ps1'\n  end\n  ```\n\
  \n  Use custom configuration data\n\n  Configuration data in DSC scripts may be\
  \ customized from a recipe. For\n  example, scripts are typically customized to\
  \ set the behavior for\n  Windows PowerShell credential data types. Configuration\
  \ data may be\n  specified in one of three ways:\n\n  -   By using the `configuration_data`\
  \ attribute\n  -   By using the `configuration_data_script` attribute\n  -   By\
  \ specifying the path to a valid Windows PowerShell data file\n\n  The following\
  \ example shows how to specify custom configuration data\n  using the `configuration_data`\
  \ property:\n\n  ``` ruby\n  dsc_script 'BackupUser' do\n    configuration_data\
  \ <<-EOH\n      @{\n       AllNodes = @(\n            @{\n            NodeName =\
  \ \"localhost\";\n            PSDscAllowPlainTextPassword = $true\n            })\n\
  \       }\n    EOH\n    code <<-EOH\n      $user = 'backup'\n      $password = ConvertTo-SecureString\
  \ -String \"YourPass$(random)\" -AsPlainText -Force\n      $cred = New-Object -TypeName\
  \ System.Management.Automation.PSCredential -ArgumentList $user, $password\n\n \
  \    User $user\n       {\n         UserName = $user\n         Password = $cred\n\
  \         Description = 'Backup operator'\n         Ensure = \"Present\"\n     \
  \    Disabled = $false\n         PasswordNeverExpires = $true\n         PasswordChangeRequired\
  \ = $false\n       }\n     EOH\n  end\n  ```\n\n  The following example shows how\
  \ to specify custom configuration data\n  using the `configuration_name` property.\
  \ For example, the following\n  Windows PowerShell script defines the `vi` configuration:\n\
  \n  ``` powershell\n  Configuration 'emacs'\n    {\n      Environment 'TextEditor'\n\
  \      {\n        Name = 'EDITOR'\n        Value = 'c:\\emacs\\bin\\emacs.exe'\n\
  \      }\n  }\n\n  Configuration 'vi'\n  {\n      Environment 'TextEditor'\n   \
  \   {\n        Name = 'EDITOR'\n        Value = 'c:\\vim\\bin\\vim.exe'\n      }\n\
  \  }\n  ```\n\n  Use the following recipe to specify that configuration:\n\n  ```\
  \ ruby\n  dsc_script 'EDITOR' do\n    configuration_name 'vi'\n    command 'C:\\\
  dsc_scripts\\editors.ps1'\n  end\n  ```\n\n  Using DSC with other Chef resources\n\
  \n  The **dsc_script** resource can be used with other resources. The\n  following\
  \ example shows how to download a file using the\n  **remote_file** resource, and\
  \ then uncompress it using the DSC\n  `Archive` resource:\n\n  ``` ruby\n  remote_file\
  \ \"#{Chef::Config[:file_cache_path]}\\\\DSCResourceKit620082014.zip\" do\n    source\
  \ 'http://gallery.technet.microsoft.com/DSC-Resource-Kit-All-c449312d/file/124481/1/DSC%20Resource%20Kit%20Wave%206%2008282014.zip'\n\
  \  end\n\n  dsc_script 'get-dsc-resource-kit' do\n    code <<-EOH\n      Archive\
  \ reskit\n      {\n        ensure = 'Present'\n        path = \"#{Chef::Config[:file_cache_path]}\\\
  \\DSCResourceKit620082014.zip\"\n        destination = \"#{ENV['PROGRAMW6432']}\\\
  \\WindowsPowerShell\\\\Modules\"\n      }\n    EOH\n  end\n  ```\n"

---
