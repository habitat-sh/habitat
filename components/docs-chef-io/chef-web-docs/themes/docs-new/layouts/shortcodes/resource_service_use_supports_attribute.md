``` ruby
service 'apache' do
  action [ :enable, :start ]
  retries 3
end
```