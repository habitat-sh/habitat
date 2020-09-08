---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: zypper_repository resource
resource: zypper_repository
aliases:
- "/resource_zypper_repository.html"
menu:
  infra:
    title: zypper_repository
    identifier: chef_infra/cookbook_reference/resources/zypper_repository zypper_repository
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **zypper_repository** resource to create Zypper package repositories
    on SUSE Enterprise Linux and openSUSE systems. This resource maintains full compatibility
    with the **zypper_repository** resource in the existing **zypper** cookbook.
resource_new_in: '13.3'
syntax_full_code_block: |-
  zypper_repository 'name' do
    autorefresh            true, false # default value: true
    baseurl                String
    cookbook               String
    description            String
    enabled                true, false # default value: true
    gpgautoimportkeys      true, false # default value: true
    gpgcheck               true, false # default value: true
    gpgkey                 String
    keeppackages           true, false # default value: false
    mirrorlist             String
    mode                   String, Integer # default value: "0644"
    path                   String
    priority               Integer # default value: 99
    refresh_cache          true, false # default value: true
    repo_name              String # default value: 'name' unless specified
    source                 String
    type                   String # default value: "NONE"
    action                 Symbol # defaults to :create if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`zypper_repository` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`autorefresh`, `baseurl`, `cookbook`, `description`, `enabled`, `gpgautoimportkeys`,
  `gpgcheck`, `gpgkey`, `keeppackages`, `mirrorlist`, `mode`, `path`, `priority`,
  `refresh_cache`, `repo_name`, `source`, and `type` are the properties available
  to this resource."
actions_list:
  :add:
    markdown: Default action. Add a new Zypper repository.
  :refresh:
    markdown: Refresh a Zypper repository.
  :remove:
    markdown: Remove a Zypper repository.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: autorefresh
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether or not the repository should be refreshed automatically.
- property: baseurl
  ruby_type: String
  required: false
  description_list:
  - markdown: The base URL for the Zypper repository, such as `http://download.opensuse.org`.
- property: cookbook
  ruby_type: String
  required: false
  description_list:
  - markdown: The cookbook to source the repository template file from. Only necessary
      if you're not using the built in template.
- property: description
  ruby_type: String
  required: false
  description_list:
  - markdown: The description of the repository that will be shown by the `zypper
      repos` command.
- property: enabled
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether or not the repository should be enabled.
- property: gpgautoimportkeys
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Automatically import the specified key when setting up the repository.
- property: gpgcheck
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether or not to perform a GPG signature check on the repository.
- property: gpgkey
  ruby_type: String
  required: false
  description_list:
  - markdown: The location of the repository key to be imported.
- property: keeppackages
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines whether or not packages should be saved.
- property: mirrorlist
  ruby_type: String
  required: false
  description_list:
  - markdown: The URL of the mirror list that will be used.
- property: mode
  ruby_type: String, Integer
  required: false
  default_value: '0644'
  description_list:
  - markdown: The file mode of the repository file.
- property: path
  ruby_type: String
  required: false
  description_list:
  - markdown: The relative path from the repository's base URL.
- property: priority
  ruby_type: Integer
  required: false
  default_value: '99'
  description_list:
  - markdown: Determines the priority of the Zypper repository.
- property: refresh_cache
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether or not the package cache should be refreshed.
- property: repo_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the repository name if it differs from the
      resource block's name.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: The name of the template for the repository file. Only necessary if
      you're not using the built in template.
- property: type
  ruby_type: String
  required: false
  default_value: NONE
  description_list:
  - markdown: Specifies the repository type.
examples: |
  **Add the Apache repo on openSUSE Leap 15**:

  ``` ruby
  zypper_repository 'apache' do
    baseurl 'http://download.opensuse.org/repositories/Apache'
    path '/openSUSE_Leap_15.0'
      type 'rpm-md'
    priority '100'
  end
  ```
---