---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_client_cron resource
resource: chef_client_cron
aliases:
- "/resource_chef_client_cron.html"
menu:
  infra:
    title: chef_client_cron
    identifier: chef_infra/cookbook_reference/resources/chef_client_cron chef_client_cron
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chef_client_cron** resource to setup the Chef Infra Client to
    run as a cron job. This resource will also create the specified log directory
    if it doesn't already exist.
resource_new_in: '16.0'
syntax_full_code_block: |-
  chef_client_cron 'name' do
    accept_chef_license      true, false # default value: false
    append_log_file          true, false # default value: true
    chef_binary_path         String # default value: "/opt/chef/bin/chef-client"
    comment                  String
    config_directory         String # default value: "/etc/chef"
    daemon_options           Array
    day                      Integer, String # default value: "*"
    environment              Hash
    hour                     Integer, String # default value: "*"
    job_name                 String # default value: "chef-client"
    log_directory            String
    log_file_name            String # default value: "client.log"
    mailto                   String
    minute                   Integer, String # default value: "0,30"
    month                    Integer, String # default value: "*"
    nice                     Integer, String
    splay                    Integer, String # default value: 300
    user                     String # default value: "root"
    weekday                  Integer, String # default value: "*"
    action                   Symbol # defaults to :add if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`chef_client_cron` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`accept_chef_license`, `append_log_file`, `chef_binary_path`, `comment`, `config_directory`,
  `daemon_options`, `day`, `environment`, `hour`, `job_name`, `log_directory`, `log_file_name`,
  `mailto`, `minute`, `month`, `nice`, `splay`, `user`, and `weekday` are the properties
  available to this resource."
actions_list:
  :add:
    markdown: Add a cron job to run Chef Infra Client 
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a cron job for Chef Infra Client
properties_list:
- property: accept_chef_license
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Accept the Chef Online Master License and Services Agreement. See <https://www.chef.io/online-master-agreement/>
- property: append_log_file
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Append to the log file instead of overwriting the log file on each run.
- property: chef_binary_path
  ruby_type: String
  required: false
  default_value: "/opt/chef/bin/chef-client"
  description_list:
  - markdown: The path to the chef-client binary.
- property: comment
  ruby_type: String
  required: false
  description_list:
  - markdown: A comment to place in the cron.d file.
- property: config_directory
  ruby_type: String
  required: false
  default_value: "/etc/chef"
  description_list:
  - markdown: The path of the config directory.
- property: daemon_options
  ruby_type: Array
  required: false
  default_value: []
  description_list:
  - markdown: An array of options to pass to the chef-client command.
- property: day
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The day of month at which Chef Infra Client is to run (1 - 31) or a
      cron pattern such as '1,7,14,21,28'.
- property: environment
  ruby_type: Hash
  required: false
  default_value: {}
  description_list:
  - markdown: A Hash containing additional arbitrary environment variables under which
      the cron job will be run in the form of `({'ENV_VARIABLE' => 'VALUE'})`.
- property: hour
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The hour at which Chef Infra Client is to run (0 - 23) or a cron pattern
      such as '0,12'.
- property: job_name
  ruby_type: String
  required: false
  default_value: chef-client
  description_list:
  - markdown: The name of the cron job to create.
- property: log_directory
  ruby_type: String
  required: false
  default_value: "/Library/Logs/Chef on macOS and /var/log/chef otherwise"
  description_list:
  - markdown: The path of the directory to create the log file in.
- property: log_file_name
  ruby_type: String
  required: false
  default_value: client.log
  description_list:
  - markdown: The name of the log file to use.
- property: mailto
  ruby_type: String
  required: false
  description_list:
  - markdown: The e-mail address to e-mail any cron task failures to.
- property: minute
  ruby_type: Integer, String
  required: false
  default_value: '0,30'
  description_list:
  - markdown: The minute at which Chef Infra Client is to run (0 - 59) or a cron pattern
      such as '0,30'.
- property: month
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The month in the year on which Chef Infra Client is to run (1 - 12,
      jan-dec, or *).
- property: nice
  ruby_type: Integer, String
  required: false
  new_in: '16.5'
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
- property: weekday
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The day of the week on which Chef Infra Client is to run (0-7, mon-sun,
      or *), where Sunday is both 0 and 7.
examples: |
  **Setup Chef Infra Client to run using the default 30 minute cadence**:

  ```ruby
  chef_client_cron "Run Chef Infra Client as a cron job"
  ```

  **Run Chef Infra Client twice a day**:

  ```ruby
  chef_client_cron "Run Chef Infra Client every 12 hours" do
    minute 0
    hour "0,12"
  end
  ```

  **Run Chef Infra Client with extra options passed to the client**:

  ```ruby
  chef_client_cron "Run an override recipe" do
    daemon_options ["--override-runlist mycorp_base::default"]
  end
  ```
---