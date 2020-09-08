---
resource_reference: true
debug_recipes_chef_shell: true
properties_shortcode: 
title: breakpoint resource
resource: breakpoint
aliases:
- "/resource_breakpoint.html"
menu:
  infra:
    title: breakpoint
    identifier: chef_infra/cookbook_reference/resources/breakpoint breakpoint
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **breakpoint** resource to add breakpoints to recipes. Run the
    chef-shell in Chef Infra Client mode, and then use those breakpoints to debug
    recipes. Breakpoints are ignored by the chef-client during an actual chef-client
    run. That said, breakpoints are typically used to debug recipes only when running
    them in a non-production environment, after which they are removed from those
    recipes before the parent cookbook is uploaded to the Chef server.
resource_new_in: '12.0'
syntax_full_code_block: |-
  breakpoint 'name' do
    action      Symbol # defaults to :break if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`breakpoint` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
actions_list:
  :break:
    markdown: 
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list: []
examples: "
  A recipe without a breakpoint\n\n  ``` ruby\n  yum_key node['yum']['elrepo']['key']\
  \ do\n    url  node['yum']['elrepo']['key_url']\n    action :add\n  end\n\n  yum_repository\
  \ 'elrepo' do\n    description 'ELRepo.org Community Enterprise Linux Extras Repository'\n\
  \    key node['yum']['elrepo']['key']\n    mirrorlist node['yum']['elrepo']['url']\n\
  \    includepkgs node['yum']['elrepo']['includepkgs']\n    exclude node['yum']['elrepo']['exclude']\n\
  \    action :create\n  end\n  ```\n\n  The same recipe with breakpoints\n\n  ```\
  \ ruby\n  breakpoint \"before yum_key node['yum']['repo_name']['key']\" do\n   \
  \ action :break\n  end\n\n  yum_key node['yum']['repo_name']['key'] do\n    url\
  \  node['yum']['repo_name']['key_url']\n    action :add\n  end\n\n  breakpoint \"\
  after yum_key node['yum']['repo_name']['key']\" do\n    action :break\n  end\n\n\
  \  breakpoint \"before yum_repository 'repo_name'\" do\n    action :break\n  end\n\
  \n  yum_repository 'repo_name' do\n    description 'description'\n    key node['yum']['repo_name']['key']\n\
  \    mirrorlist node['yum']['repo_name']['url']\n    includepkgs node['yum']['repo_name']['includepkgs']\n\
  \    exclude node['yum']['repo_name']['exclude']\n    action :create\n  end\n\n\
  \  breakpoint \"after yum_repository 'repo_name'\" do\n    action :break\n  end\n\
  \  ```\n\n  where the name of each breakpoint is an arbitrary string. In the\n \
  \ previous examples, the names are used to indicate if the breakpoint is\n  before\
  \ or after a resource, and then also to specify which resource.\n"

---
