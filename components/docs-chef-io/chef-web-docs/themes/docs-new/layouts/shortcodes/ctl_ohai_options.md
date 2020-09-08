This command has the following syntax:

``` bash
ohai OPTION
```

This tool has the following options:

`ATTRIBUTE_NAME ATTRIBUTE NAME ...`

:   Use to have Ohai show only output for named attributes.

`-c CONFIG`, `--config CONFIG`

:   The path to a configuration file to use For example:
    `/etc/ohai/config.rb`.

`-d DIRECTORY`, `--directory DIRECTORY`

:   The directory in which additional Ohai plugins are located. For
    example: `/my/extra/plugins`.

`-h`, `--help`

:   Show help for the command.

`-l LEVEL`, `--log_level LEVEL`

:   The level of logging to be stored in a log file.

`-L LOGLOCATION`, `--logfile LOGLOCATION`

:   The location of the log file.

`-v`, `--version`

:   The version of Ohai.