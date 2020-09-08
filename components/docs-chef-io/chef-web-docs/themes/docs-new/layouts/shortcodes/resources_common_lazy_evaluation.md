In some cases, the value for a property cannot be known until the
execution phase of a Chef Infra Client run. In this situation, using
lazy evaluation of property values can be helpful. Instead of a property
being assigned a value, it may instead be assigned a code block. The
syntax for using lazy evaluation is as follows:

``` ruby
attribute_name lazy { code_block }
```

where `lazy` is used to tell Chef Infra Client to evaluate the contents
of the code block later on in the resource evaluation process (instead
of immediately) and `{ code_block }` is arbitrary Ruby code that
provides the value.

For example, a resource that is **not** doing lazy evaluation:

``` ruby
template 'template_name' do
  # some attributes
  path '/foo/bar'
end
```

and a resource block that is doing lazy evaluation:

``` ruby
template 'template_name' do
  # some attributes
  path lazy { ' some Ruby code ' }
end
```

In the previous examples, the first resource uses the value `/foo/bar`
and the second resource uses the value provided by the code block, as
long as the contents of that code block are a valid resource property.

The following example shows how to use lazy evaluation with template
variables:

``` ruby
template '/tmp/canvey_island.txt' do
  source 'canvey_island.txt.erb'
  variables(
    lazy {
      { canvey_island: node.run_state['sea_power'] }
    }
  )
end
```