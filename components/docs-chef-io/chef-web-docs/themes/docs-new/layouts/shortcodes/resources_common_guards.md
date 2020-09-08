A guard property can be used to evaluate the state of a node during the
execution phase of a Chef Infra Client run. Based on the results of this
evaluation, a guard property is then used to tell Chef Infra Client if
it should continue executing a resource. A guard property accepts either
a string value or a Ruby block value:

-   A string is executed as a shell command. If the command returns `0`,
    the guard is applied. If the command returns any other value, then
    the guard property is not applied. String guards in a
    **powershell_script** run Windows PowerShell commands and may
    return `true` in addition to `0`.
-   A block is executed as Ruby code that must return either `true` or
    `false`. If the block returns `true`, the guard property is applied.
    If the block returns `false`, the guard property is not applied.

A guard property is useful for ensuring that a resource is idempotent by
allowing that resource to test for the desired state as it is being
executed, and then if the desired state is present, for Chef Infra
Client to do nothing.