The following properties can be used to define a guard that is evaluated
during the execution phase of a Chef Infra Client run:

`not_if`

:   Prevent a resource from executing when the condition returns `true`.

`only_if`

:   Allow a resource to execute only if the condition returns `true`.