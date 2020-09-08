+++
title = "Release Notes: Chef Infra Server 12.0 - 13.2"
draft = false

aliases = ["/release_notes_server.html"]

[menu]
  [menu.release_notes]
    title = "Chef Infra Server"
    identifier = "release_notes/release_notes_server.md Chef Infra Server"
    parent = "release_notes"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/release_notes_server.md)

Chef Infra Server acts as a hub for configuration data by storing
cookbooks, the policies that are applied to nodes, and metadata that
describes each registered node that is managed by the Chef Infra Client.

## What's New in 13.2

### Improvements

- Azure support for external PostgreSQL:

  In the previous release we added support for ssl while connecting to PostgreSQL.

  With this release we add the ability to connect to an external PostgreSQL database in Azure.

- Update HAProxy configuration:

  We have updated the configuration for HAProxy to make it more responsive. The changes include:
  - Set the connect, client, server, and tunnel timeouts to reasonable defaults.
  - Set client-fin and server-fin to try to mitigate connection pile-ups in the case of failing frontend services.
  - Set on-marked-down shutdown-session to avoid stale sessions to previous leaders living longer than they need to.

- Integration testing pipeline:

  We have put a lot of effort into creating a test pipeline with the test infrastructure previously created. This runs multiple scenarios for Chef Infra Server with different configurations and topologies.

- Chef Infra Server supports Elasticsearch version 6 for external Elasticsearch:

  Chef Infra Server previously supported index creation for ElasticSearch versions 2 and 5. We now support index creation for ElasticSearch 6 as well.

- Cookstyle changes applied to the cookbooks.
- Disable actions rabbitmq queue by default.
- Log all errors triggered due to Elasticsearch reindex.

### Bug Fixes

- Fix a regression that broke FIPS 140-2 support in Chef Infra Server 13.1.13.
- Fix habitat db config for external database.
- Elasticsearch recipes should not create indexes at compile time.

### Updates

- Chef Infra Client: 15.5.17 -> 15.8.23
- rack(oc-chef-pedant): 2.0.7 -> 2.0.8
- rack(oc-id): 1.6.11 -> 1.6.12
- Ruby(oc-id): 1.6.11 -> 1.6.12
- Ruby: 2.6.3 -> 2.6.5 fixes the following CVEs:
  - CVE-2019-16255: A code injection vulnerability of Shell#[] and Shell#test
  - CVE-2019-16254: HTTP response splitting in WEBrick (Additional fix)
  - CVE-2019-15845: A NUL injection vulnerability of File.fnmatch and File.fnmatch?
  - CVE-2019-16201: Regular Expression Denial of Service vulnerability of WEBrick's Digest access authentication
  - CVE-2012-6708
  - CVE-2015-9251
- rubyzip(oc-id): 1.2.3 -> 1.3.0 (fixes CVE-2019-16892)
- Erlang(habitat): 18 -> 20

## What's New in 13.1.13

### Improvements/Bug Fixes

-   The `_status` endpoint now reports healthy even if the
    `data_collector` is down which will no longer cause unnecessary
    failovers.
-   Data collector proxy-header X-Forwarded is set as expected.
-   `chef-server-ctl` is no longer installed in the user path. Now only
    the appbundled version is installed in the user path.
-   Fixed an issue with Chef Support Zendesk sign-ins when a first name
    is not set in Hosted Chef.
-   Added support for running the Chef Infra Server on Red Hat
    Enterprise Linux 8.
-   `chef-server-ctl gather-logs` was updated with the following
    improvements:

-   Add AWS to known platforms
-   Add AWS Native Chef Server info
-   Add elasticsearch info
-   Switched compression from bzip2 to gzip

### Deprecation Notice

-   SLES 11 is no longer supported per our [platform
    policy](/platforms/#platform-end-of-life-policy), as upstream
    support ended in March of this year.

### Updates and Improvements

-   Postgres 9.6.10 -\> 9.6.15
-   Chef Infra Client v15.3.14 -\> v15.4.45
-   OpenResty 1.13.6.2 -\> 1.15.8.1
-   Nokogiri 1.8.5 -\> 1.10.4
-   Rebar3 -\> 3.12.0
-   Updated erlang deps to be the latest
-   Loofah 2.2.3 -\> 2.3.1
-   Erlang R18 -\> 20.3.8.9
-   Updated for cookstyle
-   Ruby 2.5.5 -\> 2.6.3

## What's New in 13.0.11

### Chef Server is now Chef Infra Server

Chef Server has a new name, but don't worry, it's the same Chef Server
you've grown used to. You'll notice new branding throughout the
application and documentation but the command <span
class="title-ref">chef-server-ctl</span> remains the same.

### Chef EULA

Chef Infra Server requires an EULA to be accepted by users before it can
be installed. Users can accept the EULA in a variety of ways:

-   `chef-server-ctl reconfigure --chef-license accept`
-   `chef-server-ctl reconfigure --chef-license accept-no-persist`
-   `CHEF_LICENSE="accept" chef-server-ctl reconfigure`
-   `CHEF_LICENSE="accept-no-persist" chef-server-ctl reconfigure`

Finally, if users run `chef-server-ctl reconfigure` without any of these
options, they will receive an interactive prompt asking for license
acceptance. If the license is accepted, a marker file will be written to
the filesystem unless `accept-no-persist` is specified. Once this marker
file is persisted, users no longer need to set any of these flags.

See our [Frequently Asked Questions
document](https://www.chef.io/bmc-faq/) for more information on the EULA
and license acceptance.

### Deprecation notice

-   [Deprecated PowerPC and s390x
    platforms](https://blog.chef.io/2018/11/01/end-of-life-announcement-for-chef-server-for-linux-on-ibm-z-and-linux-on-ibm-power-systems/)
-   [Deprecated Keepalived/DRBD-based
    HA](https://blog.chef.io/2018/10/02/end-of-life-announcement-for-drbd-based-ha-support-in-chef-server/)
-   Deprecated Ubuntu 14.04 support. (Ubuntu 14 was EoL'd at the end of
    April 2019)

### Updates and Improvements

-   Updated OpenResty to 1.13.6.2
    -   This fixes two CVEs: CVE-2018-9230 and CVE-2017-7529.
    -   This version cannot be built on PowerPC and s390x because those
        platforms are not supported in mainline luajit.
-   Updated Ruby version to 2.5.5
-   Updated Chef Infra Client to 14.11.21
-   Updated runit cookbook to 5.1.1
-   Migrated unit tests from Travis to Buildkite. Reorganized them for
    improved speed, stability and portability.
-   Added some Habitat packaging improvements with parameterized
    search_server.
-   Erchef request size increased from 1,000,000 to 2,000,000 bytes to
    better support inspec scanning via the audit cookbook.
-   Nginx error logs no longer log 404s. In the Chef API, 404s are
    typically not errors as they are often the expected response about
    an object that doesn't exist. The logs will continue to show 404s in
    the request logs.
-   Profiles and data-collector upstreams now render correctly if their
    root_url is configured. If the data_collector token secret is not
    set, a 401 response code and an error message will be seen instead
    of 404.

## What's New in 12.19.31

This release was triggered by the update to Habitat base plans.
(<https://blog.chef.io/2019/01/28/base-plans-refresh-is-coming-to-habitat-core-plans/>)
Omnibus release was done to keep in sync with the Habitat release.

-   `chef-server-ctl` leverages HAB_LISTEN_CTL envvar if available.

## What's New in 12.19.26

This release contains some minor improvements and updates to include
software:

-   Added request id to nginx log to easily track the Chef request
    through the logs.
-   Bundler pinned to 1.17 to avoid taking the 2.0 upgrade.
-   Erlang updated to 18.3.4.9
    -   Fixed two CVEs CVE-2017-1000385 and CVE-2016-10253. SSL headers
        got stricter which unfortunately broke LDAP. (Issue \#1642)
    -   Removed `et`, `debugger`, `gs`, and `observer` as they depend on
        `wx`, which is not available on all platforms.
-   Ruby updated to 2.5.3.
-   Chef Client updated to 14.5.
-   Erchef and Bookshelf can optionally use mTLS protocol for their
    internal communications.
-   Added configuration for pedant SSL-signed requests to include mTLS
    support.
-   Habitat package improvements:
    -   Increased `authn:keygen_timeout` amount for `oc_erchef` hab pkg.
    -   Removed `do_end` function from `chef-server-ctl` hab plan.
    -   Enhanced `chef-server-ctl` to function in more habitat
        environments.
    -   `chef-server-ctl` commands pass relevant TLS options during
        bifrost API calls.
-   Used standard ruby-cleanup definition, which shrinks install size by
    \~5% on disk.
-   Removed unused couchdb configurables.

## What's New in 12.18.14

This release:

-   Segment free cookbooks are implemented.
    (<https://github.com/chef/chef-rfc/blob/master/rfc067-cookbook-segment-deprecation.md>)
    This bumps the API version.
-   ACLs for cookbook artifacts
-   /nodes/NODENAME endpoint has HEAD operation.
-   Security headers for HTTP
-   Optional disabling of welcome page
-   chef-server-ctl now has version subcommand.
-   chef-server-ctl appbundled to better control gem loading.
-   Support for SSL auth between internal Chef Server Services. This
    includes connections to bifrost and the internal Postgresql server.
-   All datestamps in logs are now in UTC. SOLR GC log now datestamped.
-   Nginx logs now include the request id.
-   Fixie is now shipped with chef server.
-   Security issue with old doorkeeper fixed by upgrading.
    CVE-2018-1000211
-   Fixed issue migrating rabbitmq passwords (migration 031).
-   Chef indexing queue times now reported in stats in log messages and
    status endpoint.
-   Ruby updated to 2.5.1
-   Update gems
-   Chef Client updated to 14.3.
-   Postgresql updated to 9.6.10 and 9.2.24 (latter only for upgrades
    from older Chef Servers).
-   Fix for SUSE SLES-11 sysvinit install
-   Removed nodejs (a build dependency that was shipped).

{{< note >}}

Chef Server 12.18.14 introduces an incompatibility between older
versions of Berkshelf and ChefDK. We recommend using the minimum
versions of Berkshelf \>= 7.0.5 and ChefDK \>= 3.2.30. This
incompatibility manifests with a Berkshelf upload to Chef Server failure
of `Net::HTTPServerException: 400 "Bad Request"` and opscode-erchef logs
containing `status=400` and `req_api_version=1` in the log line for the
corresponding cookbook upload API request.

{{< /note >}}

## What's New in 12.17.33

This release:

-   Upgrades the version of Ruby to 2.4.3
-   Adds FIPS support for PPC64 (big-endian)
-   Fixes an Elasticsearch invalid search query issue caused by forward
    slashes that were not escaped properly

## What's New in 12.17.15

This release:

-   Fixes a regression in IPv6 address handling

-   Allows you to disable request logging via the following optional
    settings:

    -   `opscode-erchef['enable_request_logging']`
    -   `oc_bifrost['enable_request_logging']`
    -   `bookshelf['enable_request_logging']`

    See the [Chef server optional
    settings](/config_rb_server_optional_settings/) guide for
    additional details

-   `chef-server-ctl reconfigure` fixes permissions on gems with an
    overly restrictive umask

-   Makes the display of the welcome page configurable via the
    `nginx['show_welcome_page']` setting. See the [Chef server optional
    settings](/config_rb_server_optional_settings/) guide for
    additional details

-   Infers the current database migration level and necessary upgrades
    for `chef-server-ctl upgrade`

-   Catches `server_name` resolution errors during
    `chef-server-ctl reconfigure`, and continues with the
    reconfiguration

-   No longer creates the default RabbitMQ `guest` user

See the detailed [change
log](https://github.com/chef/chef-server/blob/master/CHANGELOG.md#121715-2017-12-21)
for a complete list of changes.

## What's New in 12.17.5

This release fixes a regression that occurs when deploying to
DigitalOcean and potentially other non-AWS cloud platforms, where the
`nginx['use_implicit_hosts'] = true` setting results in an incorrect
nginx configuration.

See the [change
log](https://github.com/chef/chef-server/blob/master/CHANGELOG.md#12175-2017-10-25)
for a full list of changes.

## What's New in 12.17.3

The following items are new for Chef server 12.17.3:

-   Java has been updated to version 8u144 to address
    [CVE-2017-3526](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-3526)
-   A `/_stats` endpoint has been added to Erchef. It exposes statistics
    about connection pool usage inside Erchef, Postgresql, and the
    Erlang VM
-   The `strict_host_header` and `use_implicit_hosts` settings have been
    added for nginx. These options help to prevent against cache
    poisoning attacks by ensuring that nginx only responds to requests
    with host headers that match the configured FQDN for the given
    machine. See Chef server's [optional nginx
    settings](/config_rb_server_optional_settings/#nginx) for
    additional details

See the [change
log](https://github.com/chef/chef-server/blob/master/CHANGELOG.md#12173-2017-10-19)
for a full list of changes.

## What's New in 12.16.14

This release updates Ruby to version 2.2.8 to take advantage of multiple
[security
fixes](https://www.ruby-lang.org/en/news/2017/09/14/ruby-2-2-8-released/).
See the full [change
log](https://github.com/chef/chef-server/blob/master/CHANGELOG.md#121614-2017-09-21)
for details on minor changes.

## What's New in 12.16.9

The following items are new for Chef server 12.16.9:

-   **Minor fixes for the PostgreSQL upgrade process**
-   **Remove unused authorization objects from bifrost database**

### Fixes for the PostgreSQL upgrade process

Chef server 12.16.9 adds the following features to make the PostgreSQL
upgrade process easier:

-   Ensures that your disk has the required space before starting the
    PostgreSQL upgrade
-   For users with large databases, `pg_upgrade` timeout is now
    configurable. The default timeout has been increased to 2 hours.

### Remove unused authorization objects from bifrost database

This release adds the `chef-server-ctl cleanup-bifrost` command.
`cleanup-bifrost` removes unused authorization objects from the
authorization database (called bifrost). These unused objects can
accumulate on long-running Chef servers as a result of failed object
creation requests. For most users, the unused authorization objects do
not substantially affect the performance of Chef server; however in
certain situations it can be helpful to clean them up. This command is
primarily intended for use by Chef support.

See the [cleanup-bifrost](/ctl_chef_server/#cleanup-bifrost)
subcommand documentation for syntax examples and additional options.

## What's New in 12.16.2

The following items are new for Chef server 12.16.2:

-   **Upgrade to PostgreSQL 9.6**
-   **Elasticsearch 5 support**
-   **Changes to Erlang Port Mapper Daemon (EPMD) listening ports**
-   **RabbitMQ health check in status endpoint**
-   **Notification of affected services when updating secrets with
    set-secret**

### Upgrade to PostgreSQL 9.6

Chef server now uses the latest stable version of the 9.6 series
(9.6.3). Upgrades of existing installations are done automatically, but
creating backups is advised.

The information below only applies if you have set a custom value set
for `checkpoint_segments` in your `/etc/opscode/chef-server.rb`. If you
have not set a custom value, there is nothing to change:

The `checkpoint_segments` configuration setting is gone, so if you
previously used the following parameter:

``` ruby
postgresql['checkpoint_segments'] = 10
```

You would instead use:

``` ruby
postgresql['max_wal_size'] = '3G'
```

This is just an example configuration - see the [PostgreSQL release
notes](https://www.postgresql.org/docs/9.6/static/release-9-5.html) for
more information on tuning this option. The default setting for
`max_wal_size` is `1G`. The PostgreSQL release notes mention a
conversion rule: `max_wal_size = (3 * checkpoint_segments) * 16MB`. They
also state that the default value for `max_wal_size` (1GB) should be
sufficient in most settings, so this conversion is not performed
automatically.

The `shmmax` and `shmall` configuration settings are no longer used, as
PostgreSQL 9.6 relies on System V shared memory much less than
PostgreSQL 9.2. The `shared_buffers` configuration setting is still
respected, and can be used to modify the amount of shared memory used by
PostgreSQL.

This update also adds two new configurables in the "Checkpoints" group:
`min_wal_size` and `checkpoint_flush_after`.

As part of the upgrade procedure, `chef-server-ctl cleanup` will remove
Postgres 9.2's data and logs.

### Elasticsearch 5 support

Chef server now supports Elasticsearch 5. This allows Chef server and
Chef Automate 1.6 to use the same Elasticsearch instance.

### Changes to EPMD listening ports

The Erlang Port Mapper Daemon (EPMD) included in version 12.16 is
patched to only listen on the addresses specified in `ERL_EPMD_ADDRESS`.
Before, it would implicitly add `::1` and `127.0.0.1` to the set of
listening addresses, which caused trouble for systems without `::1`.

### RabbitMQ health check in status endpoint

Chef server's `_status` endpoint now checks the health of the analytics
and internal RabbitMQ vhosts. For these checks to work, the RabbitMQ
management plugin must be installed. If it is not, the checks are not
performed. If Chef server is configured not to use Actions, a check will
not be performed against the Actions vhost. If an indexing queue is not
used, the `chef_index` RabbitMQ vhost will not be checked.

### Notification of affected services when updating secrets with set-secret

`chef-server-ctl set-secret` will notify the user of services that
depend on the secret that is being changed. When used with the optional
`--with-restart` flag, `chef-server-ctl set-secret` will attempt to
automatically restart the dependent services.

## What's New in 12.15

The following items are new for Chef server 12.15:

-   **Supports SUSE Linux Enterprise on x86_64**
-   **Add required_recipe endpoint**
-   **ACLs and groups can refer to global groups**
-   **User customization of field mapping**

### Supports SUSE Linux Enterprise Server on x86_64

Support for a new platform was added: SUSE Linux Enterprise Server 11 &
12 on x86_64.

### Add required_recipe endpoint

Added the ability to serve a required recipe file to chef-clients.

The setting `required_recipe["enable"]` in chef-server.rb enables the
required recipe feature.

The setting `required_recipe["path"]` in chef-server.rb specifies the
recipe file to serve.

The `/organizations/<orgname>/required_recipe` endpoint returns 404 for
all organizations by default. It returns 401 when the request is not
made by a client from the requested org and the feature is enabled.

The `/organizations/<orgname>/required_recipe` endpoint returns the
required recipe and 200 only when the endpoint is enabled and requested
by an authorized client.

See [Chef RFC
89](https://github.com/chef/chef-rfc/blob/master/rfc089-server-enforced-recipe.md)
for a complete description on the `required_recipe` endpoint.

### ACLs and groups can refer to global groups

The server-admins group is useful, but it breaks roundtripping when it
appears in an organizations ACLs and groups. This makes it difficult
when using the API for backups.

A new syntax '::' was added to indicate scoping. `::GROUPNAME` without a
prefix indicates a global (across multiple orgs) entity, while
`ORGNAME::GROUPNAME` refers to a group in an another org. So if the
server-admins appears in an organizations ACL, you will see the name
`::server-admins`.

### User customization of field mapping

Attributes from a user's LDAP record are used during account-linking to
populate the erchef user record when it is created. Previously, the
mapping between LDAP attributes and chef user attributes were fixed.
Now, they are configurable. For example, if the user's LDAP record
stores their email address in a field named 'address' instead of 'mail',
then you could set the following in `private-chef.rb`:

``` ruby
ldap['email_attribute'] = "address"
```

### Bug Fixes

Fixed regression in oc-id. The identity service was using the wrong Chef
Server API version level.

Fixed regression in the nginx proxy that prevented Automate-based
Compliance profiles from being reachable.

Fixed regression in Bookshelf's preflight checks.

Fixed regression that would cause Manage to be misconfigured to enable
LDAP by default.

PUT to `/users/USERNAME/_acl/PERM` will no longer return a 400 when the
request is valid.

## What's New in 12.14

The following items are new for Chef server 12.14:

-   **Reduce password proliferation**

### Reduce password proliferation

We've substantially reduced the number of configuration files that
contain plaintext passwords. Now, no passwords or credentials are
rendered outside of `/etc/opscode/` in Chef server's default
configuration.

To ensure backwards compatibility, Chef server still renders passwords
and keys to multiple files in `/etc/opscode`. However, if you are not
using any Chef Server add-ons, or if you have updated to the latest
releases of all add-ons, you can set the following:

``` ruby
insecure_addon_compat false
```

in `/etc/opscode/chef-server.rb` and remove these other occurrences of
secrets as well.

If you are using LDAP integration, external postgresql, or other Chef
server features that require providing passwords in
`/etc/opscode/chef-server.rb`, we've also provided commands that allow
you to set these passwords outside of the configuration file. For
information about these commands see [Secrets
Management](/ctl_chef_server/#secrets-management).

{{< note >}}

Users of the DRBD-based HA configuration may still see passwords related
to keepalived and DRBD in `/var/opt/opscode`.

{{< /note >}}

For further information see:

See [Chef Server Credentials
Management](/server_security/#chef-infra-server-credentials-management)
for more details.

## What's New in 12.13

The following items are new for Chef server 12.13:

-   **Supports Red Hat Enterprise Linux 6 on s390x (RHEL6/s390x)**
-   **Disables the Solr4 Admin API/UI by default**
-   **FIPS runtime flag exposed on RHEL systems** Setting `fips true`
    and reconfiguring will start the server in FIPS mode. Packages for
    other systems will not have the required OpenSSL FIPS module and
    will fail to start if reconfigured with `fips true`.

### New platform: RHEL6/s390x

Support for a new platform was added: Red Hat Enterprise Linux 6 on
s390x.

### Solr4 Admin API/UI disabled by default

With this release, the admin UI of Solr4 has been removed. The
underlying API has also been disabled. Users that depend on the admin
API endpoints can enable them via adding:

``` ruby
opscode_solr4['enable_full_admin_api'] = true
```

to `chef-server.rb`.

### FIPS runtime flag exposed

The Chef Server package now exposes a `fips` configuration flag in
`chef-server.rb`. Setting `fips true` and reconfiguring will start the
server in FIPS mode. The default value of this flag is `false` except on
systems where FIPS is enabled at the Kernel where it defaults to `true`.

The only supported systems at this time for FIPS mode are RHEL. Packages
for other systems will be missing the required OpenSSL FIPS module and
will fail to start if reconfigured with `fips true`.

## What's New in 12.12

The following items are new for Chef server 12.12:

-   **chef-server-ctl backup correctly backs up configuration data**
    Starting in version 12.10.0, a bug in the `backup` command produced
    backups that did not include the configuration data in the resulting
    tarball. This bug is now resolved. We recommend taking a new backup
    after upgrading to 12.12.0.
-   **Correct number of rows are returned when searching with
    ElasticSearch** When configured to use ElasticSearch, Chef server
    now correctly respects the `rows` parameter in search requests
    rather than returning all rows.
-   **Solr 4 GC logging is now used by Chef server** Java's native
    rotation is used for the gclog.
-   **New oc_id email configuration options** Outbound email address
    can now be configured.

### Solr 4 GC Logging

Chef server now uses Java's native rotation for the gclog. This prevents
situations where logrotate creates large sparse files on disk, which may
be problematic to manage with tools that can't handle sparse files.

The Solr 4 GC log can now be found at
`/var/log/opscode/opscode-solr4/gclog.log.N.current` where *N* is an
integer. The `.current` extension denotes the log currently being
written to.

To remove the older GC logs, run `sudo chef-server-ctl cleanup` after
upgrading to Chef server 12.12.

To suppress the GC log completely, set the following option in
`/etc/opscode/chef-server.rb`:

``` ruby
## true (default) to enable gc logging,
## false to disable gc logging
opscode_solr4['log_gc'] = false
```

### oc_id Email Configuration Options

The `oc_id` service now includes configuration for outbound email to
ensure password reset emails can be sent correctly.

You can now set the following options in `/etc/opscode/chef-server.rb`:

``` ruby
# defaults to the value of the from_email configuration option
oc_id['email_from_address'] = "oc_id@example.com"
# defaults to the api_fqdn
oc_id['origin'] = "mail.yourco.io"
```

## What's New in 12.11

The following items are new for Chef server 12.11:

### New Endpoints

-   **/organizations/ORGNAME/validate/PATH** accepts a signed request
    and validates it as if it had been sent to <span
    class="title-ref">PATH</span>. It returns 200 if the request is
    authentic and 401 if it is not.

-   **/organizations/ORGNAME/data-collector** forwards requests for a
    data-collector service after authenticating the request using Chef
    Server's standard authentication headers. To use this endpoint,
    users must set both of the following options in
    /etc/opscode/chef-server.rb:

    ``` ruby
    data_collector['token']
    data_collector['root_url']
    ```

-   **/organizations/ORGNAME/owners/OWNER/compliance\[/PROFILE\]**
    forwards requests for compliance profiles to a user-configurable
    Chef Automate server after authenticating the request using Chef
    Server's standard authentication headers. To use this endpoint,
    users must set both of the following options in
    \`/etc/opscode/chef-server.rb\`:

    ``` ruby
    profiles['root_url']
    data_collector['token']
    ```

### Security Updates

-   The default allowed SSL ciphers now include AES256-GCM-SHA384 to
    ensure compatibility with AWS's Classic ELB health check tool.
-   **chef-server-ctl psql** previously revealed the postgresql password
    via <span class="title-ref">ps</span>.

## What's New in 12.10

The following items are new for Chef server 12.10:

-   Smaller download - the download size has been reduced by around 35%
    via removal of redundant, cached, and unused components. The
    installed size has been similarly reduced.
-   add retry support to opscode-expander
-   chef-server-ctl reindex will now continue even if some objects are
    not indexable, and will show which objects failed at the conclusion
    of the run.
-   Data Collector support for Policyfiles.
-   chef-server-ctl install add-on installation now pulls from the
    correct source.
-   Regression fix: that caused errors on reconfigure when LDAP bind
    password is nil has been fixed.

### Security Updates

-   Upgrade to OpenSSL 1.0.2j. The prior release (1.0.1u) is approaching
    EOL.
-   Updated TLS ciphers. See compatibility notes, below.

### Compatibility Notes

-   The change of TLS ciphers can cause older tooling to fail to
    negotiate SSL sessions with the Chef Server. The changes to the
    cipher list are captured here. Upgrading any custom clients of the
    Chef Server API to use a current SSL release will resolve this.

    Alternatively, you can set `nginx['ssl_protocols']` in
    `/etc/opscode/chef-server.rb` to a set of ciphers that are
    compatible with your tooling, then running chef-server-ctl
    reconfigure to pick up the changes.

-   With this TLS cipher suite change, the Reporting add-on will report
    errors when opscode-reporting-ctl test is run. A fix for this is
    available in the current channel for reporting, and will be released
    to stable in November. This issue does not otherwise affect the
    Reporting add-on, but you can resolve this locally by modifying
    /etc/opscode-reporting/pedant_config.rb and adding the following
    line: ssl_version :TLSv1_2

## What's New in 12.9.1

The following items are new for Chef server 12.9.1:

The update of OpenSSL 1.0.1u addresses the following CVEs:

-   CVE-2016-6304
-   CVE-2016-2183
-   CVE-2016-6303
-   CVE-2016-6302
-   CVE-2016-2182
-   CVE-2016-2180
-   CVE-2016-2177
-   CVE-2016-2178
-   CVE-2016-2179
-   CVE-2016-2181
-   CVE-2016-6306

## What's New in 12.9

The following items are new for Chef server 12.9:

-   **New warning and functionality when trying to delete user in
    multiple 'admin' groups** If a user is in an administrator group in
    any organization, the `chef-server-ctl user-delete` subcommand does
    not allow you to remove the user from that group. To provide more
    information when the `user-delete` subcommand fails for this reason,
    the error message contains a list of organizations the user is an
    administrator of. Using the new flag `--remove-from-admin-groups`,
    you can now remove that user provided they are not the only user in
    the `admin` group.
-   **LDAP bind passwords now support special characters**
-   **Updated to OpenSSL 1.0.1u** Updated version of OpenSSL to address
    security vulnerabilities.
-   **Multiple ACL updates on the Chef server API** The `_acl` endpoint
    now requires that any users being added to an object's ACL exist in
    the same organization as the object itself. Existing users that are
    not organization members and have already been added to an ACL will
    not be affected, and will still be in the GET response for this API.
    Additional changes can be found
    [here](https://github.com/chef/chef-server/blob/master/RELEASE_NOTES.md#api-changes).

## What's New in 12.8

The following items are new for Chef server 12.8:

-   **Initial support for sending updates to a data collector service**
-   **Minor bug fixes in postgresql setup**

## What's New in 12.7

The following items are new for Chef server 12.7:

-   **Support for service credential rotation through Veil** Veil is a
    library for securely creating, storing, and rotating Chef server
    secrets. It is also required when using the new
    `chef-server-ctl require-credential-rotation` command.
-   **Filtering by external authentication ID in Chef server API** Users
    can now be filtered by `external_authentication_uid`, which is
    needed to support SAML authentication in Chef Manage.
-   **Updated to OpenSSL 1.0.1t** Version 1.0.1t contains several
    security fixes.

### Service credential rotation support

[Veil](https://github.com/chef/chef-server/blob/3ff412b5a2e6ad54cfa79bca6865e1bbca28fe5e/omnibus/files/veil/README.md)
is a new library to manage Chef server secrets. It allows any Chef
server with a given set of secrets to create new service credentials and
rotate them without requiring the secrets files to be copied between
each Chef server in a cluster.

Five new commands have been created to support credential rotation:

-   [require-credential-rotation](/ctl_chef_server/#require-credential-rotation)
-   [rotate-all-credentials](/ctl_chef_server/#rotate-all-credentials)
-   [rotate-credentials](/ctl_chef_server/#rotate-credentials)
-   [rotate-shared-secrets](/ctl_chef_server/#rotate-shared-secrets)
-   [show-service-credentials](/ctl_chef_server/#show-service-credentials)

Your secrets file is located at
`/etc/opscode/private-chef-secrets.json`, so whenever you rotate your
service credentials, or update your shared secrets, this file will
contain the changes.

### Supporting SAML-authentication in Chef Manage

To support SAML-authentication in Chef Manage, you can now filter users
using `external_authentication_uid` in a GET request against the Chef
server API. For example, to retrieve users where the
`external_authentication_uid` is `jane@doe.com`, do the following:

``` none
GET /users?external_authentication_uid=jane%40doe.com
```

## What's New in 12.6

The following items are new for Chef server 12.6:

-   **Chef licenses** All Chef products have a license that governs the
    entire product and includes links to license files for any
    third-party software included in Chef packages. This release updates
    the Chef server for the Chef license.

### About Chef Licenses

All Chef products have a license that governs the entire product and
includes links to license files for any third-party software included in
Chef packages. The `/opt/<PRODUCT-NAME>/LICENSES` directory contains
individual copies of all referenced licenses.

{{< warning >}}

The `chef-server-ctl install` command no longer works in the 12.5 (and
earlier) versions of the Chef server due to a change in how packages are
downloaded from Chef.

{{< /warning >}}

### Apache 2.0

All open source Chef products---such as the Chef client, the Chef
server, or InSpec---are governed by the [Apache 2.0
license](https://www.apache.org/licenses/LICENSE-2.0).

## What's New in 12.5

The following items are new for Chef server 12.5:

-   **New group for key-related Chef server API endpoints** The
    `public_key_read_access` group defines which users and clients have
    read permissions to key-related endpoints in the Chef server API.

### public_key_read_access

The `public_key_read_access` group controls which users and clients have
[read permissions to the following endpoints](/api_chef_server/):

-   GET /clients/CLIENT/keys
-   GET /clients/CLIENT/keys/KEY
-   GET /users/USER/keys
-   GET /users/USER/keys/

By default, the `public_key_read_access` assigns all members of the
`users` and `clients` group permission to these endpoints:

<table style="width:100%;">
<colgroup>
<col style="width: 24%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
<col style="width: 15%" />
</colgroup>
<thead>
<tr class="header">
<th>Group</th>
<th>Create</th>
<th>Delete</th>
<th>Grant</th>
<th>Read</th>
<th>Update</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>admins</td>
<td>no</td>
<td>no</td>
<td>no</td>
<td>no</td>
<td>no</td>
</tr>
<tr class="even">
<td>clients</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
<tr class="odd">
<td>users</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
<td>yes</td>
</tr>
</tbody>
</table>

## What's New in 12.4

The following items are new for Chef server 12.4:

-   **/universe endpoint** Use the `/universe` endpoint to retrieve the
    known collection of cookbooks, and then use it with Berkshelf and
    Chef Supermarket.
-   **opscode-expander-reindexer service** The
    `opscode-expander-reindexer` service is deprecated.
-   **Global server administrator list** Use the
    `grant-server-admin-permissions`, `remove-server-admin-permissions`,
    and `list-server-admins` to manage the list of users who belong to
    the `server-admins` group.

### /universe

Use the `/universe` endpoint to retrieve the known collection of
cookbooks, and then use it with Berkshelf and Chef Supermarket.

The `/universe` endpoint has the following methods: `GET`.

### GET

The `GET` method is used to retrieve the universe data.

This method has no parameters.

**Request**

``` none
GET /universe
```

**Response**

The response will return an embedded hash, with the name of each
cookbook as a top-level key. Each cookbook will list each version, along
with its location information and dependencies:

``` javascript
{
  "ffmpeg": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.0/download"
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      },
    },
    "0.1.1": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.1/download"
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      },
    },
   "pssh": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/pssh.1.0/download"
      "location_type": "supermarket",
      "dependencies": {},
    }
  }
}
```



## What's New in 12.4

The following items are new for Chef server 12.4:

-   **/universe endpoint** Use the `/universe` endpoint to retrieve the
    known collection of cookbooks, and then use it with Berkshelf and
    Chef Supermarket.
-   **opscode-expander-reindexer service** The
    `opscode-expander-reindexer` service is deprecated.
-   **Global server administrator list** Use the
    `grant-server-admin-permissions`, `remove-server-admin-permissions`,
    and `list-server-admins` to manage the list of users who belong to
    the `server-admins` group.

### /universe

Use the `/universe` endpoint to retrieve the known collection of
cookbooks, and then use it with Berkshelf and Chef Supermarket.

The `/universe` endpoint has the following methods: `GET`.

### GET

The `GET` method is used to retrieve the universe data.

This method has no parameters.

**Request**

``` none
GET /universe
```

**Response**

The response will return an embedded hash, with the name of each
cookbook as a top-level key. Each cookbook will list each version, along
with its location information and dependencies:

``` javascript
{
  "ffmpeg": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.0/download"
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      },
    },
    "0.1.1": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.1/download"
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      },
    },
   "pssh": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/pssh.1.0/download"
      "location_type": "supermarket",
      "dependencies": {},
    }
  }
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. One (or more) cookbooks and associated cookbook version information was returned.</td>
</tr>
</tbody>
</table>

### Server Admins

The `server-admins` group is a global group that grants its members
permission to create, read, update, and delete user accounts, with the
exception of superuser accounts. The `server-admins` group is useful for
users who are responsible for day-to-day administration of the Chef
server, especially user management via the `knife user` subcommand.
Before members can be added to the `server-admins` group, they must
already have a user account on the Chef server.

### Scenario

The following user accounts exist on the Chef server: `pivotal` (a
superuser account), `alice`, `bob`, `carol`, and `dan`. Run the
following command to view a list of users on the Chef server:

``` bash
chef-server-ctl user-list
```

and it returns the same list of users:

``` bash
pivotal
alice
bob
carol
dan
```

Alice is a member of the IT team whose responsibilities include
day-to-day administration of the Chef server, in particular managing the
user accounts on the Chef server that are used by the rest of the
organization. From a workstation, Alice runs the following command:

``` bash
knife user list -c ~/.chef/alice.rb
```

and it returns the following error:

``` bash
ERROR: You authenticated successfully to <chef_server_url> as alice
       but you are not authorized for this action
Response: Missing read permission
```

Alice is not a superuser and does not have permissions on other users
because user accounts are global to organizations in the Chef server.
Let's add Alice to the `server-admins` group:

``` bash
chef-server-ctl grant-server-admin-permissions alice
```

and it returns the following response:

``` bash
User alice was added to server-admins.
```

Alice can now create, read, update, and delete user accounts on the Chef
server, even for organizations to which Alice is not a member. From a
workstation, Alice re-runs the following command:

``` bash
knife user list -c ~/.chef/alice.rb
```

which now returns:

``` bash
pivotal
alice
bob
carol
dan
```

Alice is now a server administrator and can use the following knife
subcommands to manage users on the Chef server:

-   `knife user-create`
-   `knife user-delete`
-   `knife user-edit`
-   `knife user-list`
-   `knife user-show`

For example, Alice runs the following command:

``` bash
knife user edit carol -c ~/.chef/alice.rb
```

and the \$EDITOR opens in which Alice makes changes, and then saves
them.

#### Superuser Accounts

Superuser accounts may not be managed by users who belong to the
`server-admins` group. For example, Alice attempts to delete the
`pivotal` superuser account:

``` bash
knife user delete pivotal -c ~/.chef/alice.rb
```

and the following error is returned:

``` bash
ERROR: You authenticated successfully to <chef_server_url> as user1
       but you are not authorized for this action
Response: Missing read permission
```

Alice's action is unauthorized even with membership in the
`server-admins` group.

### Manage server-admins Group

Membership of the `server-admins` group is managed with a set of
`chef-server-ctl` subcommands:

-   `chef-server-ctl grant-server-admin-permissions`
-   `chef-server-ctl list-server-admins`
-   `chef-server-ctl remove-server-admin-permissions`

#### Add Members

The `grant-server-admin-permissions` subcommand is used to add a user to
the `server-admins` group. Run the command once per user added.

This subcommand has the following syntax:

``` bash
chef-server-ctl grant-server-admin-permissions USER_NAME
```

where `USER_NAME` is the user to add to the list of server
administrators.

For example:

``` bash
chef-server-ctl grant-server-admin-permissions bob
```

returns:

``` bash
User bob was added to server-admins. This user can now list,
read, and create users (even for orgs they are not members of)
for this Chef Server.
```

#### Remove Members

The `remove-server-admin-permissions` subcommand is used to remove a
user from the `server-admins` group. Run the command once per user
removed.

This subcommand has the following syntax:

``` bash
chef-server-ctl remove-server-admin-permissions USER_NAME
```

where `USER_NAME` is the user to remove from the list of server
administrators.

For example:

``` bash
chef-server-ctl remove-server-admin-permissions bob
```

returns:

``` bash
User bob was removed from server-admins. This user can no longer
list, read, and create users for this Chef Server except for where
they have default permissions (such as within an org).
```

#### List Membership

The `list-server-admins` subcommand is used to return a list of users
who are members of the `server-admins` group.

This subcommand has the following syntax:

``` bash
chef-server-ctl list-server-admins
```

and will return a list of users similar to:

``` bash
pivotal
alice
bob
carol
dan
```

## What's New in 12.3

The following items are new for Chef server 12.3:

-   **Nginx stub_status module is enabled** The Nginx `stub_status`
    module is enabled by default and may be viewed at the
    `/nginx_status` endpoint. The settings for this module are
    configurable.
-   **RabbitMQ queue tuning** New settings for managing RabbitMQ queues
    allow the size of the queue used by Chef Analytics to be configured,
    including settings for the queue length monitor and for tuning the
    rabbitmq-management plugin.

### Nginx stub_status Module

The following configuration settings are new and enable the Nginx
`stub_status` module:

`nginx['enable_stub_status']`

:   Enables the Nginx `stub_status` module. See
    `nginx['stub_status']['allow_list']`,
    `nginx['stub_status']['listen_host']`,
    `nginx['stub_status']['listen_port']`, and
    `nginx['stub_status']['location']`. Default value: `true`.

`nginx['stub_status']['allow_list']`

:   The IP address on which accessing the `stub_status` endpoint is
    allowed. Default value: `["127.0.0.1"]`.

`nginx['stub_status']['listen_host']`

:   The host on which the Nginx `stub_status` module listens. Default
    value: `"127.0.0.1"`.

`nginx['stub_status']['listen_port']`

:   The port on which the Nginx `stub_status` module listens. Default
    value: `"9999"`.

`nginx['stub_status']['location']`

:   The name of the Nginx `stub_status` endpoint used to access data
    generated by the Nginx `stub_status` module. Default value:
    `"/nginx_status"`.

### RabbitMQ Queues

If the RabbitMQ queue that is used by Chef Analytics stops consuming
messages, the Chef server data partition will fill up and may affect the
overall performance of the Chef server application itself. The settings
for the RabbitMQ queue are tunable, including for queue length
monitoring, queue capacity, maximum number of messages that can be in
the queue before messages are dropped, the point at which messages are
dropped, for settings used by the rabbitmq-management plugin, and so on.

The following settings may be used for tuning RabbitMQ queues used by
Chef Analytics and the Chef server:

`rabbitmq['analytics_max_length']`

:   The maximum number of messages that can be queued before RabbitMQ
    automatically drops messages from the front of the queue to make
    room for new messages. Default value: `10000`.

`rabbitmq['drop_on_full_capacity']`

:   Specify if messages will stop being sent to the RabbitMQ queue when
    it is at capacity. Default value: `true`.

`rabbitmq['management_enabled']`

:   Specify if the rabbitmq-management plugin is enabled. Default value:
    `true`.

`rabbitmq['management_password']`

:   The rabbitmq-management plugin password. Default value:
    `'chefrocks'`.

`rabbitmq['management_port']`

:   The rabbitmq-management plugin port. Default value: `15672`.

`rabbitmq['management_user']`

:   The rabbitmq-management plugin user. Default value: `'rabbitmgmt'`.

`rabbitmq['prevent_erchef_startup_on_full_capacity']`

:   Specify if the Chef server will start when the monitored RabbitMQ
    queue is full. Default value: `false`.

`rabbitmq['queue_at_capacity_affects_overall_status']`

:   Specify if the `_status` endpoint in the Chef server API will fail
    if the monitored queue is at capacity. Default value: `false`.

`rabbitmq['queue_length_monitor_enabled']`

:   Specify if the queue length monitor is enabled. Default value:
    `true`.

`rabbitmq['queue_length_monitor_millis']`

:   The frequency (in milliseconds) at which the length of the RabbitMQ
    queue is checked. Default value: `30000`.

`rabbitmq['queue_length_monitor_timeout_millis']`

:   The timeout (in milliseconds) at which calls to the queue length
    monitor will stop if the Chef server is overloaded. Default value:
    `5000`.

`rabbitmq['queue_length_monitor_queue']`

:   The RabbitMQ queue that is observed by queue length monitor. Default
    value: `'alaska'`.

`rabbitmq['queue_length_monitor_vhost']`

:   The virtual host for the RabbitMQ queue that is observed by queue
    length monitor. Default value: `'/analytics'`.

`rabbitmq['rabbit_mgmt_http_cull_interval']`

:   The maximum cull interval (in seconds) for the HTTP connection pool
    that is used by the rabbitmq-management plugin. Default value: `60`.

`rabbitmq['rabbit_mgmt_http_init_count']`

:   The initial worker count for the HTTP connection pool that is used
    by the rabbitmq-management plugin. Default value: `25`.

`rabbitmq['rabbit_mgmt_http_max_age']`

:   The maximum connection worker age (in seconds) for the HTTP
    connection pool that is used by the rabbitmq-management plugin.
    Default value: `70`.

`rabbitmq['rabbit_mgmt_http_max_connection_duration']`

:   The maximum connection duration (in seconds) for the HTTP connection
    pool that is used by the rabbitmq-management plugin. Default value:
    `70`.

`rabbitmq['rabbit_mgmt_http_max_count']`

:   The maximum worker count for the HTTP connection pool that is used
    by the rabbitmq-management plugin. Default value: `100`.

`rabbitmq['rabbit_mgmt_ibrowse_options']`

:   An array of comma-separated key-value pairs of ibrowse options for
    the HTTP connection pool that is used by the rabbitmq-management
    plugin. Default value: `'{connect_timeout, 10000}'`.

`rabbitmq['rabbit_mgmt_timeout']`

:   The timeout for the HTTP connection pool that is used by the
    rabbitmq-management plugin. Default value: `30000`.

`rabbitmq['ssl_versions']`

:   The SSL versions used by the rabbitmq-management plugin. (See
    [RabbitMQ TLS Support](https://www.rabbitmq.com/ssl.html) for more
    details.) Default value: `['tlsv1.2', 'tlsv1.1']`.

### What's New

The following items are new for Chef server 12.2:

-   **Solr to Solr4 settings** Built-in transition for Apache Solr
    memory and JVM settings from Enterprise Chef to Chef server
    version 12.
-   **Configurable Postgresql** Postgresql can be configured for an
    external database.
-   **New endpoints for the Chef server API** Endpoints have been added
    to the Chef server API: `DELETE /policy_groups`.
-   **New subcommmands for chef-server-ctl** Use the `backup` and
    `restore` subcommmands to back up and restore Chef server data. Use
    the `psql` subcommmand to log into a PostgreSQL database that is
    associated with a service running in the Chef server configuration.
-   **New options for chef-server-ctl reindex** The `reindex` subcommand
    has new options: `--all-orgs` (reindex all organizations),
    `--disable-api` (disable the Chef server API during reindexing),
    `--with-timing` (print timing information), and `--wait` (wait for
    reindex queue to clear before exiting).

### Solr =\> Solr 4 Changes

Chef server version 12 is upgraded to Apache Solr 4. If Apache Solr
options were added to the private-chef.rb file under `opscode_solr` for
Enterprise Chef, those configuration options are now stored under
`opscode_solr4` in the chef-server.rb file for Chef server version 12.

Some `opscode_solr` settings are imported automatically, such as heap,
new size, and Java options, but many settings are ignored. If your
Enterprise Chef configuration is highly tuned for Apache Solr, review
[these configuration
settings](/config_rb_server_optional_settings/#opscode-solr4) before
re-tuning Apache Solr for Chef server version 12.

### External PostgreSQL

The following diagram highlights the specific changes that occur when
PostgreSQL is configured and managed independently of the Chef server
configuration.

<img src="/images/server_components_postgresql.svg" width="500" alt="image" />

The following table describes the components in an external PostgreSQL
configuration that are different from the default configuration of the
Chef server:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Component</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef Server</td>
<td>The Chef server configuration file is updated to point to an independently configured set of servers for PostgreSQL.</td>
</tr>
<tr class="even">
<td><p>PostgreSQL</p></td>
<td><p>PostgreSQL is the data storage repository for the Chef server.</p>
<p>This represents the independently configured set of servers that are running PostgreSQL and are configured to act as the data store for the Chef server.</p></td>
</tr>
</tbody>
</table>

{{< note >}}

The following `chef-server-ctl` subcommands for managing services are
disabled when an external PostgreSQL database is configured for the Chef
server: `hup`, `int`, `kill`, `once`, `restart`, `start`, `stop`,
`tail`, and `term`.

{{< /note >}}

### Settings

Use the following configuration settings in the chef-server.rb file to
configure PostgreSQL for use with the Chef server:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>postgresql['db_superuser']</code></td>
<td>Required when <code>postgresql['external']</code> is set to <code>true</code>. The PostgreSQL user name. This user must be granted either the <code>CREATE ROLE</code> and <code>CREATE DATABASE</code> permissions in PostgreSQL or be granted <code>SUPERUSER</code> permission. This user must also have an entry in the host-based authentication configuration file used by PostgreSQL (traditionally named <code>pg_hba.conf</code>). Default value: <code>'superuser_userid'</code>.</td>
</tr>
<tr class="even">
<td><code>postgresql['db_superuser_password']</code></td>
<td>Required when <code>postgresql['external']</code> is set to <code>true</code>. The password for the user specified by <code>postgresql['db_superuser']</code>. Default value: <code>'the password'</code>.</td>
</tr>
<tr class="odd">
<td><code>postgresql['external']</code></td>
<td>Required. Set to <code>true</code> to run PostgreSQL external to the Chef server. Must be set once only on a new installation of the Chef server before the first <code>chef-server-ctl reconfigure</code> command is run. If this is set after a reconfigure or set to <code>false</code>, any reconfigure of the Chef server will return an error. Default value: <code>false</code>.</td>
</tr>
<tr class="even">
<td><code>postgresql['port']</code></td>
<td>Optional when <code>postgresql['external']</code> is set to <code>true</code>. The port on which the service is to listen. The port used by PostgreSQL if that port is <strong>not</strong> 5432. Default value: <code>5432</code>.</td>
</tr>
<tr class="odd">
<td><code>postgresql['vip']</code></td>
<td>Required when <code>postgresql['external']</code> is set to <code>true</code>. The virtual IP address. The host for this IP address must be online and reachable from the Chef server via the port specified by <code>postgresql['port']</code>. Set this value to the IP address or hostname for the machine on which external PostgreSQL is located when <code>postgresql['external']</code> is set to <code>true</code>.</td>
</tr>
</tbody>
</table>

### Backup / Restore

Use the following commands to manage backups of Chef server data, and
then to restore those backups.

### backup

The `backup` subcommand is used to back up all Chef server data. This
subcommand:

-   Requires rsync to be installed on the Chef server prior to running
    the command
-   Requires a `chef-server-ctl reconfigure` prior to running the
    command
-   Should not be run in a Chef server configuration with an external
    PostgreSQL database; [use knife ec
    backup](https://github.com/chef/knife-ec-backup) instead
-   Puts the initial backup in the `/var/opt/chef-backup` directory as a
    tar.gz file; move this backup to a new location for safe keeping

**Options**

This subcommand has the following options:

`-y`, `--yes`

:   Use to specify if the Chef server can go offline during tar.gz-based
    backups.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl backup
```

### restore

The `restore` subcommand is used to restore Chef server data from a
backup that was created by the `backup` subcommand. This subcommand may
also be used to add Chef server data to a newly-installed server. This
subcommand:

-   Requires rsync to be installed on the Chef server prior to running
    the command
-   Requires a `chef-server-ctl reconfigure` prior to running the
    command
-   Should not be run in a Chef server configuration with an external
    PostgreSQL database; [use knife ec
    backup](https://github.com/chef/knife-ec-backup) instead

**Options**

This subcommand has the following options:

`-c`, `--cleanse`

:   Use to remove all existing data on the Chef server; it will be
    replaced by the data in the backup archive.

`-d DIRECTORY`, `--staging-dir DIRECTORY`

:   Use to specify that the path to an empty directory to be used during
    the restore process. This directory must have enough disk space to
    expand all data in the backup archive.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl restore PATH_TO_BACKUP (options)
```

**Examples**

``` bash
chef-server-ctl restore /path/to/tar/archive.tar.gz
```

### psql

The `psql` subcommand is used to log into the PostgreSQL database
associated with the named service. This subcommand:

-   Uses `psql` (the interactive terminal for PostgreSQL)
-   Has read-only access by default
-   Is the recommended way to interact with any PostgreSQL database that
    is part of the Chef server
-   Automatically handles authentication

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl psql SERVICE_NAME (options)
```

**Options**

This subcommand has the following options:

`--write`

:   Use to enable write access to the PostgreSQL database.

### reindex Options

This subcommand has the following options:

`-a`, `--all-orgs`

:   Use to reindex all organizations on the Chef server. This option
    will override any organization specified as part of the command,
    i.e. `chef-server-ctl reindex ORG_NAME -a` will reindex all
    organizations and not just the specified organization.

`-d`, `--disable-api`

:   Use to disable the Chef server API to prevent writes during
    reindexing.

`-t`, `--with-timing`

:   Use to print timing information for the reindex processes.

`-w`, `--wait`

:   Use to wait for the reindexing queue to clear before exiting. This
    option only works when run on a standalone Chef server, or on a
    primary backend Chef server within a legacy tier or DRBD HA system.
    This option should not be used on a HA frontend.

### Chef server API Endpoints

The following endpoints have been added to the Chef server API:

### /policy_groups/NAME

The `/policy_groups` endpoint has the following methods: `GET`.

#### DELETE

The `DELETE` method is used to delete a policy group that is stored on
the Chef server.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/policy_groups/NAME
```

**Response**

The response returns the policy details and is similar to:

``` javascript
{
  "uri": "https://chef.example/organizations/org1/policy_groups/dev",
  "policies": {
    "aar": {
      "revision_id": "95040c199302c85c9ccf1bcc6746968b820b1fa25d92477ea2ec5386cd58b9c5"
    },
    "jenkins": {
      "revision_id": "613f803bdd035d574df7fa6da525b38df45a74ca82b38b79655efed8a189e073"
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /policies/NAME

The `/policies/NAME` endpoint has the following methods: `DELETE` and
`GET`. These endpoints enables the management of policies as they relate
to a specific policy group.

#### GET

The `GET` method is used to return a policy document.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/policies/NAME
```

**Response**

The response is similar to:

``` none
xxxxx
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### DELETE

The `DELETE` method is used to delete a policy.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/policies/NAME
```

**Response**

The response returns the policy details and is similar to:

``` javascript
{
  "revisions":
    {
      "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9": {},
      "95040c199302c85c9ccf1bcc6746968b820b1fa25d92477ea2ec5386cd58b9c5": {},
      "d81e80ae9bb9778e8c4b7652d29b11d2111e763a840d0cadb34b46a8b2ca4347": {}
    }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /policies/NAME/revisions

The `/roles` endpoint has the following methods: `POST`.

#### POST

The `POST` method is used to create a new policy revision.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/policies/NAME/revisions
```

with a request body similar to:

``` none
xxxxx
```

**Response**

The response is similar to:

``` none
xxxxx
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /policies/NAME/revisions/ID

The `/policies/NAME/revisions/ID` endpoint has the following methods:
`DELETE` and `GET`.

#### GET

The `GET` method is used to return a policy document for a specific
policy revision.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/GROUP/policies/NAME/revisions/ID
```

**Response**

The response is similar to:

``` javascript
{
  "revision_id": "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9",
  "name": "aar",
  "run_list": [
    "recipe[aar::default]"
  ],
  "cookbook_locks": {
    "aar": {
      "version": "0.1.0",
      "identifier": "29648fe36333f573d5fe038a53256e23733618aa",
      "dotted_decimal_identifier": "11651043203167221.32604909279531813.121098535835818",
      "source": "cookbooks/aar",
      "cache_key": null,
      "scm_info": {
        "scm": "git",
        "remote": null,
        "revision": "a2c8cbb24a08625921d753cde36e8320465116c3",
        "working_tree_clean": false,
        "published": false,
        "synchronized_remote_branches": []
      },
      "source_options": {
        "path": "cookbooks/aar"
      }
    },
    "apt": {
      "version": "2.7.0",
      "identifier": "16c57abbd056543f7d5a15dabbb03261024a9c5e",
      "dotted_decimal_identifier": "6409580415309396.17870749399956400.55392231660638",
      "cache_key": "apt-2.7.0-supermarket.chef.io",
      "origin": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
      "source_options": {
        "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
        "version": "2.7.0"
      }
    }
  },
  "default_attributes": {},
  "override_attributes": {},
  "solution_dependencies": {
    "Policyfile": [
      [
        "aar",
        ">= 0.0.0"
      ],
      [
        "apt",
        "= 2.7.0"
      ],
    ],
    "dependencies": {
      "apt (2.7.0)": [],
      "aar (0.1.0)": [
        [
          "apt",
          ">= 0.0.0"
        ]
      ]
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### DELETE

The `DELETE` method is used to delete a policy document for a specific
policy revision.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/GROUP/policies/NAME/revisions/ID
```

**Response**

The response returns the policy details and is similar to:

``` javascript
{
  "revision_id": "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9",
  "name": "aar",
  "run_list": [
    "recipe[aar::default]"
  ],
  "cookbook_locks": {
    "aar": {
      "version": "0.1.0",
      "identifier": "29648fe36333f573d5fe038a53256e23733618aa",
      "dotted_decimal_identifier": "11651043203167221.32604909279531813.121098535835818",
      "source": "cookbooks/aar",
      "cache_key": null,
      "scm_info": {
        "scm": "git",
        "remote": null,
        "revision": "a2c8cbb24a08625921d753cde36e8320465116c3",
        "working_tree_clean": false,
        "published": false,
        "synchronized_remote_branches": []
      },
      "source_options": {
        "path": "cookbooks/aar"
      }
    },
    "apt": {
      "version": "2.7.0",
      "identifier": "16c57abbd056543f7d5a15dabbb03261024a9c5e",
      "dotted_decimal_identifier": "6409580415309396.17870749399956400.55392231660638",
      "cache_key": "apt-2.7.0-supermarket.chef.io",
      "origin": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
      "source_options": {
        "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
        "version": "2.7.0"
      }
    }
  },
  "default_attributes": {},
  "override_attributes": {},
  "solution_dependencies": {
    "Policyfile": [
      [
        "aar",
        ">= 0.0.0"
      ],
      [
        "apt",
        "= 2.7.0"
      ],
    ],
    "dependencies": {
      "apt (2.7.0)": [],
      "aar (0.1.0)": [
        [
          "apt",
          ">= 0.0.0"
        ]
      ]
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

## What's New in 12.1

The following items are new for Chef server 12.1:

-   **chef-server-ctl key commands use the chef-client Chef::Key
    object** The key rotation commands (`chef-server-ctl key`) for
    `create`, `delete`, `edit`, `list`, and `show` keys for users and
    clients. These were a preview in the Chef server 12.0.3 release, and
    are now fully integrated.
-   **New version headers for Chef Server API** The Chef server API uses
    the `X-Ops-Server-API-Version` header to specify the version of the
    API that is used as part of a request to the Chef server API.
-   **New endpoints for policy and policy files** The Chef server API
    adds the following endpoints: `/policies`, `/policy_groups`, and
    `/POLICY_GROUP/policies/POLICY_NAME`.
-   **New endpoints for client key management** The Chef server API adds
    the following endpoints: `/clients/CLIENT/keys` and
    `/clients/CLIENT/keys/KEY`.
-   **New endpoints for user key management** The Chef server API adds
    the following endpoints: `/user/USER/keys` and
    `/user/USER/keys/KEY`.
-   **New configuration setting** Use the `estatsd['protocol']` setting
    to send application statistics with StatsD protocol formatting.

### Key Rotation

The `knife user` and `knife client` subcommands support key rotation.
Use the `create`, `delete`, `edit`, `list`, and `show` subcommands to
manage keys for users and clients, such as creating multiple expiring
keys for a single user and also for basic key management. See
/knife_user.html and /knife_client.html for more information about
these subcommands.

### X-Ops-Server-API-Version

Use `X-Ops-Server-API-Version` to specify the version of the Chef server
API. For example: `X-Ops-Server-API-Version: 1`.
`X-Ops-Server-API-Version: 0` is supported for use with the version 12
Chef server, but will be deprecated as part of the next major release.

### /clients/CLIENT/keys/

The `/clients/CLIENT/keys` endpoint has the following methods: `GET` and
`POST`.

#### GET

The `GET` method is used to retrieve all of the named client's key
identifiers, associated URIs, and expiry states.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/clients/CLIENT/keys
```

**Response**

The response is similar to:

``` javascript
[
  { "name" : "default",
             "uri" : "https://chef.example/organizations/example/clients/client1/keys/default",
             "expired" : false },
  { "name" : "key1",
             "uri" : "https://chef.example/organizations/example/clients/client1/keys/key1",
             "expired" : true }
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to add a key for the specified client.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/clients/CLIENT/keys
```

with a request body similar to:

``` javascript
{
  "name": "key1",
  "public_key": "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date": "infinity"
}
```

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/organizations/example/clients/client1/keys/key1"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /clients/CLIENT/keys/KEY

The `/clients/CLIENT/keys/KEY` endpoint has the following methods:
`DELETE`, `GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete the specified key for the
specified client.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/clients/CLIENT/keys/KEY
```

**Response**

The response returns the information about the deleted key and is
similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return details for a specific key for a
specific client.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/clients/CLIENT/keys/KEY
```

**Response**

The response is similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update one or more properties for a specific
key for a specific client.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/clients/CLIENT/keys/KEY
```

with a request body similar to:

``` javascript
{
  "name" : "new_key_name",
  "public_key" : "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response**

The response contains the updated inforamtion for the key, and is
similar to:

``` javascript
{
  "name" : "new_key_name",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /user/USER/keys/

The `/users/USER/keys` endpoint has the following methods: `GET` and
`POST`.

#### GET

The `GET` method is used to retrieve all of the named user's key
identifiers, associated URIs, and expiry states.

This method has no parameters.

**Request**

``` none
GET /users/USER/keys/
```

**Response**

The response is similar to:

``` javascript
[
  { "name" : "default",
             "uri" : "https://chef.example/users/USER/keys/default",
             "expired" : false },
  { "name" : "key1",
             "uri" : "https://chef.example/users/USER/keys/key1",
             "expired" : false}
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to add a key for the specified user.

This method has no parameters.

**Request**

``` none
POST /users/USER/keys/
```

with a request body similar to:

``` javascript
{
  "name" : "key1",
  "public_key" : "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date" : "infinity"
}
```

**Response**

The response is similar to:

``` javascript
{
  "uri" : "https://chef.example/users/user1/keys/key1"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /user/USER/keys/KEY

The `/users/USER/keys/KEY` endpoint has the following methods: `DELETE`,
`GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete the specified key for the
specified user.

This method has no parameters.

**Request**

``` none
DELETE /users/USER/keys/KEY
```

**Response**

The response returns the information about the deleted key and is
similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return details for a specific key for a
specific user.

This method has no parameters.

**Request**

``` none
GET /users/USER/keys/KEY
```

**Response**

The response is similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update one or more properties for a specific
key for a specific user.

This method has no parameters.

**Request**

``` none
PUT /users/USER/keys/KEY
```

with a request body similar to:

``` javascript
{
  "name" : "new_key_name",
  "public_key" : "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response**

The response contains the updated inforamtion for the key, and is
similar to:

``` javascript
{
  "name" : "new_key_name",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /policies

The `/policies` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get a list of policies (including policy
revisions) from the Chef server.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/policies
```

**Response**

The response groups policies by name and revision and is similar to:

``` javascript
{
  "aar": {
    "uri": "https://chef.example/organizations/org1/policies/aar",
    "revisions": {
      "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9": {
      },
      "95040c199302c85c9ccf1bcc6746968b820b1fa25d92477ea2ec5386cd58b9c5": {
      },
      "d81e80ae9bb9778e8c4b7652d29b11d2111e763a840d0cadb34b46a8b2ca4347": {
      }
    }
  },
  "jenkins": {
    "uri": "https://chef.example/organizations/org1/policies/jenkins",
    "revisions": {
      "613f803bdd035d574df7fa6da525b38df45a74ca82b38b79655efed8a189e073": {
      },
      "6fe753184c8946052d3231bb4212116df28d89a3a5f7ae52832ad408419dd5eb": {
      },
      "cc1a0801e75df1d1ea5b0d2c71ba7d31c539423b81478f65e6388b9ee415ad87": {
      }
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

### /policy_groups

The `/policy_groups` endpoint has the following methods: `GET`.

Each node has a 1:many relationship with policy settings stored on the
Chef server. This relationship is based on the policy group to which the
node is associated, and then the policy settings assigned to that group:

-   A policy is typically named after the functional role ahost
    performs, such as "application server", "chat server", "load
    balancer", and so on
-   A policy group defines a set of hosts in a deployed units, typically
    mapped to organizational requirements such as "dev", "test",
    "staging", and "production", but can also be mapped to more detailed
    requirements as needed

#### GET

The `GET` method is used to retrieve all of the policy groups that are
stored on the Chef server.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/policy_groups
```

**Response**

The response is similar to:

``` javascript
{
  "dev": {
    "uri": "https://chef.example/organizations/org1/policy_groups/dev",
      "policies": {
        "aar": {
          "revision_id": "95040c199302c85c9ccf1bcc6746968b820b1fa25d92477ea2ec5386cd58b9c5"
        },
        "jenkins": {
          "revision_id": "613f803bdd035d574df7fa6da525b38df45a74ca82b38b79655efed8a189e073"
      }
    }
    },
    "production": {
    "uri": "https://chef.example/organizations/org1/policy_groups/production",
      "policies": {
        "aar": {
          "revision_id": "95040c199302c85c9ccf1bcc6746968b820b1fa25d92477ea2ec5386cd58b9c5"
      }
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /policies/NAME

The `/policies/NAME` endpoint has the following methods: `DELETE`,
`GET`, and `PUT`. These endpoints enable the management of policies as
they relate to a specific policy group.

Each node has a 1:many relationship with policy settings stored on the
Chef server. This relationship is based on the policy group to which the
node is associated, and then the policy settings assigned to that group:

-   A policy is typically named after the functional role ahost
    performs, such as "application server", "chat server", "load
    balancer", and so on
-   A policy group defines a set of hosts in a deployed units, typically
    mapped to organizational requirements such as "dev", "test",
    "staging", and "production", but can also be mapped to more detailed
    requirements as needed

Each policy group and individual policy are separate objects for the
purposes of authentication. This enables each policy and policy group to
have restricted access, such as for specific nodes that handle sensitive
data or for specific production groups that require sign-off as part of
organizational requirements.

A requestor must have permission to both the policy and the policy group
in order for any action to be authorized.

#### DELETE

The `DELETE` method is used to delete the association between a specific
policy document, specific policy group, and specific policy revision.
This method does not delete anything from the Chef server.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/GROUP/policies/NAME
```

**Response**

The response returns the policy details and is similar to:

``` javascript
{
  "revision_id": "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9",
  "name": "aar",
  "run_list": [
    "recipe[aar::default]"
  ],
  "cookbook_locks": {
    "aar": {
      "version": "0.1.0",
      "identifier": "29648fe36333f573d5fe038a53256e23733618aa",
      "dotted_decimal_identifier": "11651043203167221.32604909279531813.121098535835818",
      "source": "cookbooks/aar",
      "cache_key": null,
      "scm_info": {
        "scm": "git",
        "remote": null,
        "revision": "a2c8cbb24a08625921d753cde36e8320465116c3",
        "working_tree_clean": false,
        "published": false,
        "synchronized_remote_branches": [
        ]
      },
      "source_options": {
        "path": "cookbooks/aar"
      }
    },
    "apt": {
      "version": "2.7.0",
      "identifier": "16c57abbd056543f7d5a15dabbb03261024a9c5e",
      "dotted_decimal_identifier": "6409580415309396.17870749399956400.55392231660638",
      "cache_key": "apt-2.7.0-supermarket.chef.io",
      "origin": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
      "source_options": {
        "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
        "version": "2.7.0"
      }
    }
  },
  "default_attributes": {
  },
  "override_attributes": {
  },
  "solution_dependencies": {
    "Policyfile": [
      [
        "aar",
        ">= 0.0.0"
      ],
      [
        "apt",
        "= 2.7.0"
      ],
    ],
    "dependencies": {
      "apt (2.7.0)": [
      ],
      "aar (0.1.0)": [
        [
          "apt",
          ">= 0.0.0"
        ]
      ]
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return a policy document for a specific
policy group and policy.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/GROUP/policies/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "revision_id": "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9",
  "name": "aar",
  "run_list": [
    "recipe[aar::default]"
  ],
  "cookbook_locks": {
    "aar": {
      "version": "0.1.0",
      "identifier": "29648fe36333f573d5fe038a53256e23733618aa",
      "dotted_decimal_identifier": "11651043203167221.32604909279531813.121098535835818",
      "source": "cookbooks/aar",
      "cache_key": null,
      "scm_info": {
        "scm": "git",
        "remote": null,
        "revision": "a2c8cbb24a08625921d753cde36e8320465116c3",
        "working_tree_clean": false,
        "published": false,
        "synchronized_remote_branches": [
        ]
      },
      "source_options": {
        "path": "cookbooks/aar"
      }
    },
    "apt": {
      "version": "2.7.0",
      "identifier": "16c57abbd056543f7d5a15dabbb03261024a9c5e",
      "dotted_decimal_identifier": "6409580415309396.17870749399956400.55392231660638",
      "cache_key": "apt-2.7.0-supermarket.chef.io",
      "origin": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
      "source_options": {
        "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
        "version": "2.7.0"
      }
    }
  },
  "default_attributes": {
  },
  "override_attributes": {
  },
  "solution_dependencies": {
    "Policyfile": [
      [
        "aar",
        ">= 0.0.0"
      ],
      [
        "apt",
        "= 2.7.0"
      ],
    ],
    "dependencies": {
      "apt (2.7.0)": [
      ],
      "aar (0.1.0)": [
        [
          "apt",
          ">= 0.0.0"
        ]
      ]
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to create or update an association between a
specific policy document, specific policy group, and specific policy
revision.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/GROUP/policies/NAME
```

with a request body similar to:

``` javascript
{
  "revision_id": "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9",
  "name": "aar",
  "run_list": [
    "recipe[aar::default]"
  ],
  "cookbook_locks": {
    "aar": {
      "version": "0.1.0",
      "identifier": "29648fe36333f573d5fe038a53256e23733618aa",
      "dotted_decimal_identifier": "11651043203167221.32604909279531813.121098535835818",
      "source": "cookbooks/aar",
      "cache_key": null,
      "scm_info": {
        "scm": "git",
        "remote": null,
        "revision": "a2c8cbb24a08625921d753cde36e8320465116c3",
        "working_tree_clean": false,
        "published": false,
        "synchronized_remote_branches": [
        ]
      },
      "source_options": {
        "path": "cookbooks/aar"
      }
    },
    "apt": {
      "version": "2.7.0",
      "identifier": "16c57abbd056543f7d5a15dabbb03261024a9c5e",
      "dotted_decimal_identifier": "6409580415309396.17870749399956400.55392231660638",
      "cache_key": "apt-2.7.0-supermarket.chef.io",
      "origin": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
      "source_options": {
        "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
        "version": "2.7.0"
      }
    }
  },
  "default_attributes": {
  },
  "override_attributes": {
  },
  "solution_dependencies": {
    "Policyfile": [
      [
        "aar",
        ">= 0.0.0"
      ],
      [
        "apt",
        "= 2.7.0"
      ],
    ],
    "dependencies": {
      "apt (2.7.0)": [
      ],
      "aar (0.1.0)": [
        [
          "apt",
          ">= 0.0.0"
        ]
      ]
    }
  }
}
```

**Response**

The response returns the policy details and is similar to:

``` javascript
{
  "revision_id": "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9",
  "name": "aar",
  "run_list": [
    "recipe[aar::default]"
  ],
  "cookbook_locks": {
    "aar": {
      "version": "0.1.0",
      "identifier": "29648fe36333f573d5fe038a53256e23733618aa",
      "dotted_decimal_identifier": "11651043203167221.32604909279531813.121098535835818",
      "source": "cookbooks/aar",
      "cache_key": null,
      "scm_info": {
        "scm": "git",
        "remote": null,
        "revision": "a2c8cbb24a08625921d753cde36e8320465116c3",
        "working_tree_clean": false,
        "published": false,
        "synchronized_remote_branches": [
        ]
      },
      "source_options": {
        "path": "cookbooks/aar"
      }
    },
    "apt": {
      "version": "2.7.0",
      "identifier": "16c57abbd056543f7d5a15dabbb03261024a9c5e",
      "dotted_decimal_identifier": "6409580415309396.17870749399956400.55392231660638",
      "cache_key": "apt-2.7.0-supermarket.chef.io",
      "origin": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
      "source_options": {
        "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/apt/versions/2.7.0/download",
        "version": "2.7.0"
      }
    }
  },
  "default_attributes": {
  },
  "override_attributes": {
  },
  "solution_dependencies": {
    "Policyfile": [
      [
        "aar",
        ">= 0.0.0"
      ],
      [
        "apt",
        "= 2.7.0"
      ],
    ],
    "dependencies": {
      "apt (2.7.0)": [
      ],
      "aar (0.1.0)": [
        [
          "apt",
          ">= 0.0.0"
        ]
      ]
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### New Config Settings

The following configuration settings are new for the Chef server:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>estatsd['protocol']</code></td>
<td>Use to send application statistics with StatsD protocol formatting. Set this value to <code>statsd</code> to apply StatsD protocol formatting.</td>
</tr>
</tbody>
</table>

## What's New in 12.0

The following items are new for Chef server 12:

-   **Upgrades from Open Source Chef and Enterprise Chef servers to Chef
    12 server** Upgrades to Chef server 12 are supported from Enterprise
    Chef 11 high availability and standalone configurations and Open
    Source Chef 11 standalone configurations. View the topic [Upgrade to
    Chef Server 12](/upgrade_server/) for more information about
    these processes.
-   **chef-server.rb configuration file is created by default** Previous
    versions of the Chef server did not create the chef-server.rb file
    and users had to create the file first, before updates to tuneable
    settings could be made.
-   **Pluggable high availability architecture** Support for high
    availability now provides alternatives to DRBD, including using
    Amazon Web Services (AWS).
-   **High availability using Amazon Web Services** Amazon Web Services
    (AWS) is a supported high availability configuration option for the
    Chef server. Machines are stored as Amazon Elastic Block Store (EBS)
    volumes. A passive node monitors the availabilty of the active node,
    and will take over if required.
-   **Chef server replication** Chef replication provides a way to
    asynchronously distribute cookbook, environment, role, and data bag
    data from a single, primary Chef server to one (or more) replicas of
    that Chef server.
-   **New chef-server-ctl command line tool** The chef-server-ctl
    command line tool is an update of the private-chef-ctl command line
    tool. All of the previous functionality remains, with some new
    commands added that are specific to Chef server version 12.
-   **New command for installing features of the Chef server** The
    `install` subcommand may be used to install Chef management console,
    Chef Push Jobs, Chef replication, and Reporting.
-   **New commands for managing organizations** New subcommands for the
    chef-server-ctl command line tool: `org-user-add`, `org-create`,
    `org-delete`, `org-user-remove`, `org-list`, and `org-show`.
-   **New commands for managing users** New subcommands for the
    chef-server-ctl command line tool: `user-create`, `user-delete`,
    `user-edit`, `user-list`, and `user-show`.
-   **New command for log files** Use the `gather-logs` command to
    create a tarball of important log files and system information.
-   **Solr has been upgraded to Solr 4** The search capabilities of the
    Chef server now use Apache Solr 4. The config item for Apache Solr 4
    has changed names from opscode-solr to opscode-solr4. Change
    `/etc/opscode/chef-server.rb` accordingly.
-   **CouchDB removed** CouchDB is no longer a component of the Chef
    server. All data is migrated to PostgreSQL.
-   **Services removed** The following services have been removed from
    the Chef server: `opscode-account`, `opscode-certificate`,
    `oc_authz_migrator`, `opscode-org-creator`, `orgmapper`, and
    `opscode-webui`. `opscode-webui` is replaced by the Chef management
    console.
-   **private-chef.rb is now called chef-server.rb** The name of the
    configuration file used by the Chef server has been changed. A
    symlink from private-chef.rb to chef-server.rb is created during
    upgrades from older versions of the Chef server.
-   **New setting for the default organization name** Use the
    `default_orgname` setting to ensure compatibility with Open Source
    Chef version 11.
-   **New settings for oc_chef_authz** The **opscode-authz** service
    handles authorization requests to the Chef server.
-   **Organization policy changes** Users must be removed from the
    `admins` security group before they can be removed from an
    organization. The chef-client is not granted **Create**, **Delete**,
    or **Update** permissions to data bags when organizations are
    created.
-   **Administrators cannot be removed from organizations** The Chef
    server requires that a member of an organization's `admins` group
    cannot be removed from the organization without first being removed
    from the `admins` group.
-   **New settings for managing LDAP encryption** New settings that
    manage LDAP encryption have been added, existing settings have been
    deprecated.
-   **New commands for managing keys** The following commands are new:
    `add-client-key`, `add-user-key`, `delete-client-key`,
    `delete-user-key`, `list-client-keys`, and `list-user-keys`. (These
    are preview commands, new as-of the Chef server 12.0.3 release.)

### Upgrade to Chef server 12!

Upgrades to Chef server 12 are supported for both Enterprise Chef and
Open Source Chef users. See /server/upgrade_server.html for more
information about upgrades. If you are upgrading from Open Source Chef,
please see /server/upgrade_server_open_source_notes.html as well.

### HA using AWS

Amazon Web Services (AWS) is a supported high availability configuration
option for the Chef server.

<img src="/images/chef_server_ha_aws.svg" class="align-center" width="600" alt="image" />

Backend servers make use of a single Amazon Elastic Block Store (EBS)
volume.

For more information about Amazon Elastic Block Store (EBS), see
<http://aws.amazon.com/ebs/>.

View the topic [High Availability: Backend
Cluster](/install_server_ha/) for more information about how to set
up the Chef server for high availability in Amazon Web Services (AWS).

### Chef Replication

Chef replication provides a way to asynchronously distribute cookbook,
environment, role, and data bag data from a single, primary Chef server
to one (or more) replicas of that Chef server.

**Scenarios**

Replication is configured on a per-organization and also a per-replica
basis. Each organization must be configured to synchronize with each
replica instance. Each organization may be configured to synchronize
with all, some, or none of the available replica instances.

For example, a single primary Chef server and a single replica:

![image](/images/chef_server_replication.png)

and for example, a single primary Chef server and multiple replicas:

![image](/images/chef_server_replication_many.png)

Chef replication should not be used for:

-   Disaster recovery or backup/restore processes. The replication
    process is read-only and cannot be changed to read-write
-   Synchronizing a replica instance with another replica instance
-   Node re-registration. A node may be associated only with a single
    Chef server

**How Replication Works**

A daemon named **ec-syncd** runs on each of the replica instances of the
Chef server and periodically polls the primary Chef server via the
`updated_since` endpoint in the Chef server API. The **ec-syncd** daemon
requests a list of objects that have been updated since the last
successful synchronization time. If there are updates, the **ec-syncd**
daemon then pulls down the updated data from the primary Chef server to
the replica.

![image](/images/chef_server_replication_sequence.png)

View the topic [Chef Replication](/server_replication/) for more
information about how to set up the Chef server for replication.

### chef-server-ctl

The command line tool for the Chef server has been renamed from
private-chef-ctl to chef-server-ctl. The same set of subcommands
available for private-chef-ctl are also available for chef-server-ctl,
but with an updated syntax:

``` bash
chef-server-ctl command
```

In addition, the `install` subcommand is added, plus two new subcommand
groupings---`org-*` and `user-*`---have been added for managing
organizations and users. See below for more information about these new
subcommands.

### install Command

The `install` subcommand is used to install premium features of the Chef
server: Chef management console and chef-client run reporting, high
availability configurations, Chef Push Jobs, and Chef server
replication.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl install name_of_addon (options)
```

where `name_of_addon` represents the command line value associated with
the add-on or premium feature.

**Options**

This subcommand has the following options:

`--path PATH`

:   Use to specify the location of a package. This option is not
    required when packages are downloaded from
    <https://packages.chef.io/>.

**Use Downloads**

The `install` subcommand downloads packages from
<https://packages.chef.io/> by default. For systems that are not behind
a firewall (and have connectivity to <https://packages.chef.io/>), these
packages can be installed as described below.

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Feature</th>
<th>Command</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>Chef Manage</p></td>
<td><p>Use Chef management console to manage data bags, attributes, run-lists, roles, environments, and cookbooks from a web user interface.</p>
<p>On the Chef server, run:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb1-1"><a href="#cb1-1"></a><span class="fu">sudo</span> chef-server-ctl install chef-manage</span></code></pre></div>
<p>then:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb2-1"><a href="#cb2-1"></a><span class="fu">sudo</span> chef-server-ctl reconfigure</span></code></pre></div>
<p>and then:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb3-1"><a href="#cb3-1"></a><span class="fu">sudo</span> chef-manage-ctl reconfigure</span></code></pre></div>
{{< note >}}
<p>Starting with the Chef management console 2.3.0, the <a href="/chef_license/">Chef MLSA</a> must be accepted when reconfiguring the product. If the Chef MLSA has not already been accepted, the reconfigure process will prompt for a <code>yes</code> to accept it. Or run <code>chef-manage-ctl reconfigure --accept-license</code> to automatically accept the license.</p>
{{< /note >}}</td>
</tr>
</tbody>
</table>

**Use Local Packages**

The `install` subcommand downloads packages from
<https://packages.chef.io/> by default. For systems that are behind a
firewall (and may not have connectivity to packages.chef.io), these
packages can be downloaded from
<https://downloads.chef.io/chef-manage/>, and then installed manually.
First download the package that is appropriate for the platform, save it
to a local path, and then run the `install` command using the `--path`
option to specify the directory in which the package is located:

``` bash
sudo chef-server-ctl install PACKAGE_NAME --path /path/to/package/directory
```

For example:

``` bash
sudo chef-server-ctl install chef-manage --path /root/packages
```

The `chef-server-ctl` command will install the first `chef-manage`
package found in the `/root/packages` directory.

### gather-logs Command

The `gather-logs` subcommand is used to gather the Chef server log files
into a tarball that contains all of the important log files and system
information.

This subcommand has the following syntax:

``` bash
chef-server-ctl gather-logs
```

### user-\* Commands

The following subcommands can be used to manage users:

#### user-create

The `user-create` subcommand is used to create a user. (The validation
key for the organization may be returned to `STDOUT` when creating a
user with this command.)

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl user-create USER_NAME FIRST_NAME [MIDDLE_NAME] LAST_NAME EMAIL 'PASSWORD' (options)
```

**Options**

This subcommand has the following options:

`-f FILE_NAME`, `--filename FILE_NAME`

:   Write the USER.pem to a file instead of `STDOUT`.

**Examples**

``` bash
chef-server-ctl user-create john_smith John Smith john_smith@example.com p@s5w0rD!
```

``` bash
chef-server-ctl user-create jane_doe Jane Doe jane_doe@example.com p@s5w0rD! -f /tmp/jane_doe.key
```

``` bash
chef-server-ctl user-create waldendude Henry David Thoreau waldendude@example.com excursions
```

#### user-delete

The `user-delete` subcommand is used to delete a user.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl user-delete USER_NAME
```

**Examples**

``` bash
chef-server-ctl user-delete john_smith
```

``` bash
chef-server-ctl user-delete jane_doe
```

#### user-edit

The `user-edit` subcommand is used to edit the details for a user. The
data will be made available in the \$EDITOR for editing.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl user-edit USER_NAME
```

**Examples**

``` bash
chef-server-ctl user-edit john_smith
```

``` bash
chef-server-ctl user-edit jane_doe
```

#### user-list

The `user-list` subcommand is used to view a list of users.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl user-list (options)
```

**Options**

This subcommand has the following options:

`-w`, `--with-uri`

:   Show the corresponding URIs.

#### user-show

The `user-show` subcommand is used to show the details for a user.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl user-show USER_NAME (options)
```

**Options**

This subcommand has the following options:

`-l`, `--with-orgs`

:   Show all organizations.

### org-\* Commands

The following subcommands can be used to manage organizations:

#### org-create

The `org-create` subcommand is used to create an organization. (The
validation key for the organization is returned to `STDOUT` when
creating an organization with this command.)

**Syntax**

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

**Options**

This subcommand has the following options:

`-a USER_NAME`, `--association_user USER_NAME`

:   Associate a user with an organization and add them to the `admins`
    and `billing_admins` security groups.

`-f FILE_NAME`, `--filename FILE_NAME`

:   Write the ORGANIZATION-validator.pem to `FILE_NAME` instead of
    printing it to `STDOUT`.

**Examples**

``` bash
chef-server-ctl org-create prod Production
```

``` bash
chef-server-ctl org-create staging Staging -a chef-admin
```

``` bash
chef-server-ctl org-create dev Development -f /tmp/id-dev.key
```

#### org-delete

The `org-delete` subcommand is used to delete an organization.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl org-delete ORG_NAME
```

**Examples**

``` bash
chef-server-ctl org-delete infra-testing-20140909
```

``` bash
chef-server-ctl org-delete pedant-testing-org
```

#### org-list

The `org-list` subcommand is used to list all of the organizations
currently present on the Chef server.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl org-list (options)
```

**Options**

This subcommand has the following options:

`-a`, `--all-orgs`

:   Show all organizations.

`-w`, `--with-uri`

:   Show the corresponding URIs.

#### org-show

The `org-show` subcommand is used to show the details for an
organization.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl org-show ORG_NAME
```

#### org-user-add

{{< warning >}}

Early RC candidates for the Chef server 12 release named this command
`org-associate`. This is the same command, with the exception of the
`--admin` flag, which is added to the command (along with the rename)
for the upcoming final release of Chef server 12.

{{< /warning >}}

The `org-user-add` subcommand is used to add a user to an organization.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl org-user-add ORG_NAME USER_NAME (options)
```

**Options**

This subcommand has the following options:

`--admin`

:   Add the user to the `admins` group.

**Examples**

``` bash
chef-server-ctl org-user-add prod john_smith
```

``` bash
chef-server-ctl org-user-add preprod testmaster
```

``` bash
chef-server-ctl org-user-add dev grantmc --admin
```

#### org-user-remove

{{< warning >}}

Early RC candidates for the Chef server 12 release named this command
`org-disociate`. This is the same command, but renamed for the upcoming
final release of Chef server 12.

{{< /warning >}}

The `org-user-remove` subcommand is used to remove a user from an
organization.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl org-user-remove ORG_NAME USER_NAME (options)
```

**Examples**

``` bash
chef-server-ctl org-user-remove prod john_smith
```

``` bash
chef-server-ctl org-user-remove prod testmaster
```

### Configuration Settings

The name of the Chef server configuration file is now chef-server.rb.

The following configuration settings are new for Chef server version 12:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>default_orgname</code></td>
<td>The Chef server API used by the Open Source Chef server does not have an <code>/organizations/ORG_NAME</code> endpoint. Use this setting to ensure that migrated Open Source Chef servers are able to connect to the Chef server API. This value should be the same as the name of the organization that was created during the upgrade from Open Source Chef version 11 to Chef server version 12, which means it will be identical to the <code>ORG_NAME</code> part of the <code>/organizations</code> endpoint in Chef server version 12. Default value: the name of the organization specified during the upgrade process from Open Source Chef 11 to Chef server 12.</td>
</tr>
<tr class="even">
<td><code>postgresql['log_min_duration_statement']</code></td>
<td>When to log a slow PostgreSQL query statement. Possible values: <code>-1</code> (disabled, do not log any statements), <code>0</code> (log every statement), or an integer greater than zero. When the integer is greater than zero, this value is the amount of time (in milliseconds) that a query statement must have run before it is logged. Default value: <code>-1</code>.</td>
</tr>
</tbody>
</table>

The following configuration settings have updated default values
starting with Chef server version 12:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>api_version</code></td>
<td>The version of the Chef server. Default value: <code>"12.0.0"</code>.</td>
</tr>
</tbody>
</table>

The following configuration settings are new in Chef server version
12.0.5:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>opscode_erchef['nginx_bookshelf_caching']</code></td>
<td>Whether Nginx is used to cache cookbooks. When <code>:on</code>, Nginx serves up the cached content instead of forwarding the request. Default value: <code>:off</code>.</td>
</tr>
<tr class="even">
<td><code>opscode_erchef['s3_url_expiry_window_size']</code></td>
<td>The frequency at which unique URLs are generated. This value may be a specific amount of time, i.e. <code>15m</code> (fifteen minutes) or a percentage of the value of <code>s3_url_ttl</code>, i.e. <code>10%</code>. Default value: <code>:off</code>.</td>
</tr>
</tbody>
</table>

#### oc_chef_authz

The **opscode-authz** service is used to handle authorization requests
to the Chef server.

This configuration file has the following settings for `oc_chef_authz`:

`oc_chef_authz['http_cull_interval']`

:   Default value: `'{1, min}'`.

`oc_chef_authz['http_init_count']`

:   Default value: `25`.

`oc_chef_authz['http_max_age']`

:   Default value: `'{70, sec}'`.

`oc_chef_authz['http_max_connection_duration']`

:   Default value: `'{70, sec}'`.

`oc_chef_authz['http_max_count']`

:   Default value: `100`.

`oc_chef_authz['ibrowse_options']`

:   The amount of time (in milliseconds) to wait for a connection to be
    established. Default value: `'[{connect_timeout, 5000}]'`.

### Data Bag Policy Changes

In previous versions of the Chef server, the default permissions allowed
data bags to be updated by the chef-client during a chef-client run.
Starting with Chef server version 12, the chef-client is not granted
**Create**, **Delete**, or **Update** permissions to data bags when
organizations are created. Use the Chef management console or the
`knife-acl` plugin (<https://github.com/chef/knife-acl>) to manage
permissions to data bags as required. For example:

``` bash
knife acl add containers data update group clients
```

For cookbooks that create or delete data bags:

``` bash
knife acl add containers data create group clients

knife acl add containers data delete group clients
```

For existing organizations that want to remove the **Create**,
**Delete**, or **Update** permissions from existing nodes:

``` bash
knife acl remove containers data update group clients

knife acl remove containers data delete group clients

knife acl remove containers data create group clients
```

See this blog post for more information about the `knife-acl` plugin:
<https://www.chef.io/blog/2014/11/10/security-update-hosted-chef/>

### New Settings for LDAP

The following settings are new:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>ldap['ssl_enabled']</code></td>
<td>Use to enable SSL. Default value: <code>false</code>. Must be <code>false</code> when <code>ldap['tls_enabled']</code> is <code>true</code>.</td>
</tr>
<tr class="even">
<td><code>ldap['tls_enabled']</code></td>
<td>Use to enable TLS. When enabled, communication with the LDAP server is done via a secure SSL connection on a dedicated port. When <code>true</code>, <code>ldap['port']</code> is also set to <code>636</code>. Default value: <code>false</code>. Must be <code>false</code> when <code>ldap['ssl_enabled']</code> is <code>true</code>.</td>
</tr>
</tbody>
</table>

{{< note >}}

Previous versions of the Chef server used the `ldap['ssl_enabled']`
setting to first enable SSL, and then the `ldap['encryption']` setting
to specify the encryption type. These settings are deprecated.

{{< /note >}}

### Key Rotation

Use the following commands to manage public and private key rotation for
users and clients.

### add-client-key

Use the `add-client-key` subcommand to add a client key.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl add-client-key ORG_NAME CLIENT_NAME [--public-key-path PATH] [--expiration-date DATE] [--key-name NAME]
```

{{< warning >}}

All options for this subcommand must follow all arguments.

{{< /warning >}}

**Options**

This subcommand has the following options:

`CLIENT_NAME`

:   The name of the client that you wish to add a key for.

`-e DATE` `--expiration-date DATE`

:   An ISO 8601 formatted string: `YYYY-MM-DDTHH:MM:SSZ`. For example:
    `2013-12-24T21:00:00Z`. If not passed, expiration will default to
    infinity.

`-k NAME` `--key-name NAME`

:   String defining the name of your new key for this client. If not
    passed, it will default to the fingerprint of the public key.

`ORG_NAME`

:   The short name for the organization to which the client belongs.

`-p PATH` `--public-key-path PATH`

:   The location to a file containing valid PKCS\#1 public key to be
    added. If not passed, then the server will generate a new one for
    you and return the private key to STDOUT.

### add-user-key

Use the `add-user-key` subcommand to add a user key.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl add-user-key USER_NAME [--public-key-path PATH] [--expiration-date DATE] [--key-name NAME]
```

{{< warning >}}

All options for this subcommand must follow all arguments.

{{< /warning >}}

**Options**

This subcommand has the following options:

`-e DATE` `--expiration-date DATE`

:   An ISO 8601 formatted string: `YYYY-MM-DDTHH:MM:SSZ`. For example:
    `2013-12-24T21:00:00Z`. If not passed, expiration will default to
    infinity.

`-k NAME` `--key-name NAME`

:   String defining the name of your new key for this user. If not
    passed, it will default to the fingerprint of the public key.

`-p PATH` `--public-key-path PATH`

:   The location to a file containing valid PKCS\#1 public key to be
    added. If not passed, then the server will generate a new one for
    you and return the private key to STDOUT.

`USER_NAME`

:   The user name for the user for which a key is added.

### delete-client-key

Use the `delete-client-key` subcommand to delete a client key.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl delete-client-key ORG_NAME CLIENT_NAME KEY_NAME
```

**Options**

This subcommand has the following arguments:

`ORG_NAME`

:   The short name for the organization to which the client belongs.

`CLIENT_NAME`

:   The name of the client.

`KEY_NAME`

:   The unique name to be assigned to the key you wish to delete.

### delete-user-key

Use the `delete-user-key` subcommand to delete a user key.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl delete-user-key USER_NAME KEY_NAME
```

{{< warning >}}

The parameters for this subcommand must be in the order specified above.

{{< /warning >}}

**Options**

This subcommand has the following arguments:

`USER_NAME`

:   The user name.

`KEY_NAME`

:   The unique name to be assigned to the key you wish to delete.

### list-client-key

Use the `list-client-keys` subcommand to list client keys.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl list-client-keys ORG_NAME CLIENT_NAME [--verbose]
```

{{< warning >}}

All options for this subcommand must follow all arguments.

{{< /warning >}}

**Options**

This subcommand has the following options:

`CLIENT_NAME`

:   The name of the client.

`ORG_NAME`

:   The short name for the organization to which the client belongs.

`--verbose`

:   Use to show the full public key strings in command output.

### list-user-key

Use the `list-user-keys` subcommand to list client keys.

**Syntax**

This subcommand has the following syntax:

``` bash
chef-server-ctl list-user-keys USER_NAME [--verbose]
```

{{< warning >}}

All options for this subcommand must follow all arguments.

{{< /warning >}}

**Options**

This subcommand has the following options:

`USER_NAME`

:   The user name you wish to list keys for.

`--verbose`

:   Use to show the full public key strings in command output.

**Example**

``` bash
chef-server-ctl list-user-keys applejack
```

Returns:

``` bash
2 total key(s) found for user applejack

key_name: test-key
expires_at: Infinity
public_key:
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4q9Dh+bwJSjhU/VI4Y8s
9WsbIPfpmBpoZoZVPL7V6JDfIaPUkdcSdZpynhRLhQwv9ScTFh65JwxC7wNhVspB
4bKZeW6vugNGwCyBIemMfxMlpKZQDOc5dnBiRMMOgXSIimeiFtL+NmMXnGBBHDaE
b+XXI8oCZRx5MTnzEs90mkaCRSIUlWxOUFzZvnv4jBrhWsd/yBM/h7YmVfmwVAjL
VST0QG4MnbCjNtbzToMj55NAGwSdKHCzvvpWYkd62ZOquY9f2UZKxYCX0bFPNVQM
EvBQGdNG39XYSEeF4LneYQKPHEZDdqe7TZdVE8ooU/syxlZgADtvkqEoc4zp1Im3
2wIDAQAB
-----END PUBLIC KEY-----

key_name: default
expires_at: Infinity
public_key:
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4q9Dh+bwJSjhU/VI4Y8s
9WsbIPfpmBpoZoZVPL7V6JDfIaPUkdcSdZpynhRLhQwv9ScTFh65JwxC7wNhVspB
4bKZeW6vugNGwCyBIemMfxMlpKZQDOc5dnBiRMMOgXSIimeiFtL+NmMXnGBBHDaE
b+XXI8oCZRx5MTnzEs90mkaCRSIUlWxOUFzZvnv4jBrhWsd/yBM/h7YmVfmwVAjL
VST0QG4MnbCjNtbzToMj55NAGwSdKHCzvvpWYkd62ZOquY9f2UZKxYCX0bFPNVQM
EvBQGdNG39XYSEeF4LneYQKPHEZDdqe7TZdVE8ooU/syxlZgADtvkqEoc4zp1Im3
2wIDAQAB
-----END PUBLIC KEY-----
```

### Changelog

For the list of issues that were addressed for this release, please see
the changelog on GitHub:
<https://github.com/chef/chef-server/blob/master/CHANGELOG.md>
