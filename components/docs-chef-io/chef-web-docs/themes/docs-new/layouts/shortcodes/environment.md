An environment is a way to map an organization's real-life workflow to
what can be configured and managed when using Chef Infra. This mapping
is accomplished by setting attributes and pinning cookbooks at the
environment level. With environments, you can change cookbook
configurations depending on the system's designation. For example, by
designating different staging and production environments, you can then
define the correct URL of a database server for each environment.
Environments also allow organizations to move new cookbook releases from
staging to production with confidence by stepping releases through
testing environments before entering production.