The output format: `doc` (default) or `min`.

-   Use `doc` to print the progress of a Chef Infra Client run using
    full strings that display a summary of updates as they occur.
-   Use `min` to print the progress of a Chef Infra Client run using
    single characters.

A summary of updates is printed at the end of a Chef Infra Client run. A
dot (`.`) is printed for events that do not have meaningful status
information, such as loading a file or synchronizing a cookbook. For
resources, a dot (`.`) is printed when the resource is up to date, an
`S` is printed when the resource is skipped by `not_if` or `only_if`,
and a `U` is printed when the resource is updated.

Other formatting options are available when those formatters are
configured in the client.rb file using the `add_formatter` option.