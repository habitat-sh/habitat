---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_printer resource
resource: windows_printer
aliases:
- "/resource_windows_printer.html"
menu:
  infra:
    title: windows_printer
    identifier: chef_infra/cookbook_reference/resources/windows_printer windows_printer
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_printer** resource to setup Windows printers. Note that
    this doesn't currently install a printer driver. You must already have the driver
    installed on the system.
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_printer 'name' do
    comment           String
    default           true, false # default value: false
    device_id         String # default value: 'name' unless specified
    driver_name       String
    ipv4_address      String
    location          String
    share_name        String
    shared            true, false # default value: false
    action            Symbol # defaults to :create if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`windows_printer` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`comment`, `default`, `device_id`, `driver_name`, `ipv4_address`, `location`, `share_name`,
  and `shared` are the properties available to this resource."
actions_list:
  :create:
    markdown: Default. Create a new printer and printer port, if one doesn't already
      exist.
  :delete:
    markdown: Delete an existing printer. Note that this resource does not delete
      the associated printer port.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: comment
  ruby_type: String
  required: false
  description_list:
  - markdown: Optional descriptor for the printer queue.
- property: default
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines whether or not this should be the system's default printer.
- property: device_id
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: 'An optional property to set the printer queue name if it differs from
      the resource block''s name. Example: `HP LJ 5200 in fifth floor copy room`.'
- property: driver_name
  ruby_type: String
  required: true
  description_list:
  - markdown: The exact name of printer driver installed on the system.
- property: ipv4_address
  ruby_type: String
  required: false
  description_list:
  - markdown: The IPv4 address of the printer, such as `10.4.64.23`
- property: location
  ruby_type: String
  required: false
  description_list:
  - markdown: Printer location, such as `Fifth floor copy room`.
- property: share_name
  ruby_type: String
  required: false
  description_list:
  - markdown: The name used to identify the shared printer.
- property: shared
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines whether or not the printer is shared.
examples: |
  **Create a printer**:

  ```ruby
  windows_printer 'HP LaserJet 5th Floor' do
    driver_name 'HP LaserJet 4100 Series PCL6'
    ipv4_address '10.4.64.38'
  end
  ```

  **Delete a printer**:

  Note: this doesn't delete the associated printer port. See windows_printer_port above for how to delete the port.

  ```ruby
  windows_printer 'HP LaserJet 5th Floor' do
    action :delete
  end
  ```
---