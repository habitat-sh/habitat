---
resource_reference: true
properties_shortcode:
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_client_launchd resource
resource: chef_client_launchd
aliases:
- "/resource_chef_client_launchd.html"
menu:
  infra:
    title: chef_client_launchd
    identifier: chef_infra/cookbook_reference/resources/chef_client_launchd chef_client_launchd
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chef_client_launchd** resource to configure the Chef Infra Client
    to run on a schedule.
resource_new_in: '16.5'
syntax_full_code_block: |-
  chef_client_launchd 'name' do
    accept_chef_license      true, false # default value: false
    chef_binary_path         String # default value: "/opt/chef/bin/chef-client"
    config_directory         String # default value: "/etc/chef"
    daemon_options           Array
    environment              Hash
    interval                 Integer, String # default value: 30
    log_directory            String # default value: "/Library/Logs/Chef"
    log_file_name            String # default value: "client.log"
    low_priority_io          true, false # default value: true
    nice                     Integer, String
    splay                    Integer, String # default value: 300
    user                     String # default value: "root"
    working_directory        String # default value: "/var/root"
    action                   Symbol # defaults to :enable if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`chef_client_launchd` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`accept_chef_license`, `chef_binary_path`, `config_directory`, `daemon_options`,
  `environment`, `interval`, `log_directory`, `log_file_name`, `low_priority_io`,
  `nice`, `splay`, `user`, and `working_directory` are the properties available to
  this resource."
actions_list:
  :disable:
    markdown: Disable running Chef Infra Client on a schedule using launchd
  :enable:
    markdown: Enable running Chef Infra Client on a schedule using launchd
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: accept_chef_license
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Accept the Chef Online Master License and Services Agreement. See <https://www.chef.io/online-master-agreement/>
- property: chef_binary_path
  ruby_type: String
  required: false
  default_value: "/opt/chef/bin/chef-client"
  description_list:
  - markdown: The path to the chef-client binary.
- property: config_directory
  ruby_type: String
  required: false
  default_value: "/etc/chef"
  description_list:
  - markdown: The path of the config directory.
- property: daemon_options
  ruby_type: Array
  required: false
  description_list:
  - markdown: An array of options to pass to the chef-client command.
- property: environment
  ruby_type: Hash
  required: false
  description_list:
  - markdown: A Hash containing additional arbitrary environment variables under which
      the launchd daemon will be run in the form of `({'ENV_VARIABLE' => 'VALUE'})`.
- property: interval
  ruby_type: Integer, String
  required: false
  default_value: '30'
  description_list:
  - markdown: Time in minutes between Chef Infra Client executions.
- property: log_directory
  ruby_type: String
  required: false
  default_value: "/Library/Logs/Chef"
  description_list:
  - markdown: The path of the directory to create the log file in.
- property: log_file_name
  ruby_type: String
  required: false
  default_value: client.log
  description_list:
  - markdown: The name of the log file to use.
- property: low_priority_io
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Run the chef-client process with low priority disk IO
- property: nice
  ruby_type: Integer, String
  required: false
  description_list:
  - markdown: The process priority to run the chef-client process at. A value of -20
      is the highest priority and 19 is the lowest priority.
- property: splay
  ruby_type: Integer, String
  required: false
  default_value: '300'
  description_list:
  - markdown: A random number of seconds between 0 and X to add to interval so that
      all chef-client commands don't execute at the same time.
- property: user
  ruby_type: String
  required: false
  default_value: root
  description_list:
  - markdown: The name of the user that Chef Infra Client runs as.
- property: working_directory
  ruby_type: String
  required: false
  default_value: "/var/root"
  description_list:
  - markdown: The working directory to run the Chef Infra Client from.
examples: |
  **Set the Chef Infra Client to run on a schedule**:

  ```ruby
  chef_client_launchd 'Setup the Chef Infra Client to run every 30 minutes' do
    interval 30
    action :enable
  end
  ```

  **Disable the Chef Infra Client running on a schedule**:

  ```ruby
  chef_client_launchd 'Prevent the Chef Infra Client from running on a schedule' do
    action :disable
  end
  ```
---