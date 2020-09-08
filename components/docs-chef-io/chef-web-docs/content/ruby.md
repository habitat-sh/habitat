+++
title = "Ruby Guide"
draft = false

aliases = ["/ruby.html"]

[menu]
  [menu.infra]
    title = "Ruby Guide"
    identifier = "chef_infra/cookbook_reference/ruby.md Ruby Guide"
    parent = "chef_infra/cookbook_reference"
    weight = 130
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/ruby.md)

{{% ruby_summary %}}

As of Chef Infra Client 15.x, Chef Infra Client ships with Ruby 2.6.

## Ruby Basics

This section covers the basics of Ruby.

### Verify Syntax

Many people who are new to Ruby often find that it doesn't take very
long to get up to speed with the basics. For example, it's useful to
know how to check the syntax of a Ruby file, such as the contents of a
cookbook named `my_cookbook.rb`:

``` bash
ruby -c my_cookbook_file.rb
```

to return:

``` bash
Syntax OK
```

### Comments

Use a comment to explain code that exists in a cookbook or recipe.
Anything after a `#` is a comment.

``` ruby
# This is a comment.
```

### Local Variables

Assign a local variable:

``` ruby
x = 1
```

### Math

Do some basic arithmetic:

``` ruby
1 + 2           # => 3
2 * 7           # => 14
5 / 2           # => 2   (because both arguments are whole numbers)
5 / 2.0         # => 2.5 (because one of the numbers had a decimal place)
1 + (2 * 3)     # => 7   (you can use parentheses to group expressions)
```

### Strings

Work with strings:

``` ruby
'single quoted'   # => "single quoted"
"double quoted"   # => "double quoted"
'It\'s alive!'    # => "It's alive!" (the \ is an escape character)
'1 + 2 = 5'       # => "1 + 2 = 5" (numbers surrounded by quotes behave like strings)
```

Convert a string to uppercase or lowercase. For example, a hostname
named "Foo":

``` ruby
node['hostname'].downcase    # => "foo"
node['hostname'].upcase      # => "FOO"
```

#### Ruby in Strings

Embed Ruby in a string:

``` ruby
x = 'Bob'
"Hi, #{x}"      # => "Hi, Bob"
'Hello, #{x}'   # => "Hello, \#{x}" Notice that single quotes don't work with #{}
```

#### Escape Character

Use the backslash character (`\`) as an escape character when quotes
must appear within strings. However, you do not need to escape single
quotes inside double quotes. For example:

``` ruby
'It\'s alive!'                        # => "It's alive!"
"Won\'t you read Grant\'s book?"      # => "Won't you read Grant's book?"
```

#### Interpolation

When strings have quotes within quotes, use double quotes (`" "`) on the
outer quotes, and then single quotes (`' '`) for the inner quotes. For
example:

``` ruby
Chef::Log.info("Loaded from aws[#{aws['id']}]")
```

``` ruby
"node['mysql']['secretpath']"
```

``` ruby
"#{ENV['HOME']}/chef.txt"
```

``` ruby
antarctica_hint = hint?('antarctica')
if antarctica_hint['snow']
  "There are #{antarctica_hint['penguins']} penguins here."
else
  'There is no snow here, and penguins like snow.'
end
```

### Truths

Work with basic truths:

``` ruby
true            # => true
false           # => false
nil             # => nil
0               # => true ( the only false values in Ruby are false
                #    and nil; in other words: if it exists in Ruby,
                #    even if it exists as zero, then it is true.)
1 == 1          # => true ( == tests for equality )
1 == true       # => false ( == tests for equality )
```

#### Untruths

Work with basic untruths (`!` means not!):

``` ruby
!true           # => false
!false          # => true
!nil            # => true
1 != 2          # => true (1 is not equal to 2)
1 != 1          # => false (1 is not equal to itself)
```

#### Convert Truths

Convert something to either true or false (`!!` means not not!!):

``` ruby
!!true          # => true
!!false         # => false
!!nil           # => false (when pressed, nil is false)
!!0             # => true (zero is NOT false).
```

### Arrays

Create lists using arrays:

``` ruby
x = ['a', 'b', 'c']   # => ["a", "b", "c"]
x[0]                  # => "a" (zero is the first index)
x.first               # => "a" (see?)
x.last                # => "c"
x + ['d']             # => ["a", "b", "c", "d"]
x                     # => ["a", "b", "c"] ( x is unchanged)
x = x + ['d']         # => ["a", "b", "c", "d"]
x                     # => ["a", "b", "c", "d"]
```

#### Whitespace Arrays

The `%w` syntax is a Ruby shortcut for creating an array without
requiring quotes and commas around the elements.

For example:

``` ruby
if %w(debian ubuntu).include?(node['platform'])
  # do debian/ubuntu things with the Ruby array %w() shortcut
end
```

{{% ruby_style_patterns_string_quoting_vs_whitespace_array %}}

**Example**

WiX includes several tools -- such as `candle` (preprocesses and
compiles source files into object files), `light` (links and binds
object files to an installer database), and `heat` (harvests files from
various input formats). The following example uses a whitespace array
and the Chef InSpec `file` audit resource to verify if these three tools
are present:

``` ruby
%w(
  candle.exe
  heat.exe
  light.exe
).each do |utility|
  describe file("C:/wix/#{utility}") do
    it { should be_file }
  end
end
```

### Hash

A Hash is a list with keys and values. Sometimes hashes don't have a set
order:

``` ruby
h = {
  'first_name' => 'Bob',
  'last_name'  => 'Jones'
}
```

And sometimes they do. For example, first name then last name:

``` ruby
h.keys              # => ["first_name", "last_name"]
h['first_name']     # => "Bob"
h['last_name']      # => "Jones"
h['age'] = 23
h.keys              # => ["first_name", "age", "last_name"]
h.values            # => ["Jones", "Bob", 23]
```

### Regular Expressions

Use Perl-style regular expressions:

``` ruby
'I believe'  =~ /I/                       # => 0 (matches at the first character)
'I believe'  =~ /lie/                     # => 4 (matches at the 5th character)
'I am human' =~ /bacon/                   # => nil (no match - bacon comes from pigs)
'I am human' !~ /bacon/                   # => true (correct, no bacon here)
/give me a ([0-9]+)/ =~ 'give me a 7'     # => 0 (matched)
```

### Statements

Use conditions! For example, an `if` statement

``` ruby
if false
  # this won't happen
elsif nil
  # this won't either
else
  # code here will run though
end
```

or a `case` statement:

``` ruby
x = 'dog'
case x
when 'fish'
 # this won't happen
when 'dog', 'cat', 'monkey'
  # this will run
else
  # the else is an optional catch-all
end
```

#### if

An `if` statement can be used to specify part of a recipe to be used
when certain conditions are met. `else` and `elsif` statements can be
used to handle situations where either the initial condition is not met
or when there are other possible conditions that can be met. Since this
behavior is 100% Ruby, do this in a recipe the same way here as anywhere
else.

For example, using an `if` statement with the `platform` node attribute:

``` ruby
if node['platform'] == 'ubuntu'
  # do ubuntu things
end
```

#### case

A `case` statement can be used to handle a situation where there are a
lot of conditions. Use the `when` statement for each condition, as many
as are required.

For example, using a `case` statement with the `platform` node
attribute:

``` ruby
case node['platform']
when 'debian', 'ubuntu'
  # do debian/ubuntu things
when 'redhat', 'centos', 'fedora'
  # do redhat/centos/fedora things
end
```

For example, using a `case` statement with the `platform_family` node
attribute:

``` ruby
case node['platform_family']
when 'debian'
  # do things on debian-ish platforms (debian, ubuntu, linuxmint)
when 'rhel'
  # do things on RHEL platforms (redhat, centos, scientific, etc)
end
```

### Call a Method

Call a method on something with `.method_name()`:

``` ruby
x = 'My String'
x.split(' ')            # => ["My", "String"]
x.split(' ').join(', ') # => "My, String"
```

### Define a Method

Define a method (or a function, if you like):

``` ruby
def do_something_useless( first_argument, second_argument)
  puts "You gave me #{first_argument} and #{second_argument}"
end

do_something_useless( 'apple', 'banana')
# => "You gave me apple and banana"
do_something_useless 1, 2
# => "You gave me 1 and 2"
# see how the parentheses are optional if there's no confusion about what to do
```

### Ruby Class

Use the Ruby `File` class in a recipe. Because Chef has the **file**
resource, use `File` to use the Ruby `File` class. For example:

``` ruby
execute 'apt-get-update' do
  command 'apt-get update'
  ignore_failure true
  not_if { File.exist?('/var/lib/apt/periodic/update-success-stamp') }
end
```

### Include a Class

Use `:include` to include another Ruby class. For example:

``` ruby
::Chef::Recipe.send(:include, Opscode::OpenSSL::Password)
```

In non-Chef Ruby, the syntax is `include` (without the `:` prefix), but
without the `:` prefix Chef Infra Client will try to find a provider
named `include`. Using the `:` prefix tells Chef Infra Client to look
for the specified class that follows.

### Include a Parameter

The `include?` method can be used to ensure that a specific parameter is
included before an action is taken. For example, using the `include?`
method to find a specific parameter:

``` ruby
if %w(debian ubuntu).include?(node['platform'])
  # do debian/ubuntu things
end
```

or:

``` ruby
if %w{rhel}.include?(node['platform_family'])
  # do RHEL things
end
```

## Patterns to Follow

This section covers best practices for cookbook and recipe authoring.

### git Etiquette

Although not strictly a Chef style thing, please always ensure your
`user.name` and `user.email` are set properly in your `.gitconfig` file.

-   `user.name` should be your given name (e.g., "Julian Dunn")
-   `user.email` should be an actual, working e-mail address

This will prevent commit log entries similar to
`"guestuser <login@Bobs-Macbook-Pro.local>"`, which are unhelpful.

### Use of Hyphens

{{% ruby_style_patterns_hyphens %}}

### Cookbook Naming

Use a short organizational prefix for application cookbooks that are
part of your organization. For example, if your organization is named
SecondMarket, use `sm` as a prefix: `sm_postgresql` or `sm_httpd`.

### Cookbook Versioning

-   Use semantic versioning when numbering cookbooks.
-   Only upload stable cookbooks from master.
-   Only upload unstable cookbooks from the dev branch. Merge to master
    and bump the version when stable.
-   Always update CHANGELOG.md with any changes, with the JIRA ticket
    and a brief description.

### Cookbook Patterns

Good cookbook examples:

-   <https://github.com/chef-cookbooks/tomcat>
-   <https://github.com/chef-cookbooks/apparmor>
-   <https://github.com/chef-cookbooks/mysql>
-   <https://github.com/chef-cookbooks/httpd>

### Naming

Name things uniformly for their system and component. For example:

-   attributes: `node['foo']['bar']`
-   recipe: `foo::bar`
-   role: `foo-bar`
-   directories: `foo/bar` (if specific to component), `foo` (if not).
    For example: `/var/log/foo/bar`.

Name attributes after the recipe in which they are primarily used. e.g.
`node['postgresql']['server']`.

### Parameter Order

Follow this order for information in each resource declaration:

-   Source
-   Cookbook
-   Resource ownership
-   Permissions
-   Notifications
-   Action

For example:

``` ruby
template '/tmp/foobar.txt' do
  source 'foobar.txt.erb'
  owner  'someuser'
  group  'somegroup'
  mode   '0644'
  variables(
    foo: 'bar'
  )
  notifies :reload, 'service[whatever]'
  action :create
end
```

### File Modes

Always specify the file mode with a quoted 3-5 character string that
defines the octal mode:

``` ruby
mode '755'
```

``` ruby
mode '0755'
```

Wrong:

``` ruby
mode 755
```

### Specify Resource Action?

A resource declaration does not require the action to be specified
because Chef Infra Client will apply the default action for a resource
automatically if it's not specified within the resource block. For
example:

``` ruby
package 'monit'
```

will install the `monit` package because the `:install` action is the
default action for the **package** resource.

However, if readability of code is desired, such as ensuring that a
reader understands what the default action is for a custom resource or
stating the action for a resource whose default may not be immediately
obvious to the reader, specifying the default action is recommended:

``` ruby
ohai 'apache_modules' do
  action :reload
end
```

### Symbols or Strings?

Prefer strings over symbols, because they're easier to read and you
don't need to explain to non-Rubyists what a symbol is. Please retrofit
old cookbooks as you come across them.

Right:

``` ruby
default['foo']['bar'] = 'baz'
```

Wrong:

``` ruby
default[:foo][:bar] = 'baz'
```

### String Quoting

Use single-quoted strings in all situations where the string doesn't
need interpolation.

#### Whitespace Arrays

{{% ruby_style_patterns_string_quoting_vs_whitespace_array %}}

### Recipes

A recipe should be clean and well-commented. For example:

``` ruby
###########
# variables
###########

connection_info = {
  host: '127.0.0.1',
  port: '3306',
  username: 'root',
  password: 'm3y3sqlr00t'
}

#################
# Mysql resources
#################

mysql_service 'default' do
  port '3306'
  initial_root_password 'm3y3sqlr00t'
  action [:create, :start]
end

mysql_database 'wordpress_demo' do
  connection connection_info
  action :create
end

mysql_database_user 'wordpress_user' do
  connection connection_info
  database_name 'wordpress_demo'
  password 'w0rdpr3ssdem0'
  privileges [:create, :delete, :select, :update, :insert]
  action :grant
end

##################
# Apache resources
##################

httpd_service 'default' do
  listen_ports %w(80)
  mpm 'prefork'
  action [:create, :start]
end

httpd_module 'php' do
  notifies :restart, 'httpd_service[default]'
  action :create
end

###############
# Php resources
###############

package 'php-gd' do
  action :install
end

package 'php-mysql' do
  action :install
end

directory '/etc/php.d' do
  action :create
end

template '/etc/php.d/mysql.ini' do
  source 'mysql.ini.erb'
  action :create
end

httpd_config 'php' do
  source 'php.conf.erb'
  notifies :restart, 'httpd_service[default]'
  action :create
end

#####################
# wordpress resources
#####################

directory '/srv/wordpress_demo' do
  user 'apache'
  recursive true
  action :create
end

tar_extract 'https://wordpress.org/wordpress-4.1.tar.gz' do
  target_dir '/srv/wordpress_demo'
  tar_flags ['--strip-components 1']
  user 'apache'
  creates '/srv/wordpress_demo/index.php'
  action :extract
end

directory '/srv/wordpress_demo/wp-content' do
  user 'apache'
  action :create
end

httpd_config 'wordpress' do
  source 'wordpress.conf.erb'
  variables(
    servername: 'wordpress',
    server_aliases: %w(computers.biz www.computers.biz),
    document_root: '/srv/wordpress_demo'
    )
  notifies :restart, 'httpd_service[default]'
  action :create
end

template '/srv/wordpress_demo/wp-config.php' do
  source 'wp-config.php.erb'
  owner 'apache'
  variables(
    db_name: 'wordpress_demo',
    db_user: 'wordpress_user',
    db_password: 'w0rdpr3ssdem0',
    db_host: '127.0.0.1',
    db_prefix: 'wp_',
    db_charset: 'utf8',
    auth_key: 'You should probably use randomly',
    secure_auth_key: 'generated strings. These can be hard',
    logged_in_key: 'coded, pulled from encrypted databags,',
    nonce_key: 'or a ruby function that accessed an',
    auth_salt: 'arbitrary data source, such as a password',
    secure_auth_salt: 'vault. Node attributes could work',
    logged_in_salt: 'as well, but you take special care',
    nonce_salt: 'so they are not saved to your chef-server.',
    allow_multisite: 'false'
    )
  action :create
end
```

## Cookstyle Linting

Chef Workstation includes Cookstyle for linting the Ruby-specific and
Chef-specific portions of your cookbook code. All cookbooks should pass
Cookstyle rules before being uploaded.

``` bash
cookstyle your-cookbook
```

should return `no offenses detected`
