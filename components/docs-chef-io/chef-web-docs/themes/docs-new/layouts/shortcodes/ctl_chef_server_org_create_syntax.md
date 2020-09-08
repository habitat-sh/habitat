This subcommand has the following syntax:

``` bash
chef-server-ctl org-create ORG_NAME "ORG_FULL_NAME" (options)
```

where:

-   The name must begin with a lower-case letter or digit, may only
    contain lower-case letters, digits, hyphens, and underscores, and
    must be between 1 and 255 characters. For example: `chef`.
-   The full name must begin with a non-white space character and must
    be between 1 and 1023 characters. For example:
    `"Chef Software, Inc."`.