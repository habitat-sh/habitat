---
title: script resource
resource: script
draft: false
aliases:
- /resource_script.html
menu:
  infra:
    title: script
    identifier: chef_infra/cookbook_reference/resources/script script
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **script** resource to execute scripts using a specified

    interpreter, such as Bash, csh, Perl, Python, or Ruby. This resource may

    also use any of the actions and properties that are available to the

    **execute** resource. Commands that are executed with this resource are

    (by their nature) not idempotent, as they are typically unique to the

    environment in which they are run. Use `not_if` and `only_if` to guard

    this resource for idempotence.'
- markdown: 'This resource is the base resource for several other resources used for

    scripting on specific platforms. For more information about specific

    resources for specific platforms, see the following topics:


    -   [bash](/resources/bash/)

    -   [csh](/resources/csh/)

    -   [ksh](/resources/ksh/)

    -   [perl](/resources/perl/)

    -   [python](/resources/python/)

    -   [ruby](/resources/ruby/)


    Changed in 12.19 to support windows alternate user identity in execute

    resources'
resource_new_in: null
handler_types: false
syntax_description: "A **script** resource block typically executes scripts using\
  \ a specified\ninterpreter, such as Bash, csh, Perl, Python, or Ruby:\n\n``` ruby\n\
  script 'extract_module' do\n  interpreter \"bash\"\n  cwd ::File.dirname(src_filepath)\n\
  \  code <<-EOH\n    mkdir -p #{extract_path}\n    tar xzf #{src_filename} -C #{extract_path}\n\
  \    mv #{extract_path}/*/* #{extract_path}/\n  EOH\n  not_if { ::File.exist?(extract_path)\
  \ }\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`interpreter` specifies the command shell to use'
- '`cwd` specifies the directory from which the command is run'
- |
  `code` specifies the command to run

  It is more common to use the **script**-based resource that is specific to the
  command shell. Chef has shell-specific resources for Bash, csh, ksh, Perl,
  Python, and Ruby.


  The same command as above, but run using the **bash** resource:

  ``` ruby
  bash 'extract_module' do
    cwd ::File.dirname(src_filepath)
    code <<-EOH
      mkdir -p #{extract_path}
      tar xzf #{src_filename} -C #{extract_path}
      mv #{extract_path}/*/* #{extract_path}/
    EOH
    not_if { ::File.exist?(extract_path) }
  end
  ```
syntax_full_code_block: "script 'name' do\n  code                       String\n \
  \ creates                    String\n  cwd                        String\n  environment\
  \                Hash\n  flags                      String\n  group            \
  \          String, Integer\n  interpreter                String\n  path        \
  \               Array\n  returns                    Integer, Array\n  timeout  \
  \                  Integer, Float\n  user                       String\n  password\
  \                   String\n  domain                     String\n  umask       \
  \               String, Integer\n  action                     Symbol # defaults\
  \ to :run if not specified\nend"
syntax_full_properties_list:
- '`script` is the resource'
- '`name` is the name of the resource block'
- '`cwd` is the location from which the command is run'
- '`action` identifies the steps Chef Infra Client will take to bring the node into
  the desired state'
- '`code`, `creates`, `cwd`, `environment`, `flags`, `group`, `interpreter`, `path`,
  `returns`, `timeout`, `user`, `password`, `domain` and `umask` are properties of
  this resource, with the Ruby type shown. See "Properties" section below for more
  information about all of the properties that may be used with this resource.'
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
- property: interpreter
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The script interpreter to use during code execution.
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

      process. Default value: <span class="title-ref">nil</span>. The user

      name may optionally be specified with a domain, i.e. <span

      class="title-ref">domainuser</span> or <span

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
  - markdown: '*Windows only*: The domain of the user user specified by the <span

      class="title-ref">user</span> property. Default value: <span

      class="title-ref">nil</span>. If not specified, the user name and

      password specified by the <span class="title-ref">user</span> and

      <span class="title-ref">password</span> properties will be used to

      resolve that user against the domain in which the system running

      Chef client is joined, or if that system is not joined to a domain

      it will resolve the user as a local account on that system. An

      alternative way to specify the domain is to leave this property

      unspecified and specify the domain as part of the <span

      class="title-ref">user</span> property.'
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
resources_common_guard_interpreter: true
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Use a named provider to run a script\n\n  ``` ruby\n  bash 'install_something'\
  \ do\n    user 'root'\n    cwd '/tmp'\n    code <<-EOH\n      wget http://www.example.com/tarball.tar.gz\n\
  \      tar -zxf tarball.tar.gz\n      cd tarball\n      ./configure\n      make\n      make\
  \   install\n    EOH\n  end\n  ```\n\n  Run a script\n\n  ``` ruby\n  script 'install_something'\
  \ do\n    interpreter 'bash'\n    user 'root'\n    cwd '/tmp'\n    code <<-EOH\n\
  \      wget http://www.example.com/tarball.tar.gz\n      tar -zxf tarball.tar.gz\n \
  \     cd tarball\n      ./configure\n      make\n      make install\n    EOH\n  end\n  ```\n\
  \n  or something like:\n\n  ``` ruby\n  bash 'openvpn-server-key' do\n    environment('KEY_CN'\
  \ => 'server')\n    code <<-EOF\n      openssl req -batch -days #{node['openvpn']['key']['expire']}\
  \ \\\n        -nodes -new -newkey rsa:#{key_size} -keyout #{key_dir}/server.key\
  \ \\\n        -out #{key_dir}/server.csr -extensions server \\\n        -config\
  \ #{key_dir}/openssl.cnf\n    EOF\n    not_if { File.exist?('#{key_dir}/server.crt')\
  \ }\n  end\n  ```\n\n  where `code` contains the OpenSSL command to be run. The\
  \ `not_if`\n  property tells Chef Infra Client not to run the command if the file\n\
  \  already exists.\n\n  Install a file from a remote location using bash\n\n  The\
  \ following is an example of how to install the `foo123` module for\n  Nginx. This\
  \ module adds shell-style functionality to an Nginx\n  configuration file and does\
  \ the following:\n\n  -   Declares three variables\n  -   Gets the Nginx file from\
  \ a remote location\n  -   Installs the file using Bash to the path specified by\
  \ the\n      `src_filepath` variable\n\n  <!-- -->\n\n  ``` ruby\n  # the following\
  \ code sample is similar to the ``upload_progress_module``\n  # recipe in the ``nginx``\
  \ cookbook:\n  # https://github.com/chef-cookbooks/nginx\n\n  src_filename = \"\
  foo123-nginx-module-v#{\n    node['nginx']['foo123']['version']\n  }.tar.gz\"\n\
  \  src_filepath = \"#{Chef::Config['file_cache_path']}/#{src_filename}\"\n  extract_path\
  \ = \"#{\n    Chef::Config['file_cache_path']\n    }/nginx_foo123_module/#{\n  \
  \  node['nginx']['foo123']['checksum']\n  }\"\n\n  remote_file 'src_filepath' do\n\
  \    source node['nginx']['foo123']['url']\n    checksum node['nginx']['foo123']['checksum']\n\
  \    owner 'root'\n    group 'root'\n    mode '0755'\n  end\n\n  bash 'extract_module'\
  \ do\n    cwd ::File.dirname(src_filepath)\n    code <<-EOH\n      mkdir -p #{extract_path}\n\
  \      tar xzf #{src_filename} -C #{extract_path}\n      mv #{extract_path}/*/*\
  \ #{extract_path}/\n    EOH\n    not_if { ::File.exist?(extract_path) }\n  end\n\
  \  ```\n\n  Install an application from git using bash\n\n  The following example\
  \ shows how Bash can be used to install a plug-in\n  for rbenv named `ruby-build`,\
  \ which is located in git version source\n  control. First, the application is synchronized,\
  \ and then Bash changes\n  its working directory to the location in which `ruby-build`\
  \ is located,\n  and then runs a command.\n\n  ``` ruby\n  git \"#{Chef::Config[:file_cache_path]}/ruby-build\"\
  \ do\n    repository 'git://github.com/sstephenson/ruby-build.git'\n    revision\
  \ 'master'\n    action :sync\n  end\n\n  bash 'install_ruby_build' do\n    cwd \"\
  #{Chef::Config[:file_cache_path]}/ruby-build\"\n    user 'rbenv'\n    group 'rbenv'\n\
  \    code <<-EOH\n      ./install.sh\n    EOH\n    environment 'PREFIX' => '/usr/local'\n\
  \  end\n  ```\n\n  To read more about `ruby-build`, see here:\n  <https://github.com/sstephenson/ruby-build>.\n\
  \n  Store certain settings\n\n  The following recipe shows how an attributes file\
  \ can be used to store\n  certain settings. An attributes file is located in the\
  \ `attributes/`\n  directory in the same cookbook as the recipe which calls the\
  \ attributes\n  file. In this example, the attributes file specifies certain settings\n\
  \  for Python that are then used across all nodes against which this recipe\n  will\
  \ run.\n\n  Python packages have versions, installation directories, URLs, and\n\
  \  checksum files. An attributes file that exists to support this type of\n  recipe\
  \ would include settings like the following:\n\n  ``` ruby\n  default['python']['version']\
  \ = '2.7.1'\n\n  if python['install_method'] == 'package'\n    default['python']['prefix_dir']\
  \ = '/usr'\n  else\n    default['python']['prefix_dir'] = '/usr/local'\n  end\n\n\
  \  default['python']['url'] = 'http://www.python.org/ftp/python'\n  default['python']['checksum']\
  \ = '80e387...85fd61'\n  ```\n\n  and then the methods in the recipe may refer to\
  \ these values. A recipe\n  that is used to install Python will need to do the following:\n\
  \n  -   Identify each package to be installed (implied in this example, not\n  \
  \    shown)\n  -   Define variables for the package `version` and the `install_path`\n\
  \  -   Get the package from a remote location, but only if the package does\n  \
  \    not already exist on the target system\n  -   Use the **bash** resource to\
  \ install the package on the node, but\n      only when the package is not already\
  \ installed\n\n  <!-- -->\n\n  ``` ruby\n  #  the following code sample comes from\
  \ the ``oc-nginx`` cookbook on |github|: https://github.com/cookbooks/oc-nginx\n\
  \n  version = node['python']['version']\n  install_path = \"#{node['python']['prefix_dir']}/lib/python#{version.split(/(^\\\
  d+\\.\\d+)/)[1]}\"\n\n  remote_file \"#{Chef::Config[:file_cache_path]}/Python-#{version}.tar.bz2\"\
  \ do\n    source \"#{node['python']['url']}/#{version}/Python-#{version}.tar.bz2\"\
  \n    checksum node['python']['checksum']\n    mode '0755'\n    not_if { ::File.exist?(install_path)\
  \ }\n  end\n\n  bash 'build-and-install-python' do\n    cwd Chef::Config[:file_cache_path]\n\
  \    code <<-EOF\n      tar -jxvf Python-#{version}.tar.bz2\n      (cd Python-#{version}\
  \ && ./configure #{configure_options})\n      (cd Python-#{version} && make && make\
  \ install)\n    EOF\n    not_if { ::File.exist?(install_path) }\n  end\n  ```\n\n\
  \  Run a command as an alternate user\n\n  *Note*: When Chef is running as a service,\
  \ this feature requires that\n  the user that Chef runs as has 'SeAssignPrimaryTokenPrivilege'\
  \ (aka\n  'SE_ASSIGNPRIMARYTOKEN_NAME') user right. By default only LocalSystem\n\
  \  and NetworkService have this right when running as a service. This is\n  necessary\
  \ even if the user is an Administrator.\n\n  This right can be added and checked\
  \ in a recipe using this example:\n\n  ``` ruby\n  # Add 'SeAssignPrimaryTokenPrivilege'\
  \ for the user\n  Chef::ReservedNames::Win32::Security.add_account_right('<user>',\
  \ 'SeAssignPrimaryTokenPrivilege')\n\n  # Check if the user has 'SeAssignPrimaryTokenPrivilege'\
  \ rights\n  Chef::ReservedNames::Win32::Security.get_account_right('<user>').include?('SeAssignPrimaryTokenPrivilege')\n\
  \  ```\n\n  The following example shows how to run `mkdir test_dir` from a Chef\n\
  \  Infra Client run as an alternate user.\n\n  ``` ruby\n  # Passing only username\
  \ and password\n  script 'mkdir test_dir' do\n   interpreter \"bash\"\n   code \
  \ \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n   user \"username\"\
  \n   password \"password\"\n  end\n\n  # Passing username and domain\n  script 'mkdir\
  \ test_dir' do\n   interpreter \"bash\"\n   code  \"mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n\
  \   domain \"domain-name\"\n   user \"username\"\n   password \"password\"\n  end\n\
  \n  # Passing username = 'domain-name\\\\username'. No domain is passed\n  script\
  \ 'mkdir test_dir' do\n   interpreter \"bash\"\n   code  \"mkdir test_dir\"\n  \
  \ cwd Chef::Config[:file_cache_path]\n   user \"domain-name\\\\username\"\n   password\
  \ \"password\"\n  end\n\n  # Passing username = 'username@domain-name'. No domain\
  \ is passed\n  script 'mkdir test_dir' do\n   interpreter \"bash\"\n   code  \"\
  mkdir test_dir\"\n   cwd Chef::Config[:file_cache_path]\n   user \"username@domain-name\"\
  \n   password \"password\"\n  end\n  ```\n"

---
