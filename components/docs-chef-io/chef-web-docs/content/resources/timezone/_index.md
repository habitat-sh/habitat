---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: timezone resource
resource: timezone
aliases:
- "/resource_timezone.html"
menu:
  infra:
    title: timezone
    identifier: chef_infra/cookbook_reference/resources/timezone timezone
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **timezone** resource to change the system timezone on Windows,
    Linux, and macOS hosts. Timezones are specified in tz database format, with a
    complete list of available TZ values for Linux and macOS here: <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>.
    On Windows systems run `tzutil /l` for a complete list of valid timezones.'
resource_new_in: '14.6'
syntax_full_code_block: |-
  timezone 'name' do
    timezone      String # default value: 'name' unless specified
    action        Symbol # defaults to :set if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`timezone` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`timezone` is the property available to this resource."
actions_list:
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :set:
    markdown: Set the system timezone.
properties_list:
- property: timezone
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the timezone value if it differs from the
      resource block's name.
examples: |
  **Set the timezone to UTC**

  ```ruby
  timezone 'UTC'
  ```

  **Set the timezone to America/Los_Angeles with a friendly resource name on Linux/macOS**

  ```ruby
  timezone 'Set the host's timezone to America/Los_Angeles' do
    timezone 'America/Los_Angeles'
  end
  ```

  **Set the timezone to PST with a friendly resource name on Windows**

  ```ruby
  timezone 'Set the host's timezone to PST' do
    timezone 'Pacific Standard time'
  end
  ```
---