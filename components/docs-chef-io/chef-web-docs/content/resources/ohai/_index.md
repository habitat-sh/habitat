---
title: ohai resource
resource: ohai
draft: false
aliases:
- /resource_ohai.html
menu:
  infra:
    title: ohai
    identifier: chef_infra/cookbook_reference/resources/ohai ohai
    parent: chef_infra/cookbook_reference/resources

resource_reference: true
robots: null
resource_description_list:
- markdown: 'Use the **ohai** resource to reload the Ohai configuration on a node.

    This allows recipes that change system attributes (like a recipe that

    adds a user) to refer to those attributes later on during a Chef Infra

    Client run.'
resource_new_in: null
handler_types: false
syntax_description: "The ohai resource has the following syntax:\n\n``` ruby\nohai\
  \ 'name' do\n  plugin      String\n  action      Symbol # defaults to :reload if\
  \ not specified\nend\n```"
syntax_code_block: null
syntax_properties_list:
- '`ohai` is the resource.'
- '`name` is the name given to the resource block.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`plugin` is the property available to this resource.'
syntax_full_code_block: null
syntax_full_properties_list: null
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list: {}
properties_list: []
properties_shortcode: resource_ohai_properties.md
properties_multiple_packages: false
resource_directory_recursive_directories: false
resources_common_atomic_update: false
properties_resources_common_windows_security: false
remote_file_prevent_re_downloads: false
remote_file_unc_path: false
ps_credential_helper: false
ruby_style_basics_chef_log: false
debug_recipes_chef_shell: false
template_requirements: false
resources_common_properties: true
resources_common_notification: true
resources_common_guards: true
common_resource_functionality_multiple_packages: false
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Reload Ohai\n\n  ``` ruby\n  ohai 'reload' do\n    action :reload\n\
  \  end\n  ```\n\n  Reload Ohai after a new user is created\n\n  ``` ruby\n  ohai\
  \ 'reload_passwd' do\n    action :nothing\n    plugin 'etc'\n  end\n\n  user 'daemonuser'\
  \ do\n    home '/dev/null'\n    shell '/sbin/nologin'\n    system true\n    notifies\
  \ :reload, 'ohai[reload_passwd]', :immediately\n  end\n\n  ruby_block 'just an example'\
  \ do\n    block do\n      # These variables will now have the new values\n     \
  \ puts node['etc']['passwd']['daemonuser']['uid']\n      puts node['etc']['passwd']['daemonuser']['gid']\n\
  \    end\n  end\n  ```\n"

---
