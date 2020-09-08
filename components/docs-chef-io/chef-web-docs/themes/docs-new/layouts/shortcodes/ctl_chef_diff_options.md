This subcommand has the following options:

`-c CONFIG_FILE`, `--config CONFIG_FILE`

:   The path to the knife configuration file.

`-D`, `--debug`

:   Enable stack traces and other debug output. Default value: `false`.

`-g GIT_REF`, `--git GIT_REF`

:   Compare the specified git reference against the current revision of
    a `Policyfile.lock.json` file or against another git reference.

`-h`, `--help`

:   Show help for the command.

`--head`

:   A shortcut for `chef diff --git HEAD`. When a git-specific flag is
    not provided, the on-disk `Policyfile.lock.json` file is compared to
    one on the Chef Infra Server or (if a `Policyfile.lock.json` file is
    not present on-disk) two `Policyfile.lock.json` files in the
    specified policy group on the Chef Infra Server are compared.

`--[no-]pager`

:   Use `--pager` to enable paged output for a `Policyfile.lock.json`
    file. Default value: `--pager`.

`-v`, `--version`

:   The Chef Infra Client version.