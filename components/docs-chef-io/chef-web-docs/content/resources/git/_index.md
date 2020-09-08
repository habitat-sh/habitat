---
title: git resource
resource: git
draft: false
aliases:
- /resource_git.html
menu:
  infra:
    title: git
    identifier: chef_infra/cookbook_reference/resources/git git
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **git** resource to manage source control resources that exist

    in a git repository. git version 1.6.5 (or higher) is required to use

    all of the functionality in the **git** resource.'
resource_new_in: null
handler_types: false
syntax_description: "A **git** resource block manages source control resources that\
  \ exist in\na git repository:\n\n``` ruby\ngit \"#{Chef::Config[:file_cache_path]}/app_name\"\
  \ do\n  repository node[:app_name][:git_repository]\n  revision node[:app_name][:git_revision]\n\
  \  action :sync\nend\n```"
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "git 'name' do\n  additional_remotes      Hash\n  checkout_branch\
  \         String # default value: \"deploy\"\n  depth                   Integer\n\
  \  destination             String # default value: 'name' unless specified\n  enable_checkout\
  \         true, false # default value: true\n  enable_submodules       true, false\
  \ # default value: false\n  environment             Hash\n  group              \
  \     String, Integer\n  remote                  String # default value: \"origin\"\
  \n  repository              String\n  revision                String # default value:\
  \ \"HEAD\"\n  ssh_wrapper             String\n  timeout                 Integer\n\
  \  user                    String, Integer\n  action                  Symbol # defaults\
  \ to :sync if not specified\nend"
syntax_full_properties_list:
- '`git` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`additional_remotes`, `checkout_branch`, `depth`, `destination`, `enable_checkout`,
  `enable_submodules`, `environment`, `group`, `remote`, `repository`, `revision`,
  `ssh_wrapper`, `timeout`, and `user` are the properties available to this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :checkout:
    markdown: Clone or check out the source. When a checkout is available, this provider
      does nothing.
  :export:
    markdown: Export the source, excluding or removing any version control artifacts.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :sync:
    markdown: Default. Update the source to the specified version, or get a new clone
      or checkout. This action causes a hard reset of the index and working tree,
      discarding any uncommitted changes.
properties_list:
- property: additional_remotes
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A Hash of additional remotes that are added to the git repository

      configuration.'
- property: checkout_branch
  ruby_type: String
  required: false
  default_value: deploy
  new_in: null
  description_list:
  - markdown: 'Do a one-time checkout from git **or** use when a branch in the

      upstream repository is named `deploy`. To prevent the **git**

      resource from attempting to check out master from master, set

      `enable_checkout` to `false` when using the `checkout_branch`

      property. See `revision`.'
- property: depth
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The number of past revisions to be included in the git shallow

      clone. Unless specified the default behavior will do a full clone.'
- property: destination
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'The location path to which the source is to be cloned, checked out,

      or exported. Default value: the `name` of the resource block.'
- property: enable_checkout
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: 'Check out a repo from master. Set to `false` when using the

      `checkout_branch` attribute to prevent the **git** resource from

      attempting to check out master from master.'
- property: enable_submodules
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Perform a sub-module initialization and update.
- property: environment
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A Hash of environment variables in the form of

      `({"ENV_VARIABLE" => "VALUE"})`. (These variables must exist for a

      command to be run successfully.)'
  - note:
    - markdown: 'The **git** provider automatically sets the `ENV[''HOME'']` and

        `ENV[''GIT_SSH'']` environment variables. To override this behavior

        and provide different values, add `ENV[''HOME'']` and/or

        `ENV[''GIT_SSH'']` to the `environment` Hash.'
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The system group that is responsible for the checked-out code.
- property: remote
  ruby_type: String
  required: false
  default_value: '"origin"'
  new_in: null
  description_list:
  - markdown: The remote repository to use when synchronizing an existing clone.
- property: repository
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The URI for the git repository.
- property: revision
  ruby_type: String
  required: false
  default_value: HEAD
  new_in: null
  description_list:
  - markdown: 'A branch, tag, or commit to be synchronized with git. This can be

      symbolic, like `HEAD` or it can be a source control

      management-specific revision identifier. See `checkout_branch`.


      The value of the `revision` attribute may change over time. From one

      branch to another, to a tag, to a specific SHA for a commit, and

      then back to a branch. The `revision` attribute may even be changed

      in a way where history gets rewritten.


      Instead of tracking a specific branch or doing a headless checkout,

      Chef Infra Client maintains its own branch (via the **git**

      resource) that does not exist in the upstream repository. Chef Infra

      Client is then free to forcibly check out this branch to any commit

      without destroying the local history of an existing branch.


      For example, to explicitly track an upstream repository''s master

      branch:


      ``` ruby

      revision ''master''

      ```


      Use the `git rev-parse` and `git ls-remote` commands to verify that

      Chef Infra Client is synchronizing commits correctly. (Chef Infra

      Client always runs `git ls-remote` on the upstream repository to

      verify the commit is made to the correct repository.)'
- property: ssh_wrapper
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to the wrapper script used when running SSH with git. The

      `GIT_SSH` environment variable is set to this.'
- property: timeout
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The amount of time (in seconds) to wait for a command to execute

      before timing out. When this property is specified using the

      **deploy** resource, the value of the `timeout` property is passed

      from the **deploy** resource to the **git** resource.'
- property: user
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The system user that is responsible for the checked-out code.

      Default value: the home directory of this user, as indicated by the

      `HOME` environment variable.'
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
  Use the git mirror\n\n  ``` ruby\n  git '/opt/mysources/couch' do\n\
  \    repository 'git://git.apache.org/couchdb.git'\n    revision 'master'\n    action\
  \ :sync\n  end\n  ```\n\n  Use different branches\n\n  To use different branches,\
  \ depending on the environment of the node:\n\n  ``` ruby\n  if node.chef_environment\
  \ == 'QA'\n     branch_name = 'staging'\n  else\n     branch_name = 'master'\n \
  \ end\n\n  git '/home/user/deployment' do\n     repository 'git@github.com:gitsite/deployment.git'\n\
  \     revision branch_name\n     action :sync\n     user 'user'\n     group 'test'\n\
  \  end\n  ```\n\n  where the `branch_name` variable is set to `staging` or `master`,\n\
  \  depending on the environment of the node. Once this is determined, the\n  `branch_name`\
  \ variable is used to set the revision for the repository.\n  If the `git status`\
  \ command is used after running the example above, it\n  will return the branch\
  \ name as `deploy`, as this is the default value.\n  Run Chef Infra Client in debug\
  \ mode to verify that the correct branches\n  are being checked out:\n\n  ``` bash\n\
  \  sudo chef-client -l debug\n  ```\n\n  Install an application from git using bash\n\
  \n  The following example shows how Bash can be used to install a plug-in\n  for\
  \ rbenv named `ruby-build`, which is located in git version source\n  control. First,\
  \ the application is synchronized, and then Bash changes\n  its working directory\
  \ to the location in which `ruby-build` is located,\n  and then runs a command.\n\
  \n  ``` ruby\n  git \"#{Chef::Config[:file_cache_path]}/ruby-build\" do\n    repository\
  \ 'git://github.com/sstephenson/ruby-build.git'\n    revision 'master'\n    action\
  \ :sync\n  end\n\n  bash 'install_ruby_build' do\n    cwd \"#{Chef::Config[:file_cache_path]}/ruby-build\"\
  \n    user 'rbenv'\n    group 'rbenv'\n    code <<-EOH\n      ./install.sh\n   \
  \   EOH\n    environment 'PREFIX' => '/usr/local'\n  end\n  ```\n\n  To read more\
  \ about `ruby-build`, see here:\n  <https://github.com/sstephenson/ruby-build>.\n\
  \n  Upgrade packages from git\n\n  The following example uses the **git** resource\
  \ to upgrade packages:\n\n  ``` ruby\n  # the following code sample comes from the\
  \ ``source`` recipe\n  # in the ``libvpx-cookbook`` cookbook:\n  # https://github.com/enmasse-entertainment/libvpx-cookbook\n\
  \n  git \"#{Chef::Config[:file_cache_path]}/libvpx\" do\n    repository node[:libvpx][:git_repository]\n\
  \    revision node[:libvpx][:git_revision]\n    action :sync\n    notifies :run,\
  \ 'bash[compile_libvpx]', :immediately\n  end\n  ```\n\n  Pass in environment variables\n\
  \n  ``` ruby\n  git '/opt/mysources/couch' do\n    repository 'git://git.apache.org/couchdb.git'\n\
  \    revision 'master'\n    environment 'VAR' => 'whatever'\n    action :sync\n\
  \  end\n  ```\n"

---
