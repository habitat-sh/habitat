+++
title = "Writing Custom Knife Plugins"
draft = false

aliases = ["/plugin_knife_custom.html"]

[menu]
  [menu.api]
    title = "Writing Custom Plugins"
    identifier = "extension_apis/knife_plugins/plugin_knife_custom.md Writing Custom Plugins"
    parent = "extension_apis/knife_plugins"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/plugin_knife_custom.md)

{{% plugin_knife_summary %}}

The Chef Infra Client will load knife plugins from the following
locations:

-   The home directory: `~/.chef/plugins/knife/`
-   A `.chef/plugins/knife` directory in the cookbook repository
-   A plugin installed from RubyGems. (For more information about
    releasing a plugin on RubyGems, see:
    <http://guides.rubygems.org/make-your-own-gem/>.)

This approach allows knife plugins to be reused across projects in the
home directory, kept in a repository that is accessible to other team
members, and distributable to the community using RubyGems.

## Syntax

There are many ways to structure a knife plugin. The following syntax
shows a typical knife plugin:

``` ruby
require 'chef/knife'
# other require attributes, as needed

module ModuleName
  class SubclassName < Chef::Knife

    deps do
      require 'chef/dependency'
      # other dependencies, as needed
    end

    banner "knife subcommand argument VALUE (options)"

    option :name_of_option,
      :short => "-l VALUE",
      :long => "--long-option-name VALUE",
      :description => "The description for the option.",
      :proc => Proc.new { code_to_run }
      :boolean => true | false
      :default => default_value

    def run
      # Ruby code goes here
    end
  end
end
```

where:

-   `require` identifies any other knife subcommands and/or knife
    plugins required by this plugin
-   `module ModuleName` declares the knife plugin as its own namespace
-   `class SubclassName < Chef::Knife` declares the plugin as a subclass
    of `Knife`, which is in the `Chef` namespace. The capitalization of
    this name is important. For example, `SubclassName` would have a
    knife command of `knife subclass name`, whereas `Subclassname` would
    have a knife command of `knife subclassname`
-   `deps do` is a list of dependencies
-   `banner "knife subcommand argument VALUE (options)"` is displayed
    when a user enters `knife subclassName --help`
-   `option :name_of_option` defines each of the command-line options
    that are available for this plugin. For example,
    `knife subclass -l VALUE` or
    `knife subclass --long-option-name VALUE`
-   `def run` is the Ruby code that is executed when the command is run

and where for each command-line option:

-   `:short` defines the short option name
-   `:long` defines the long option name
-   `:description` defines a description that is displayed when a user
    enters `knife subclassName --help`
-   `:boolean` defines whether the option is `true` or `false`; if the
    `:short` and `:long` names define a `VALUE`, then this attribute
    must not be used
-   `:proc` defines code that determines the value for this option
-   `:default` defines a default value

The following example shows part of a knife plugin named
`knife windows`:

``` ruby
require 'chef/knife'
require 'chef/knife/winrm_base'

class Chef
  class Knife
    class Winrm < Knife

      include Chef::Knife::WinrmBase

      deps do
        require 'readline'
        require 'chef/search/query'
        require 'em-winrm'
      end

      attr_writer :password

      banner "knife winrm QUERY COMMAND (options)"

      option :attribute,
        :short => "-a ATTR",
        :long => "--attribute ATTR",
        :description => "The attribute to use for opening the connection - default is fqdn",
        :default => "fqdn"

      ... # more options

      def session
        session_opts = {}
        session_opts[:logger] = Chef::Log.logger if Chef::Log.level == :debug
        @session ||= begin
          s = EventMachine::WinRM::Session.new(session_opts)
          s.on_output do |host, data|
            print_data(host, data)
          end
          s.on_error do |host, err|
            print_data(host, err, :red)
          end
          s.on_command_complete do |host|
            host = host == :all ? 'All Servers' : host
            Chef::Log.debug("command complete on #{host}")
          end
          s
        end

      end

      ... # more def blocks

    end
  end
end
```

Take a look at all of the code for this plugin on GitHub:
<https://github.com/chef/knife-windows/blob/master/lib/chef/knife/winrm.rb>.

### Namespace

A knife plugin should have its own namespace (even though knife will
load a command regardless of its namespace). The namespace is declared
using the `module` method, for example:

``` ruby
require 'chef/knife'
## other require attributes, as needed

module MyNamespace
  class SubclassName < Chef::Knife
```

where `module MyNamespace` declares that the knife plugin has its own
namespace, with a namespace of `MyNamespace`.

### Class Name

The class name declares a plugin as a subclass of both `Knife` and
`Chef`. For example:

``` ruby
class SubclassName < Chef::Knife
```

where `SubclassName` is the class name used by this plugin. The
capitalization of this name is important. For example, `OMG` would have
a knife command of `knife o m g`, whereas `Omg` would have a knife
command of `knife omg`. Use the capitalization pattern to define the
word grouping that best makes sense for the plugin.

A plugin can override an existing knife subcommand by using the same
class name as the existing subcommand. For example, to override the
current functionality of `knife cookbook upload`, use the following
class name:

``` ruby
class CookbookUpload < Chef::Knife
```

### Banner

A banner displays the syntax for the plugin to users when they enter the
`--help` option. Use the `banner` method in the class body similar to
the following:

``` ruby
module example
  class example < Chef::Knife

  banner "knife example"

  ...
end
```

and the when a user enters `knife --help`, the following will be
displayed:

``` bash
**EXAMPLE COMMANDS**
knife example
```

### Dependencies

The functionality of other knife plugins can be accessed from a plugin
by using the `deps` method to ensure the necessary files are available.
The `deps` method acts as a lazy loader, ensuring that dependencies are
only loaded into knife when the plugin which requires them is run. Use
the following syntax just below the class declaration:

``` ruby
class subclassName < Chef::Knife

deps do
  require 'chef/knife/name_of_command'
  require 'chef/search/query'
  # other dependencies, as needed
end
```

where the actual path may vary from plugin to plugin, but is typically
located in the `chef/knife/` directory.

{{< note >}}

Using the `deps` method instead of `require` is recommended, especially
if the environment in which knife is being run contains a lot of plugins
and/or any of those plugins have a lot of dependencies and/or
requirements on other plugins and search functionality.

{{< /note >}}

#### Requirements

The functionality of other knife plugins can be accessed from a plugin
by using the `require` method to ensure the necessary files are
available, and then within the code for the plugin, to create a new
object of the class of the plugin to be used.

First, ensure that the correct files are available using the following
syntax:

``` ruby
require 'chef/knife/name_of_command'
```

where the actual path may vary from plugin to plugin, but is typically
located in the `chef/knife/` directory.

{{< note >}}

Using the `deps` method instead of `require` is recommended, especially
when the environment in which knife is being run contains a lot of
plugins and/or any of those plugins have a lot of dependencies and/or
requirements on other plugins and search functionality.

{{< /note >}}

For example, use the following to require a plugin named `bootstrap`:

``` ruby
require 'chef/knife/bootstrap'
```

Next, for the required plugin, create an object of that plugin, like
this:

``` ruby
bootstrap = Chef::Knife::Bootstrap.new
```

and then pass arguments or options to that object. This is done by
altering that object's `config` and `name_arg` variables. For example:

``` ruby
bootstrap.config[:ssh_user] = "myuser"
bootstrap.config[:distro] = "ubuntu10.04-gems"
bootstrap.config[:use_sudo] = true

bootstrap.name_args = "some_host_name"
```

where the available configuration objects vary from plugin to plugin.
Make sure those configuration objects are correct by verifying them in
the source files for each plugin.

And then call the object's `run` method, like this:

``` ruby
bootstrap.run
```

### Options

Command-line options can be added to a knife plugin using the `option`
method. An option can have a true/false value:

``` ruby
option :true_or_false,
  :short => "-t",
  :long => "--true-or-false",
  :description => "Is this value true? Or is this value false?",
  :boolean => true | false
  :default => true
```

and it can have a string value:

``` ruby
option :some_type_of_string_value,
  :short => "-s VALUE",
  :long => "--some-type-of-string-value VALUE",
  :description => "This is not a random string value.",
  :default => 47
```

and can specify code that is run to determine the option's value:

``` ruby
option :tags,
  :short => "-T T=V[,T=V,...]",
  :long => "--tags Tag=Value[,Tag=Value...]",
  :description => "A list of tags associated with the virtual machine",
  :proc => Proc.new { |tags| tags.split(',') }
```

where the knife command allows a comma-separated list of values and the
`:proc` attribute converts that list of values into an array.

When a user enters `knife --help`, the description attributes are
displayed as part of the help. Using the previous examples, something
like the following will be displayed:

``` bash
**EXAMPLE COMMANDS**
knife example
  -s, --some-type-of-string-value     This is not a random string value.
  -t, --true-or-false                 Is this value true? Or is this value false?
  -T, --tags                          A list of tags associated with the virtual machine.
```

When knife runs the command, the options are parsed from the
command-line and make the settings available as a hash that can be used
to access the `config` method. For example, the following option:

``` ruby
option :omg,
  :short => '-O',
  :long => '--omg',
  :boolean => true,
  :description => "I'm so excited!"
```

can be used to update the `run` method of a class to change its behavior
based on the `config` flag, similar to the following:

``` ruby
def run
  if config[:omg]
    # Oh yeah, we are pumped.
    puts "OMG HELLO WORLD!!!1!!11"
  else
    # meh
    puts "I am just a boring example."
  end
end
```

For a knife plugin with the `--omg` option, run `knife example --omg` to
return something like:

``` bash
OMG HELLO WORLD!!!1!!11
```

or just `knife example` to return:

``` bash
I am just a boring example.
```

### Arguments

A knife plugin can also take command-line arguments that are not
specified using the `option` flag, for example: `knife node show NODE`.
These arguments are added using the `name_args` method. For example:

``` ruby
banner "knife hello world WHO"

def run
  unless name_args.size == 1
    puts "You need to say hello to someone!"
    show_usage
    exit 1
  end

  who = name_args.first

  if config[:omg]
    puts "OMG HELLO #{who.upcase}!!!1!!11"
  else
    puts "Hello, #{who.capitalize}!"
  end
end
```

where

-   `unless name_args.size == 1` is used to check the number of
    arguments given; the command should fail if the input does not make
    sense
-   `who = name_args.first` is used to access arguments using
    `name_args`
-   `show_usage` is used to display the correct usage before exiting (if
    the command fails)

For example, the following command:

``` bash
knife hello world
```

will return:

``` bash
You need to say hello to someone!
USAGE: knife hello world WHO
```

the following command:

``` bash
knife hello world chefs
```

will return:

``` bash
Hello, Chefs!
```

and the following command:

``` bash
knife hello world chefs --omg
```

will return:

``` bash
OMG HELLO CHEFS!!!1!!11
```

### config.rb Settings

Certain settings defined by a knife plugin can be configured so that
they can be set using the config.rb file. This can be done in two ways:

-   By using the `:proc` attribute of the `option` method and code that
    references `Chef::Config[:knife][:setting_name]`
-   By specifying the configuration setting directly within the `def`
    Ruby blocks using either `Chef::Config[:knife][:setting_name]` or
    `config[:setting_name]`

An option that is defined in this manner may be configured using the
config.rb file with the following syntax:

``` ruby
knife[:setting_name]
```

This approach can be useful when a particular setting is used a lot. The
order of precedence for a knife option is:

1.  A value passed via the command line
2.  A value saved in the config.rb file
3.  A default value

The following example shows how the `knife bootstrap` subcommand checks
for a value in the config.rb file by using the `:proc` attribute:

``` ruby
option :ssh_port,
  :short => "-p PORT",
  :long => "--ssh-port PORT",
  :description => "The ssh port",
  :proc => Proc.new { |key| Chef::Config[:knife][:ssh_port] = key }
```

where `Chef::Config[:knife][:ssh_port]` tells knife to check the
config.rb file for a setting named `knife[:ssh_port]`.

And the following example shows the `knife bootstrap` subcommand calling
the `knife ssh` subcommand for the actual SSH part of running a
bootstrap operation:

``` ruby
def knife_ssh
  ssh = Chef::Knife::Ssh.new
  ssh.ui = ui
  ssh.name_args = [ server_name, ssh_command ]
  ssh.config[:ssh_user] = Chef::Config[:knife][:ssh_user] || config[:ssh_user]
  ssh.config[:ssh_password] = config[:ssh_password]
  ssh.config[:ssh_port] = Chef::Config[:knife][:ssh_port] || config[:ssh_port]
  ssh.config[:ssh_gateway] = Chef::Config[:knife][:ssh_gateway] || config[:ssh_gateway]
  ssh.config[:identity_file] = Chef::Config[:knife][:identity_file] || config[:identity_file]
  ssh.config[:manual] = true
  ssh.config[:host_key_verify] = Chef::Config[:knife][:host_key_verify] || config[:host_key_verify]
  ssh.config[:on_error] = :raise
  ssh
end
```

where

-   `ssh = Chef::Knife::Ssh.new` creates a new instance of the `Ssh`
    subclass named `ssh`
-   A series of settings in `knife ssh` are associated with
    `knife bootstrap` using the `ssh.config[:setting_name]` syntax
-   `Chef::Config[:knife][:setting_name]` tells knife to check the
    config.rb file for various settings
-   Raises an exception if any aspect of the SSH operation fails

### Search

Use the Chef Infra Server search capabilities from a plugin to return
information about the infrastructure to that plugin. Use the `require`
method to ensure that search functionality is available with the
following:

``` ruby
require 'chef/search/query'
```

Create a search query object and assign it to a variable:

``` ruby
variable_name = Chef::Search::Query.new
```

After the search object is created it can be used by the plugin to
execute search queries for objects on the Chef Infra Server. For
example, using a variable named `query_nodes` a plugin could search for
nodes with the `webserver` role and then return the name of each node
found:

``` ruby
query = "role:webserver"

query_nodes.search('node', query) do |node_item|
  puts "Node Name: #{node_item.name}"
end
```

This result can then be used to edit nodes. For example, searching for
nodes with the `webserver` role, and then changing the run_list for
those nodes to a role named `apache2`:

``` ruby
query = "role:webserver"

query_nodes.search('node', query) do |node_item|
  ui.msg "Changing the run_list to role[apache2] for #{node_item.name}"
  node_item.run_list("role[apache2]")
  node_item.save
  ui.msg "New run_list: #{node_item.run_list}"
end
```

It's also possible to specify multiple items to add to the run_list:

``` ruby
node_item.run_list("role[apache2]", "recipe[mysql]")
```

And arguments sent with a plugin command can also be used to search. For
example, if the command `knife envchange "web*"` is sent, then the
command will search for any nodes in roles beginning with "web" and then
change their environment to "web":

``` ruby
module MyKnifePlugins

  class Envchange < Chef::Knife

    banner "knife envchange ROLE"

    deps do
      require 'chef/search/query'
    end

    def run
      if name_args.size == 1
        role = name_args.first
      else
        ui.fatal "Please provide a role name to search for"
        exit 1
      end

      query = "role:#{role}"
      query_nodes = Chef::Search::Query.new

      query_nodes.search('node', query) do |node_item|
        ui.msg "Moving #{node_item.name} to the web environment"
        node_item.chef_environment("web")
        node_item.save
      end

    end
  end
```

### User Interaction

The `ui` object provides a set of methods that can be used to define
user interactions and to help ensure a consistent user experience across
knife plugins. The following methods should be used in favor of manually
handling user interactions:

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
<td><code>ui.ask(*args, &amp;block)</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>ui.ask_question(question, opts={})</code></td>
<td>Use to ask a user the question contained in <code>question</code>. If <code>:default =&gt; default_value</code> is passed as the second argument, <code>default_value</code> will be used if the user does not provide an answer. This method will respect the <code>--default</code> command-line option.</td>
</tr>
<tr class="odd">
<td><p><code>ui.color(string, *colors)</code></p></td>
<td><p>Use to specify a color. For example, from the <code>knife rackspace server list</code> subcommand:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>server_list = [</span>
<span id="cb1-2"><a href="#cb1-2"></a>  ui.color(<span class="st">&#39;Instance ID&#39;</span>, <span class="st">:bold</span>),</span>
<span id="cb1-3"><a href="#cb1-3"></a>  ui.color(<span class="st">&#39;Name&#39;</span>, <span class="st">:bold</span>),</span>
<span id="cb1-4"><a href="#cb1-4"></a>  ui.color(<span class="st">&#39;Public IP&#39;</span>, <span class="st">:bold</span>),</span>
<span id="cb1-5"><a href="#cb1-5"></a>  ui.color(<span class="st">&#39;Private IP&#39;</span>, <span class="st">:bold</span>),</span>
<span id="cb1-6"><a href="#cb1-6"></a>  ui.color(<span class="st">&#39;Flavor&#39;</span>, <span class="st">:bold</span>),</span>
<span id="cb1-7"><a href="#cb1-7"></a>  ui.color(<span class="st">&#39;Image&#39;</span>, <span class="st">:bold</span>),</span>
<span id="cb1-8"><a href="#cb1-8"></a>  ui.color(<span class="st">&#39;State&#39;</span>, <span class="st">:bold</span>)</span>
<span id="cb1-9"><a href="#cb1-9"></a>]</span></code></pre></div>
<p>and from the <code>knife eucalyptus server create</code> subcommand:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>server = connection.servers.create(server_def)</span>
<span id="cb2-2"><a href="#cb2-2"></a>  puts <span class="st">&quot;</span><span class="ot">#{</span>ui.color(<span class="st">&quot;Instance ID&quot;</span>, <span class="st">:cyan</span>)<span class="ot">}</span><span class="st">: </span><span class="ot">#{</span>server.id<span class="ot">}</span><span class="st">&quot;</span></span>
<span id="cb2-3"><a href="#cb2-3"></a>  puts <span class="st">&quot;</span><span class="ot">#{</span>ui.color(<span class="st">&quot;Flavor&quot;</span>, <span class="st">:cyan</span>)<span class="ot">}</span><span class="st">: </span><span class="ot">#{</span>server.flavor_id<span class="ot">}</span><span class="st">&quot;</span></span>
<span id="cb2-4"><a href="#cb2-4"></a>  puts <span class="st">&quot;</span><span class="ot">#{</span>ui.color(<span class="st">&quot;Image&quot;</span>, <span class="st">:cyan</span>)<span class="ot">}</span><span class="st">: </span><span class="ot">#{</span>server.image_id<span class="ot">}</span><span class="st">&quot;</span></span>
<span id="cb2-5"><a href="#cb2-5"></a>  ...</span>
<span id="cb2-6"><a href="#cb2-6"></a>  puts <span class="st">&quot;</span><span class="ot">#{</span>ui.color(<span class="st">&quot;SSH Key&quot;</span>, <span class="st">:cyan</span>)<span class="ot">}</span><span class="st">: </span><span class="ot">#{</span>server.key_name<span class="ot">}</span><span class="st">&quot;</span></span>
<span id="cb2-7"><a href="#cb2-7"></a>print <span class="st">&quot;\n</span><span class="ot">#{</span>ui.color(<span class="st">&quot;Waiting for server&quot;</span>, <span class="st">:magenta</span>)<span class="ot">}</span><span class="st">&quot;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><code>ui.color?()</code></td>
<td>Indicates that colored output should be used. (Colored output can only be used when output is sent to a terminal.)</td>
</tr>
<tr class="odd">
<td><code>ui.confirm(question, append_instructions=true)</code></td>
<td>Use to ask a Y/N question. If the user responds with <code>N</code>, immediately exit with status code 3.</td>
</tr>
<tr class="even">
<td><code>ui.edit_data(data, parse_output=true)</code></td>
<td>Use to edit data. This opens the $EDITOR.</td>
</tr>
<tr class="odd">
<td><code>ui.edit_object(klass, name)</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>ui.error</code></td>
<td>Use to present an error to the user.</td>
</tr>
<tr class="odd">
<td><code>ui.fatal</code></td>
<td>Use to present a fatal error to the user.</td>
</tr>
<tr class="even">
<td><code>ui.highline</code></td>
<td>Use to provide direct access to the <a href="http://highline.rubyforge.org/doc/">Highline object</a> used by many <code>ui</code> methods.</td>
</tr>
<tr class="odd">
<td><code>ui.info</code></td>
<td>Use to present a message to a user.</td>
</tr>
<tr class="even">
<td><code>ui.interchange</code></td>
<td>Use to determine if the output is a data interchange format such as JSON or YAML.</td>
</tr>
<tr class="odd">
<td><code>ui.list(*args)</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>ui.msg(message)</code></td>
<td>Use to present a message to the user.</td>
</tr>
<tr class="odd">
<td><code>ui.output(data)</code></td>
<td>Use to present a data structure to the user. This method will respect the output requested when the <code>-F</code> command-line option is used. The output will use the generic default presenter.</td>
</tr>
<tr class="even">
<td><code>ui.pretty_print(data)</code></td>
<td>Use to enable pretty-print output for JSON data.</td>
</tr>
<tr class="odd">
<td><code>ui.use_presenter(presenter_class)</code></td>
<td>Use to specify a custom output presenter.</td>
</tr>
<tr class="even">
<td><code>ui.warn(message)</code></td>
<td>Use to present a warning to the user.</td>
</tr>
</tbody>
</table>

For example, to show a fatal error in a plugin in the same way that it
would be shown in knife do something similar to the following:

``` ruby
unless name_args.size == 1
  ui.fatal "Be sure to say hello to someone!"
  show_usage
  exit 1
end
```

## Create a Plugin

A knife command is a Ruby class that inherits from the `Chef::Knife`
class. A knife command is run by calling the `run` method on an instance
of the command class. For example:

``` ruby
module MyKnifePlugins
  class HelloWorld < Chef::Knife

    def run
      puts "Hello, World!"
    end
  end
end
```

and is run from the command line using:

``` bash
knife hello world
```

## Exceptions

In most cases, the exception handling available within knife is enough
to ensure that exception handling for a plugin is consistent with how
knife ordinarily behaves. That said, exceptions can also be handled
within a knife plugin in the same way they are handled in any Ruby
program.

## Install a Plugin

To install a knife plugin from a file, do one of the following:

-   Copy the file to the `~/.chef/plugins/knife` directory; the file's
    extension must be `.rb`
-   Add the file to the chef-repo at the
    `CHEF_REPO/.chef/plugins/knife`; the file's extension must be `.rb`
-   Install the plugin from RubyGems
