``` ruby
bash 'install_something' do
  user 'root'
  cwd '/tmp'
  code <<-EOH
  wget http://www.example.com/tarball.tar.gz
  tar -zxf tarball.tar.gz
  cd tarball
  ./configure
  make
  make install
  EOH
end
```