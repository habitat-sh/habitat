---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: reboot resource
resource: reboot
aliases:
- "/resource_reboot.html"
menu:
  infra:
    title: reboot
    identifier: chef_infra/cookbook_reference/resources/reboot reboot
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **reboot** resource to reboot a node, a necessary step with some

    installations on certain platforms. This resource is supported for use

    on the Microsoft Windows, macOS, and Linux platforms.'
resource_new_in: null
syntax_full_code_block: |-
  reboot 'name' do
    delay_mins      Integer # default value: 0
    reason          String # default value: "Reboot by Chef Infra Client"
    action          Symbol # defaults to :nothing if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`reboot` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`delay_mins` and `reason` are the properties available to this resource."
actions_list:
  :cancel:
    markdown: Cancel a reboot request.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :reboot_now:
    markdown: Reboot a node so that Chef Infra Client may continue the installation
      process.
  :request_reboot:
    markdown: Reboot a node at the end of a Chef Infra Client run.
properties_list:
- property: delay_mins
  ruby_type: Integer
  required: false
  default_value: '0'
  description_list:
  - markdown: The amount of time (in minutes) to delay a reboot request.
- property: reason
  ruby_type: String
  required: false
  default_value: Reboot by Chef Infra Client
  description_list:
  - markdown: A string that describes the reboot action.
examples: "
  Reboot a node immediately\n\n  ``` ruby\n  reboot 'now' do\n    action\
  \ :nothing\n    reason 'Cannot continue Chef run without a reboot.'\n    delay_mins\
  \ 2\n  end\n\n  execute 'foo' do\n    command '...'\n    notifies :reboot_now, 'reboot[now]',\
  \ :immediately\n  end\n  ```\n\n  Reboot a node at the end of a Chef Infra Client\
  \ run\n\n  ``` ruby\n  reboot 'app_requires_reboot' do\n    action :request_reboot\n\
  \    reason 'Need to reboot when the run completes successfully.'\n    delay_mins\
  \ 5\n  end\n  ```\n\n  Cancel a reboot\n\n  ``` ruby\n  reboot 'cancel_reboot_request'\
  \ do\n    action :cancel\n    reason 'Cancel a previous end-of-run reboot request.'\n\
  \  end\n  ```\n\n  Rename computer, join domain, reboot\n\n  The following example\
  \ shows how to rename a computer, join a domain, and\n  then reboot the computer:\n\
  \n  ``` ruby\n  reboot 'Restart Computer' do\n    action :nothing\n  end\n\n  powershell_script\
  \ 'Rename and Join Domain' do\n    code <<-EOH\n      ...your rename and domain\
  \ join logic here...\n    EOH\n    not_if <<-EOH\n      $ComputerSystem = gwmi win32_computersystem\n\
  \      ($ComputerSystem.Name -like '#{node['some_attribute_that_has_the_new_name']}')\
  \ -and\n        $ComputerSystem.partofdomain)\n    EOH\n    notifies :reboot_now,\
  \ 'reboot[Restart Computer]', :immediately\n  end\n  ```\n\n  where:\n\n  -   The\
  \ **powershell_script** resource block renames a computer, and\n      then joins\
  \ a domain\n  -   The **reboot** resource restarts the computer\n  -   The `not_if`\
  \ guard prevents the Windows PowerShell script from\n      running when the settings\
  \ in the `not_if` guard match the desired\n      state\n  -   The `notifies` statement\
  \ tells the **reboot** resource block to run\n      if the **powershell_script**\
  \ block was executed during a Chef Infra\n      Client run\n"

---
