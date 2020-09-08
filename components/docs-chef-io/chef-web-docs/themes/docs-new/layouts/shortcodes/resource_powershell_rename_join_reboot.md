The following example shows how to rename a computer, join a domain, and
then reboot the computer:

``` ruby
reboot 'Restart Computer' do
  action :nothing
end

powershell_script 'Rename and Join Domain' do
  code <<-EOH
    ...your rename and domain join logic here...
  EOH
  not_if <<-EOH
    $ComputerSystem = gwmi win32_computersystem
    ($ComputerSystem.Name -like '#{node['some_attribute_that_has_the_new_name']}') -and
      $ComputerSystem.partofdomain)
  EOH
  notifies :reboot_now, 'reboot[Restart Computer]', :immediately
end
```

where:

-   The **powershell_script** resource block renames a computer, and
    then joins a domain
-   The **reboot** resource restarts the computer
-   The `not_if` guard prevents the Windows PowerShell script from
    running when the settings in the `not_if` guard match the desired
    state
-   The `notifies` statement tells the **reboot** resource block to run
    if the **powershell_script** block was executed during a Chef Infra
    Client run