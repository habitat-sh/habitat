## How to enable FIPS mode for the Chef Automate server

### Prerequisites

-   Supported Systems - CentOS or Red Hat Enterprise Linux 6 or later

### Configuration

If you have FIPS compliance enabled in the operating system at the
kernel level and install or reconfigure the Chef Automate server then it
will default to running in FIPS mode.

A Chef Automate server running in FIPS mode can only communicate with
workstations that are also running in FIPS mode.

If you do need to use FIPS mode, there are a few steps to get it up and
running in Delivery CLI on your workstation.

### Check if Chef Automate server has enabled FIPS mode

You can see if your Chef Automate server is in FIPS mode by running
`delivery status`. It will say `FIPS mode: enabled` if it is enabled as
well as output some instructions on how to set up your `cli.toml` to
enable FIPS mode locally. If `delivery status` reports either
`FIPS mode: disabled` or FIPS is missing completely from the report,
please see [FIPS kernel settings](/fips.html#fips-kernel-settings) on
how to enable FIPS mode in your Chef Automate server before proceeding.

### Enable FIPS mode in your cli.toml file

Now that you have confirmed that the Chef Automate server is in FIPS
mode, you must enable FIPS mode locally on your workstation for Delivery
CLI. This can be done by adding the following to your
`.delivery/cli.toml`:

``` none
fips = true
fips_git_port = "OPEN_PORT"
fips_custom_cert_filename = "/full/path/to/your/certificate-chain.pem" # optional
```

Replace `OPEN_PORT` with any port that is free locally on localhost.

If you are using a custom certificate authority or a self-signed
certificate then you will need the third option. This file should
contain to the entire certificate chain in <span
class="title-ref">pem</span> format. See [FIPS Certificate
Management](/fips#certificate_management) for an example on how to
generate the file.

## How to enable FIPS mode for workstations

### Prerequisites

-   Supported Systems - Windows, CentOS and Red Hat Enterprise Linux

Now that FIPS mode is enabled in your `.delivery/cli.toml`, running any
project-specific Delivery CLI command will automatically use
FIPS-compliant encrypted git traffic between your workstation and the
Chef Automate server. As long as the Chef Automate server is in FIPS
mode, no other action is needed on your part to operate Delivery CLI in
FIPS mode. If you ever stop using FIPS mode on the Chef Automate server,
simply delete the above two lines from your `.delivery/cli.toml` file
and Delivery CLI will stop running in FIPS mode.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

You could also pass `--fips` and `--fips-git-port=OPEN_PORT` into
project specific commands if you do not wish to edit your
`.delivery/cli.toml`. See list of commands below for details..



</div>

</div>