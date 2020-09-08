The following settings are typically added to the server configuration
file (no equal sign is necessary to set the value):

`api_fqdn`

:   The FQDN for the Chef Infra Server. This setting is not in the
    server configuration file by default. When added, its value should
    be equal to the FQDN for the service URI used by the Chef Infra
    Server. For example: `api_fqdn "chef.example.com"`.

`bootstrap`

:   Default value: `true`.

`ip_version`

:   Use to set the IP version: `"ipv4"` or `"ipv6"`. When set to
    `"ipv6"`, the API listens on IPv6 and front end and back end
    services communicate via IPv6 when a high availability configuration
    is used. When configuring for IPv6 in a high availability
    configuration, be sure to set the netmask on the IPv6 `backend_vip`
    attribute. Default value: `"ipv4"`.

`notification_email`

:   Default value: `info@example.com`.