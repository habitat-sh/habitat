---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: user_ulimit resource
resource: user_ulimit
aliases:
- "/resource_user_ulimit.html"
menu:
  infra:
    title: user_ulimit
    identifier: chef_infra/cookbook_reference/resources/user_ulimit user_ulimit
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **user_ulimit** resource to create individual ulimit files that
    are installed into the `/etc/security/limits.d/` directory.
resource_new_in: '16.0'
syntax_full_code_block: |-
  user_ulimit 'name' do
    core_hard_limit            String, Integer
    core_limit                 String, Integer
    core_soft_limit            String, Integer
    filehandle_hard_limit      String, Integer
    filehandle_limit           String, Integer
    filehandle_soft_limit      String, Integer
    filename                   String
    memory_limit               String, Integer
    process_hard_limit         String, Integer
    process_limit              String, Integer
    process_soft_limit         String, Integer
    rtprio_hard_limit          String, Integer
    rtprio_limit               String, Integer
    rtprio_soft_limit          String, Integer
    stack_hard_limit           String, Integer
    stack_limit                String, Integer
    stack_soft_limit           String, Integer
    username                   String # default value: 'name' unless specified
    virt_limit                 String, Integer
    action                     Symbol # defaults to :create if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`user_ulimit` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`core_hard_limit`, `core_limit`, `core_soft_limit`, `filehandle_hard_limit`, `filehandle_limit`,
  `filehandle_soft_limit`, `filename`, `memory_limit`, `process_hard_limit`, `process_limit`,
  `process_soft_limit`, `rtprio_hard_limit`, `rtprio_limit`, `rtprio_soft_limit`,
  `stack_hard_limit`, `stack_limit`, `stack_soft_limit`, `username`, and `virt_limit`
  are the properties available to this resource."
actions_list:
  :create:
    markdown: Create a ulimit configuration file
  :delete:
    markdown: Delete a ulimit configuration file
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: core_hard_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: core_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: core_soft_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: filehandle_hard_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: filehandle_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: filehandle_soft_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: filename
  ruby_type: String
  required: false
  default_value: lazy default
  description_list:
  - markdown: 
- property: memory_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: process_hard_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: process_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: process_soft_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: rtprio_hard_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: rtprio_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: rtprio_soft_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: stack_hard_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: stack_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: stack_soft_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
- property: username
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: 
- property: virt_limit
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: 
examples: |
  **Set filehandle limit for the tomcat user**:

  ```ruby
  user_ulimit 'tomcat' do
    filehandle_limit 8192
  end
  ```

  **Specify a username that differs from the name given to the resource block**:

  ```ruby
  user_ulimit 'Bump filehandle limits for tomcat user' do
    username 'tomcat'
    filehandle_limit 8192
  end
  ```

  **Set filehandle limit for the tomcat user with a non-default filename**:

  ```ruby
  user_ulimit 'tomcat' do
    filehandle_limit 8192
    filename 'tomcat_filehandle_limits.conf'
  end
  ```
---