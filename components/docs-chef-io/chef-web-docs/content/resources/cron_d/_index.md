---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: cron_d resource
resource: cron_d
aliases:
- "/resource_cron_d.html"
menu:
  infra:
    title: cron_d
    identifier: chef_infra/cookbook_reference/resources/cron_d cron_d
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **cron_d** resource to manage cron job files in the /etc/cron.d

    directory.'
- warning:
    markdown: 'Chef Infra Client also ships with the **cron** resource for managing
      the

      monolithic `/etc/crontab` file on platforms that lack cron.d support.

      See the [cron resource](/resources/cron/) for information on using

      that resource.'
resource_new_in: '14.4'
syntax_description: "A **cron_d** resource block manages cron.d files. For example,\
  \ to get a\nweekly cookbook report from the Chef Supermarket:\n\n``` ruby\ncron_d\
  \ 'cookbooks_report' do\n  action :create\n  minute '0'\n  hour '0'\n  weekday '1'\n\
  \  user 'getchef'\n  mailto 'sysadmin@example.com'\n  home '/srv/supermarket/shared/system'\n\
  \  command %W{\n    cd /srv/supermarket/current &&\n    env RUBYLIB=\"/srv/supermarket/current/lib\"\
  \n    RAILS_ASSET_ID=`git rev-parse HEAD` RAILS_ENV=\"#{rails_env}\"\n    bundle\
  \ exec rake cookbooks_report\n  }.join(' ')\nend\n```"
syntax_full_code_block: |-
  cron_d 'name' do
    command               String
    comment               String
    cron_name             String # default value: 'name' unless specified
    day                   Integer, String # default value: "*"
    environment           Hash
    home                  String
    hour                  Integer, String # default value: "*"
    mailto                String
    minute                Integer, String # default value: "*"
    mode                  String, Integer # default value: "0600"
    month                 Integer, String # default value: "*"
    path                  String
    predefined_value      String
    random_delay          Integer
    shell                 String
    time_out              Hash
    user                  String # default value: "root"
    weekday               Integer, String, Symbol # default value: "*"
    action                Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`cron_d` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`command`, `comment`, `cron_name`, `day`, `environment`, `home`, `hour`, `mailto`,
  `minute`, `mode`, `month`, `path`, `predefined_value`, `random_delay`, `shell`,
  `time_out`, `user`, and `weekday` are the properties available to this resource."
actions_list:
  :create:
    markdown: Default. Add a cron definition file to /etc/cron.d.
  :delete:
    markdown: Remove a cron definition file from /etc/cron.d if it exists.
  :create_if_missing:
    markdown: Add a cron definition file to /etc/cron.d, but do not update an existing
      file.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: command
  ruby_type: String
  required: true
  description_list:
  - markdown: The command to be run, or the path to a file that contains the command
      to be run.
- property: comment
  ruby_type: String
  required: false
  description_list:
  - markdown: A comment to place in the cron.d file.
- property: cron_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the cron name if it differs from the resource
      block's name.
- property: day
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The day of month at which the cron entry should run (`1 - 31`).
- property: environment
  ruby_type: Hash
  required: false
  description_list:
  - markdown: 'A Hash containing additional arbitrary environment variables under
      which the cron job will be run in the form of `({''ENV_VARIABLE'' => ''VALUE''})`.
      **Note**: These variables must exist for a command to be run successfully.'
- property: home
  ruby_type: String
  required: false
  description_list:
  - markdown: Set the `HOME` environment variable.
- property: hour
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The hour at which the cron entry is to run (`0 - 23`).
- property: mailto
  ruby_type: String
  required: false
  description_list:
  - markdown: Set the `MAILTO` environment variable.
- property: minute
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The minute at which the cron entry should run (`0 - 59`).
- property: mode
  ruby_type: String, Integer
  required: false
  default_value: '"0600"'
  description_list:
  - markdown: The octal mode of the generated crontab file.
- property: month
  ruby_type: Integer, String
  required: false
  default_value: "*"
  description_list:
  - markdown: The month in the year on which a cron entry is to run (`1 - 12`, `jan-dec`,
      or `*`).
- property: path
  ruby_type: String
  required: false
  description_list:
  - markdown: Set the `PATH` environment variable.
- property: predefined_value
  ruby_type: String
  required: false
  allowed_values: '"@annually", "@daily", "@hourly", "@midnight", "@monthly", "@reboot",
    "@weekly", "@yearly"'
  description_list:
  - markdown: Schedule your cron job with one of the special predefined value instead
      of ** * pattern.
- property: random_delay
  ruby_type: Integer
  required: false
  description_list:
  - markdown: Set the `RANDOM_DELAY` environment variable in the cron.d file.
- property: shell
  ruby_type: String
  required: false
  description_list:
  - markdown: Set the `SHELL` environment variable.
- property: time_out
  ruby_type: Hash
  required: false
  new_in: '15.7'
  description_list:
  - markdown: |-
      A Hash of timeouts in the form of `({'OPTION' => 'VALUE'})`. Accepted valid options are:
        - `preserve-status` (BOOL, default: 'false'),
        - `foreground` (BOOL, default: 'false'),
        - `kill-after` (in seconds),
        - `signal` (a name like 'HUP' or a number)
- property: user
  ruby_type: String
  required: false
  default_value: root
  description_list:
  - markdown: The name of the user that runs the command.
- property: weekday
  ruby_type: Integer, String, Symbol
  required: false
  default_value: "*"
  description_list:
  - markdown: The day of the week on which this entry is to run (`0-7`, `mon-sun`,
      `monday-sunday`, or `*`), where Sunday is both `0` and `7`.
examples: |
  **Run a program on the fifth hour of the day**

  ```ruby
  cron_d 'noop' do
    hour '5'
    minute '0'
    command '/bin/true'
  end
  ```

  **Run an entry if a folder exists**

  ```ruby
  cron_d 'ganglia_tomcat_thread_max' do
    command "/usr/bin/gmetric
      -n 'tomcat threads max'
      -t uint32
      -v '/usr/local/bin/tomcat-stat
      --thread-max'"
    only_if { ::File.exist?('/home/jboss') }
  end
  ```

  **Run an entry every Saturday, 8:00 AM**

  ```ruby
  cron_d 'name_of_cron_entry' do
    minute '0'
    hour '8'
    weekday '6'
    mailto 'admin@example.com'
    command "/bin/true"
    action :create
  end
  ```

  **Run an entry at 8:00 PM, every weekday (Monday through Friday), but only in November**

  ```ruby
  cron_d 'name_of_cron_entry' do
    minute '0'
    hour '20'
    day '*'
    month '11'
    weekday '1-5'
    command "/bin/true"
    action :create
  end
  ```

  **Remove a cron job by name**:

  ```ruby
  cron_d 'job_to_remove' do
    action :delete
  end
  ```
---
