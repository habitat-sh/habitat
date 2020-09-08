The log resource has the following properties:

`level`

:   **Ruby Type:** Symbol \| **Default Value:** `:info`

    The logging level for displaying this message. Options (in order of
    priority): `:debug`, `:info`, `:warn`, `:error`, and `:fatal`.

`message`

:   **Ruby Type:** String \| **Default Value:**
    `The resource block's name`

    The message to be added to a log file. Default value: the `name` of
    the resource block. See "Syntax" section above for more information.