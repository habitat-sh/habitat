---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: subversion resource
resource: subversion
aliases:
- "/resource_subversion.html"
menu:
  infra:
    title: subversion
    identifier: chef_infra/cookbook_reference/resources/subversion subversion
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **subversion** resource to manage source control resources that
    exist in a Subversion repository.
- warning:
    markdown: 'The subversion resource has known bugs and may not work as expected.
      For

      more information see Chef GitHub issues, particularly

      [\#4050](https://github.com/chef/chef/issues/4050) and

      [\#4257](https://github.com/chef/chef/issues/4257).'
syntax_full_code_block: |-
  subversion 'name' do
    destination        String # default value: 'name' unless specified
    environment        Hash
    group              String, Integer
    repository         String
    revision           String # default value: "HEAD"
    svn_arguments      String, false # default value: "--no-auth-cache"
    svn_binary         String
    svn_info_args      String, false # default value: "--no-auth-cache"
    svn_password       String
    svn_username       String
    timeout            Integer
    user               String, Integer
    action             Symbol # defaults to :sync if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`subversion` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`destination`, `environment`, `group`, `repository`, `revision`, `svn_arguments`,
  `svn_binary`, `svn_info_args`, `svn_password`, `svn_username`, `timeout`, and `user`
  are the properties available to this resource."
actions_list:
  :checkout:
    markdown: Clone or check out the source. When a checkout is available, this provider
      does nothing.
  :export:
    markdown: Export the source, excluding or removing any version control artifacts.
  :force_export:
    markdown: Export the source, excluding or removing any version control artifacts
      and force an export of the source that is overwriting the existing copy (if
      it exists).
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :sync:
    markdown: Default. Update the source to the specified version, or get a new clone
      or checkout. This action causes a hard reset of the index and working tree,
      discarding any uncommitted changes.
properties_list:
- property: destination
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: 'The location path to which the source is to be cloned, checked out,
      or exported. Default value: the name of the resource block.'
- property: environment
  ruby_type: Hash
  required: false
  description_list:
  - markdown: A Hash of environment variables in the form of ({'ENV_VARIABLE' => 'VALUE'}).
- property: group
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The system group that will own the checked-out code.
- property: repository
  ruby_type: String
  required: false
  description_list:
  - markdown: The URI of the code repository.
- property: revision
  ruby_type: String
  required: false
  default_value: HEAD
  description_list:
  - markdown: 'A branch, tag, or commit to be synchronized with git. This can be

      symbolic, like `HEAD` or it can be a source control

      management-specific revision identifier.'
- property: svn_arguments
  ruby_type: String, false
  required: false
  default_value: "--no-auth-cache"
  description_list:
  - markdown: The extra arguments that are passed to the Subversion command.
- property: svn_binary
  ruby_type: String
  required: false
  description_list:
  - markdown: The location of the svn binary.
- property: svn_info_args
  ruby_type: String, false
  required: false
  default_value: "--no-auth-cache"
  description_list:
  - markdown: Use when the `svn info` command is used by Chef Infra Client and arguments
      need to be passed. The `svn_arguments` command does not work when the `svn info`
      command is used.
- property: svn_password
  ruby_type: String
  required: false
  description_list:
  - markdown: The password for a user that has access to the Subversion repository.
- property: svn_username
  ruby_type: String
  required: false
  description_list:
  - markdown: The user name for a user that has access to the Subversion repository.
- property: timeout
  ruby_type: Integer
  required: false
  description_list:
  - markdown: 'The amount of time (in seconds) to wait for a command to execute

      before timing out. When this property is specified using the

      **deploy** resource, the value of the `timeout` property is passed

      from the **deploy** resource to the **subversion** resource.'
- property: user
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: The system user that will own the checked-out code.
examples: "
  Get the latest version of an application\n\n  ``` ruby\n  subversion\
  \ 'CouchDB Edge' do\n    repository 'http://svn.apache.org/repos/asf/couchdb/trunk'\n\
  \    revision 'HEAD'\n    destination '/opt/mysources/couch'\n    action :sync\n\
  \  end\n  ```\n"

---
