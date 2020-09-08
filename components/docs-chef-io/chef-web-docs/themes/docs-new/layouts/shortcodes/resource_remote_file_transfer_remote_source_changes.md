``` ruby
remote_file '/tmp/couch.png' do
  source 'http://couchdb.apache.org/img/sketch.png'
  action :nothing
end

http_request 'HEAD http://couchdb.apache.org/img/sketch.png' do
  message ''
  url 'http://couchdb.apache.org/img/sketch.png'
  action :head
  if ::File.exist?('/tmp/couch.png')
    headers 'If-Modified-Since' => File.mtime('/tmp/couch.png').httpdate
  end
  notifies :create, 'remote_file[/tmp/couch.png]', :immediately
end
```