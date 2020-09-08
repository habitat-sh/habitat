``` ruby
template "#{ENV['HOME']}/chef-getting-started.txt" do
  source 'chef-getting-started.txt.erb'
  mode '0755'
end
```