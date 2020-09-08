---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: windows_pagefile resource
resource: windows_pagefile
aliases:
- "/resource_windows_pagefile.html"
menu:
  infra:
    title: windows_pagefile
    identifier: chef_infra/cookbook_reference/resources/windows_pagefile windows_pagefile
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **windows_pagefile** resource to configure pagefile settings on
    Windows.
resource_new_in: '14.0'
syntax_full_code_block: |-
  windows_pagefile 'name' do
    automatic_managed      true, false # default value: false
    initial_size           Integer
    maximum_size           Integer
    path                   String # default value: 'name' unless specified
    system_managed         true, false
    action                 Symbol # defaults to :set if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`windows_pagefile` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`automatic_managed`, `initial_size`, `maximum_size`, `path`, and `system_managed`
  are the properties available to this resource."
actions_list:
  :delete:
    markdown: Deletes the specified pagefile.
  :set:
    markdown: Default. Configures the default pagefile, creating it if it doesn't
      exist.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: automatic_managed
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Enable automatic management of pagefile initial and maximum size. Setting
      this to true ignores `initial_size` and `maximum_size` properties.
- property: initial_size
  ruby_type: Integer
  required: false
  description_list:
  - markdown: Initial size of the pagefile in megabytes.
- property: maximum_size
  ruby_type: Integer
  required: false
  description_list:
  - markdown: Maximum size of the pagefile in megabytes.
- property: path
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the pagefile name if it differs from the
      resource block's name.
- property: system_managed
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Configures whether the system manages the pagefile size.
examples: |
  **Set the system to manage pagefiles**:

  ```ruby
  windows_pagefile 'Enable automatic management of pagefiles' do
    automatic_managed true
  end
  ```

  **Delete a pagefile**:

  ```ruby
  windows_pagefile 'Delete the pagefile' do
    path 'C:pagefile.sys'
    action :delete
  end
  ```

  **Create a pagefile with an initial and maximum size**:

  ```ruby
  windows_pagefile 'create the pagefile' do
    path 'C:pagefile.sys'
    initial_size 100
    maximum_size 200
  end
  ```
---