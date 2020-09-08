Use the following code block to trigger the exception and have the Chef
Infra Client send email to the specified email address:

``` ruby
ruby_block 'fail the run' do
  block do
    fail 'deliberately fail the run'
  end
end
```