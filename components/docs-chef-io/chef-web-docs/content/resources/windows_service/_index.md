---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_service resource
resource: windows_service
aliases:
- "/resource_windows_service.html"
menu:
  infra:
    title: windows_service
    identifier: chef_infra/cookbook_reference/resources/windows_service windows_service
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_service** resource to create, delete, or manage a service
    on the Microsoft Windows platform.
syntax_description: "A **windows_service** resource block manages the state of a service\
  \ on\na machine that is running Microsoft Windows. For example:\n\n``` ruby\nwindows_service\
  \ 'BITS' do\n  action :configure_startup\n  startup_type :manual\nend\n```"
syntax_full_code_block: "windows_service 'name' do\n  binary_path_name      String\n\
  \  delayed_start         true, false # default value: false\n  dependencies    \
  \      String, Array\n  description           String\n  desired_access        Integer\
  \ # default value: 983551\n  display_name          String\n  error_control     \
  \    Integer # default value: 1\n  init_command          String\n  load_order_group\
  \      String\n  pattern               String\n  reload_command        String, false\n\
  \  restart_command       String, false\n  run_as_password       String\n  run_as_user\
  \           String # default value: \"LocalSystem\"\n  service_name          String\
  \ # default value: 'name' unless specified\n  service_type          Integer # default\
  \ value: \"SERVICE_WIN32_OWN_PROCESS\"\n  start_command         String, false\n\
  \  startup_type          Symbol # default value: :automatic\n  status_command  \
  \      String, false\n  stop_command          String, false\n  supports        \
  \      Hash # default value: {\"restart\"=>nil, \"reload\"=>nil, \"status\"=>nil}\n\
  \  timeout               Integer\n  action                Symbol # defaults to :nothing\
  \ if not specified\nend"
syntax_full_properties_list:
- '`windows_service` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`binary_path_name`, `display_name`, `desired_access`, `delayed_start`, `dependencies`,
  `description`, `error_control`, `init_command`, `load_order_group`, `pattern`, `reload_command`,
  `restart_command`, `run_as_password`, `run_as_user`, `service_name`, `service_type`,
  `start_command`, `startup_type`, `status_command`, `stop_command`, `supports`, and
  `timeout` are properties of this resource, with the Ruby type shown. See "Properties"
  section below for more information about all of the properties that may be used
  with this resource.'
actions_list:
  :configure:
    markdown: "Configure a pre-existing service.\n *New in Chef Client 14.0.*"
  :configure_startup:
    markdown: Configure a service based on the value of the `startup_type` property.
  :create:
    markdown: "Create the service based on the value of the `binary_path_name`, `service_name`\
      \ and/or `display_name` property.\n *New in Chef Client 14.0.*"
  :delete:
    markdown: "Delete the service based on the value of the `service_name` property.\n\
      \ *New in Chef Client 14.0.*"
  :disable:
    markdown: Disable a service. This action is equivalent to a `Disabled` startup
      type on the Microsoft Windows platform.
  :enable:
    markdown: Enable a service at boot. This action is equivalent to an `Automatic`
      startup type on the Microsoft Windows platform.
  :nothing:
    markdown: Default. Do nothing with a service.
  :reload:
    markdown: Reload the configuration for this service. This action is not supported
      on the Windows platform and will raise an error if used.
  :restart:
    markdown: Restart a service.
  :start:
    markdown: Start a service, and keep it running until stopped or disabled.
  :stop:
    markdown: Stop a service.
properties_list:
- property: binary_path_name
  ruby_type: String
  required: false
  new_in: '14.0'
  description_list:
  - markdown: The fully qualified path to the service binary file. The path can also
      include arguments for an auto-start service. This is required for `:create`
      and `:configure` actions
- property: delayed_start
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: '14.0'
  description_list:
  - markdown: Set the startup type to delayed start. This only applies if `startup_type`
      is `:automatic`
- property: dependencies
  ruby_type: String, Array
  required: false
  new_in: '14.0'
  description_list:
  - markdown: A pointer to a double null-terminated array of null-separated names
      of services or load ordering groups that the system must start before this service.
      Specify `nil` or an empty string if the service has no dependencies. Dependency
      on a group means that this service can run if at least one member of the group
      is running after an attempt to start all members of the group.
- property: description
  ruby_type: String
  required: false
  new_in: '14.0'
  description_list:
  - markdown: Description of the service.
- property: desired_access
  ruby_type: Integer
  required: false
  default_value: '983551'
  new_in: '14.0'
  description_list:
  - markdown: 
- property: display_name
  ruby_type: String
  required: false
  new_in: '14.0'
  description_list:
  - markdown: The display name to be used by user interface programs to identify the
      service. This string has a maximum length of 256 characters.
- property: error_control
  ruby_type: Integer
  required: false
  default_value: '1'
  new_in: '14.0'
  description_list:
  - markdown: 
- property: load_order_group
  ruby_type: String
  required: false
  new_in: '14.0'
  description_list:
  - markdown: The name of the service's load ordering group(s).
- property: pattern
  ruby_type: String
  required: false
  default_value: The value provided to 'service_name' or the resource block's name
  description_list:
  - markdown: The pattern to look for in the process table.
- property: reload_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to tell a service to reload its configuration.
- property: restart_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to restart a service.
- property: run_as_password
  ruby_type: String
  required: false
  description_list:
  - markdown: The password for the user specified by `run_as_user`.
- property: run_as_user
  ruby_type: String
  required: false
  default_value: localsystem
  description_list:
  - markdown: The user under which a Microsoft Windows service runs.
- property: service_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the service name if it differs from the
      resource block's name.
- property: service_type
  ruby_type: Integer
  required: false
  default_value: '16'
  new_in: '14.0'
  description_list:
  - markdown: 
- property: start_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to start a service.
- property: startup_type
  ruby_type: Symbol
  required: false
  default_value: ":automatic"
  allowed_values: ":automatic, :disabled, :manual"
  description_list:
  - markdown: Use to specify the startup type of the service.
- property: status_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to check the run status for a service.
- property: stop_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to stop a service.
- property: supports
  ruby_type: Hash
  required: false
  default_value: '{"restart"=>nil, "reload"=>nil, "status"=>nil}'
  description_list:
  - markdown: 'A list of properties that controls how Chef Infra Client is to

      attempt to manage a service: `:restart`, `:reload`, `:status`. For

      `:restart`, the init script or other service provider can use a

      restart command; if `:restart` is not specified, Chef Infra Client

      attempts to stop and then start a service. For `:reload`, the init

      script or other service provider can use a reload command. For

      `:status`, the init script or other service provider can use a

      status command to determine if the service is running; if `:status`

      is not specified, Chef Infra Client attempts to match the

      `service_name` against the process table as a regular expression,

      unless a pattern is specified as a parameter property. Default

      value: `{ restart: false, reload: false, status: false }` for all

      platforms (except for the Red Hat platform family, which defaults to

      `{ restart: false, reload: false, status: true }`.)'
- property: timeout
  ruby_type: Integer
  required: false
  default_value: '60'
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
examples: "
  Start a service manually\n\n  ``` ruby\n  windows_service 'BITS'\
  \ do\n    action :configure_startup\n    startup_type :manual\n  end\n  ```\n\n\
  \  Create a service\n\n  ``` ruby\n  windows_service 'chef-client' do\n    action\
  \ :create\n    binary_path_name \"C:\\\\opscode\\\\chef\\\\bin\"\n  end\n  ```\n\
  \n  Create service with 'service_name' and 'display_name':\n\n  ``` ruby\n  windows_service\
  \ 'Create chef client as service' do\n    action :create\n    display_name \"CHEF-CLIENT\"\
  \n    service_name \"chef-client\"\n    binary_path_name \"C:\\\\opscode\\\\chef\\\
  \\bin\"\n  end\n  ```\n\n  Create service with the `:manual` startup type:\n\n \
  \ ``` ruby\n  windows_service 'chef-client' do\n    action :create\n    binary_path_name\
  \ \"C:\\\\opscode\\\\chef\\\\bin\"\n    startup_type :manual\n  end\n  ```\n\n \
  \ Create a service with the `:disabled` startup type:\n\n  ``` ruby\n  windows_service\
  \ 'chef-client' do\n    action :create\n    binary_path_name \"C:\\\\opscode\\\\\
  chef\\\\bin\"\n    startup_type :disabled\n  end\n  ```\n\n  Create service with\
  \ the `:automatic` startup type and delayed start\n  enabled:\n\n  ``` ruby\n  windows_service\
  \ 'chef-client' do\n    action :create\n    binary_path_name \"C:\\\\opscode\\\\\
  chef\\\\bin\"\n    startup_type :automatic\n    delayed_start true\n  end\n  ```\n\
  \n  Create service with a description:\n\n  ``` ruby\n  windows_service 'chef-client'\
  \ do\n    action :create\n    binary_path_name \"C:\\\\opscode\\\\chef\\\\bin\"\n\
  \    startup_type :automatic\n    description \"Chef client as service\"\n  end\n\
  \  ```\n\n  Delete a service\n\n  Delete service with the `'name'` of `chef-client`:\n\
  \n  ``` ruby\n  windows_service 'chef-client' do\n    action :delete\n  end\n  ```\n\
  \n  Delete service with `'service_name'`:\n\n  ``` ruby\n  windows_service 'Delete\
  \ chef client' do\n    action :delete\n    service_name \"chef-client\"\n  end\n\
  \  ```\n\n  Configure a service\n\n  Change an existing service from automatic to\
  \ manual startup:\n\n  ``` ruby\n  windows_service 'chef-client' do\n    action\
  \ :configure\n    binary_path_name \"C:\\\\opscode\\\\chef\\\\bin\"\n    startup_type\
  \ :manual\n  end\n  ```\n"

---
