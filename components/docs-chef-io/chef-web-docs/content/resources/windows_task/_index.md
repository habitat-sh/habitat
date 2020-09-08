---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_task resource
resource: windows_task
aliases:
- "/resource_windows_task.html"
menu:
  infra:
    title: windows_task
    identifier: chef_infra/cookbook_reference/resources/windows_task windows_task
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_task** resource to create, delete or run a Windows scheduled
    task.
- note:
    markdown: 'The `windows_task` resource that was provided as part of the `windows`

      cookbook included the `:change` action, which has been removed from

      `windows_task` in Chef client. The `:create` action can be used instead

      to update an existing task.'
resource_new_in: '13.0'
syntax_full_code_block: |-
  windows_task 'name' do
    command                             String
    cwd                                 String
    day                                 String, Integer
    description                         String
    disallow_start_if_on_batteries      true, false # default value: false
    execution_time_limit                String, Integer # default value: "PT72H (72 hours in ISO8601 duration format)"
    force                               true, false # default value: false
    frequency                           Symbol
    frequency_modifier                  Integer, String # default value: 1
    idle_time                           Integer
    interactive_enabled                 true, false # default value: false
    minutes_duration                    String, Integer
    minutes_interval                    String, Integer
    months                              String
    password                            String
    priority                            Integer # default value: 7
    random_delay                        String, Integer
    run_level                           Symbol # default value: :limited
    start_day                           String # default value: The current date.
    start_time                          String
    start_when_available                true, false # default value: false
    stop_if_going_on_batteries          true, false # default value: false
    task_name                           String # default value: 'name' unless specified
    user                                String # default value: The localized SYSTEM user for the node.
    action                              Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_task` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`command`, `cwd`, `day`, `description`, `disallow_start_if_on_batteries`, `execution_time_limit`,
  `force`, `frequency`, `frequency_modifier`, `idle_time`, `interactive_enabled`,
  `minutes_duration`, `minutes_interval`, `months`, `password`, `priority`, `random_delay`,
  `run_level`, `start_day`, `start_time`, `start_when_available`, `stop_if_going_on_batteries`,
  `task_name`, and `user` are the properties available to this resource."
actions_list:
  :create:
    markdown: Creates a task, or updates an existing task if any property has changed.
  :delete:
    markdown: Deletes a task.
  :disable:
    markdown: Disables a task.
  :enable:
    markdown: Enables a task.
  :end:
    markdown: Ends a task.
  :run:
    markdown: Runs a task.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: command
  ruby_type: String
  required: false
  description_list:
  - markdown: The command to be executed by the windows scheduled task.
- property: cwd
  ruby_type: String
  required: false
  description_list:
  - markdown: The directory the task will be run from.
- property: day
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: "The day(s) on which the task runs.\n\n-   Use with frequency `:monthly`\
      \ and `:weekly` tasks,\n-   Valid values with frequency `:weekly` are `MON-SUN`\
      \ or `\\*`.\n-   Valid values with frequency `:monthly` are `1-31` or `MON`\
      \ to\n    `SUN` and `LASTDAY`.\n    -   Use `MON-SUN` or `LASTDAY` if you are\
      \ setting\n        `frequency_modifier` as `\"FIRST, SECOND, THIRD etc.\"` else\n\
      \        use `1-31`.\n    -   Multiple days should be comma separated. e.g `\"\
      1, 2, 3\"` or\n        `\"MON, WEN, FRI\"`."
- property: description
  ruby_type: String
  required: false
  new_in: '14.7'
  description_list:
  - markdown: The task description.
- property: disallow_start_if_on_batteries
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '14.4'
  description_list:
  - markdown: Disallow start of the task if the system is running on battery power.
- property: execution_time_limit
  ruby_type: String, Integer
  required: false
  default_value: PT72H (72 hours in ISO8601 duration format)
  description_list:
  - markdown: The maximum time the task will run. This field accepts either seconds
      or an ISO8601 duration value.
- property: force
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: When used with create, will update the task.
- property: frequency
  ruby_type: Symbol
  required: false
  allowed_values: ":daily, :hourly, :minute, :monthly, :none, :on_idle, :on_logon,
    :once, :onstart, :weekly"
  description_list:
  - markdown: "-   Frequency with which to run the task.\n-   This is a mandatory\
      \ property in Chef 14.1\n-   Valid values: `:minute`, `:hourly`, `:daily`, `:weekly`,\n\
      \    `:monthly`, `:none`, `:once`, `:on_logon`, `:onstart`,\n    `:on_idle`.\n\
      -   The `:once` value requires the `start_time` property."
- property: frequency_modifier
  ruby_type: Integer, String
  required: false
  default_value: '1'
  description_list:
  - markdown: "-   For frequency `:minute` valid values are 1 to 1439\n-   For frequency\
      \ `:hourly` valid values are 1 to 23\n-   For frequency `:daily` valid values\
      \ are 1 to 365\n-   For frequency `:weekly` valid values are 1 to 52\n-   \n\
      \n    For frequency `:monthly` valid values are `('FIRST', 'SECOND', 'THIRD',\
      \ 'FOURTH', 'LAST')` OR `1-12`.\n\n    :   -   e.g. If user want to run the\
      \ task on\n            `second week of the month` use `frequency_modifier`\n\
      \            value as `SECOND`. Multiple values for weeks of the\n         \
      \   month should be comma separated e.g.\n            `\"FIRST, THIRD, LAST\"\
      `.\n        -   To run task every (n) months user values '1-12'."
- property: idle_time
  ruby_type: Integer
  required: false
  description_list:
  - markdown: For `:on_idle` frequency, the time (in minutes) without user activity
      that must pass to trigger the task, from `1` - `999`.
- property: interactive_enabled
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Allow task to run interactively or non-interactively. Requires user
      and password to also be set.
- property: minutes_duration
  ruby_type: String, Integer
  required: false
  description_list: []
- property: minutes_interval
  ruby_type: String, Integer
  required: false
  description_list: []
- property: months
  ruby_type: String
  required: false
  description_list:
  - markdown: 'The Months of the year on which the task runs, such as: `JAN, FEB`
      or `*`. Multiple months should be comma delimited. e.g. `Jan, Feb, Mar, Dec`.'
- property: password
  ruby_type: String
  required: false
  description_list:
  - markdown: The user's password. The user property must be set if using this property.
- property: priority
  ruby_type: Integer
  required: false
  default_value: '7'
  description_list:
  - markdown: Use to set Priority Levels range from 0 to 10.
- property: random_delay
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: Delays the task up to a given time (in seconds).
- property: run_level
  ruby_type: Symbol
  required: false
  default_value: ":limited"
  allowed_values: ":highest, :limited"
  description_list:
  - markdown: Run with `:limited` or `:highest` privileges.
- property: start_day
  ruby_type: String
  required: false
  default_value: The current date.
  description_list:
  - markdown: Specifies the first date on which the task runs in **MM/DD/YYYY** format.
- property: start_time
  ruby_type: String
  required: false
  description_list:
  - markdown: Specifies the start time to run the task, in **HH:mm** format.
- property: start_when_available
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '14.15'
  description_list:
  - markdown: To start the task at any time after its scheduled time has passed.
- property: stop_if_going_on_batteries
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '14.4'
  description_list:
  - markdown: Scheduled task option when system is switching on battery.
- property: task_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: 'An optional property to set the task name if it differs from the resource
      block''s name. Example: `Task Name` or `/Task Name`'
- property: user
  ruby_type: String
  required: false
  default_value: The localized SYSTEM user for the node.
  description_list:
  - markdown: The user to run the task as.
examples: |
  **Create a scheduled task to run every 15 minutes as the Administrator user**:

  ```ruby
  windows_task 'chef-client' do
    user 'Administrator'
    password 'password'
    command 'chef-client'
    run_level :highest
    frequency :minute
    frequency_modifier 15
  end
  ```

  **Create a scheduled task to run every 2 days**:

  ``` ruby
  windows_task 'chef-client' do
    command 'chef-client'
    run_level :highest
    frequency :daily
    frequency_modifier 2
  end
  ```

  **Create a scheduled task to run on specific days of the week**:

  ```ruby
  windows_task 'chef-client' do
    command 'chef-client'
    run_level :highest
    frequency :weekly
    day 'Mon, Thu'
  end
  ```

  **Create a scheduled task to run only once**:

  ```ruby
  windows_task 'chef-client' do
    command 'chef-client'
    run_level :highest
    frequency :once
    start_time "16:10"
  end
  ```

  **Create a scheduled task to run on current day every 3 weeks and delay upto 1 min**:

  ```ruby
  windows_task 'chef-client' do
    command 'chef-client'
    run_level :highest
    frequency :weekly
    frequency_modifier 3
    random_delay '60'
  end
  ```

  **Create a scheduled task to run weekly starting on Dec 28th 2018**:

  ```ruby
  windows_task 'chef-client 8' do
    command 'chef-client'
    run_level :highest
    frequency :weekly
    start_day '12/28/2018'
  end
  ```

  **Create a scheduled task to run every Monday, Friday every 2 weeks**:

  ```ruby
  windows_task 'chef-client' do
    command 'chef-client'
    run_level :highest
    frequency :weekly
    frequency_modifier 2
    day 'Mon, Fri'
  end
  ```

  **Create a scheduled task to run when computer is idle with idle duration 20 min**:
  ```ruby
  windows_task 'chef-client' do
    command 'chef-client'
    run_level :highest
    frequency :on_idle
    idle_time 20
  end
  ```

  **Delete a task named "old task"**:
  ```ruby
  windows_task 'old task' do
    action :delete
  end
  ```

  **Enable a task named "chef-client"**:
  ```ruby
  windows_task 'chef-client' do
    action :enable
  end
  ```

  **Disable a task named "ProgramDataUpdater" with TaskPath "\Microsoft\Windows\Application Experience\ProgramDataUpdater"**
  ```ruby
  windows_task '\Microsoft\Windows\Application Experience\ProgramDataUpdater' do
    action :disable
  end
  ```
---