+++
title = "Working with Proxies"
draft = false

aliases = ["/proxies.html"]

[menu]
  [menu.infra]
    title = "Working with Proxies"
    identifier = "chef_infra/setup/proxies.md Working with Proxies"
    parent = "chef_infra/setup"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/proxies.md)

In an environment that requires proxies to reach the Internet, many Chef
commands will not work until they are configured correctly. To configure
Chef to work in an environment that requires proxies, set the
`http_proxy`, `https_proxy`, `ftp_proxy`, and/or `no_proxy` environment
variables to specify the proxy settings using a lowercase value.

## Microsoft Windows

{{% proxy_windows %}}

## Linux

To determine the current proxy server on the macOS and Linux platforms,
check the environment variables. Run the following:

``` bash
env | grep -i http_proxy
```

If an environment variable is set, it **MUST** be lowercase. If it is
not, add a lowercase version of that proxy variable to the shell (e.g.
`~/.bashrc`) using one (or more) the following commands.

For HTTP:

``` bash
export http_proxy=http://myproxy.com:3168
```

For HTTPS:

``` bash
export https_proxy=http://myproxy.com:3168
```

For FTP:

``` bash
export ftp_proxy=ftp://myproxy.com:3168
```

## Proxy Settings

Proxy settings are defined in configuration files for Chef Infra Client
and for knife and may be specified for HTTP, HTTPS, and FTP.

### HTTP

Use the following settings in the client.rb or config.rb files for
environments that use an HTTP proxy:

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
<td><code>http_proxy</code></td>
<td>The proxy server for HTTP connections. Default value: <code>nil</code>.</td>
</tr>
<tr class="even">
<td><code>http_proxy_pass</code></td>
<td>The password for the proxy server when the proxy server is using an HTTP connection. Default value: <code>nil</code>.</td>
</tr>
<tr class="odd">
<td><code>http_proxy_user</code></td>
<td>The user name for the proxy server when the proxy server is using an HTTP connection. Default value: <code>nil</code>.</td>
</tr>
</tbody>
</table>

### HTTPS

Use the following settings in the client.rb or config.rb files for
environments that use an HTTPS proxy:

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
<td><code>https_proxy</code></td>
<td>The proxy server for HTTPS connections. Default value: <code>nil</code>.</td>
</tr>
<tr class="even">
<td><code>https_proxy_pass</code></td>
<td>The password for the proxy server when the proxy server is using an HTTPS connection. Default value: <code>nil</code>.</td>
</tr>
<tr class="odd">
<td><code>https_proxy_user</code></td>
<td>The user name for the proxy server when the proxy server is using an HTTPS connection. Default value: <code>nil</code>.</td>
</tr>
</tbody>
</table>

### FTP

Use the following settings in the client.rb or config.rb files for
environments that use an FTP proxy:

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
<td><code>ftp_proxy</code></td>
<td>The proxy server for FTP connections.</td>
</tr>
<tr class="even">
<td><code>ftp_proxy_pass</code></td>
<td>The password for the proxy server when the proxy server is using an FTP connection. Default value: <code>nil</code>.</td>
</tr>
<tr class="odd">
<td><code>ftp_proxy_user</code></td>
<td>The user name for the proxy server when the proxy server is using an FTP connection. Default value: <code>nil</code>.</td>
</tr>
</tbody>
</table>

### No Proxy

The `no_proxy` setting is used to specify addresses for which the proxy
should not be used. This can be a single address or a comma-separated
list of addresses.

Example:

``` ruby
no_proxy 'test.example.com,test.example2.com,test.example3.com'
```

{{< note >}}

Wildcard matching may be used in the `no_proxy` list---such as
`no_proxy '*.*.example.*'`---however, many situations require hostnames
to be specified explicitly (i.e. "without wildcards").

{{< /note >}}

## Environment Variables

Consider the following for situations where environment variables are
used to set the proxy:

-   Proxy settings may not be honored by all applications. For example,
    proxy settings may be ignored by the underlying application when
    specifying a `ftp` source with a `remote_file` resource. Consider a
    workaround. For example, in this situation try doing a `wget` with
    an `ftp` URL instead.
-   Proxy settings may be honored inconsistently by applications. For
    example, the behavior of the `no_proxy` setting may not work with
    certain applications when wildcards are specified. Consider
    specifying the hostnames without using wildcards.

### ENV

{{% proxy_env %}}
