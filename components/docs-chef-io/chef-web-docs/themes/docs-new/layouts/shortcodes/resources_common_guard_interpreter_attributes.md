The `guard_interpreter` property may be set to any of the following
values:

`:bash`

:   Evaluates a string command using the **bash** resource.

`:batch`

:   Evaluates a string command using the **batch** resource. Default
    value (within a **batch** resource block): `:batch`.

`:csh`

:   Evaluates a string command using the **csh** resource.

`:default`

:   Default. Executes the default interpreter as identified by Chef
    Infra Client.

`:perl`

:   Evaluates a string command using the **perl** resource.

`:powershell_script`

:   Evaluates a string command using the **powershell_script**
    resource. Default value (within a **powershell_script** resource block):
    `:powershell_script`.

`:python`

:   Evaluates a string command using the **python** resource.

`:ruby`

:   Evaluates a string command using the **ruby** resource.
