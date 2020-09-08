---
######## Page Data ########
title: TITLE
resource: RESOURCE
description: DESCRIPTION
draft: false

# redirect from old sphinx url
aliases: OLD_SPHINX_URL

menu:
  docs:
    title: TITLE
    identifier: IDENTIFIER
    parent: PARENT
    weight: WEIGHT


######## Basic Resource Data ########

resource_description:
resource_note:
resource_new_in:      


######## Handler Types ########
handler_types: false


######## Package Resource ########
package_resource: false


######## Syntax ########

## Resource Block: For example, under Syntax in batch_resource
resource_block_description: 
resource_block_codeblock: |
resource_block_list:

syntax_codeblock: |
syntax_property_list: 


##Activates the Registry Key Path Separators and Recipe DSL Methods in registry_key resource
registry_key: false


######## Nameless ########

##Activates the Nameless section in apt_update resource
nameless: false


######## Gem Package Options ########

## Activates Gem Package Options in gem_package resource
resource_package_options: false


########Actions ########

actions_list:
  key: description


########Properties ########

properties_list:
  - property:
    ruby_type:
    default_value:
    description: 
    new_in:

## Multiple Packages in Properties section from, for example, dnf_package resource
properties_multiple_packages: false

## Recursive Directories from remote_directory resource and directory resource
resource_directory_recursive_directory: false

## Atomic File Updates in the Properties Section of, for example, cookbook_file resource
resources_common_atomic_update: false 

## Windows File Security in the Properties section of, for example, cookbook_file resource
properties_resources_common_windows_security: false 

## Prevent Re-downloads from remote_file resource
remote_file_prevent_re_downloads: false 

## Access a remote UNC path on Windows from remote_file resource
remote_file_unc_path: false 

## ps_credential Helper from dsc_script resource
ps_credential_helper: false


######## Chef::Log Entries ########

##Chef::Log Entries from log resource
ruby_style_basics_chef_log: false


######## Debug Recipes with chef-shell ########

## Debug Recipes with chef-shell from breakpoint resource 
debug_recipes_chef_shell: false


######## Using Templates ########

## Using Templates in template resource
template_requirements: false


########Common Resource Functionality ########

## Common Properties in, for example, apt_package resource 
resources_common_properties: false

## Notifications in, for example, apt_package resource 
resources_common_notification: false

## Guards in, for example, apt_package resource  
resources_common_guards: false

## Multiple Packages in, for example, apt_package resource   
common_resource_functionality_multiple_packages: false

## Guard Interpreters in, for example, common resource
resources_common_guard_interpreter: false

## Recursive Directories in, for example,  remote_directory resource
remote_directory_recursive_directories: false

## Windows File Security under Common Resource Functionality in, for example, remote_directory resource
common_resource_functionality_resources_common_windows_security: false 


########Custom Handlers ########

## Custom Handlers in chef_handler resource
handler_custom: false 


########File Specificity ########

## File Specificity in cookbook_file resource
cookbook_file_specificity: false 


########Examples ########
examples_list:
  - example1:
      heading: 
      description: 
      codeblock:


---