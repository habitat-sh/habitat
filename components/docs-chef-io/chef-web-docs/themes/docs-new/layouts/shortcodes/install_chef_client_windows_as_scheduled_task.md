To run Chef Infra Client at periodic intervals (so that it can check in
with Chef Infra Server automatically), configure Chef Infra Client to
run as a scheduled task. This can be done via the MSI, by selecting the
**Chef Unattended Execution Options** --\> **Chef Infra Client Scheduled
Task** option on the **Custom Setup** page or by running the following
command after Chef Infra Client is installed:

For example:

``` none
SCHTASKS.EXE /CREATE /TN ChefClientSchTask /SC MINUTE /MO 30 /F /RU "System" /RP /RL HIGHEST /TR "cmd /c \"C:\opscode\chef\embedded\bin\ruby.exe C:\opscode\chef\bin\chef-client -L C:\chef\chef-client.log -c C:\chef\client.rb\""
```

Refer to the [Schtasks
documentation](https://docs.microsoft.com/en-us/windows/win32/taskschd/schtasks)
for more details.

After Chef Infra Client is configured to run as a scheduled task, the
default file path is: `c:\chef\chef-client.log`.