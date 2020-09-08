When running knife in Microsoft Windows, a string may be interpreted as
a wildcard pattern when quotes are not present in the command. The
number of quotes to use depends on the shell from which the command is
being run.

When running knife from the command prompt, a string should be
surrounded by single quotes (`' '`). For example:

``` bash
knife node run_list set test-node 'recipe[iptables]'
```

When running knife from Windows PowerShell, a string should be
surrounded by triple single quotes (`''' '''`). For example:

``` bash
knife node run_list set test-node '''recipe[iptables]'''
```