For example, the following code block will ensure the command is
evaluated using the default interpreter as identified by Chef Infra
Client:

``` ruby
resource 'name' do
  guard_interpreter :default
  # code
end
```