+++
title = "knife exec"
draft = false

aliases = ["/knife_exec.html", "/knife_exec/"]

[menu]
  [menu.workstation]
    title = "knife exec"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_exec.md knife exec"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_exec.md)

{{% knife_exec_summary %}}

## Authenticated API Requests

The `knife exec` subcommand can be used to make authenticated API
requests to the Chef Infra Server using the following methods:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Method</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>api.delete</code></td>
<td>Use to delete an object from the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><code>api.get</code></td>
<td>Use to get the details of an object on the Chef Infra Server.</td>
</tr>
<tr class="odd">
<td><code>api.post</code></td>
<td>Use to add an object to the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><code>api.put</code></td>
<td>Use to update an object on the Chef Infra Server.</td>
</tr>
</tbody>
</table>

These methods are used with the `-E` option, which executes that string
locally on the workstation using chef-shell. These methods have the
following syntax:

``` bash
knife exec -E 'api.method(/endpoint)'
```

where:

-   `api.method` is the corresponding authentication method ---
    `api.delete`, `api.get`, `api.post`, or `api.put`
-   `/endpoint` is an endpoint in the Chef Infra Server API

For example, to get the data for a node named "Example_Node":

``` bash
knife exec -E 'puts api.get("/nodes/Example_Node")'
```

and to ensure that the output is visible in the console, add the `puts`
in front of the API authorization request:

``` bash
knife exec -E 'puts api.get("/nodes/Example_Node")'
```

where `puts` is the shorter version of the `$stdout.puts` predefined
variable in Ruby.

The following example shows how to add a client named "IBM305RAMAC" and
the `/clients` endpoint, and then return the private key for that user
in the console:

``` bash
client_desc = {
    "name"  => "IBM305RAMAC",
    "admin" => false
  }

  new_client = api.post("/clients", client_desc)
  puts new_client["private_key"]
```

## Ruby Scripts

For Ruby scripts that will be run using the `exec` subcommand, note the
following:

-   The Ruby script must be located on the system from which knife is
    run (and not be located on any of the systems that knife will be
    managing).
-   Shell commands will be run from a management workstation. For
    example, something like `%x[ls -lash /opt/only-on-a-node]` would
    give you the directory listing for the "opt/only-on-a-node"
    directory or a "No such file or directory" error if the file does
    not already exist locally.
-   When the chef-shell DSL is available, the Chef Infra Client DSL will
    not be (unless the management workstation is also a Chef Infra
    Client). Without the Chef Infra Client DSL, a bash block cannot be
    used to run bash commands.

## Syntax

This subcommand has the following syntax:

``` bash
knife exec SCRIPT (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`-E CODE`, `--exec CODE`

:   A string of code to be executed.

`-p PATH:PATH`, `--script-path PATH:PATH`

:   A colon-separated path at which Ruby scripts are located. Use to
    override the default location for scripts. When this option is not
    specified, knife will look for scripts located in
    `chef-repo/.chef/scripts` directory.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**Run Ruby scripts**

There are three ways to use `knife exec` to run Ruby script files. For
example:

``` bash
knife exec /path/to/script_file
```

or:

``` bash
knife exec -E 'RUBY CODE'
```

or:

``` bash
knife exec
RUBY CODE
^D
```

**Chef Knife status**

To check the status of knife using a Ruby script named `status.rb`
(which looks like):

``` ruby
printf "%-5s %-12s %-8s %s\n", "Check In", "Name", "Ruby", "Recipes"
nodes.all do |n|
   checkin = Time.at(n['ohai_time']).strftime("%F %R")
   rubyver = n['languages']['ruby']['version']
   recipes = n.run_list.expand(_default).recipes.join(", ")
   printf "%-20s %-12s %-8s %s\n", checkin, n.name, rubyver, recipes
end
```

and is located in a directory named `scripts/`, enter:

``` bash
knife exec scripts/status.rb
```

**List available free memory**

To show the available free memory for all nodes, enter:

``` bash
knife exec -E 'nodes.all {|n| puts "#{n.name} has #{n.memory.total} free memory"}'
```

**List available search indexes**

To list all of the available search indexes, enter:

``` bash
knife exec -E 'puts api.get("search").keys'
```

**Query for multiple attributes**

To query a node for multiple attributes using a Ruby script named
`search_attributes.rb` (which looks like):

``` ruby
% cat scripts/search_attributes.rb
query = ARGV[2]
attributes = ARGV[3].split(",")
puts "Your query: #{query}"
puts "Your attributes: #{attributes.join(" ")}"
results = {}
search(:node, query) do |n|
   results[n.name] = {}
   attributes.each {|a| results[n.name][a] = n[a]}
end

puts results
exit 0
```

enter:

``` bash
% knife exec scripts/search_attributes.rb "hostname:test_system" ipaddress,fqdn
```

to return something like:

``` bash
Your query: hostname:test_system
Your attributes: ipaddress fqdn
{"test_system.example.com"=>{"ipaddress"=>"10.1.1.200", "fqdn"=>"test_system.example.com"}}
```

**Find shadow cookbooks**

To find all of the locations in which cookbooks exist that may shadow
each other, create a file called `shadow-check.rb` that contains the
following Ruby code:

``` ruby
config = Chef::Config

cookbook_loader = begin
  Chef::Cookbook::FileVendor.on_create { |manifest| Chef::Cookbook::FileSystemFileVendor.new(manifest, config[:cookbook_path]) }
  Chef::CookbookLoader.new(config[:cookbook_path])
end

ui = Chef::Knife::UI.new($stdout, $stderr, $stdin, {})

cookbook_loader.load_cookbooks

if cookbook_loader.merged_cookbooks.empty?
  ui.msg "cookbooks ok"
else
  ui.warn "* " * 40
  ui.warn(<<-WARNING)
The cookbooks: #{cookbook_loader.merged_cookbooks.join(', ')} exist in multiple places in your cookbook_path.
A composite version of these cookbooks has been compiled for uploading.

#{ui.color('IMPORTANT:', :red, :bold)} In a future version of Chef, this behavior will be removed and you will no longer
be able to have the same version of a cookbook in multiple places in your cookbook_path.
WARNING
  ui.warn "The affected cookbooks are located:"
  ui.output ui.format_for_display(cookbook_loader.merged_cookbook_paths)
  ui.warn "* " * 40
end
```

Put this file in the directory of your choice. Run the following
command:

``` bash
knife exec shadow-check.rb
```

and be sure to edit `shadow-check.rb` so that it defines the path to
that file correctly.
