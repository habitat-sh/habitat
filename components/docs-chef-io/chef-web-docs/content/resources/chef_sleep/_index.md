---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_sleep resource
resource: chef_sleep
aliases:
- "/resource_chef_sleep.html"
menu:
  infra:
    title: chef_sleep
    identifier: chef_infra/cookbook_reference/resources/chef_sleep chef_sleep
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **chef_sleep** resource to pause (sleep) for a number of seconds
    during a Chef Infra Client run. Only use this resource when a command or service
    exits successfully but is not ready for the next step in a recipe.
resource_new_in: '15.5'
syntax_full_code_block: |-
  chef_sleep 'name' do
    seconds      String, Integer # default value: 'name' unless specified
    action       Symbol # defaults to :sleep if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`chef_sleep` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`seconds` is the property available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :sleep:
    markdown: Pause the Chef Infra Client run for a specified number of seconds.
properties_list:
- property: seconds
  ruby_type: String, Integer
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The number of seconds to sleep.
examples: |
  **Sleep for 10 seconds**:

  ```ruby
  chef_sleep '10'
  ```

  **Sleep for 10 seconds with a descriptive resource name for logging**:

  ```ruby
  chef_sleep 'wait for the service to start' do
    seconds 10
  end
  ````

  **Use a notification from another resource to sleep only when necessary**:

  ```ruby
  service 'Service that is slow to start and reports as started' do
    service_name 'my_database'
    action :start
    notifies :sleep, chef_sleep['wait for service start']
  end

  chef_sleep 'wait for service start' do
    seconds 30
    action :nothing
  end
  ```
---