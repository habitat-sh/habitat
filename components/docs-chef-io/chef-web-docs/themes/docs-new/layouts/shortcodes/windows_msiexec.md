Msiexec.exe is used to install Chef Infra Client on a node as part of a
bootstrap operation. The actual command that is run by the default
bootstrap script is:

``` bash
msiexec /qn /i "%LOCAL_DESTINATION_MSI_PATH%"
```

where `/qn` is used to set the user interface level to "No UI", `/i` is
used to define the location in which Chef Infra Client is installed, and
`"%LOCAL_DESTINATION_MSI_PATH%"` is a variable defined in the default
[windows-chef-client-msi.erb](https://github.com/chef/chef/blob/master/lib/chef/knife/bootstrap/templates/windows-chef-client-msi.erb)
bootstrap template. See
<https://docs.microsoft.com/en-us/windows/win32/msi/command-line-options>
for more information about the options available to Msiexec.exe.