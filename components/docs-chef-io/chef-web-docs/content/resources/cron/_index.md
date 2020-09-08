---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: cron resource
resource: cron
aliases:
- "/resource_cron.html"
menu:
  infra:
    title: cron
    identifier: chef_infra/cookbook_reference/resources/cron cron
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **cron** resource to manage cron entries for time-based job scheduling.
- warning:
    markdown: 'The **cron** resource should only be used to modify an entry in a

      crontab file. The `cron_d` resource directly manages cron.d files. This

      resource ships in Chef 14.4 or later and can also be found in the

      [cron](https://github.com/chef-cookbooks/cron) cookbook) for previous

      chef-client releases.'
resource_new_in: null
handler_types: false
syntax_description: "A **cron** resource block manages cron entries. For example,\
  \ to get a\nweekly cookbook report from the Chef Supermarket:\n\n``` ruby\ncron\
  \ 'cookbooks_report' do\n  action :create\n  minute '0'\n  hour '0'\n  weekday '1'\n\
  \  user 'chefio'\n  mailto 'sysadmin@example.com'\n  home '/srv/supermarket/shared/system'\n\
  \  command %W{\n    cd /srv/supermarket/current &&\n    env RUBYLIB=\"/srv/supermarket/current/lib\"\
  \n    RAILS_ASSET_ID=`git rev-parse HEAD` RAILS_ENV=\"#{rails_env}\"\n    bundle\
  \ exec rake cookbooks_report\n  }.join(' ')\nend\n```"
syntax_full_code_block: |-
  cron 'name' do
    command          String
    day              Integer, String # default value: "*"
    environment      Hash
    home             String
    hour             Integer, String # default value: "*"
    mailto           String
    minute           Integer, String # default value: "*"
    month            Integer, String # default value: "*"
    path             String
    shell            String
    time             Symbol
    time_out         Hash
    user             String # default value: "root"
    weekday          Integer, String, Symbol # default value: "*"
    action           Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`cron` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`command`, `day`, `environment`, `home`, `hour`, `mailto`, `minute`, `month`, `path`,
  `shell`, `time`, `time_out`, `user`, and `weekday` are the properties available
  to this resource."
actions_list:
  :create:
    markdown: Default. Create an entry in a cron table file (crontab). If an entry
      already exists (but does not match), update that entry to match.
  :delete:
    markdown: Delete an entry from a cron table file (crontab).
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: command
  ruby_type: String
  required: true
  description_list:
  - markdown: The command to be run, or the path to a file that contains the command
      to be run.
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
- property: shell
  ruby_type: String
  required: false
  description_list:
  - markdown: Set the `SHELL` environment variable.
- property: time
  ruby_type: Symbol
  required: false
  allowed_values: ":annually, :daily, :hourly, :midnight, :monthly, :reboot, :weekly,
    :yearly"
  description_list:
  - markdown: A time interval.
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
  - markdown: The name of the user that runs the command. If the user property is
      changed, the original user for the crontab program continues to run until that
      crontab program is deleted. This property is not applicable on the AIX platform.
- property: weekday
  ruby_type: Integer, String, Symbol
  required: false
  default_value: "*"
  description_list:
  - markdown: The day of the week on which this entry is to run (`0-7`, `mon-sun`,
      `monday-sunday`, or `*`), where Sunday is both `0` and `7`.
examples: "
  Run a program at a specified interval\n\n  ``` ruby\n  cron 'noop'\
  \ do\n    hour '5'\n    minute '0'\n    command '/bin/true'\n  end\n  ```\n\n  Run\
  \ an entry if a folder exists\n\n  ``` ruby\n  cron 'ganglia_tomcat_thread_max'\
  \ do\n    command \"/usr/bin/gmetric\n      -n 'tomcat threads max'\n      -t uint32\n\
  \      -v '/usr/local/bin/tomcat-stat\n      --thread-max'\"\n    only_if { ::File.exist?('/home/jboss')\
  \ }\n  end\n  ```\n\n  Run every Saturday, 8:00 AM\n\n  The following example shows\
  \ a schedule that will run every hour at 8:00\n  each Saturday morning, and will\
  \ then send an email to\n  \"<admin@example.com>\" after each run.\n\n  ``` ruby\n\
  \  cron 'name_of_cron_entry' do\n    minute '0'\n    hour '8'\n    weekday '6'\n\
  \    mailto 'admin@example.com'\n    action :create\n  end\n  ```\n\n  Run only\
  \ in November\n\n  The following example shows a schedule that will run at 8:00\
  \ PM, every\n  weekday (Monday through Friday), but only in November:\n\n  ``` ruby\n\
  \  cron 'name_of_cron_entry' do\n    minute '0'\n    hour '20'\n    day '*'\n  \
  \  month '11'\n    weekday '1-5'\n    action :create\n  end\n  ```\n"

---
