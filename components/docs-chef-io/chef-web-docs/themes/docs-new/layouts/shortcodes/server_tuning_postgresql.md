The following setting is often modified from the default as part of the
tuning effort for the **postgresql** service:

`postgresql['max_connections']`

:   The maximum number of allowed concurrent connections. This value
    should only be tuned when the `opscode_erchef['db_pool_size']` value
    used by the **opscode-erchef** service is modified. Default value:
    `350`.

    If there are more than two front end machines in a cluster, the
    `postgresql['max_connections']` setting should be increased. The
    increased value depends on the number of machines in the front end,
    but also the number of services that are running on each of these
    machines.

    -   Each front end machine always runs the **oc_bifrost** and
        **opscode-erchef** services.
    -   The Reporting add-on adds the **reporting** service.
    -   The Chef Push Jobs service adds the **push_jobs** service.

    Each of these services requires 25 connections, above the default
    value.

    Use the following formula to help determine what the increased value
    should be:

    ``` ruby
    new_value = current_value + [
                (# of front end machines - 2) * (25 * # of services)
             ]
    ```

    For example, if the current value is 350, there are four front end
    machines, and all add-ons are installed, then the formula looks
    like:

    ``` ruby
    550 = 350 + [(4 - 2) * (25 * 4)]
    ```