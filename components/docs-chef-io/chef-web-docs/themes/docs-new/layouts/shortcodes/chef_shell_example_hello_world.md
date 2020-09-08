This example shows how to run chef-shell in standalone mode. (For
chef-solo or Chef Infra Client modes, you would need to run chef-shell
using the `-s` or `-z` command line options, and then take into
consideration the necessary configuration settings.)

When Chef Infra Client is installed using RubyGems or a package manager,
chef-shell should already be installed. When Chef Infra Client is run
from a git clone, it will be located in `chef/bin/chef shell`. To start
chef-shell, just run it without any options. You'll see the loading
message, then the banner, and then the chef-shell prompt:

``` bash
bin/chef-shell

  loading configuration: none (standalone session)
  Session type: standalone
  Loading.............done.

  Welcome to the chef-shell 15.8.23
  For usage see https://docs.chef.io/chef_shell.html

  run `help' for help, `exit' or ^D to quit.

  chef (15.8.23)>
```

(Use the help command to print a list of supported commands.) Use the
recipe_mode command to switch to recipe context:

``` bash
chef > recipe_mode
  chef:recipe_mode >
```

Typing is evaluated in the same context as recipes. Create a file
resource:

``` bash
chef:recipe_mode > file "/tmp/ohai2u_shef"
    => #< Chef::Resource::File:0x1b691ac
       @enclosing_provider=nil,
       @resource_name=:file,
       @before=nil,
       @supports={},
       @backup=5,
       @allowed_actions=[:nothing, :create, :delete, :touch, :create_if_missing],
       @only_if=nil,
       @noop=nil,
       @collection=#< Chef::ResourceCollection:0x1b9926c
       @insert_after_idx=nil,
       @resources_by_name={"file[/tmp/ohai2u_shef]"=>0},
       @resources=[#< Chef::Resource::File:0x1b691ac ...>]>,
       @updated=false,
       @provider=nil,
       @node=< Chef::Node:0xdeeaae
       @name="eigenstate.local">,
       @recipe_name=nil,
       @not_if=nil,
       @name="/tmp/ohai2u_shef",
       @action="create",
       @path="/tmp/ohai2u_shef",
       @source_line="/Users/username/ruby/chef/chef/(irb#1) line 1",
       @params={},
       @actions={},
       @cookbook_name=nil,
       @ignore_failure=false>
```

(The previous example was formatted for presentation.) At this point,
chef-shell has created the resource and put it in the run-list, but not
yet created the file. To initiate a Chef Infra Client run, use the
`run_chef` command:

``` bash
chef:recipe_mode > run_chef
  [Fri, 15 Jan 2020 10:42:47 -0800] DEBUG: Processing file[/tmp/ohai2u_shef]
  [Fri, 15 Jan 2020 10:42:47 -0800] DEBUG: file[/tmp/ohai2u_shef] using Chef::Provider::File
  [Fri, 15 Jan 2020 10:42:47 -0800] INFO: Creating file[/tmp/ohai2u_shef] at /tmp/ohai2u_shef
    => true
```

chef-shell can also switch to the same context as attribute files. Set
an attribute with the following syntax:

``` bash
chef:recipe_mode > attributes_mode
  chef:attributes > set[:hello] = "ohai2u-again"
    => "ohai2u-again"
  chef:attributes >
```

Switch back to recipe_mode context and use the attributes:

``` bash
chef:attributes > recipe_mode
    => :attributes
  chef:recipe_mode > file "/tmp/#{node.hello}"
```

Now, run Chef Infra Client again:

``` bash
chef:recipe_mode > run_chef
  [Fri, 15 Jan 2020 10:53:22 -0800] DEBUG: Processing file[/tmp/ohai2u_shef]
  [Fri, 15 Jan 2020 10:53:22 -0800] DEBUG: file[/tmp/ohai2u_shef] using Chef::Provider::File
  [Fri, 15 Jan 2020 10:53:22 -0800] DEBUG: Processing file[/tmp/ohai2u-again]
  [Fri, 15 Jan 2020 10:53:22 -0800] DEBUG: file[/tmp/ohai2u-again] using Chef::Provider::File
  [Fri, 15 Jan 2020 10:53:22 -0800] INFO: Creating file[/tmp/ohai2u-again] at /tmp/ohai2u-again
    => true
  chef:recipe_mode >
```

Because the first resource (`file[/tmp/ohai2u_shef]`) is still in the
run-list, it gets executed again. And because that file already exists,
Chef Infra Client doesn't attempt to re-create it. Finally, the files
were created using the `ls` method:

``` bash
chef:recipe_mode > ls("/tmp").grep(/ohai/)
    => ["ohai2u-again", "ohai2u_shef"]
  Shell Tutorial
```