---
resource_reference: true
properties_shortcode: 
title: ruby_block resource
resource: ruby_block
aliases:
- /resource_ruby_block.html
menu:
  infra:
    title: ruby_block
    identifier: chef_infra/cookbook_reference/resources/ruby_block ruby_block
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **ruby_block** resource to execute Ruby code during a Chef

    Infra Client run. Ruby code in the `ruby_block` resource is evaluated

    with other resources during convergence, whereas Ruby code outside of a

    `ruby_block` resource is evaluated before other resources, as the recipe

    is compiled.'
resource_new_in: null
handler_types: false
syntax_description: "A **ruby_block** resource block executes a block of arbitrary\
  \ Ruby\ncode. For example, to reload the client.rb file during a Chef Infra\nClient\
  \ run:\n\n``` ruby\nruby_block 'reload_client_config' do\n  block do\n    Chef::Config.from_file(\"\
  /etc/chef/client.rb\")\n  end\n  action :run\nend\n```"
syntax_code_block: null
syntax_properties_list: null
syntax_full_code_block: "ruby_block 'name' do\n  block                      Block\n\
  \  block_name                 String # defaults to 'name' if not specified\n  action\
  \                     Symbol # defaults to :run if not specified\nend"
syntax_full_properties_list:
- '`ruby_block` is the resource.'
- '`name` is the name given to the resource block.'
- '`block` is the block of Ruby code to be executed.'
- '`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state.'
- '`block` and `block_name` are properties of this resource, with the Ruby type shown.
  See "Properties" section below for more information about all of the properties
  that may be used with this resource.'
syntax_shortcode: null
registry_key: false
nameless_apt_update: false
nameless_build_essential: false
resource_package_options: false
actions_list:
  :create:
    markdown: The same as `:run`.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :run:
    markdown: Default. Run a Ruby block.
properties_list:
- property: block
  ruby_type: Block
  required: false
  default_value: null
  new_in: null
  description_list:
  - markdown: A block of Ruby code.
- property: block_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: null
  description_list:
  - markdown: 'The name of the Ruby block. Default value: the `name` of the

      resource block. See "Syntax" section above for more information.'
- property: ignore_failure
  ruby_type: true, false
  required: false
  default_value: 'false'
  new_in: null
  description_list:
  - markdown: Continue running a recipe if a resource fails for any reason.
- property: notifies
  ruby_type: Symbol, Chef::Resource\[String\]
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: resources_common_notification_notifies.md
  - markdown: ''
  - shortcode: resources_common_notification_timers.md
  - markdown: ''
  - shortcode: resources_common_notification_notifies_syntax.md
- property: retries
  ruby_type: Integer
  required: false
  default_value: '0'
  new_in: null
  description_list:
  - markdown: The number of attempts to catch exceptions and retry the resource.
- property: retry_delay
  ruby_type: Integer
  required: false
  default_value: '2'
  new_in: null
  description_list:
  - markdown: The retry delay (in seconds).
- property: subscribes
  ruby_type: Symbol, Chef::Resource\[String\]
  required: false
  default_value: null
  new_in: null
  description_list:
  - shortcode: resources_common_notification_subscribes.md
  - markdown: ''
  - shortcode: resources_common_notification_timers.md
  - markdown: ''
  - shortcode: resources_common_notification_subscribes_syntax.md
properties_shortcode: null
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
resources_common_properties: false
resources_common_notification: false
resources_common_guards: false
common_resource_functionality_multiple_packages: false
resources_common_guard_interpreter: false
remote_directory_recursive_directories: false
common_resource_functionality_resources_common_windows_security: false
handler_custom: false
cookbook_file_specificity: false
unit_file_verification: false
examples: "
  Re-read configuration data\n\n  ``` ruby\n  ruby_block 'reload_client_config'\
  \ do\n    block do\n      Chef::Config.from_file('/etc/chef/client.rb')\n    end\n\
  \    action :run\n  end\n  ```\n\n  Install repositories from a file, trigger a\
  \ command, and force the\n  internal cache to reload\n\n  The following example\
  \ shows how to install new Yum repositories from a\n  file, where the installation\
  \ of the repository triggers a creation of\n  the Yum cache that forces the internal\
  \ cache for Chef Infra Client to\n  reload:\n\n  ``` ruby\n  execute 'create-yum-cache'\
  \ do\n   command 'yum -q makecache'\n   action :nothing\n  end\n\n  ruby_block 'reload-internal-yum-cache'\
  \ do\n    block do\n      Chef::Provider::Package::Yum::YumCache.instance.reload\n\
  \    end\n    action :nothing\n  end\n\n  cookbook_file '/etc/yum.repos.d/custom.repo'\
  \ do\n    source 'custom'\n    mode '0755'\n    notifies :run, 'execute[create-yum-cache]',\
  \ :immediately\n    notifies :create, 'ruby_block[reload-internal-yum-cache]', :immediately\n\
  \  end\n  ```\n\n  Use an if statement with the platform recipe DSL method\n\n \
  \ The following example shows how an if statement can be used with the\n  `platform?`\
  \ method in the Recipe DSL to run code specific to Microsoft\n  Windows. The code\
  \ is defined using the **ruby_block** resource:\n\n  ``` ruby\n  # the following\
  \ code sample comes from the ``client`` recipe\n  # in the following cookbook: https://github.com/chef-cookbooks/mysql\n\
  \n  if platform?('windows')\n    ruby_block 'copy libmysql.dll into ruby path' do\n\
  \      block do\n        require 'fileutils'\n        FileUtils.cp \"#{node['mysql']['client']['lib_dir']}\\\
  \\libmysql.dll\",\n          node['mysql']['client']['ruby_dir']\n      end\n  \
  \    not_if { File.exist?(\"#{node['mysql']['client']['ruby_dir']}\\\\libmysql.dll\"\
  ) }\n    end\n  end\n  ```\n\n  Stash a file in a data bag\n\n  The following example\
  \ shows how to use the **ruby_block** resource to\n  stash a BitTorrent file in\
  \ a data bag so that it can be distributed to\n  nodes in the organization.\n\n\
  \  ``` ruby\n  # the following code sample comes from the ``seed`` recipe\n  # in\
  \ the following cookbook: https://github.com/mattray/bittorrent-cookbook\n\n  ruby_block\
  \ 'share the torrent file' do\n    block do\n      f = File.open(node['bittorrent']['torrent'],'rb')\n\
  \      #read the .torrent file and base64 encode it\n      enc = Base64.encode64(f.read)\n\
  \      data = {\n        'id'=>bittorrent_item_id(node['bittorrent']['file']),\n\
  \        'seed'=>node.ipaddress,\n        'torrent'=>enc\n      }\n      item =\
  \ Chef::DataBagItem.new\n      item.data_bag('bittorrent')\n      item.raw_data\
  \ = data\n      item.save\n    end\n    action :nothing\n    subscribes :create,\
  \ \"bittorrent_torrent[#{node['bittorrent']['torrent']}]\", :immediately\n  end\n\
  \  ```\n\n  **Update the /etc/hosts file**\n\n  The following example shows how\
  \ the **ruby_block** resource can be used\n  to update the `/etc/hosts` file:\n\n\
  \  ``` ruby\n  # the following code sample comes from the ``ec2`` recipe\n  # in\
  \ the following cookbook: https://github.com/chef-cookbooks/dynect\n\n  ruby_block\
  \ 'edit etc hosts' do\n    block do\n      rc = Chef::Util::FileEdit.new('/etc/hosts')\n\
  \      rc.search_file_replace_line(/^127\\.0\\.0\\.1 localhost$/,\n         '127.0.0.1\
  \ #{new_fqdn} #{new_hostname} localhost')\n      rc.write_file\n    end\n  end\n\
  \  ```\n\n  Set environment variables\n\n  The following example shows how to use\
  \ variables within a Ruby block to\n  set environment variables using rbenv.\n\n\
  \  ``` ruby\n  node.override[:rbenv][:root] = rbenv_root\n  node.override[:ruby_build][:bin_path]\
  \ = rbenv_binary_path\n\n  ruby_block 'initialize' do\n    block do\n      ENV['RBENV_ROOT']\
  \ = node[:rbenv][:root]\n      ENV['PATH'] = \"#{node[:rbenv][:root]}/bin:#{node[:ruby_build][:bin_path]}:#{ENV['PATH']}\"\
  \n    end\n  end\n  ```\n\n  Set JAVA_HOME\n\n  The following example shows how\
  \ to use a variable within a Ruby block to\n  set the `java_home` environment variable:\n\
  \n  ``` ruby\n  ruby_block 'set-env-java-home' do\n    block do\n      ENV['JAVA_HOME']\
  \ = java_home\n    end\n  end\n  ```\n\n  Run specific blocks of Ruby code on specific\
  \ platforms\n\n  The following example shows how the `platform?` method and an if\n\
  \  statement can be used in a recipe along with the `ruby_block` resource\n  to\
  \ run certain blocks of Ruby code on certain platforms:\n\n  ``` ruby\n  if platform_family?('debian',\
  \ 'rhel', 'fedora', 'amazon')\n    ruby_block 'update-java-alternatives' do\n  \
  \    block do\n        if platform?('ubuntu', 'debian') and version == 6\n     \
  \     run_context = Chef::RunContext.new(node, {})\n          r = Chef::Resource::Execute.new('update-java-alternatives',\
  \ run_context)\n          r.command 'update-java-alternatives -s java-6-openjdk'\n\
  \          r.returns [0,2]\n          r.run_action(:create)\n        else\n\n  \
  \        require 'fileutils'\n          arch = node['kernel']['machine'] =~ /x86_64/\
  \ ? 'x86_64' : 'i386'\n          Chef::Log.debug(\"glob is #{java_home_parent}/java*#{version}*openjdk*\"\
  )\n          jdk_home = Dir.glob(\"#{java_home_parent}/java*#{version}*openjdk{,[-\\\
  .]#{arch}}\")[0]\n          Chef::Log.debug(\"jdk_home is #{jdk_home}\")\n\n   \
  \       if File.exist? java_home\n            FileUtils.rm_f java_home\n       \
  \   end\n          FileUtils.ln_sf jdk_home, java_home\n\n          cmd = Chef::ShellOut.new(\n\
  \                %Q[ update-alternatives --install /usr/bin/java java #{java_home}/bin/java\
  \ 1;\n                update-alternatives --set java #{java_home}/bin/java ]\n \
  \               ).run_command\n             unless cmd.exitstatus == 0 or cmd.exitstatus\
  \ == 2\n            Chef::Application.fatal!('Failed to update-alternatives for\
  \ openjdk!')\n          end\n        end\n      end\n      action :nothing\n   \
  \ end\n  end\n  ```\n\n  Reload the configuration\n\n  The following example shows\
  \ how to reload the configuration of a\n  chef-client using the **remote_file**\
  \ resource to:\n\n  -   using an if statement to check whether the plugins on a\
  \ node are the\n      latest versions\n  -   identify the location from which Ohai\
  \ plugins are stored\n  -   using the `notifies` property and a **ruby_block** resource\
  \ to\n      trigger an update (if required) and to then reload the client.rb\n \
  \     file.\n\n  <!-- -->\n\n  ``` ruby\n  directory 'node[:ohai][:plugin_path]'\
  \ do\n    owner 'chef'\n    recursive true\n  end\n\n  ruby_block 'reload_config'\
  \ do\n    block do\n      Chef::Config.from_file('/etc/chef/client.rb')\n    end\n\
  \    action :nothing\n  end\n\n  if node[:ohai].key?(:plugins)\n    node[:ohai][:plugins].each\
  \ do |plugin|\n      remote_file node[:ohai][:plugin_path] +\"/#{plugin}\" do\n\
  \        source plugin\n        owner 'chef'\n        notifies :run, 'ruby_block[reload_config]',\
  \ :immediately\n      end\n    end\n  end\n  ```\n"

---
