+++
title = "About Environments"
draft = false

aliases = ["/environments.html"]

[menu]
  [menu.infra]
    title = "Environments"
    identifier = "chef_infra/concepts/policy/environments.md Environments"
    parent = "chef_infra/concepts/policy"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/environments.md)

{{% environment %}}

## The _default Environment

Every Chef Infra Server organization must have at least one environment.
Each organization starts out with a single `_default` environment. The
`_default` environment cannot be modified in any way. Nodes, roles,
run-lists, cookbooks (and cookbook versions), and attributes specific to
an organization can only be associated with a custom environment.
Additional environments can be created to reflect each organization's
patterns and workflow. For example, creating `production`, `staging`,
`testing`, and `development` environments.

## Environment Attributes

{{% environment_attribute %}}

{{< note >}}

{{% notes_see_attributes_overview %}}

{{< /note >}}

### Environment Attribute Types

There are two types of attributes that can be used with environments:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Attribute Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>default</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_default.md" >}}</td>
</tr>
<tr class="even">
<td><code>override</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_override.md" >}}</td>
</tr>
</tbody>
</table>

### Attribute Precedence

Environments are one of several locations where attributes can be
applied in Chef Infra. It's important to understand how the precedence
level works in order to understand, which attributes will be applied
when Chef Infra Client runs.

{{% node_attribute_precedence %}}

## Cookbook Pinning

Cookbook versions can be pinned in each environment, which allows you to
control the rollout of new cookbook releases through successive testing
environments before releasing new cookbook versions into production
environments. See the environment format examples below for the cookbook
pinning syntax.

## Environment Formats

Environments may be stored on disk (any in source control) in two
formats:

-   As Ruby (i.e. a file that ends with `.rb`); this format is not
    available when running Chef Infra Client in local mode
-   As JSON (i.e. a file that ends with `.json`)

### Ruby DSL

Each environment is defined as a Ruby file (i.e. a file that ends with
`.rb`). Each environment file should contain the following
domain-specific attributes:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>cookbook</code></p></td>
<td><p>A version constraint for a single cookbook. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>cookbook <span class="st">&#39;couchdb&#39;</span>, <span class="st">&#39;&lt; 11.0.0&#39;</span></span></code></pre></div>
<p>or:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>cookbook <span class="st">&#39;my_rails_app&#39;</span>, <span class="st">&#39;= 1.2.0&#39;</span></span></code></pre></div>
<p>or:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>cookbook <span class="st">&#39;gems&#39;</span>, <span class="st">&#39;~&gt; 1.4&#39;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>cookbook_versions</code></p></td>
<td><p>A version constraint for a group of cookbooks. For example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>cookbook_versions(</span>
<span id="cb4-2"><a href="#cb4-2"></a>  <span class="st">&#39;couchdb&#39;</span> =&gt; <span class="st">&#39;= 11.0.0&#39;</span>,</span>
<span id="cb4-3"><a href="#cb4-3"></a>  <span class="st">&#39;my_rails_app&#39;</span> =&gt; <span class="st">&#39;~&gt; 1.2.0&#39;</span></span>
<span id="cb4-4"><a href="#cb4-4"></a>)</span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>default_attributes</code></p></td>
<td><p>Optional. A set of attributes to be applied to all nodes, assuming the node does not already have a value for the attribute. This is useful for setting global defaults that can then be overridden for specific nodes. If more than one role attempts to set a default value for the same attribute, the last role applied is the role to set the attribute value. When nested attributes are present, they are preserved. For example, to specify that a node that has the attribute <code>apache2</code> should listen on ports 80 and 443 (unless ports are already specified):</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a>default_attributes <span class="st">&#39;apache2&#39;</span> =&gt; { <span class="st">&#39;listen_ports&#39;</span> =&gt;<span class="ot"> %w(</span><span class="st">80 443</span><span class="ot">)</span> }</span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>description</code></p></td>
<td><p>A description of the functionality that is covered. For example:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb6-1"><a href="#cb6-1"></a>description <span class="st">&#39;The development environment&#39;</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>name</code></p></td>
<td><p>A unique name within the organization. Each name must be made up of letters (upper- and lower-case), numbers, underscores, and hyphens: [A-Z][a-z][0-9] and [_-]. Spaces are not allowed. For example:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb7-1"><a href="#cb7-1"></a>name <span class="st">&#39;dev01-24&#39;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>override_attributes</code></p></td>
<td><p>Optional. A set of attributes to be applied to all nodes, even if the node already has a value for an attribute. This is useful for ensuring that certain attributes always have specific values. If more than one role attempts to set an override value for the same attribute, the last role applied wins. When nested attributes are present, they are preserved. For example:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb8-1"><a href="#cb8-1"></a>override_attributes <span class="st">&#39;apache2&#39;</span> =&gt; { <span class="st">&#39;max_children&#39;</span> =&gt; <span class="st">&#39;50&#39;</span> }</span></code></pre></div>
<p>The parameters in a Ruby file are actually Ruby method calls, so parentheses can be used to provide clarity when specifying numerous or deeply-nested attributes. For example:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb9-1"><a href="#cb9-1"></a>override_attributes(</span>
<span id="cb9-2"><a href="#cb9-2"></a>  <span class="st">apache2: </span>{</span>
<span id="cb9-3"><a href="#cb9-3"></a>    <span class="st">prefork: </span>{ <span class="st">min_spareservers: </span><span class="ch">&#39;5&#39;</span> },</span>
<span id="cb9-4"><a href="#cb9-4"></a>  }</span>
<span id="cb9-5"><a href="#cb9-5"></a>)</span></code></pre></div>
<p>or:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb10-1"><a href="#cb10-1"></a>override_attributes(</span>
<span id="cb10-2"><a href="#cb10-2"></a>  <span class="st">apache2: </span>{</span>
<span id="cb10-3"><a href="#cb10-3"></a>    <span class="st">prefork: </span>{ <span class="st">min_spareservers: </span><span class="ch">&#39;5&#39;</span> },</span>
<span id="cb10-4"><a href="#cb10-4"></a>  },</span>
<span id="cb10-5"><a href="#cb10-5"></a>  <span class="st">tomcat: </span>{</span>
<span id="cb10-6"><a href="#cb10-6"></a>    <span class="st">worker_threads: &#39;100&#39;</span>,</span>
<span id="cb10-7"><a href="#cb10-7"></a>  }</span>
<span id="cb10-8"><a href="#cb10-8"></a>)</span></code></pre></div></td>
</tr>
</tbody>
</table>

A Ruby file for each non-default environment must exist in the
`environments/` subdirectory of the chef-repo. (If the chef-repo does
not have this subdirectory, then it should be created.) The complete
environment has the following syntax:

``` ruby
name 'environment_name'
description 'environment_description'
cookbook OR cookbook_versions  'cookbook' OR 'cookbook' => 'cookbook_version'
default_attributes 'node' => { 'attribute' => [ 'value', 'value', 'etc.' ] }
override_attributes 'node' => { 'attribute' => [ 'value', 'value', 'etc.' ] }
```

where both default and override attributes are optional and either a
cookbook or cookbook versions (one or more) are specified. For example,
an environment named `dev` that uses the `couchdb` cookbook (version
11.0.0 or higher) that listens on ports 80 and 443:

``` ruby
name 'dev'
description 'The development environment'
cookbook_versions  'couchdb' => '= 11.0.0'
default_attributes 'apache2' => { 'listen_ports' => %w(80 443) }
```

Or (using the same scenario) to specify a version constraint for only
one cookbook:

``` ruby
cookbook 'couchdb', '= 11.0.0'
```

More than one cookbook version can be specified:

``` ruby
cookbook_versions({
  'couchdb' => '= 11.0.0',
  'my_rails_app' => '~> 1.2.0'
})
```

Attributes are optional and can be set at the default and override
levels. These will be processed according to attribute precedence. An
environment attribute will be applied to all nodes within the
environment, except in places where it is overridden by an attribute
with higher precedence. For example:

``` ruby
default_attributes 'apache2' => { 'listen_ports' => %w(80 443) }
```

will have all nodes in the environment (`node[:apache2][:listen_ports]`)
set to `'80'` and `'443'` unless they were overridden by an attribute
with higher precedence. For example:

``` ruby
override_attributes 'apache2' => { 'listen_ports' => %w(80 443) }
```

### JSON

The JSON format for environments maps directly to the domain-specific
Ruby format: the same settings, attributes, and values, and a similar
structure and organization, just formatted as JSON. When an environment
is defined as JSON the file that contains that data must be defined as a
file that ends with `.json`. For example:

``` javascript
{
  "name": "dev",
  "default_attributes": {
    "apache2": {
      "listen_ports": [
        "80",
        "443"
      ]
    }
  },
  "json_class": "Chef::Environment",
  "description": "",
  "cookbook_versions": {
    "couchdb": "= 11.0.0"
  },
  "chef_type": "environment"
}
```

The JSON format has two additional settings:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>chef_type</code></td>
<td>Always set this to <code>environment</code>. Use this setting for any custom process that consumes environment objects outside of Ruby.</td>
</tr>
<tr class="even">
<td><code>json_class</code></td>
<td>Always set this to <code>Chef::Environment</code>. Chef Infra Client uses this setting to auto-inflate an environment object. If objects are being rebuilt outside of Ruby, ignore it.</td>
</tr>
</tbody>
</table>

## Create Environments

An environment can be created in five different ways:

-   Creating a Ruby file in the environments sub-directory of the
    chef-repo and then pushing it to the Chef server
-   Creating a JSON file directly in the chef-repo and then pushing it
    to the Chef server
-   Using knife
-   Using the Chef management console web user interface
-   Using the Chef Infra Server REST API

Once an environment exists on the Chef Infra Server, a node can be
associated with that environment using the `chef_environment` method.

## Manage Environments

Once created, an environment can be managed in several ways:

-   By using knife and passing the `-E ENVIRONMENT_NAME` option with
    `knife cookbook upload`
-   By using Ruby or JSON files that are stored in a version source
    control system. These files are pushed to the Chef Infra Server
    using the `knife environment` subcommand and the `from file`
    argument. This approach allows environment data to be dynamically
    generated. This approach will not work unless these files are
    defined in the proper format---Ruby file end with `.rb`; JSON files
    end with `.json`.

These workflows are mutually exclusive: only the most recent environment
changes will be kept on the Chef Infra Server, regardless of the source
of those changes. All previous changes are overwritten when environment
data is updated.

The settings for environments can be modified and environments can be
integrated into the larger infrastructure by associating them with nodes
and by using recipes to call specific environment settings.

### Find Environment from Recipe

Use the following syntax to find the current environment from a recipe:

``` ruby
node.environment
```

or:

``` ruby
node.chef_environment
```

### Save in a Data Bag

Values that are stored in a data bag are global to the organization and
are available to any environment. There are two main strategies that can
be used to store per-environment data within a data bag: by using a
top-level key that corresponds to the environment or by using separate
items for each environment.

A data bag that is storing a top-level key for an environment might look
something like this:

``` javascript
{
  "id": "some_data_bag_item",
  "production" : {
    // Hash with all your data here
  },
  "testing" : {
    // Hash with all your data here
  }
}
```

When using the data bag in a recipe, that data can be accessed from a
recipe using code similar to:

``` ruby
bag_item[node.chef_environment]['some_other_key']
```

The other approach is to use separate items for each environment.
Depending on the amount of data, it may all fit nicely within a single
item. If this is the case, then creating different items for each
environment may be a simple approach to providing per-environment values
within a data bag. However, this approach is more time-consuming and may
not scale to very large environments or when the data must be stored in
many data bag items.

### Override Attributes in Roles

Environment attributes that are used with roles can be overridden.
Typically, this is done by using attribute precedence, but sometimes it
may be necessary to ensure that specific attributes are used based on
the presence of specific environments. This type of scenario is best
addressed in using a recipe that relies on a top-level key that is
stored in a data bag.

For example, to retrieve a value from a data bag based on a specific
environment:

``` ruby
mything = data_bag_item('things', 'mything')
attribute_i_want = mything[node.chef_environment]
```

### Set for a Node

A node is considered to be associated with an environment when the
`chef_environment` attribute is set. The `chef_environment` attribute
cannot be set with normal or override attributes (i.e. in a role)
because it is actually a method. An environment may be set explicitly
using the following methods:

-   By using the `knife edit` and `knife exec` subcommands

-   By editing the `chef_environment` directly using knife or the Chef
    management console

-   By editing the `environment` configuration details in the client.rb
    file, and then using `knife bootstrap -e environment_name` to
    bootstrap the changes to the specified environment

    {{< note spaces=4 >}}

    After the environment has been set via bootstrap, the environment is
    set in the client.rb file and may not be modified using the Chef
    management console or the `edit` argument of the `knife node`
    subcommand.

    {{< /note >}}

-   By setting the `environment` configuration entry in the client.rb
    file ; when Chef Infra Client runs, it will pick up the value and
    then set the `chef_environment` attribute of the node

### Move Nodes

Use the `knife exec` subcommand to move nodes between environments, such
as from a "dev" to a "production" environment. For example:

``` bash
knife exec -E 'nodes.transform("chef_environment:dev") { |n| n.chef_environment("production") }'
```

### Search Environments

{{% search_environment %}}

## Environments in Chef Solo

{{% chef_solo_environments %}}
