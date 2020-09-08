---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: service resource
resource: service
aliases:
- "/resource_service.html"
menu:
  infra:
    title: service
    identifier: chef_infra/cookbook_reference/resources/service service
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **service** resource to manage a service.
syntax_full_code_block: |-
  service 'name' do
    init_command         String
    options              Array, String
    parameters           Hash
    pattern              String
    priority             Integer, String, Hash
    reload_command       String, false
    restart_command      String, false
    run_levels           Array
    service_name         String # default value: 'name' unless specified
    start_command        String, false
    status_command       String, false
    stop_command         String, false
    supports             Hash # default value: {"restart"=>nil, "reload"=>nil, "status"=>nil}
    timeout              Integer # default value: 900
    user                 String
    action               Symbol # defaults to :nothing if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`service` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`init_command`, `options`, `parameters`, `pattern`, `priority`, `reload_command`,
  `restart_command`, `run_levels`, `service_name`, `start_command`, `status_command`,
  `stop_command`, `supports`, `timeout`, and `user` are the properties available to
  this resource."
actions_list:
  :disable:
    markdown: Disable a service. This action is equivalent to a `Disabled` startup
      type on the Microsoft Windows platform. This action is not supported when using
      System Resource Controller (SRC) on the AIX platform because System Resource
      Controller (SRC) does not have a standard mechanism for enabling and disabling
      services on system boot.
  :enable:
    markdown: Enable a service at boot. This action is equivalent to an `Automatic`
      startup type on the Microsoft Windows platform. This action is not supported
      when using System Resource Controller (SRC) on the AIX platform because System
      Resource Controller (SRC) does not have a standard mechanism for enabling and
      disabling services on system boot.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :reload:
    markdown: Reload the configuration for this service.
  :restart:
    markdown: Restart a service.
  :start:
    markdown: Start a service, and keep it running until stopped or disabled.
  :stop:
    markdown: Stop a service.
properties_list:
- property: init_command
  ruby_type: String
  required: false
  description_list:
  - markdown: 'The path to the init script that is associated with the service. Use

      `init_command` to prevent the need to specify overrides for the

      `start_command`, `stop_command`, and `restart_command` properties.

      When this property is not specified, Chef Infra Client will use the

      default init command for the service provider being used.'
- property: options
  ruby_type: Array, String
  required: false
  description_list:
  - markdown: 'Solaris platform only. Options to pass to the service command. See

      the `svcadm` manual for details of possible options.'
- property: parameters
  ruby_type: Hash
  required: false
  description_list:
  - markdown: 'Upstart only: A hash of parameters to pass to the service command for
      use in the service definition.'
- property: pattern
  ruby_type: String
  required: false
  default_value: The value provided to 'service_name' or the resource block's name
  description_list:
  - markdown: The pattern to look for in the process table.
- property: priority
  ruby_type: Integer, String, Hash
  required: false
  description_list:
  - markdown: 'Debian platform only. The relative priority of the program for start

      and shutdown ordering. May be an integer or a Hash. An integer is

      used to define the start run levels; stop run levels are then

      100-integer. A Hash is used to define values for specific run

      levels. For example, `{ 2 => [:start, 20], 3 => [:stop, 55] }` will

      set a priority of twenty for run level two and a priority of

      fifty-five for run level three.'
- property: reload_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to tell a service to reload its configuration.
- property: restart_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to restart a service.
- property: run_levels
  ruby_type: Array
  required: false
  description_list:
  - markdown: 'RHEL platforms only: Specific run_levels the service will run under.'
- property: service_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the service name if it differs from the
      resource block's name.
- property: start_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to start a service.
- property: status_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to check the run status for a service.
- property: stop_command
  ruby_type: String, false
  required: false
  description_list:
  - markdown: The command used to stop a service.
- property: supports
  ruby_type: Hash
  required: false
  default_value: '{"restart" => nil, "reload" => nil, "status" => nil}'
  description_list:
  - markdown: 'A list of properties that controls how Chef Infra Client is to

      attempt to manage a service: `:restart`, `:reload`, `:status`. For

      `:restart`, the init script or other service provider can use a

      restart command; if `:restart` is not specified, Chef Infra Client

      attempts to stop and then start a service. For `:reload`, the init

      script or other service provider can use a reload command. For

      `:status`, the init script or other service provider can use a

      status command to determine if the service is running; if `:status`

      is not specified, Chef Infra Client attempts to match the

      `service_name` against the process table as a regular expression,

      unless a pattern is specified as a parameter property. Default

      value: `{ restart: false, reload: false, status: false }` for all

      platforms (except for the Red Hat platform family, which defaults to

      `{ restart: false, reload: false, status: true }`.)'
- property: timeout
  ruby_type: Integer
  required: false
  default_value: '900'
  description_list:
  - markdown: The amount of time (in seconds) to wait before timing out.
- property: user
  ruby_type: String
  required: false
  new_in: '12.21'
  description_list:
  - markdown: 'systemd only: A username to run the service under.'
examples: "
  Start a service\n\n  ``` ruby\n  service 'example_service' do\n \
  \   action :start\n  end\n  ```\n\n  Start a service, enable it\n\n  ``` ruby\n\
  \  service 'example_service' do\n    supports status: true, restart: true, reload:\
  \ true\n    action [ :enable, :start ]\n  end\n  ```\n\n  Use a pattern\n\n  ```\
  \ ruby\n  service 'samba' do\n    pattern 'smbd'\n    action [:enable, :start]\n\
  \  end\n  ```\n\n  Use the :nothing common action\n\n  ``` ruby\n  service 'memcached'\
  \ do\n    action :nothing\n  end\n  ```\n\n  Use the retries common attribute\n\n\
  \  ``` ruby\n  service 'apache' do\n    action [ :enable, :start ]\n    retries\
  \ 3\n  end\n  ```\n\n  Manage a service, depending on the node platform\n\n  ```\
  \ ruby\n  service 'example_service' do\n    if redhat?\n      service_name 'redhat_name'\n\
  \    else\n      service_name 'other_name'\n    end\n    supports restart: true\n\
  \    action [ :enable, :start ]\n  end\n  ```\n\n  Reload a service using a template\n\
  \n  To reload a service that is based on a template, use the **template**\n  and\
  \ **service** resources together in the same recipe, similar to the\n  following:\n\
  \n  ``` ruby\n  template '/tmp/somefile' do\n    mode '0755'\n    source 'somefile.erb'\n\
  \  end\n\n  service 'apache' do\n    action :enable\n    subscribes :reload, 'template[/tmp/somefile]',\
  \ :immediately\n  end\n  ```\n\n  where the `subscribes` notification is used to\
  \ reload the service\n  whenever the template is modified.\n\n  Enable a service\
  \ after a restart or reload\n\n  ``` ruby\n  service 'apache' do\n    supports restart:\
  \ true, reload: true\n    action :enable\n  end\n  ```\n\n  Set an IP address using\
  \ variables and a template\n\n  The following example shows how the **template**\
  \ resource can be used in\n  a recipe to combine settings stored in an attributes\
  \ file, variables\n  within a recipe, and a template to set the IP addresses that\
  \ are used by\n  the Nginx service. The attributes file contains the following:\n\
  \n  ``` ruby\n  default['nginx']['dir'] = '/etc/nginx'\n  ```\n\n  The recipe then\
  \ does the following to:\n\n  -   Declare two variables at the beginning of the\
  \ recipe, one for the\n      remote IP address and the other for the authorized\
  \ IP address\n  -   Use the **service** resource to restart and reload the Nginx\
  \ service\n  -   Load a template named `authorized_ip.erb` from the `/templates`\n\
  \      directory that is used to set the IP address values based on the\n      variables\
  \ specified in the recipe\n\n  <!-- -->\n\n  ``` ruby\n  node.default['nginx']['remote_ip_var']\
  \ = 'remote_addr'\n  node.default['nginx']['authorized_ips'] = ['127.0.0.1/32']\n\
  \n  service 'nginx' do\n    supports :status => true, :restart => true, :reload\
  \ => true\n  end\n\n  template 'authorized_ip' do\n    path \"#{node['nginx']['dir']}/authorized_ip\"\
  \n    source 'modules/authorized_ip.erb'\n    owner 'root'\n    group 'root'\n \
  \   mode '0755'\n    variables(\n      :remote_ip_var => node['nginx']['remote_ip_var'],\n\
  \      :authorized_ips => node['nginx']['authorized_ips']\n    )\n\n    notifies\
  \ :reload, 'service[nginx]', :immediately\n  end\n  ```\n\n  where the `variables`\
  \ property tells the template to use the variables\n  set at the beginning of the\
  \ recipe and the `source` property is used to\n  call a template file located in\
  \ the cookbook's `/templates` directory.\n  The template file looks similar to:\n\
  \n  ``` ruby\n  geo $<%= @remote_ip_var %> $authorized_ip {\n    default no;\n \
  \   <% @authorized_ips.each do |ip| %>\n    <%= \"#{ip} yes;\" %>\n    <% end %>\n\
  \  }\n  ```\n\n  Use a cron timer to manage a service\n\n  The following example\
  \ shows how to install the crond application using\n  two resources and a variable:\n\
  \n  ``` ruby\n  # the following code sample comes from the ``cron`` cookbook:\n\
  \  # https://github.com/chef-cookbooks/cron\n\n  cron_package = case node['platform']\n\
  \    when 'redhat', 'centos', 'scientific', 'fedora', 'amazon'\n      node['platform_version'].to_f\
  \ >= 6.0 ? 'cronie' : 'vixie-cron'\n    else\n      'cron'\n    end\n\n  package\
  \ cron_package do\n    action :install\n  end\n\n  service 'crond' do\n    case\
  \ node['platform']\n    when 'redhat', 'centos', 'scientific', 'fedora', 'amazon'\n\
  \      service_name 'crond'\n    when 'debian', 'ubuntu', 'suse'\n      service_name\
  \ 'cron'\n    end\n    action [:start, :enable]\n  end\n  ```\n\n  where\n\n  -\
  \   `cron_package` is a variable that is used to identify which\n      platforms\
  \ apply to which install packages\n  -   the **package** resource uses the `cron_package`\
  \ variable to\n      determine how to install the crond application on various nodes\n\
  \      (with various platforms)\n  -   the **service** resource enables the crond\
  \ application on nodes that\n      have Red Hat, CentOS, Red Hat Enterprise Linux,\
  \ Fedora, or Amazon\n      Web Services (AWS), and the cron service on nodes that\
  \ run Debian,\n      Ubuntu, or openSUSE\n\n  Restart a service, and then notify\
  \ a different service\n\n  The following example shows how start a service named\
  \ `example_service`\n  and immediately notify the Nginx service to restart.\n\n\
  \  ``` ruby\n  service 'example_service' do\n    action :start\n    notifies :restart,\
  \ 'service[nginx]', :immediately\n  end\n  ```\n\n  Restart one service before restarting\
  \ another\n\n  This example uses the `:before` notification to restart the `php-fpm`\n\
  \  service before restarting `nginx`:\n\n  ``` ruby\n  service 'nginx' do\n    action\
  \ :restart\n    notifies :restart, 'service[php-fpm]', :before\n  end\n  ```\n\n\
  \  With the `:before` notification, the action specified for the `nginx`\n  resource\
  \ will not run until action has been taken on the notified\n  resource (`php-fpm`).\n\
  \n  Stop a service, do stuff, and then restart it\n\n  The following example shows\
  \ how to use the **execute**, **service**, and\n  **mount** resources together to\
  \ ensure that a node running on Amazon EC2\n  is running MySQL. This example does\
  \ the following:\n\n  -   Checks to see if the Amazon EC2 node has MySQL\n  -  \
  \ If the node has MySQL, stops MySQL\n  -   Installs MySQL\n  -   Mounts the node\n\
  \  -   Restarts MySQL\n\n  <!-- -->\n\n  ``` ruby\n  # the following code sample\
  \ comes from the ``server_ec2``\n  # recipe in the following cookbook:\n  # https://github.com/chef-cookbooks/mysql\n\
  \n  if (node.attribute?('ec2') && ! FileTest.directory?(node['mysql']['ec2_path']))\n\
  \n    service 'mysql' do\n      action :stop\n    end\n\n    execute 'install-mysql'\
  \ do\n      command \"mv #{node['mysql']['data_dir']} #{node['mysql']['ec2_path']}\"\
  \n      not_if do FileTest.directory?(node['mysql']['ec2_path']) end\n    end\n\n\
  \    [node['mysql']['ec2_path'], node['mysql']['data_dir']].each do |dir|\n    \
  \  directory dir do\n        owner 'mysql'\n        group 'mysql'\n      end\n \
  \   end\n\n    mount node['mysql']['data_dir'] do\n      device node['mysql']['ec2_path']\n\
  \      fstype 'none'\n      options 'bind,rw'\n      action [:mount, :enable]\n\
  \    end\n\n    service 'mysql' do\n      action :start\n    end\n\n  end\n  ```\n\
  \n  where\n\n  -   the two **service** resources are used to stop, and then restart\
  \ the\n      MySQL service\n  -   the **execute** resource is used to install MySQL\n\
  \  -   the **mount** resource is used to mount the node and enable MySQL\n\n  Control\
  \ a service using the execute resource\n\n  <div class=\"admonition-warning\"><p\
  \ class=\"admonition-warning-title\">Warning</p><div class=\"admonition-warning-text\"\
  >\n\n  This is an example of something that should NOT be done. Use the\n  **service**\
  \ resource to control a service, not the **execute** resource.\n\n  </div></div>\n\
  \n  Do something like this:\n\n  ``` ruby\n  service 'tomcat' do\n    action :start\n\
  \  end\n  ```\n\n  and NOT something like this:\n\n  ``` ruby\n  execute 'start-tomcat'\
  \ do\n    command '/etc/init.d/tomcat6 start'\n    action :run\n  end\n  ```\n\n\
  \  There is no reason to use the **execute** resource to control a service\n  because\
  \ the **service** resource exposes the `start_command` property\n  directly, which\
  \ gives a recipe full control over the command issued in a\n  much cleaner, more\
  \ direct manner.\n\n  Enable a service on AIX using the mkitab command\n\n  The\
  \ **service** resource does not support using the `:enable` and\n  `:disable` actions\
  \ with resources that are managed using System Resource\n  Controller (SRC). This\
  \ is because System Resource Controller (SRC) does\n  not have a standard mechanism\
  \ for enabling and disabling services on\n  system boot.\n\n  One approach for enabling\
  \ or disabling services that are managed by\n  System Resource Controller (SRC)\
  \ is to use the **execute** resource to\n  invoke `mkitab`, and then use that command\
  \ to enable or disable the\n  service.\n\n  The following example shows how to install\
  \ a service:\n\n  ``` ruby\n  execute \"install #{node['chef_client']['svc_name']}\
  \ in SRC\" do\n    command \"mkssys -s #{node['chef_client']['svc_name']}\n    \
  \                -p #{node['chef_client']['bin']}\n                    -u root\n\
  \                    -S\n                    -n 15\n                    -f 9\n \
  \                   -o #{node['chef_client']['log_dir']}/client.log\n          \
  \          -e #{node['chef_client']['log_dir']}/client.log -a '\n              \
  \      -i #{node['chef_client']['interval']}\n                    -s #{node['chef_client']['splay']}'\"\
  \n    not_if \"lssrc -s #{node['chef_client']['svc_name']}\"\n    action :run\n\
  \  end\n  ```\n\n  and then enable it using the `mkitab` command:\n\n  ``` ruby\n\
  \  execute \"enable #{node['chef_client']['svc_name']}\" do\n    command \"mkitab\
  \ '#{node['chef_client']['svc_name']}:2:once:/usr/bin/startsrc\n               \
  \     -s #{node['chef_client']['svc_name']} > /dev/console 2>&1'\"\n    not_if \"\
  lsitab #{node['chef_client']['svc_name']}\"\n  end\n  ```\n"

---
