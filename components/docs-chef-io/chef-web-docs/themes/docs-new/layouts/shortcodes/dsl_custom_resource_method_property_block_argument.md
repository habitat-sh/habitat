Any properties that are marked `identity: true`, `desired_state: false`,
or `name_property: true` will be directly available from
`load_current_value`. If access to other properties of a resource is
needed, use a block argument with load_current_value. The block
argument will have the values of the requested resource. For example:

``` ruby
// Property is directly available example
property :action, String, name_property: true
property :content, String

load_current_value do |desired|
  puts "The user requested action = #{action} in the resource"
  puts "The user typed content = #{desired.content} in the resource"
end
```

``` ruby
// Block argument example
property :action, String
property :content, String

load_current_value do |desired|
  puts "The user requested action = #{desired.action} in the resource"
  puts "The user typed content = #{desired.content} in the resource"
end
```