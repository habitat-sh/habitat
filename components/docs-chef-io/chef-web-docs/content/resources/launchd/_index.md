---
title: launchd resource
resource: launchd
draft: false
aliases:
- /resource_launchd.html
menu:
  infra:
    title: launchd
    identifier: chef_infra/cookbook_reference/resources/launchd launchd
    parent: chef_infra/cookbook_reference/resources
resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **launchd** resource to manage system-wide services (daemons)

    and per-user services (agents) on the macOS platform.'
resource_new_in: '12.8'
handler_types: false
syntax_description: "The launchd resource has the following syntax:\n\n``` ruby\n\
  launchd 'name' do\n  abandon_process_group           true, false\n  backup     \
  \                     Integer, false\n  cookbook                        String\n\
  \  debug                           true, false\n  disabled                     \
  \   true, false # default value: false\n  enable_globbing                 true,\
  \ false\n  enable_transactions             true, false\n  environment_variables\
  \           Hash\n  exit_timeout                    Integer\n  group           \
  \                String, Integer\n  hard_resource_limits            Hash\n  inetd_compatibility\
  \             Hash\n  init_groups                     true, false\n  keep_alive\
  \                      true, false, Hash\n  label                           String\
  \ # default value: 'name' unless specified\n  launch_events                   Hash\n\
  \  launch_only_once                true, false\n  ld_group                     \
  \   String\n  limit_load_from_hosts           Array\n  limit_load_to_hosts     \
  \        Array\n  limit_load_to_session_type      Array, String\n  low_priority_io\
  \                 true, false\n  mach_services                   Hash\n  mode  \
  \                          String, Integer\n  nice                            Integer\n\
  \  on_demand                       true, false\n  owner                        \
  \   String, Integer\n  path                            String\n  plist_hash    \
  \                  Hash\n  process_type                    String\n  program   \
  \                      String\n  program_arguments               Array\n  queue_directories\
  \               Array\n  root_directory                  String\n  run_at_load \
  \                    true, false\n  session_type                    String\n  sockets\
  \                         Hash\n  soft_resource_limits            Array\n  source\
  \                          String\n  standard_error_path             String\n  standard_in_path\
  \                String\n  standard_out_path               String\n  start_calendar_interval\
  \         Hash, Array\n  start_interval                  Integer\n  start_on_mount\
  \                  true, false\n  throttle_interval               Integer\n  time_out\
  \                        Integer\n  type                            String # default\
  \ value: \"daemon\"\n  umask                           Integer\n  username     \
  \                   String\n  wait_for_debugger               true, false\n  watch_paths\
  \                     Array\n  working_directory               String\n  action\
  \                          Symbol # defaults to :create if not specified\nend\n\
  ```"
syntax_code_block: null
syntax_properties_list:
- '`launchd` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`abandon_process_group`, `backup`, `cookbook`, `debug`, `disabled`, `enable_globbing`,
  `enable_transactions`, `environment_variables`, `exit_timeout`, `group`, `hard_resource_limits`,
  `inetd_compatibility`, `init_groups`, `keep_alive`, `label`, `launch_events`, `launch_only_once`,
  `ld_group`, `limit_load_from_hosts`, `limit_load_to_hosts`, `limit_load_to_session_type`,
  `low_priority_io`, `mach_services`, `mode`, `nice`, `on_demand`, `owner`, `path`,
  `plist_hash`, `process_type`, `program`, `program_arguments`, `queue_directories`,
  `root_directory`, `run_at_load`, `session_type`, `sockets`, `soft_resource_limits`,
  `source`, `standard_error_path`, `standard_in_path`, `standard_out_path`, `start_calendar_interval`,
  `start_interval`, `start_on_mount`, `throttle_interval`, `time_out`, `type`, `umask`,
  `username`, `wait_for_debugger`, `watch_paths`, and `working_directory` are the
  properties available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: Default. Create a launchd property list.
  :create_if_missing:
    markdown: Create a launchd property list, if it does not already exist.
  :delete:
    markdown: Delete a launchd property list. This will unload a daemon or agent,
      if loaded.
  :disable:
    markdown: Disable a launchd property list.
  :enable:
    markdown: Create a launchd property list, and then ensure that it is enabled.
      If a launchd property list already exists, but does not match, updates the property
      list to match, and then restarts the daemon or agent.
  :restart:
    markdown: Restart a launchd managed daemon or agent.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: abandon_process_group
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'If a job dies, all remaining processes with the same process ID may

      be kept running. Set to true to kill all remaining processes.'
- property: backup
  ruby_type: Integer, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The number of backups to be kept in `/var/chef/backup`. Set to

      `false` to prevent backups from being kept.'
- property: cookbook
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The name of the cookbook in which the source files are located.
- property: group
  ruby_type: String, Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'When launchd is run as the root user, the group to run the job as.

      If the `username` property is specified and this property is not,

      this value is set to the default group for the user.'
- property: label
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The unique identifier for the job.
- property: mode
  ruby_type: Integer, String
  required: false
  default_value: '''0755'''
  new_in: null
  description_list:
  - markdown: 'A quoted 3-5 character string that defines the octal mode. For

      example: `''755''`, `''0755''`, or `00755`.'
- property: owner
  ruby_type: Integer, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A string or ID that identifies the group owner by user name,

      including fully qualified user names such as `domain\user` or

      `user@domain`. If this value is not specified, existing owners

      remain unchanged and new owner assignments use the current user

      (when necessary).'
- property: path
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The path to the directory. Using a fully qualified path is

      recommended, but is not always required. Default value: the `name`

      of the resource block. See "Syntax" section above for more

      information.'
- property: plist_hash
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A Hash of key value pairs used to create the launchd property list.
- property: session_type
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The type of launchd plist to be created. Possible values: `system`

      (default) or `user`.'
- property: source
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The path to the launchd property list.
- property: supports
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Specify a Hash of supported mount features. Default value:

      `remount: false`.'
- property: type
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The type of resource. Possible values: `daemon` (default), `agent`.'
- property: abandon_process_group
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'If a job dies, all remaining processes with the same process ID may

      be kept running. Set to true to kill all remaining processes.'
- property: debug
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Sets the log mask to `LOG_DEBUG` for this job.
- property: disabled
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Hints to `launchctl` to not submit this job to launchd.
- property: enable_globbing
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Update program arguments before invocation.
- property: enable_transactions
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Track in-progress transactions; if none, then send the `SIGKILL`

      signal.'
- property: environment_variables
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Additional environment variables to set before running a job.
- property: exit_timeout
  ruby_type: Integer
  required: false
  default_value: '20'
  new_in: null
  description_list:
  - markdown: 'The amount of time (in seconds) launchd waits before sending a

      `SIGKILL` signal.'
- property: hard_resource_limits
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A Hash of resource limits to be imposed on a job.
- property: inetd_compatibility
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Specifies if a daemon expects to be run as if it were launched from

      `inetd`. Set to `wait => true` to pass standard input, output, and

      error file descriptors. Set to `wait => false` to call the `accept`

      system call on behalf of the job, and then pass standard input,

      output, and error file descriptors.'
- property: init_groups
  ruby_type: true, false
  required: false
  default_value: 'true'
  new_in: null
  description_list:
  - markdown: Specify if `initgroups` is called before running a job.
- property: keep_alive
  ruby_type: true, false, Hash
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: 'Keep a job running continuously (`true`) or allow demand and

      conditions on the node to determine if the job keeps running

      (`false`).'
- property: launch_events
  ruby_type: Hash
  required: false
  default_value: null
  new_in: '15.1'
  description_list:
  - markdown: 'Specify higher-level event types to be used as launch-on-demand

      event sources.'
- property: launch_only_once
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Specify if a job can be run only one time. Set this value to `true`

      if a job cannot be restarted without a full machine reboot.'
- property: limit_load_from_hosts
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'An array of hosts to which this configuration file does not apply,

      i.e. "apply this configuration file to all hosts not specified in

      this array".'
- property: limit_load_to_hosts
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: An array of hosts to which this configuration file applies.
- property: limit_load_to_session_type
  ruby_type: Array, String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The session type(s) to which this configuration file applies.
- property: low_priority_io
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Specify if the kernel on the node should consider this daemon to be

      low priority during file system I/O.'
- property: mach_services
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Specify services to be registered with the bootstrap subsystem.
- property: nice
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The program scheduling priority value in the range `-20` to `20`.
- property: on_demand
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Keep a job alive. Only applies to macOS version 10.4 (and earlier);

      use `keep_alive` instead for newer versions.'
- property: process_type
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The intended purpose of the job: `Adaptive`, `Background`,

      `Interactive`, or `Standard`.'
- property: program
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The first argument of `execvp`, typically the file name associated

      with the file to be executed. This value must be specified if

      `program_arguments` is not specified, and vice-versa.'
- property: program_arguments
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The second argument of `execvp`. If `program` is not specified, this

      property must be specified and will be handled as if it were the

      first argument.'
- property: queue_directories
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'An array of non-empty directories which, if any are modified, will

      cause a job to be started.'
- property: root_directory
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '`chroot` to this directory, and then run the job.'
- property: run_at_load
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Launch a job once (at the time it is loaded).
- property: sockets
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A Hash of on-demand sockets that notify launchd when a job should be

      run.'
- property: soft_resource_limits
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A Hash of resource limits to be imposed on a job.
- property: standard_error_path
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The file to which standard error (`stderr`) is sent.
- property: standard_in_path
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The file to which standard input (`stdin`) is sent.
- property: standard_out_path
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The file to which standard output (`stdout`) is sent.
- property: start_calendar_interval
  ruby_type: Hash
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'A Hash (similar to `crontab`) that defines the calendar frequency at

      which a job is started. For example:

      `{ Minute => "0", Hour => "20", Day => "*", Weekday => "1-5", Month => "*" }`

      will run a job at 8:00 PM every day, Monday through Friday, every

      month of the year.'
- property: start_interval
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: The frequency (in seconds) at which a job is started.
- property: start_on_mount
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: Start a job every time a file system is mounted.
- property: throttle_interval
  ruby_type: Integer
  required: false
  default_value: '10'
  new_in: null
  description_list:
  - markdown: The frequency (in seconds) at which jobs are allowed to spawn.
- property: time_out
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'The amount of time (in seconds) a job may be idle before it times

      out. If no value is specified, the default timeout value for launchd

      will be used.'
- property: umask
  ruby_type: Integer
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A decimal value to pass to `umask` before running a job.
- property: username
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: When launchd is run as the root user, the user to run the job as.
- property: wait_for_debugger
  ruby_type: true, false
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'Specify if launchd has a job wait for a debugger to attach before

      executing code.'
- property: watch_paths
  ruby_type: Array
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: 'An array of paths which, if any are modified, will cause a job to be

      started.'
- property: working_directory
  ruby_type: String
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: '`chdir` to this directory, and then run the job.'
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
  Create a Launch Daemon from a cookbook file\n\n  ``` ruby\n  launchd\
  \ 'com.chef.every15' do\n    source 'com.chef.every15.plist'\n  end\n  ```\n\n \
  \ Create a Launch Daemon using keys\n\n  ``` ruby\n  launchd 'call.mom.weekly' do\n\
  \    program '/Library/scripts/call_mom.sh'\n    start_calendar_interval 'Weekday'\
  \ => 7, 'Hourly' => 10\n    time_out 300\n  end\n  ```\n\n  Remove a Launch Daemon\n\
  \n  ``` ruby\n  launchd 'com.chef.every15' do\n    action :delete\n  end\n  ```\n"

---