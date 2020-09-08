---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_client_systemd_timer resource
resource: chef_client_systemd_timer
aliases:
- "/resource_chef_client_systemd_timer.html"
menu:
  infra:
    title: chef_client_systemd_timer
    identifier: chef_infra/cookbook_reference/resources/chef_client_systemd_timer
      chef_client_systemd_timer
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chef_client_systemd_timer** resource to setup the Chef Infra
    Client to run as a systemd timer.
resource_new_in: '16.0'
syntax_full_code_block: |-
  chef_client_systemd_timer 'name' do
    accept_chef_license      true, false # default value: false
    chef_binary_path         String # default value: "/opt/chef/bin/chef-client"
    config_directory         String # default value: "/etc/chef"
    daemon_options           Array
    delay_after_boot         String # default value: "1min"
    description              String # default value: "Chef Infra Client periodic execution"
    environment              Hash
    interval                 String # default value: "30min"
    job_name                 String # default value: "chef-client"
    run_on_battery           true, false # default value: true
    splay                    String # default value: "5min"
    user                     String # default value: "root"
    action                   Symbol # defaults to :add if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`chef_client_systemd_timer` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`accept_chef_license`, `chef_binary_path`, `config_directory`, `daemon_options`,
  `delay_after_boot`, `description`, `environment`, `interval`, `job_name`, `run_on_battery`,
  `splay`, and `user` are the properties available to this resource."
actions_list:
  :add:
    markdown: Add a systemd timer that runs Chef Infra Client.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a systemd timer that runs Chef Infra Client.
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
  default_value: []
  description_list:
  - markdown: An array of options to pass to the chef-client command.
- property: delay_after_boot
  ruby_type: String
  required: false
  default_value: 1min
  description_list:
  - markdown: The time to wait after booting before the interval starts. This is expressed
      as a systemd time span such as `300seconds`, `1hr`, or `1m`. See <https://www.freedesktop.org/software/systemd/man/systemd.time.html>
      for a complete list of allowed time span values.
- property: description
  ruby_type: String
  required: false
  default_value: Chef Infra Client periodic execution
  description_list:
  - markdown: The description to add to the systemd timer. This will be displayed
      when running `systemctl status` for the timer.
- property: environment
  ruby_type: Hash
  required: false
  default_value: {}
  description_list:
  - markdown: A Hash containing additional arbitrary environment variables under which
      the systemd timer will be run in the form of `({'ENV_VARIABLE' => 'VALUE'})`.
- property: interval
  ruby_type: String
  required: false
  default_value: 30min
  description_list:
  - markdown: The interval to wait between executions. This is expressed as a systemd
      time span such as `300seconds`, `1hr`, or `1m`. See <https://www.freedesktop.org/software/systemd/man/systemd.time.html>
      for a complete list of allowed time span values.
- property: job_name
  ruby_type: String
  required: false
  default_value: chef-client
  description_list:
  - markdown: The name of the system timer to create.
- property: run_on_battery
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Run the timer for Chef Infra Client if the system is on battery.
- property: splay
  ruby_type: String
  required: false
  default_value: 5min
  description_list:
  - markdown: A interval between 0 and X to add to the interval so that all chef-client
      commands don't execute at the same time. This is expressed as a systemd time
      span such as `300seconds`, `1hr`, or `1m`. See <https://www.freedesktop.org/software/systemd/man/systemd.time.html>
      for a complete list of allowed time span values.
- property: user
  ruby_type: String
  required: false
  default_value: root
  description_list:
  - markdown: The name of the user that Chef Infra Client runs as.
examples: |
  **Setup Chef Infra Client to run using the default 30 minute cadence**:

  ```ruby
  chef_client_systemd_timer "Run Chef Infra Client as a systemd timer"
  ```

  **Run Chef Infra Client every 1 hour**:

  ```ruby
  chef_client_systemd_timer "Run Chef Infra Client every 1 hour" do
    interval "1hr"
  end
  ```

  **Run Chef Infra Client with extra options passed to the client**:

  ```ruby
  chef_client_systemd_timer "Run an override recipe" do
    daemon_options ["--override-runlist mycorp_base::default"]
  end
  ```
---