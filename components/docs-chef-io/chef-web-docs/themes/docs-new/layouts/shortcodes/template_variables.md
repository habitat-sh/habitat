An Embedded Ruby (ERB) template allows Ruby code to be embedded inside a
text file within specially formatted tags. Ruby code can be embedded
using expressions and statements. An expression is delimited by `<%=`
and `%>`. For example:

```ruby
<%= "my name is #{$ruby}" %>
```

A statement is delimited by a modifier, such as `if`, `elsif`, and
`else`. For example:

```ruby
if false
# this won't happen
elsif nil
      # this won't either
    end
```

Using a Ruby expression is the most common approach for defining
template variables because this is how all variables that are sent to a
template are referenced. Whenever a template needs to use an `each`,
`if`, or `end`, use a Ruby statement.

When a template is rendered, Ruby expressions and statements are
evaluated by Chef Infra Client. The variables listed in the **template**
resource's `variables` parameter and in the node object are evaluated.
Chef Infra Client then passes these variables to the template, where
they will be accessible as instance variables within the template. The
node object can be accessed just as if it were part of a recipe, using
the same syntax.

For example, a simple template resource like this:

```ruby
node['fqdn'] = 'latte'
template '/tmp/foo' do
  source 'foo.erb'
  variables(x_men: 'are keen')
end
```

And a simple Embedded Ruby (ERB) template like this:

```ruby
The node <%= node[:fqdn] %> thinks the x-men <%= @x_men %>
```

Would render something like:

```
The node latte thinks the x-men are keen
```

Even though this is a very simple example, the full capabilities of Ruby
can be used to tackle even the most complex and demanding template
requirements.