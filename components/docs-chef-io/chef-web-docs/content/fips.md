+++
title = "FIPS (Federal Information Processing Standards)"
draft = false

aliases = ["/fips.html"]

[menu]
  [menu.infra]
    title = "FIPS"
    identifier = "chef_infra/features/fips.md FIPS"
    parent = "chef_infra/features"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/fips.md)

## What is FIPS?

{{% fips_intro %}}

### Who should enable FIPS?

You may be legally required to enable FIPS if you are a United States
non-military government agency, or are contracting with one. If you are
not sure if you need to enable FIPS, please check with your compliance
department.

### Who shouldn't enable FIPS?

You will only need to enable FIPS if you are a US non-military
government agency, or contracting with one, and you are contractually
obligated to meet federal government security standards. If you are not
a US non-military governmental agency, or you are not contracting with
one, and you are not contractually obligated to meet federal government
security standards, then do not enable FIPS. Chef products have robust
security standards even without FIPS, and FIPS prevents the use of
certain hashing algorithms you might want to use, so we only recommend
enabling FIPS if it is contractually necessary.

## Supported Products

**Supported:**

- [Chef Infra Client](/fips/#how-to-enable-fips-mode-for-the-chef-client)
- [Chef Workstation](/fips/#how-to-enable-fips-mode-for-workstations)
- [Chef Infra Server](/fips/#how-to-enable-fips-mode-for-the-chef-server)

**Unsupported:**

FIPS mode is not supported for Chef Infra Server add-ons. This includes:

- Chef Manage
- Push Jobs

## How to enable FIPS mode in the Operating System

### FIPS kernel settings

Windows and Red Hat Enterprise Linux can both be configured for FIPS
mode using a kernel-level setting. After FIPS mode is enabled at the
kernel level, the operating system will only use FIPS approved
algorithms and keys during operation.

All of the tools Chef produces that have FIPS support read this kernel
setting and default their mode of operation to match it with the
exception of the workstation, which requires designating a port in the
`fips_git_port` setting of the `cli.toml`. For the other Chef tools,
Chef Infra Client, for example, if `chef-client` is run on an operating
system configured into FIPS mode and you run, that Chef run will
automatically be in FIPS mode unless the user disables it.

To enable FIPS on your platform follow these instructions:

- [Red Hat Enterprise Linux 6](https://access.redhat.com/documentation/en-US/Red_Hat_Enterprise_Linux/6/html/Security_Guide/sect-Security_Guide-Federal_Standards_And_Regulations-Federal_Information_Processing_Standard.html)
- [Red Hat Enterprise Linux 7](https://access.redhat.com/documentation/en-US/Red_Hat_Enterprise_Linux/7/html/Security_Guide/chap-Federal_Standards_and_Regulations.html#sec-Enabling-FIPS-Mode)
- [Red Hat Enterprise Linux 8](https://www.redhat.com/en/blog/how-rhel-8-designed-fips-140-2-requirements)
- [Windows](https://technet.microsoft.com/en-us/library/cc750357.aspx)

## How to enable FIPS mode for the Chef Infra Server

### Prerequisites

- Supported Systems - CentOS or Red Hat Enterprise Linux 6 or greater
- Chef Infra Server version <span class="title-ref">12.13.0</span> or greater

### Configuration

If you have FIPS compliance enabled at the kernel level and install or
reconfigure the Chef Infra Server then it will default to running in
FIPS mode.

To enable FIPS manually for the Chef Infra Server, can add `fips true`
to the `/etc/opscode/chef-server.rb` and reconfigure. For more
configuration information see [Chef
Server](/config_rb_server_optional_settings/).

## How to enable FIPS mode for the Chef Infra Client

### Prerequisites

- Supported Systems - CentOS, Oracle Linux, or Red Hat Enterprise Linux 6 or later

### Configuration

If you have FIPS compliance enabled at the kernel level then Chef Infra
Client will default to running in FIPS mode. Otherwise you can add
`fips true` to the `/etc/chef/client.rb` or `C:\\chef\\client.rb`.

**Bootstrap a node using FIPS**

{{% knife_bootstrap_node_fips %}}

## How to enable FIPS mode on Automate 1 (DEPRECATED)

### Prerequisites

- Supported Systems - Windows, CentOS, Oracle Linux, and Red Hat Enterprise Linux

Now that FIPS mode is enabled in your `.delivery/cli.toml`, running any
project-specific Delivery CLI command will automatically use
FIPS-compliant encrypted git traffic between your workstation and the
Chef Automate server. As long as the Chef Automate server is in FIPS
mode, no other action is needed on your part to operate Delivery CLI in
FIPS mode. If you ever stop using FIPS mode on the Chef Automate server,
simply delete the above two lines from your `.delivery/cli.toml` file
and Delivery CLI will stop running in FIPS mode.

{{< note >}}

You could also pass `--fips` and `--fips-git-port=OPEN_PORT` into
project specific commands if you do not wish to edit your
`.delivery/cli.toml`. See list of commands below for details..

{{< /note >}}

For more information on configuring the Chef Automate server, see
[Delivery CLI](/delivery_cli/).

{{< note >}}

If you set up any runners using a Chef Automate server version `0.7.61`
or earlier, then you will need to re-run [automate-ctl
install-runner](/ctl_automate_server/#install-runner) on every
existing runner after upgrading your Chef Automate server. Your runners
will not work with FIPS enabled without re-running the installer.

{{< /note >}}

## Architecture Overview

![](/images/automate-fips.png)

When Automate is running in FIPS mode, it uses stunnel to stand up
encrypted tunnels between servers and clients to carry traffic generated
by programs that do not support FIPS 140-2 validation, thus wrapping
non-FIPS compliant traffic within a FIPS-compliant tunnel. The stunnel
is stood up prior to a request and torn down thereafter. Enabling FIPS
in Chef Automate disables its git server and isolates it on localhost,
where it listens for stunnel traffic over port 8989.

## Certificate Management

If you are using a certificate purchased from a well-known certificate
authority then no additional configuration should be required.

The well-known certificate authorities are those trusted by Mozilla and
captured in a file known as cacert.pem, which can be referenced here:
<https://curl.haxx.se/docs/caextract.html>

If you have a self-signed certificate or a customer certificate
authority then you will need some additional steps to get your Automate
stack configured.

{{< note >}}

Any time this certificate changes you must re-run this process.

{{< /note >}}

-   Generate a pem file with your entire certificate chain of the Chef
    Automate instance and save it to a file. A client machine may run
    the above openssl command to avoid having to copy/paste the
    certificate chain around as well. For Example:

    ``` none
    echo "q" | openssl s_client -showcerts -connect yourautomateserver.com:443 </dev/null 2> /dev/null

    CONNECTED(00000003)
    ---
    Certificate chain
    0 s:/C=US/O=Acme/OU=Profit Center/CN=yourautomateserver.com
    i:/C=US/O=Acme/OU=Profit Center/CN=Root CA
    -----BEGIN CERTIFICATE-----
    (server certificate)
    -----END CERTIFICATE-----
    1 s:/C=US/O=Acme/OU=Profit Center/CN=Root CA
    i:/C=US/O=Acme/OU=Profit Center/CN=Root CA
    -----BEGIN CERTIFICATE-----
    (root certificate)
    -----END CERTIFICATE-----
    ---
    ...
    ```

    Create a new file `yourautomateserver.com.pem` and copy both of the
    certificate sections in order. In this example the file should look
    like:

    ``` none
    -----BEGIN CERTIFICATE-----
    (server certificate)
    -----END CERTIFICATE-----
    -----BEGIN CERTIFICATE-----
    (root certificate)
    -----END CERTIFICATE-----
    ```

-   Every workstation will need a copy of this file and the cli.toml
    should be updated to include this configuration option.

    ``` none
    fips_custom_cert_filename = "/full/path/to/your/certificate-chain.pem"
    ```

-   When configuring runners you'll need to include the file generated
    above as an argument to the <span
    class="title-ref">install-runner</span> command. See [Install
    Runner](/ctl_automate_server/#install-runner).

    ``` none
    automate-ctl install-runner [server fqdn] [ssh user] --fips-custom-cert-filename path/to/your/certificate-chain.pem [other options...]
    ```

## Troubleshooting

If you experience configuration errors, check the Chef Automate
configuration by running `delivery status` from any client machine. This
command is further documented in [Check if Chef Automate has enabled
FIPS
mode](/delivery_cli/#check-if-chef-automate-server-has-enabled-fips-mode).

Running `delivery status` should return something like:

``` none
Status information for Automate server automate-server.dev

Status: up (request took 97 ms)
Configuration Mode: standalone
FIPS Mode: enabled
Upstreams:
Lsyncd:
   status: not_running
PostgreSQL:
   status: up
RabbitMQ:
   status: up
   node_health:
      status: up
   vhost_aliveness:
      status: up
```

Your Automate Server is configured in FIPS mode. Please add the
following to your cli.toml to enable Automate FIPS mode on your machine:

``` none
fips = true
fips_git_port = "OPEN_PORT"
```

Replace OPEN_PORT with any port that is free on your machine.

### Unable to run any delivery commands when FIPS is enabled

1.  Confirm FIPS is enabled on Chef Automate with `delivery status`. You
    should see `FIPS Mode: enabled`.

2.  Confirm your project's `cli.toml` is configured correctly. The
    following configuration items should be present:

    ``` none
    fips_enabled = true
    fips_git_port = "<some open port>"

    # Below is only used with self-signed certificates or custom certificate
    # authorities

    fips_custom_cert_filename = "/path/to/file/with/certificate-chain.pem"
    ```

3.  On Windows you will need to kill the tunnel whenever you make a fips
    configuration change to `cli.toml`. To restart the tunnel:

    ``` none
    PS C:\Users\user> tasklist /fi "imagename eq stunnel.exe"

    Image Name                     PID Session Name        Session#    Mem Usage
    ========================= ======== ================ =========== ============
    stunnel.exe                   2520 Console                    1      9,040 K

    PS C:\Users\user> taskkill 2520
    PS C:\Users\user\example-project> delivery review # will restart the tunnel on the next execution
    ```

### Self-signed certificate or custom certificate authority

See the section on [Certificate
Management](/fips/#certificate-management).

### Nothing above has helped

If you continue to have issues you should include the following logs
with your support request:

1.  Stunnel client log `~/.chefdk/log/stunnel.log` on your workstation
2.  Stunnel server log `sudo automate-ctl log stunnel`
3.  Stunnel configuration file on your workstation
    `C:\\opscode\\chefdk\\embedded\\stunnel.conf` or
    `~/.chefdk/etc/stunnel.conf`
