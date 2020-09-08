---
draft: false
resource_reference: true
robots:
syntax_code_block:
common_resource_functionality_multiple_packages: false
common_resource_functionality_resources_common_windows_security: false
cookbook_file_specificity: false
debug_recipes_chef_shell: false
handler_custom: false
handler_types: false
nameless_apt_update: false
nameless_build_essential: false
properties_multiple_packages: false
properties_resources_common_windows_security: false
properties_shortcode:
ps_credential_helper: false
registry_key: false
remote_directory_recursive_directories: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
resource_directory_recursive_directories: false
resource_package_options: false
resources_common_atomic_update: false
resources_common_guard_interpreter: false
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
ruby_style_basics_chef_log: false
syntax_shortcode:
template_requirements: false
unit_file_verification: false
title: notify_group resource
resource: notify_group
aliases:
- "/resource_notify_group.html"
menu:
  infra:
    title: notify_group
    identifier: chef_infra/cookbook_reference/resources/notify_group notify_group
    parent: chef_infra/cookbook_reference/resources

resource_description_list:
- markdown: The notify_group resource does nothing, and always fires notifications
    which are set on it.  Use it to DRY blocks of notifications that are common to
    multiple resources, and provide a single target for other resources to notify.  Unlike
    most resources, its default action is :nothing.
resource_new_in: '15.8'
syntax_full_code_block: |-
  notify_group 'name' do
    action      Symbol # defaults to :nothing if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`notify_group` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
properties_list: []

---
