``` ruby
gem_package 'nokogiri' do
  gem_binary('/opt/ree/bin/gem')
  options('--prerelease --no-format-executable')
end
```