The Chef Client 12.4 release adds an optional feature to the Microsoft
Installer Package (MSI) for Chef. This feature enables the ability to
pass quoted strings from the Windows PowerShell command line without the
need for triple single quotes (`''' '''`). This feature installs a
Windows PowerShell module (typically in `C:\opscode\chef\modules`) that
is also appended to the `PSModulePath` environment variable. This
feature is not enabled by default. To activate this feature, run the
following command from within Windows PowerShell:

``` bash
Import-Module chef
```

or add `Import-Module chef` to the profile for Windows PowerShell
located at:

``` bash
~\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1
```

This module exports cmdlets that have the same name as the command-line
tools---chef-client, knife, chef-apply---that are built into Chef.

For example:

``` bash
knife exec -E 'puts ARGV' """&s0meth1ng"""
```

is now:

``` bash
knife exec -E 'puts ARGV' '&s0meth1ng'
```

and:

``` bash
knife node run_list set test-node '''role[ssssssomething]'''
```

is now:

``` bash
knife node run_list set test-node 'role[ssssssomething]'
```

To remove this feature, run the following command from within Windows
PowerShell:

``` bash
Remove-Module chef
```