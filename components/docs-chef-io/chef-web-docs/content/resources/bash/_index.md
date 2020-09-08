---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: bash resource
resource: bash
aliases:
- "/resource_bash.html"
menu:
  infra:
    title: bash
    identifier: chef_infra/cookbook_reference/resources/bash bash
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **bash** resource to execute scripts using the Bash interpreter.

    This resource may also use any of the actions and properties that are

    available to the **execute** resource. Commands that are executed with

    this resource are (by their nature) not idempotent, as they are

    typically unique to the environment in which they are run. Use `not_if`

    and `only_if` to guard this resource for idempotence.'
syntax_description: "A **bash** resource block executes scripts using Bash:\n\n```\
  \ ruby\nbash 'extract_module' do\n  cwd ::File.dirname(src_filepath)\n  code <<-EOH\n\
  \    mkdir -p #{extract_path}\n    tar xzf #{src_filename} -C #{extract_path}\n\
  \    mv #{extract_path}/*/* #{extract_path}/\n  EOH\n  not_if { ::File.exist?(extract_path)\
  \ }\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`cwd` specifies the directory from which the command is run'
- '`code` specifies the command to run'
syntax_full_code_block: "bash 'name' do\n  code                       String\n  creates\
  \                    String\n  cwd                        String\n  environment\
  \                Hash\n  flags                      String\n  group            \
  \          String, Integer\n  path                       Array\n  returns      \
  \              Integer, Array\n  timeout                    Integer, Float\n  user\
  \                       String, Integer\n  umask                      String, Integer\n\
  \  action                     Symbol # defaults to :run if not specified\nend"
syntax_full_properties_list:
- '`bash` is the resource.'
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
    shortcode: resources_common_actions_nothing.md
  :run:
    markdown: Default. Run a script.
properties_list:
- property: code
  ruby_type: String
  required: true
  default_value:
  new_in:
  description_list:
  - markdown: A quoted (" ") string of code to be executed.
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
- property: environment
  ruby_type: Hash
  required: false
  default_value:
  new_in:
  description_list:
  - markdown: 'A Hash of environment variables in the form of

      `({"ENV_VARIABLE" => "VALUE"})`. (These variables must exist for a

      command to be run successfully.)'
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
  ruby_type: Integer, String, Float
  required: false
  default_value: '3600'
  description_list:
  - markdown: The amount of time (in seconds) a command is to wait before timing out.
- property: user
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 'The user name or user ID that should be changed before running a

      command.'
- property: umask
  ruby_type: String, Integer
  required: false
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
examples: "
  Use a named provider to run a script\n\n  ``` ruby\n  bash 'install_something'\
  \ do\n    user 'root'\n    cwd '/tmp'\n    code <<-EOH\n      wget http://www.example.com/tarball.tar.gz\n\
  \      tar -zxf tarball.tar.gz\n      cd tarball\n      ./configure\n      make\n      make\
  \ install\n    EOH\n  end\n  ```\n\n  Install a file from a remote location using\
  \ bash\n\n  The following is an example of how to install the `foo123` module for\n\
  \  Nginx. This module adds shell-style functionality to an Nginx\n  configuration\
  \ file and does the following:\n\n  -   Declares three variables\n  -   Gets the\
  \ Nginx file from a remote location\n  -   Installs the file using Bash to the path\
  \ specified by the\n      `src_filepath` variable\n\n  <!-- -->\n\n  ``` ruby\n\
  \  # the following code sample is similar to the ``upload_progress_module``\n  #\
  \ recipe in the ``nginx`` cookbook:\n  # https://github.com/chef-cookbooks/nginx\n\
  \n  src_filename = \"foo123-nginx-module-v#{\n    node['nginx']['foo123']['version']\n\
  \  }.tar.gz\"\n  src_filepath = \"#{Chef::Config['file_cache_path']}/#{src_filename}\"\
  \n  extract_path = \"#{\n    Chef::Config['file_cache_path']\n    }/nginx_foo123_module/#{\n\
  \    node['nginx']['foo123']['checksum']\n  }\"\n\n  remote_file 'src_filepath'\
  \ do\n    source node['nginx']['foo123']['url']\n    checksum node['nginx']['foo123']['checksum']\n\
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
  \ install)\n    EOF\n    not_if { ::File.exist?(install_path) }\n  end\n  ```\n"

---
