A cookbook may have a `/templates` directory structure like this:

``` ruby
/templates/
  windows-10
  windows-6.3
  windows
  default
```

and a resource that looks something like the following:

``` ruby
template 'C:\path\to\file\text_file.txt' do
  source 'text_file.txt'
  mode '0755'
  owner 'root'
  group 'root'
end
```

This resource would be matched in the same order as the `/templates`
directory structure. For a node named `host-node-desktop` that is
running Windows 8.1, the second item would be the matching item and the
location:

``` ruby
/templates
  windows-10/text_file.txt
  windows-6.3/text_file.txt
  windows/text_file.txt
  default/text_file.txt
```