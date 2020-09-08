+++
title = "Effortless Variables and Config"
draft = false

[menu]
  [menu.effortless]
    title = "Variables and Config"
    identifier = "effortless/variables_and_config.md Effortless Variables and Config"
    parent = "effortless"
    weight = 40
+++

# Plan Variables and Chef Habitat Configurations

This documents the options for both your plan and your `habitat` configuration file.

## Effortless Config

### Effortless Config Plan Variables

You can use all the default plan variables shipped with Chef Habitat. Read more about [plan variables](https://www.habitat.sh/docs/reference/#plan-variables) in the Chef Habitat documentation.

scaffold_chef_client
: The Chef Habitat `chef-infra-client` package used. Change to use a different package. Default is `chef/chef-infra-client`

scaffold_cacerts
: The Chef Habitat `cacerts` package used during the Chef Infra Client run. Change to use a different package.Default is `chef/cacerts`

scaffold_policyfile_path
: Path to the policyfile. Default is `$PLAN_CONTEXT/../policyfiles`

scaffold_data_bags_path
: Path to the `data_bags` directory that contains the `data_bags`, if needed by your cookbook. Default is `$PLAN_CONTEXT/../data_bags`

### Effortless Config Chef Habitat Settings

interval
: Frequency of how often in seconds that the Chef Infra Client runs. Default is `1800`

splay
: A randomly generated sleep time value in seconds added to the interval and helps determine when the chef-client runs again. Default is `1800`

splay_first_run
: Splay value in seconds for the first run of the Chef Infra Client. Default is `0`

run_lock_timeout
: Amount of time in seconds for the run lock timeout for the Chef Infra Client run. Default is `1800`

log_level
: Log level for the `chef-client`. Default is `warn`

env_path_prefix
: String that will be the Environment Path variable for the chef-client run. Linux Default is `/sbin:/usr/sbin:/usr/local/sbin:/usr/local/bin:/usr/bin:/bin`. Windows Default is `;C:/WINDOWS;C:/WINDOWS/system32/;C:/WINDOWS/system32/WindowsPowerShell/v1.0;C:/ProgramData/chocolatey/bin`

ssl_verify_mode
: SSL Verification mode for the Chef Infra Client. Default is `:verify_peer`

verify_api_cert
: Boolean option to determine if the API certification should be verified. Default is `false`

#### Effortless Config Chef License

This configuration needs to be under the `[chef_license]` block in the .toml file.

acceptance
: Determines the Chef license acceptance at run time. Required for Chef Infra Client. See Chef License [here](https://docs.chef.io/chef_license_accept/#accepting-the-chef-license). Default is `undefined`

#### Effortless Config Chef Automate

These configurations need to be under the `[automate]` block in the .toml file.

enable
: Enables or disables reporting to Chef Automate. Boolean. Default is `false`

server_url
: Chef Automate server URL. Example: `https://automate.example.com/data-collector/v0/`. Default is `https://<automate_url>`

token
: Chef Automate API token. Example: `GR4_yqRNUtWFVgnVh57GQL9Hh9I=`. Default is `<automate_token>`

## Effortless Audit

### Effortless Audit Plan Variables

scaffold_inspec_client
: The Chef Habitat `inspec` package. Change to use a different package. Default is `chef/inspec` This variable is required if the profile had a `depends` line for compliance in the `inspec.yml` example as shown below

scaffold_cacerts
: The Chef Habitat `cacerts` package during the Chef Infra Client run. Change to use a different package. Default is `chef/cacerts`. This variable is required if the profile had a `depends` line for compliance in the `inspec.yml` example as shown below
 
inspec.yml

```yml
depends:
  - name: cis-rhel7-level1-server
    compliance: admin/cis-rhel7-level1-server
```

scaffold_automate_server_url
: Points to the Chef Automate server needed to fetch profile dependencies from the Chef Automate Asset Store. Example: `https://automate.example.com`. Required if the profile uses a line for compliance in the `inspec.yml`

scaffold_automate_user
: Chef Automate user for the installed profile

scaffold_automate_token
: Chef Automate API token

### Effortless Audit Chef Habitat Variables

interval
: How often in seconds that `inspec` runs. Default is `1800`

splay
: A randomly generated sleep time value in seconds added to the interval and helps determine when the chef-client runs again. Default is `1800`

splay_first_run
: Splay value in seconds for the first run of the `inspec` client. Default is `0`

log_level
: Log level for the `inspec` client. Default is `warn`

#### Effortless Audit Chef License

This configuration needs to be under the `[chef_license]` block in the .toml file.

acceptance
: Determines the Chef license acceptance at run time. This setting is required for chef-client to run successfully. See Chef License [here](https://docs.chef.io/chef_license_accept/#accepting-the-chef-license). Default is `undefined`

#### Effortless Audit Chef Automate

These configurations need to be under the `[automate]` block in your toml file.

enable
: Enables or disables reporting to Chef Automate. Boolean. Default is `false`

server_url
: The Chef Automate server URL. Example: `https://automate.example.com/data-collector/v0/`. Default is `https://<automate_url>`

token
: Chef Automate API token. Example: `GR4_yqRNUtWFVgnVh57GQL9Hh9I=`. Default is `<automate_token>`

environment
: (Optional) Environment tag for the Chef InSpec report. Use this tag to help with filtering in Chef Automate.
