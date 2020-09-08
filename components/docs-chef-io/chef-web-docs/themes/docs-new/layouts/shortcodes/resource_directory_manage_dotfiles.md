The following example shows using the **directory** and
**cookbook_file** resources to manage dotfiles. The dotfiles are
defined by a JSON data structure similar to:

``` javascript
"files": {
  ".zshrc": {
    "mode": '0755',
    "source": "dot-zshrc"
    },
  ".bashrc": {
    "mode": '0755',
    "source": "dot-bashrc"
     },
  ".bash_profile": {
    "mode": '0755',
    "source": "dot-bash_profile"
    },
  }
```

and then the following resources manage the dotfiles:

``` ruby
if u.has_key?('files')
  u['files'].each do |filename, file_data|

  directory "#{home_dir}/#{File.dirname(filename)}" do
    recursive true
    mode '0755'
  end if file_data['subdir']

  cookbook_file "#{home_dir}/#{filename}" do
    source "#{u['id']}/#{file_data['source']}"
    owner 'u['id']'
    group 'group_id'
    mode 'file_data['mode']'
    ignore_failure true
    backup 0
  end
end
```