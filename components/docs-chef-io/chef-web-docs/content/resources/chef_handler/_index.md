---
resource_reference: true
handler_custom: true
handler_types: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: chef_handler resource
resource: chef_handler
aliases:
- "/resource_chef_handler.html"
menu:
  infra:
    title: chef_handler
    identifier: chef_infra/cookbook_reference/resources/chef_handler chef_handler
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: 'Use the **chef_handler** resource to enable handlers during a Chef

    Infra Client run. The resource allows arguments to be passed to Chef

    Infra Client, which then applies the conditions defined by the custom

    handler to the node attribute data collected during a Chef Infra Client

    run, and then processes the handler based on that data.


    The **chef_handler** resource is typically defined early in a node''s

    run-list (often being the first item). This ensures that all of the

    handlers will be available for the entire Chef Infra Client run.'
resource_new_in: '14.0'
syntax_description: "A **chef_handler** resource block enables handlers during a chef-client\n\
  run. Two handlers---`JsonFile` and `ErrorReport`---are built into Chef:\n\n``` ruby\n\
  chef_handler 'Chef::Handler::JsonFile' do\n  source 'chef/handler/json_file'\n \
  \ arguments :path => '/var/chef/reports'\n  action :enable\nend\n```\n\nand:\n\n\
  ``` ruby\nchef_handler 'Chef::Handler::ErrorReport' do\n  source 'chef/handler/error_report'\n\
  \  action :enable\nend\n```\n\nshow how to enable those handlers in a recipe."
syntax_full_code_block: |-
  chef_handler 'name' do
    arguments       Array, Hash
    class_name      String # default value: 'name' unless specified
    source          String
    type            Hash # default value: {"report"=>true, "exception"=>true}
    action          Symbol # defaults to :enable if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`chef_handler` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`arguments`, `class_name`, `source`, and `type` are the properties available to
  this resource."
actions_list:
  :disable:
    markdown: Disable the handler for the current chef-client run on the current node.
  :enable:
    markdown: Enable the handler for the current chef-client run on the current node.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: arguments
  ruby_type: Array, Hash
  required: false
  description_list:
  - markdown: 'An array of arguments that are passed to the initializer for the

      handler class. For example:


      ``` ruby

      arguments :key1 => ''val1''

      ```


      or:


      ``` ruby

      arguments [:key1 => ''val1'', :key2 => ''val2'']

      ```'
- property: class_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: The name of the handler class. This can be module name-spaced.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: The full path to the handler file. Can also be a gem path if the handler
      ships as part of a Ruby gem.
- property: type
  ruby_type: Hash
  required: false
  default_value: '{"report"=>true, "exception"=>true}'
  description_list:
  - markdown: The type of handler to register as, i.e. :report, :exception or both.
examples: "
  Enable the CloudkickHandler handler\n\n  The following example shows\
  \ how to enable the `CloudkickHandler`\n  handler, which adds it to the default\
  \ handler path and passes the\n  `oauth` key/secret to the handler's initializer:\n\
  \n  ``` ruby\n  chef_handler \"CloudkickHandler\" do\n    source \"#{node['chef_handler']['handler_path']}/cloudkick_handler.rb\"\
  \n    arguments [node['cloudkick']['oauth_key'], node['cloudkick']['oauth_secret']]\n\
  \    action :enable\n  end\n  ```\n\n  Enable handlers during the compile phase\n\
  \n  ``` ruby\n  chef_handler \"Chef::Handler::JsonFile\" do\n    source \"chef/handler/json_file\"\
  \n    arguments :path => '/var/chef/reports'\n    action :nothing\n  end.run_action(:enable)\n\
  \  ```\n\n  Handle only exceptions\n\n  ``` ruby\n  chef_handler \"Chef::Handler::JsonFile\"\
  \ do\n    source \"chef/handler/json_file\"\n    arguments :path => '/var/chef/reports'\n\
  \    supports :exception => true\n    action :enable\n  end\n  ```\n\n  Cookbook\
  \ Versions (a custom handler)\n\n  Community member `juliandunn` created a custom\
  \ [report handler that logs\n  all of the cookbooks and cookbook\n  versions](https://github.com/juliandunn/cookbook_versions_handler)\
  \ that\n  were used during a Chef Infra Client run, and then reports after the run\n\
  \  is complete. This handler requires the **chef_handler** resource (which\n  is\
  \ available from the **chef_handler** cookbook).\n\n  cookbook_versions.rb:\n\n\
  \  The following custom handler defines how cookbooks and cookbook versions\n  that\
  \ are used during a Chef Infra Client run will be compiled into a\n  report using\
  \ the `Chef::Log` class in Chef Infra Client:\n\n  ``` ruby\n  require 'chef/log'\n\
  \n  module Opscode\n    class CookbookVersionsHandler < Chef::Handler\n\n      def\
  \ report\n        cookbooks = run_context.cookbook_collection\n        Chef::Log.info('Cookbooks\
  \ and versions run: #{cookbooks.keys.map {|x| cookbooks[x].name.to_s + ' ' + cookbooks[x].version}\
  \ }')\n      end\n    end\n  end\n  ```\n\n  default.rb:\n\n  The following recipe\
  \ is added to the run-list for every node on which a\n  list of cookbooks and versions\
  \ will be generated as report output after\n  every Chef Infra Client run.\n\n \
  \ ``` ruby\n  include_recipe 'chef_handler'\n\n  cookbook_file \"#{node['chef_handler']['handler_path']}/cookbook_versions.rb\"\
  \ do\n    source 'cookbook_versions.rb'\n    owner 'root'\n    group 'root'\n  \
  \  mode '0755'\n    action :create\n  end\n\n  chef_handler 'Opscode::CookbookVersionsHandler'\
  \ do\n    source \"#{node['chef_handler']['handler_path']}/cookbook_versions.rb\"\
  \n    supports :report => true\n    action :enable\n  end\n  ```\n\n  This recipe\
  \ will generate report output similar to the following:\n\n  ``` ruby\n  [2013-11-26T03:11:06+00:00]\
  \ INFO: Chef Run complete in 0.300029878 seconds\n  [2013-11-26T03:11:06+00:00]\
  \ INFO: Running report handlers\n  [2013-11-26T03:11:06+00:00] INFO: Cookbooks and\
  \ versions run: [\"chef_handler 1.1.4\", \"cookbook_versions_handler 1.0.0\"]\n\
  \  [2013-11-26T03:11:06+00:00] INFO: Report handlers complete\n  ```\n\n  JsonFile\
  \ Handler\n\n  The\n  [json_file](https://github.com/chef/chef/blob/master/lib/chef/handler/json_file.rb)\n\
  \  handler is available from the **chef_handler** cookbook and can be used\n  with\
  \ exceptions and reports. It serializes run status data to a JSON\n  file. This\
  \ handler may be enabled in one of the following ways.\n\n  By adding the following\
  \ lines of Ruby code to either the client.rb file\n  or the solo.rb file, depending\
  \ on how Chef Infra Client is being run:\n\n  ``` ruby\n  require 'chef/handler/json_file'\n\
  \  report_handlers << Chef::Handler::JsonFile.new(:path => '/var/chef/reports')\n\
  \  exception_handlers << Chef::Handler::JsonFile.new(:path => '/var/chef/reports')\n\
  \  ```\n\n  By using the **chef_handler** resource in a recipe, similar to the\n\
  \  following:\n\n  ``` ruby\n  chef_handler 'Chef::Handler::JsonFile' do\n    source\
  \ 'chef/handler/json_file'\n    arguments :path => '/var/chef/reports'\n    action\
  \ :enable\n  end\n  ```\n\n  After it has run, the run status data can be loaded\
  \ and inspected via\n  Interactive Ruby (IRb):\n\n  ``` ruby\n  irb(main):002:0>\
  \ require 'json' => true\n  irb(main):003:0> require 'chef' => true\n  irb(main):004:0>\
  \ r = JSON.parse(IO.read('/var/chef/reports/chef-run-report-20110322060731.json'))\
  \ => ... output truncated\n  irb(main):005:0> r.keys => ['end_time', 'node', 'updated_resources',\
  \ 'exception', 'all_resources', 'success', 'elapsed_time', 'start_time', 'backtrace']\n\
  \  irb(main):006:0> r['elapsed_time'] => 0.00246\n  ```\n\n  Register the JsonFile\
  \ handler\n\n  ``` ruby\n  chef_handler \"Chef::Handler::JsonFile\" do\n    source\
  \ \"chef/handler/json_file\"\n    arguments :path => '/var/chef/reports'\n    action\
  \ :enable\n  end\n  ```\n\n  ErrorReport Handler\n\n  The\n  [error_report](https://github.com/chef/chef/blob/master/lib/chef/handler/error_report.rb)\n\
  \  handler is built into Chef Infra Client and can be used for both\n  exceptions\
  \ and reports. It serializes error report data to a JSON file.\n  This handler may\
  \ be enabled in one of the following ways.\n\n  By adding the following lines of\
  \ Ruby code to either the client.rb file\n  or the solo.rb file, depending on how\
  \ Chef Infra Client is being run:\n\n  ``` ruby\n  require 'chef/handler/error_report'\n\
  \  report_handlers << Chef::Handler::ErrorReport.new()\n  exception_handlers <<\
  \ Chef::Handler::ErrorReport.new()\n  ```\n\n  By using the [chef_handler](/resources/chef_handler/)\
  \ resource in a\n  recipe, similar to the following:\n\n  ``` ruby\n  chef_handler\
  \ 'Chef::Handler::ErrorReport' do\n    source 'chef/handler/error_report'\n    action\
  \ :enable\n  end\n  ```\n"

---
