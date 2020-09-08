+++
title = "Release Notes: Chef Infra Client 12.0 - 16.4"
draft = false

aliases = ["/release_notes.html", "/release_notes_ohai.html", "/release_notes/"]

[menu]
  [menu.release_notes]
    title = "Chef Infra Client"
    identifier = "release_notes/release_notes_client.md Chef Infra Client"
    parent = "release_notes"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/release_notes.md)

Chef Infra Client is released on a monthly schedule with new releases
the first Wednesday of every month. Below are the major changes for each
release. For a detailed list of changes see the [Chef Infra Client
changelog](https://github.com/chef/chef/blob/master/CHANGELOG.md)

## What's New in 16.4

### Resource Updates

#### chef_client_systemd_timer

The `chef_client_systemd_timer` resource has been updated to prevent failures running the `:remove` action.

#### openssl resource

The various openssl_* resources were refactored to better report the changed state of the resource to Automate or other handlers.

#### osx_profile

The `osx_profile` resource has been refactored as a custom resource internally. This update also better reports the changed state of the resource to Automate or other handlers and no longer silently continues if the attempts to shellout fail.

#### powershell_package_source

The `powershell_package_source` resource no longer requires the `url` property to be set when using the `:unregister` action. Thanks for this fix [@kimbernator](https://github.com/kimbernator)!

#### powershell_script

The `powershell_script` resource has been refactored to better report the changed state of the resource to Automate or other handlers.

#### windows_feature

The `windows_feature` resource has been updated to allow installing features that have been removed if a source location is provided. Thanks for reporting this [@stefanwb](https://github.com/stefanwb)!

#### windows_font

The `windows_font` resource will no longer fail on newer releases of Windows if a font is already installed. Thanks for reporting this [@bmiller08](https://github.com/bmiller08)!

#### windows_workgroup

The `windows_workgroup` resource has been updated to treat the `password` property as a sensitive property. The value of the `password` property will no longer be shown in logs or handlers.

### Security

#### CA Root Certificates

The included `cacerts` bundle in Chef Infra Client has been updated to the 7-22-2020 release. This new release removes 4 legacy root certificates and adds 4 additional root certificates.

#### Reduced Dependencies

We've audited the included dependencies that we ship with Chef Infra Client to reduce the 3rd party code we ship. We've removed many of the embedded binaries that shipped with the client in the past, but were not directly used. We've also reduced the feature set built into many of the libraries that we depend on, and removed several Ruby gem dependencies that were no longer necessary. This reduces the future potential for CVEs in the product and reduces package size at the same time.

## What's New in 16.3.45

- Resolved failures negotiating protocol versions with the Chef Infra Server.
- Improved log output on Windows systems in the `hostname` resource.
- Added support to the `archive_file` resource for `pzstd` compressed files.

## What's New in 16.3.38

### Renamed Client Configuration Options

We took a hard look at many of the terms we've historically used throughout the Chef Infra Client configuration sub-system and came to the realization that we weren't living up to the words of our [Community Code of Conduct](https://community.chef.io/code-of-conduct/). From the code of conduct: "Be careful in the words that you choose. Be kind to others. Practice empathy". Terms such as blacklist and sanity don't meet that bar so we've chosen to rename these configuration options:

- `automatic_attribute_blacklist` -> `blocked_automatic_attributes`
- `default_attribute_blacklist` -> `blocked_default_attributes`
- `normal_attribute_blacklist` -> `blocked_normal_attributes`
- `override_attribute_blacklist` -> `blocked_override_attributes`
- `automatic_attribute_whitelist` -> `allowed_automatic_attributes`
- `default_attribute_whitelist` -> `allowed_default_attributes`
- `normal_attribute_whitelist` -> ``allowed_normal_attributes``
- `override_attribute_whitelist` -> `allowed_override_attributes`
- `enforce_path_sanity` -> `enforce_default_paths`

Existing configuration options will continue to function for now, but will raise a deprecation warning and will be removed entirely from a future release of Chef Infra Client.

### Chef InSpec 4.22.1

Chef InSpec has been updated from 4.21.1 to 4.22.1. This new release includes the following improvements:

- The `=` character is now allowed for command line inputs
- `apt-cdrom` repositories are now skipped when parsing out the list of apt repositories
- Faulty profiles are now reported instead of causing a crash
- Errors are no longer logged to stdout with the `html2` reporter
- macOS Big Sur is now correctly identified as macOS

### New Resources

#### windows_firewall_profile

The `windows_firewall_profile` allows you to `enable`, `disable`, or `configure` Windows Firewall profiles. For example, you can now set up default actions and configure rules for the `Public` profile using this single resource instead of managing your own PowerShell code in a `powershell_script` resource:

```ruby
windows_firewall_profile 'Public' do
  default_inbound_action 'Block'
  default_outbound_action 'Allow'
  allow_inbound_rules false
  display_notification false
  action :enable
end
```

For a complete guide to all properties and additional examples, see the [windows_firewall_profile documentation](https://docs.chef.io/resources/windows_firewall_profile).

### Resource Updates

#### build_essential

Log output has been improved in the `build_essential` resource when running on macOS systems.

#### chef_client_scheduled_task

The `chef_client_scheduled_task` resource no longer sets up the schedule task with invalid double quoting around the specified command. Thanks for reporting this issue [@tiobagio](https://github.com/tiobagio/).

#### execute

The `user` property in the `execute` resource can now accept user IDs as Integers.

#### git

The `git` resource will no longer fail if syncing a branch that already exists locally. Thanks for fixing this [@lotooo](https://github.com/lotooo/).

#### macos_user_defaults

The `macos_user_defaults` has received a ground-up refactoring with new actions, additional properties, and better overall reliability:

- Improved idempotency by properly loading the current state of domains.
- Improved how we set `dict` and `array` type data.
- Improved logging to show the existing key/value pair that is changed, and improved the property state data that the resource sends to handlers and/or Chef Automate.
- Fixed a failure when setting keys or values that included a space.
- Replaced the existing non-functional `global` property with a new default for the `domain` property. To set a key/value pair on the `NSGlobalDomain` domain, you can either set that value explicitly or just skip the `domain` property entirely and Chef Infra Client will default to `NSGlobalDomain`. The existing property has been marked as deprecated and we will ship a Cookstyle rule to detect cookbooks using this property in the future.
- Fixed the `type` property to only accept valid inputs. Previously typos or otherwise incorrect values would just be ignored resulting in unexpected behavior. This may cause failures in your codebase if you previously used incorrect values. We will be shipping a Cookstyle rule to detect and correct these values in the future.
- Added a new `delete` action to allow users to remove a key from a domain.
- Added a new `host` property that lets you set per-host values. If you set this to `:current` it sets the -currentHost flag.

#### windows_dns_record

The `windows_dns_record` resource includes a new optional property, `dns_server`, allowing you to make changes against remote servers. Thanks for this addition [@jeremyciak](https://github.com/jeremyciak/).

#### windows_package

A Chef Infra Client 16 regression within `windows_package` that prevented specifying `path` in the `remote_file_attributes` property has been resolved. Thanks for reporting this issue [@asvinours](https://github.com/asvinours/).

#### windows_security_policy

The `windows_security_policy` resource has been refactored to improve idempotency and improve log output when changes are made. You'll now see more complete change information in logs and any handler consuming this data will also receive more detailed change information.

### Knife Improvements

- Ctrl-C can now be used to exit knife even when being prompted for input.
- `knife bootstrap` will now properly error if attempting to bootstrap an AIX system using an account with an expired password.
- `knife profile` commands will no longer error if an invalid profile was previously set.
- The `-o` flag for `knife cookbook upload` can now be used on Windows systems.
- `knife ssh` now once again accepts legacy DSS host keys although we highly recommend upgrading to a more secure key algorithm if possible.
- Several changes were made to knife to that may prevent intermittent failures running cookbook commands

### Habitat Package Improvements

Habitat packages for Windows, Linux and Linux2 are now built and tested against each pull request to Chef Infra Client. Additionally we've improved how these packages are built to reduce the size of the package, which reduces network utilization when using the Effortless deployment pattern.

## What's New in 16.2.72

- Habitat packages for Chef Infra Client 16 are now published with full support for the `powershell_exec` helper now added.
- Added a new `clear` action to the `windows_user_privilege` resource.
- Resolved a regression in Chef Infra Client 16.1 and later that caused failures running on FIPS enabled systems.
- Resolved failures in the `archive_file` resource when running on Windows hosts.
- Resolved a failure when running `chef-apply` with the `-j` option. Thanks [@komazarari](https://github.com/komazarari).
- Chef Infra Client running within GitHub Actions is now properly identified as running in a Docker container. Thanks [@jaymzh](http://github.com/jaymzh).
- SSH connections are now reused, improving the speed of knife bootstrap and remote resources on slow network links. Thanks [@tecracer-theinen](https://github.com/tecracer-theinen).
- `node['network']['interfaces']` data now correctly identifies IPv6 next hops for IPv4 routes. Thanks [@cooperlees](https://github.com/cooperlees).
- Updated  InSpec from 4.20.10 to 4.21.1.

## What's New in 16.2.50

- Correctly identify the new macOS Big Sur (11.0) beta as platform "mac_os_x".
- Fix `knife config use-profile` to fail if an invalid profile is provided.
- Fix failures running the `windows_security_policy` resource.
- Update InSpec from 4.20.6 to 4.20.10.

## What's New in 16.2.44

### Breaking Change in Resources

In Chef Infra Client 16.0, we changed the way that custom resource names are applied in order to resolve some longstanding edge-cases. This change had several unintended side effects, so we're further changing how custom names are set in this release of Chef Infra Client.

Previously you could set a custom name for a resource via `resource_name` and under the hood this would also magically set the `provides` for the resource. Magic is great when it works, but is confusing when it doesn't. We've decided to remove some of this magic and instead rely on more explicit `provides` statements in resources. For cookbooks that support just Chef Infra Client 16 and later, you should change any `resource_name` calls to `provides` instead. If you need to support older releases of Chef Infra Client as well as 16+, you'll want to include both `resource_name` and `provides` for full compatibility.

**Pre-16 code:**

```ruby
resource_name :foo
```

**Chef Infra Client 16+ code**

```ruby
provides :foo
```

**Chef Infra Client < 16 backwards compatible code**

```ruby
resource_name :foo
provides :foo
```

We've introduced several Cookstyle rules to detect both custom resources and legacy HWRPs that need to be updated for this change:

**[ChefDeprecations/ResourceUsesOnlyResourceName](https://github.com/chef/cookstyle/blob/master/docs/cops_chefdeprecations.md#chefdeprecationsresourceusesonlyresourcename)**: detects resources that only set resource_name and automatically adds a provides call as well.

**[ChefDeprecations/HWRPWithoutProvides](https://github.com/chef/cookstyle/blob/master/docs/cops_chefdeprecations.md#chefdeprecationshwrpwithoutprovides)**: detects legacy HWRPs that don't include the necessary provides and resource_name calls for Chef Infra Client 16.

### Chef InSpec 4.20.6

Chef InSpec has been updated from 4.18.114 to 4.2.0.6. This new release includes the following improvements:

- Develop your own Chef InSpec Reporter plugins to control how Chef InSpec will report result data.
- The `inspec archive` command packs your profile into a `tar.gz` file that includes the profile in JSON form as the inspec.json file.
- Certain substrings within a `.toml` file no longer cause unexpected crashes.
- Accurate InSpec CLI input parsing for numeric values and structured data, which were previously treated as strings. Numeric values are cast to an `integer` or `float` and `YAML` or `JSON` structures are converted to a hash or an array.
- Suppress deprecation warnings on inspec exec with the `--silence-deprecations` option.

### New Resources

#### windows_audit_policy

The `windows_audit_policy` resource is used to configure system-level and per-user Windows advanced audit policy settings. See the [windows_audit_policy Documentation](/resources/windows_audit_policy/) for complete usage information.

For example, you can enable auditing of successful credential validation:

```ruby
windows_audit_policy "Set Audit Policy for 'Credential Validation' actions to 'Success'" do
  subcategory  'Credential Validation'
  success true
  failure false
  action :set
end
```

#### homebrew_update

The `homebrew_update` resource is used to update the available package cache for the Homebrew package system similar to the behavior of the `apt_update` resource. See the [homebrew_update Documentation](/resources/homebrew_update/) for complete usage information. Thanks for adding this new resource, [@damacus](http://github.com/damacus).

### Resource Updates

#### All resources now include umask property

All resources, including custom resources, now have a `umask` property which allows you to specify a umask for file creation. If not specified the system default will continue to be used.

#### archive_file

The `archive_file` resource has been updated with two important fixes. The resource will no longer fail with uninitialized constant errors under some scenarios. Additionally, the behavior of the `mode` property has been improved to prevent incorrect file modes from being applied to the decompressed files. Due to how file modes and Integer values are processed in Ruby, this resource will now produce a deprecation warning if integer values are passed. Using string values lets us accurately pass values such as '644' or '0644' without ambiguity as to the user's intent. Thanks for reporting these issues [@sfiggins](http://github.com/sfiggins) and [@hammerhead](http://github.com/hammerhead).

#### chef_client_scheduled_task

The `chef_client_scheduled_task` resource has been updated to default the `frequency_modifier` property to `30` if the `frequency` property is set to `minutes`, otherwise it still defaults to `1`. This provides a more predictable schedule behavior for users.

#### cron / cron_d

The `cron` and `cron_d` resources have been updated using the new Custom Resource Partials functionality introduced in Chef Infra Client 16. This has allowed us to standardize the properties used to declare cron job timing between the two resources. The timing properties in both resources all accept the same types and ranges, and include the same validation, which makes moving from `cron` to `cron_d` seamless.

#### cron_access

The `cron_access` resource has been updated to support Solaris and AIX systems. Thanks [@aklyachkin](http://github.com/aklyachkin).

#### execute

The `execute` resource has a new `input` property which allows you to pass `stdin` input to the command being executed.

#### powershell_package

The `powershell_package` resource has been updated to use TLS 1.2 when communicating with the PowerShell Gallery on Windows Server 2012-2016. Previously this resource used the system default cipher suite which did not include TLS 1.2. The PowerShell Gallery now requires TLS 1.2 for all communication, which caused failures on Windows Server 2012-2016. Thanks for reporting this issue [@Xorima](http://github.com/Xorima).

#### remote_file

The `remote_file` resource has a new property `ssl_verify_mode` which allows you to control SSL validation at the property level. This can be used to verify certificates (Chef Infra Client's defaults) with `:verify_peer` or to skip verification in the case of a self-signed certificate with `:verify_none`. Thanks [@jaymzh](http://github.com/jaymzh).

#### script

The various `script` resources such as `bash` or `ruby` now pass the provided script content to the interpreter using system pipes instead of writing to a temporary file and executing it. Executing script content using pipes is faster, more secure as potentially sensitive scripts aren't written to disk, and bypasses issues around user privileges.

#### snap_package

Multiple issues with the `snap_package` resource have been resolved, including an infinite wait that occurred, and issues with specifying the package version or channel. Thanks [@jaymzh](http://github.com/jaymzh).

#### zypper_repository

The `zypper_repository` resource has been updated to work with the newer release of GPG in openSUSE 15 and SLES 15. This prevents failures when importing GPG keys in the resource.

### Knife bootstrap updates

- Knife bootstrap will now warn when bootstrapping a system using a validation key. Users should instead use `validatorless bootstrapping` with `knife bootstrap` which generates node and client keys using the client key of the user bootstrapping the node. This method is far more secure as an org-wide validation key does not not need to be distributed or rotated. Users can switch to `validatorless bootstrapping` by removing any `validation_key` entries in their `config.rb (knife.rb)` file.
- Resolved an error bootstrapping Linux nodes from Windows hosts
- Improved information messages during the bootstrap process

### Platform Packages

- Debian 8 packages are no longer being produced as Debian 8 is now end-of-life.
- We now produce Windows 8 packages

## What's New in 16.1.16

This release resolves high-priority bugs in the 16.1 release of Chef Infra Client:

- Resolved a critical performance regression in the Rubygems release within Ruby 2.7, which was discovered by a Chef engineer.
- Resolved several Ruby 2.7 deprecation warnings.
- Added `armv6l` and `armv7l` architectures to the `arm?` and `armhf?` helpers
- Resolved failures in the Windows bootstrap script
- Resolved incorrect paths when bootstrapping Windows nodes

### Security Updates

#### openSSL

openSSL has been updated from 1.0.2u to 1.0.2v which does not address any particular CVEs, but includes multiple security hardening updates.

## What's New in 16.1

### Ohai 16.1

Ohai 16.1 includes a new `Selinux` plugin which exposes `node['selinux']['status']`, `node['selinux']['policy_booleans']`, `node['selinux']['process_contexts']`, and `node['selinux']['file_contexts']`. Thanks [@davide125](http://github.com/davide125) for this contribution. This new plugin is an optional plugin which is disabled by default. It can be enabled within your `client.rb`:

```ruby
ohai.optional_plugins = [ :Selinux ]
```

### Chef InSpec 4.18.114

InSpec has been updated from 4.18.111 to 4.18.114. This update adds new `--reporter_message_truncation` and `--reporter_backtrace_inclusion` reporter options to truncate messages and suppress backtraces.

### Debian 10 aarch64

Chef Infra Client packages are now produced for Debian 10 on the aarch64 architecture. These packages are available at [downloads.chef.io](https://downloads.chef.io/chef/).

### Bug Fixes

- Resolved a regression in the `launchd` resource that prevented it from converging.
- The `:disable` action in the `launchd` resource no longer fails if the plist was not found.
- Several Ruby 2.7 deprecation warnings have been resolved.

## What's New in 16.0.287

The Chef Infra Client 16.0.287 release includes important bug fixes for the Chef Infra Client 16 release:

- Fixes the failure to install Windows packages on the 2nd convergence of the Chef Infra Client.
- Resolves several failures in the `launchd` resource.
- Removes an extra `.java` file on Windows installations that would cause a failure in the IIS 8.5 Server Security Technical Implementation Guide audit.
- Updates the `windows_printer` resource so that the driver property will only be required when using the `:create` action.
- Fixes the incorrectly spelled `knife user invite recind` command to be `knife user invite rescind`.
- Update Chef InSpec to 4.8.111 with several minor improvements.

## What's New in 16.0.275

The Chef Infra Client 16.0.275 release includes important regression fixes for the Chef Infra Client 16 release:

- Resolved failures when using the `windows_package` resource. Thanks for reporting this issue [@cookiecurse](https://github.com/cookiecurse).
- Resolved log warnings when running `execute` resources.
- The appropriate `cron` or `cron_d` resource call is now called when using the `:delete` action in chef_client_cron. Thanks for reporting this issue [jimwise](https://github.com/jimwise).
- The `chef_client_cron` resource now creates the log directory with `750` permissions not `640`. Thanks for this fix [DhaneshRaghavan](https://github.com/DhaneshRaghavan).
- The `knife yaml convert` command now correctly converts symbol values.
- The `sysctl`, `apt_preference`, and `cron_d` remove actions no longer fail with missing property warnings.

## What's New in 16.0

### Breaking Changes

#### Log Resource Notification Behavior

The `log` resource in a recipe or resource will no longer trigger notifications by default. This allows authors to more liberally use `log` resources without impacting the updated resources count or impacting reporting to Chef Automate. This change will impact users that used the `log` resource to aggregate notifications from other resources, so they could limit the number of times a notification would fire. If you used the `log` resource to aggregate multiple notifications, you should convert to using the `notify group` resource, which was introduced in Chef Infra Client 15.8.

Example of notification aggregation with `log` resource:

```ruby
template '/etc/foo' do
  source 'foo.erb'
  notifies :write, 'log[Aggregate notifications using a single log resource]', :immediately
end

template '/etc/bar' do
  source 'bar.erb'
  notifies :write, 'log[Aggregate notifications using a single log resource]', :immediately
end

log 'Aggregate notifications using a single log resource' do
  notifies :restart, 'service[foo]', :delayed
end
```

Example of notification aggregation with `notify_group` resource:

```ruby
template '/etc/foo' do
  source 'foo.erb'
  notifies :run, 'notify_group[Aggregate notifications using a single notify_group resource]', :immediately
end

template '/etc/bar' do
  source 'bar.erb'
  notifies :run, 'notify_group[Aggregate notifications using a single notify_group resource]', :immediately
end

notify_group 'Aggregate notifications using a single notify_group resource' do
  notifies :restart, 'service[foo]', :delayed
end
```

The `ChefDeprecations/LogResourceNotifications` cop in Cookstyle 6.0 and later detects using the `log` resource for notifications in cookbooks.

To restore the previous behavior, set `count_log_resource_updates true` in your `client.rb`.

#### HWRP Style Resources Now Require resource_name / provides

Legacy HWRP-style resources, written as Ruby classes in the libraries directory of a cookbook, will now require either the use of `resource_name` or `provides` methods to define the resource names. Previously, Chef Infra Client would infer the desired resource name from the class, but this magic was problematic and has been removed.

The `ChefDeprecations/ResourceWithoutNameOrProvides` cop in Cookstyle 6.0 and later detects this deprecation.

#### build_essential GCC Updated on Solaris

On Solaris systems, we no longer constrain the version of GCC to 4.8.2 in the `build_essential` resource to allow for GCC 5 installations.

#### git Resource Branch Checkout Changes

The `git` resource no longer checks out to a new branch named `deploy` by default. Many users found this branching behavior confusing and unexpected so we've decided to implement a more predictable default. The resource will now default to either checking out the branch specified with the `checkout_branch` property or a detached HEAD state. If you'd like to revert to the previous behavior you can set the `checkout_branch` to `deploy`.

#### s390x Packaging

As outlined in our blog post at <https://blog.chef.io/chef-infra-end-of-life-announcement-for-linux-client-on-ibm-s390x-architecture/>, we will no longer be producing s390x platform packages for Chef Infra Client.

#### filesystem2 Node Data Replaces filesystem on FreeBSD / AIX / Solaris

In Chef Infra Client 14 we introduced a modernized filesystem layout of Ohai data on FreeBSD, AIX, and Solaris at `node['fileystem2']`. With the release of 16.0, we are now replacing the existing data at `node['filesystem']` with this updated filesystem data. This data has a standardized format that matches Linux and macOS data to make it easier to write cross-platform cookbooks. In a future release of Chef Infra Client we'll remove the `node['filesystem2']` as we complete this migration.

#### required: true on Properties Now Behaves As Expected

The behavior of `required: true` has been changed to better align with the expected behavior. Previously, if you set a property `required: true` on a custom resource property and did not explicitly reference the property in an action, then Chef Infra Client would not raise an exception. This meant many users would add their own validation to raise for resources they wanted to ensure they were always set. `required: true` will now properly raise if a property has not been set.

We have also expanded the `required` field for added flexibility in defining exactly which actions a property is required for. See [Improved property require behavior](#improved-property-require-behavior) below for more details.

#### Removal of Legacy metadata.rb depends Version Constraints

Support for the `<<` and `>>` version constraints in metadata.rb has been removed. This was an undocumented feature from the Chef 0.10 era, which is not used in any cookbooks on the Supermarket. We are mentioning it since it is technically a breaking change, but it unlikely that this change will be impacting.

Examples:

```ruby
depends 'windows', '<< 1.0'
depends 'windows', '>> 1.0'
```

#### Logging Improvements May Cause Behavior Changes

We've made low level changes to how logging behaves in Chef Infra Client that resolves many complaints we've heard of the years. With these change you'll now see the same logging output when you run `chef-client` on the command line as you will in logs from a daemonzed client run. This also corrects often confusing behavior where running `chef-client` on the command line would log to the console, but not to the log file location defined your `client.rb`. In that scenario you'll now see logs in your console and in your log file. We believe this is the expected behavior and will mean that your on-disk log files can always be the source of truth for changes that were made by Chef Infra Client. This may cause unexpected behavior changes for users that relied on using the command line flags to override the `client.rb` log location. If you have daemons running that log using the command line options you want to make sure that `client.rb` log location either matches or isn't defined.

#### Red Hat / CentOS 6 Systems Require C11 GCC for Some Gem Installations

The included release of Ruby in Chef Infra Client 16 now requires a [C99](https://en.wikipedia.org/wiki/C99) compliant compiler when using the `chef_gem` resource with gems that require compilation. Some systems, such as RHEL 6, do not ship with a C99 compiler and will fail if the gems they're attempting to install require compilation. If it is necessary to install compiled gems into the Chef Infra Client installation on one of these systems you can upgrade to a modern GCC release.

CentOS:

```bash
yum install centos-release-scl
yum install devtoolset-7
scl enable devtoolset-7 bash
```

Red Hat:

```bash
yum-config-manager --enable rhel-server-rhscl-7-rpms
yum install devtoolset-7
scl enable devtoolset-7 bash
```

#### Changes to Improve Gem Source behavior

We've improved the behavior for those that use custom rubygem sources, particularly those operating in air-gapped installations. These improvements involved changes to many of the default `client.rb` values and `gem_package`/`chef_gem` properties that require updating your usage of `chef_gem` and `gem_package` resources

The default value of the `clear_sources` property of `gem_package` and `chef_gem` resources has been changed to `nil`. The possible behaviors for clear_sources are now:

- `true`: Always clear sources.
- `false`: Never clear sources.
- `nil`: Clear sources if `source` property is set, but don't clear sources otherwise.

The default value of the `include_default_source` property of `gem_package` and `chef_gem` resources has been changed to `nil`. The possible behaviors for include_default_source are now:

- `true`: Always include the default source.
- `false`: Never include the default source.
- `nil`: Include the default source if `rubygems_url` `client.rb` value is set or if `source` and `clear_sources` are not set on the resource.

The default values of the `rubygems_url` `client.rb` config option has been changed to `nil`. Setting to nil previously had similar behavior to setting `clear_sources` to true, but with some differences. The new behavior is to always use `https://rubygems.org` as the default rubygems repo unless explicitly changed, and whether to use this value is determined by `clear_sources` and `include_default_source`.

#### Behavior Changes in Knife

**knife status --long uses cloud attribute**

The `knife status --long` resource now uses Ohai's cloud data instead of ec2 specific data. This improves, but changes, the data output for users on non-AWS clouds.

**knife download role/environment format update**

The `knife download role` and `knife download environment` commands now include all possible data fields including those without any data set. This new output behavior matches the behavior of other commands such as `knife role show` or `knife environment show`

**Deprecated knife cookbook site command removed**

The previously deprecated `knife cookbook site` commands have been removed. Use the `knife supermarket` commands instead.

**Deprecated knife data bag create -s short option removed**

The deprecated `knife data bag create -s` option that was not properly honored has been removed. Use the `--secret` option instead to set a data bag secret file during data bag creation.

**sites-cookbooks directory no longer in cookbook_path**

The legacy `sites-cookbooks` directory is no longer added to the default `cookbook_path` value. With this change, any users with a legacy `sites-cookbooks` directory will need to use the `-O` flag to override the cookbook directory when running commands such as `knife cookbook upload`.

If you have a repository that contains a `site-cookbooks` directory, we highly recommend using Policyfiles or Berkshelf to properly resolve these external cookbook dependencies without the need to copy them locally. Alternatively, you can move the contents of this folder into your main cookbook directory and they will continue to be seen by knife commands.

### New Resources

#### alternatives

Use the `alternatives` resource to manage symbolic links to specify default command versions on Linux hosts. See the [alternatives documentation](https://docs.chef.io/resources/alternatives/) for full usage information. Thanks [@vkhatri](https://github.com/vkhatri) for the original cookbook alternatives resource.

#### chef_client resources

We've added new resources to Chef Infra Client for setting the client to run on an interval using native system schedulers. We believe that these native schedulers provide a more flexible and reliable method for running the client than the traditional method of running as a full service. Using the native schedulers reduces hung clients and eases upgrades. This is the first of many steps towards removing the need for the `chef-client` cookbook and allowing Chef Infra Client to configure itself out of the box.

**chef_client_cron**

Use the `chef_client_cron` resource to setup the Chef Infra Client to run on a schedule using cron on Linux, Solaris, and AIX systems. See the [chef_client_cron documentation](https://docs.chef.io/resources/chef_client_cron/) for full usage information.

**chef_client_systemd_timer**

Use the `chef_client_systemd_timer` resource to setup the Chef Infra Client to run on a schedule using a systemd timer on systemd based Linux systems (RHEL 7+, Debian 8+, Ubuntu 16.04+ SLES 12+). See the [chef_client_systemd_timer documentation](https://docs.chef.io/resources/chef_client_systemd_timer/) for full usage information.

**chef_client_scheduled_task**

Use the `chef_client_scheduled_task` resource to setup the Chef Infra Client to run on a schedule using Windows Scheduled Tasks. See the [chef_client_scheduled_task documentation](https://docs.chef.io/resources/chef_client_scheduled_task) for full usage information.

#### plist

Use the `plist` resource to generate plist files on macOS hosts. See the [plist documentation](https://docs.chef.io/resources/plist/) for full usage information. Thanks Microsoft and [@americanhanko](https://github.com/americanhanko) for the original work on this resource in the [macos cookbook](https://supermarket.chef.io/cookbooks/macos).

#### user_ulimit

Use the `user_ulimit` resource to set per user ulimit values on Linux systems. See the [user_ulimit documentation](https://docs.chef.io/resources/user_ulimit/) for full usage information. Thanks [@bmhatfield](https://github.com/bmhatfield) for the original work on this resource in the [ulimit cookbook](https://supermarket.chef.io/cookbooks/ulimit).

#### windows_security_policy

Use the `windows_security_policy` resource to modify location security policies on Windows hosts. See the [windows_security_policy documentation](https://docs.chef.io/resources/windows_security_policy/) for full usage information.

#### windows_user_privilege

Use the `windows_user_privilege` resource to add users and groups to the specified privileges on Windows hosts. See the [windows_user_privilege documentation](https://docs.chef.io/resources/windows_user_privilege/) for full usage information.

### Improved Resources

#### compile_time on all resources

The `compile_time` property is now available for all resources so that they can be set to run at compile time without the need to force the action.

Set the `compile_time` property instead of forcing the resource to run at compile time:

```ruby
  my_resource "foo" do
    action :nothing
  end.run_action(:run)
```

With the simpler `compile_time` property:

```ruby
  my_resource "foo" do
    compile_time true
  end
```

#### build_essential

The `build_essential` resource includes a new `:upgrade` action for macOS systems that allows you to install updates to the Xcode Command Line Tools available via Software Update.

#### cron

The `cron` resource has been updated to use the same property validation for cron times that the `cron_d` resource uses. This improves failure messages when invalid inputs are set and also allows for `jan`-`dec` values to be used in the `month` property.

#### dnf_package

The `dnf_package` resource, which provides `package` under the hood on any system shipping with DNF, has been greatly refactored to resolve multiple issues. The version behavior and overall resource capabilities now match that of the `yum_package` resource.

- The `:lock` action now works on RHEL 8.
- Fixes to prevent attempting to install the same package during each Chef Infra Client run.
- Resolved several idempotency issues.
- Resolved an issue where installing a package with `options '--enablerepo=foo'` would fail.

#### git

The `git` resource now fully supports why-run mode and no longer checks out the `deploy` branch by default as mentioned in the breaking changes section.

#### locale

The `locale` resource now supports setting the system locale on Windows hosts.

#### msu_package resource improvements

The `msu_package` resource has been improved to work better with Microsoft's cumulative update packages. Newer releases of these cumulative update packages will not correctly install over the previous versions. We also extended the default timeout for installing MSU packages to 60 minutes. Thanks for reporting the timeout issue, [@danielfloyd](https://github.com/danielfloyd).

#### package

The `package` resource on macOS and Arch Linux systems now supports passing multiple packages into a single package resource via an array. This allows you to collapse multiple resources into a single resource for simpler cookbook authoring, which is significantly faster as it requires fewer calls to the packaging systems. Thanks for the Arch Linux support, [@ingobecker](https://github.com/ingobecker)!

Using multiple resources to install a package:

```ruby
package 'git'
package 'curl'
package 'packer'
```

or

```ruby
%w(git curl packer).each do |pkg|
  package pkg
end
```

can now be simplified to:

```ruby
package %w(git curl packer)
```

#### service

The `service` resource has been updated to support newer releases of `update-rc.d` so that it properly disables sys-v init services on Debian Linux distributions. Thanks [@robuye](https://github.com/robuye)!

#### windows_firewall_rule

The `windows_firewall_rule` resource has been greatly improved thanks to work by [@pschaumburg](https://github.com/pschaumburg) and [@tecracer-theinen](https://github.com/tecracer-theinen).

- New `icmp_type` property, which allows setting the ICMP type when setting up ICMP protocol rules.
- New `displayname` property, which allows defining the display name of the firewall rule.
- New `group` property, which allows you to specify that only matching firewall rules of the indicated group association are copied.
- The `description` property will now update if changed.
- Fixed setting rules with multiple profiles.

#### windows_package

The `windows_package` resource now considers `3010` to be a valid exit code by default. The `3010` exit code means that a package has been successfully installed, but requires a reboot.

**knife-acl is now built-in**

The `knife-acl` gem is now part of Chef Infra Client. This gives you the ability to manage Chef organizations and ACLs directly.

### YAML Recipes

We added support for writing recipes in YAML to provide a low-code syntax for simple use cases. To write recipes in YAML, Chef resources and any user-defined parameters can be added as elements in a `resources` hash, such as the example below:

```yaml
---
resources:
  - type: "package"
    name: "httpd"
  - type: "template"
    name: "/var/www/html/index.html"
    source: "index.html.erb"
  - type: "service"
    name: "httpd"
    action:
      - enable
      - start
```

This implementation is restrictive and does not support arbitrary Ruby code, helper functions, or attributes. However, if the need for additional customization arises, YAML recipes can be automatically converted into the DSL via the `knife yaml convert` command.

### Custom Resource Improvements

#### Improved property require behavior

As noted in the breaking changes above, we improved how the required value is set on custom resource properties, in order to give a more predictable behavior. This new behavior now allows you to specify actions where individual properties are required. This is especially useful when `:create` actions require certain properties that may not be required for a `:remove` type property.

Example required field defining specific actions:

```ruby
property :password, String, required: [:create]

action :create do
  # code to create something
end

action :remove do
  # code to remove it that doesn't need a password
end
```

#### Resource Partials

Resource partials allow you to define reusable portions of code that can be included in multiple custom resources. This feature is particularly useful when there are common properties, such as authentication properties, that you want to define in a single location, but use for multiple resources. Internally in the Chef Infra Client codebase, we have already used this feature to remove duplicate properties from our `subversion` and `git` resources and make them easier to maintain.

Resource partials are stored in a cookbook's `/resources` directory just like existing custom resources, but they start with the `_` prefix. They're then called using a new `use` helper within the resource where they're needed:

`resources/_api_auth_properties.rb:`

```ruby
property :api_endpoint, String
property :api_key, String
property :api_retries, Integer
```

`resources/mything.rb`:

```ruby
property :another_property, String
property :yet_another_property, String

use 'api_auth_properties'

action :create do
  # some create logic
end
```

The example above shows a resource partial that contains properties for use in multiple resources. You can also use resource partials to define helper methods that you want to use in your actions instead of defining the same helper methods in each action_class.

`resources/_api_auth_helpers.rb:`

```ruby
def make_api_call(endpoint, value)
  # API call code here
end
```

`resources/mything.rb`:

```ruby
property :another_property, String
property :yet_another_property, String

action :create do
  # some create logic
end

action_class do
  use 'api_auth_helpers'
end
```

#### after_resource

A new `after_resource` state has been added to resources that allows you to better control the resource state information reported to Chef Automate when a resource converges. If your custom resource uses the `load_current_value` helper, then this after state is calculated automatically. If you don't utilize the `load_current_value` helper and would like fine grained control over the state information sent to Chef Automate, you can use a new `load_after_resource` helper to load the state of each property for reporting.

#### identity Improvements

A resource's name property is now set to be the identity property by default and to have `desired_state: false` set by default. This eliminates the need to set `identity: true, desired_state: false` on these properties and better exposes identity data to handler and reporting.

#### compile_time property

The `compile_time` property is now defined for all custom resources,  so there is no need to add your own compile-time logic to your resource.

### Other Improvements

#### Up to 33% smaller on disk

We optimized the files that ship with Chef Infra Client and eliminated many unnecessary files from the installation, reducing the on-disk size of Chef Infra Client by up to 33%.

#### Windows Performance Improvements

We've optimized the Chef Infra Client for modern Windows releases and improved the performance on these systems.

#### Simpler Version Comparisons with node['platform_version']

The `node['platform_version']` attribute returned from Ohai can now be intelligently compared as a version instead of as a String or Integer. Previously, to compare the platform_version, many users would first convert the version String to a Float with `node['platform_version']`. This introduced problems on many platforms, such as macOS, where macOS 10.9 would appear to be a greater version number than 10.15. You can now directly compare the version without converting it first.

Greater than or equal comparison:

```ruby
node['platform_version'] >= '10.15'
```

Comparison using Ruby's pessimistic operator:

```ruby
node['platform_version'] =~ '~> 10.15'
```

#### New helpers for recipes and resources

Several helpers introduced in Chef Infra Client 15.5 are now available for use in any resource or recipe. These helpers include:

`sanitized_path`

`sanitize_path` is a cross-platform method that returns the system's path along with the Chef Infra Client Ruby bin dir / gem bin dir and common system paths such as `/sbin` and `/usr/local/bin`.

`which(foo)`

The `which` helper searches the system's path and returns the first occurrence of a binary, similar to the `which` command on *nix systems. It also allows you to pass an `extra_path` value for additional directories to search.

```ruby
which('systemctl')
```

```ruby
which('my_app', extra_path: '/opt/my_app/bin')
```

#### eager_load_libraries metadata.rb setting

By default, Chef Infra Client eagerly loads all ruby files in each cookbook's libraries directory at runtime. A new metadata.rb option `eager_load_libraries` has been introduced and allows you to control if and when a cookbook library is loaded. Depending on the construction of your libraries, this new option may greatly improve the runtime performance of your cookbook. With eager loading disabled, you may manually load libraries included in your cookbook using Ruby's standard `require` method. Metadata.rb configuration options:

```ruby
eager_load_libraries false # disable eager loading all libraries
eager_load_libraries 'helper_library.rb' # eager load just the file helper_library.rb
eager_load_libraries %w(helper_library_1.rb helper_library_2.rb) # eager load both helper_library_1.rb and helper_library_2.rb files
```

Note: Unless you are experiencing performance issues in your libraries, we advise against changing the loading behavior.

#### always_dump_stacktrace client.rb option

A new `always_dump_stacktrace` client.rb configuration option and command line option allows you to have any Ruby stacktraces from Chef Infra Client logged directly to the log file. This may help troubleshooting when used in conjunction with centralized logging systems such as Splunk. To enable this new option, run `chef-client --always-dump-stacktrace` or add the following to your `client.rb`:

```ruby
always_dump_stacktrace true
```

#### Chef Vault Functionality Out of the Box

Chef Infra Client now ships with built-in Chef Vault functionality, so there's no need to depend on the `chef-vault` cookbook or gem. Chef Vault helpers `chef_vault_item`, `chef_vault`, and `chef_vault_item_for_environment` are included, as well as the `chef_vault_secret` resource. Additionally, the Chef Vault knife commands are also available out of the box. We do not recommend new users adopt the Chef Vault workflow due to limitations with autoscaling new systems, so these resources should only be consumed by existing Chef Vault users.

#### Ruby 2.7

Chef Infra Client's ruby installation has been updated to from Ruby 2.6 to Ruby 2.7, which includes many features available for use in resources and libraries.

See <https://medium.com/rubyinside/whats-new-in-ruby-2-7-79c98b265502> for details on many of the new features.

#### Ohai 16 Improvements

Ohai has been improved to gather additional system configuration information for use when authoring recipes and resources.

**filesystem2 Node Data available on Windows**
In previous Chef Infra Clients we've introduced a modernized filesystem layout of Ohai data for many platforms. In Chef Infra Client 16.0, Windows now has this layout available in `node['filesystem2']`. In Chef Infra Client 17, it will replace `node['filesystem']` to match all other platforms.
**Extended Azure Metadata**

The `Azure` Ohai plugin now gathers the latest version of the metadata provided by the Azure metadata endpoint. This greatly expands the information available on Azure instances. See [Ohai PR 1427](https://github.com/chef/ohai/pull/1427) for an example of the new data gathered.

**New Ohai Plugins**

New `IPC` and `Interupts` plugins have been added to Ohai. The IPC plugin exposes SysV IPC shmem information and interupts plugin exposes data from `/proc/interrupts` and `/proc/irq`. Thanks [@jsvana](https://github.com/jsvana) and [@davide125](https://github.com/davide125) for these new plugins.

Note: Both `IPC` and `Interupts` plugins are optional plugins, which are disabled by default. They can be enabled via your `client.rb`:

```ruby
ohai.optional_plugins = [
  :IPC,
  :Interupts
]
```

**Improved Linux Network Plugin Data**

The Linux Network plugin has been improved to gather additional information from the `ethtool` utility. This includes the number of queues (`ethtool -l`), the coalesce parameters (`ethtool -c`), and information about the NIC driver (`ethtool -i`). Thanks [@matt-c-clark](https://github.com/matt-c-clark) for these improvements.

**Windows DMI plugin**

Windows systems now include a new `DMI` plugin which presents data in a similar format to the `DMI` plugin on *nix systems. This makes it easier to detect system information like manufacturer, serial number, or asset tag number in a cross-platform way.

### New Platforms

Over the last quarter, we worked to greatly expand the platforms that we support with the addition of Chef Infra Client packages for Ubuntu 20.04 amd64, Amazon Linux 2 x86_64/aarch64, and Debian 10 amd64. With the release of Chef Infra Client 16, we expanded our platform support again with the following new platforms:

- RHEL 8 aarch64
- Ubuntu 20.04 aarch64
- SLES 16 aarch64

### Newly Introduced Deprecations

Several legacy Windows helpers have been deprecated as they will always return true when running on Chef Infra Client's currently supported platforms. The helpers previously detected systems prior to Windows 2012 and systems running Windows Nano, which has been discontinued by Microsoft. These helpers were never documented externally so their usage is most likely minimal. A new Cookstyle rule has been introduced to detect the usage of `older_than_win_2012_or_8?`: [ChefDeprecations/DeprecatedWindowsVersionCheck](https://github.com/chef/cookstyle/blob/master/docs/cops_chefdeprecations.md#chefdeprecationsdeprecatedwindowsversioncheck).

- Chef::Platform.supports_msi?
- Chef::Platform.older_than_win_2012_or_8?
- Chef::Platform.supports_powershell_execution_bypass?
- Chef::Platform.windows_nano_server?

## What's New In 15.13

### Chef InSpec 4.22.1

Chef InSpec has been updated from 4.20.6 to 4.22.1. This new release includes the following improvements:

- `apt-cdrom` repositories are now skipped when parsing out the list of apt repositories
- Faulty profiles are now reported instead of causing a crash
- Errors are no longer logged to stdout with the `html2` reporter
- macOS Big Sur is now correctly identified as macOS
- macOS/BSD support added to the interface resource along with new `ipv4_address`, `ipv4_addresses`, `ipv4_addresses_netmask`, `ipv4_cidrs`, `ipv6_addresses`, and `ipv6_cidrs` properties

### Fixes and Improvements

- Support for legacy DSA host keys has been restored in `knife ssh` and `knife bootstrap` commands.
- The collision warning error message when a cookbook includes a resource that now ships in Chef Infra Client has been improved to better explain the issue.
- Package sizes have been reduced with fewer installed files on disk.
- The `archive_file` resource now supports `pzstd` compressed files.

### New Deprecations

Chef Infra Client 16.2 and later require `provides` when assigning a name to a custom resource. In order to prepare for Chef Infra Client 16, make sure to include both `resource_name` and `provides` in resources when specifying a custom name.

## What's New In 15.12

### Chef InSpec 4.20.6

Chef InSpec has been updated from 4.18.114 to 4.2.0.6. This new release includes the following improvements:

- Develop your own Chef InSpec Reporter plugins to control how Chef InSpec will report result data.
- The `inspec archive` command packs your profile into a `tar.gz` file that includes the profile in JSON form as the inspec.json file.
- Certain substrings within a `.toml` file no longer cause unexpected crashes.
- Accurate InSpec CLI input parsing for numeric values and structured data, which were previously treated as strings. Numeric values are cast to an `integer` or `float` and `YAML` or `JSON` structures are converted to a hash or an array.
- Suppress deprecation warnings on `inspec exec` with the `--silence-deprecations` option.

### Resource Updates

#### archive_file

The `archive_file` resource has been updated with two important fixes. The resource will no longer fail with uninitialized constant errors under some scenarios. Additionally, the behavior of the `mode` property has been improved to prevent incorrect file modes from being applied to the decompressed files. Due to how file modes and Integer values are processed in Ruby, this resource will now produce a deprecation warning if integer values are passed. Using string values lets us accurately pass values such as '644' or '0644' without ambiguity as to the user's intent. Thanks for reporting these issues [@sfiggins](http://github.com/sfiggins) and [@hammerhead](http://github.com/hammerhead).

#### cron_access

The `cron_access` resource has been updated to support Solaris and AIX systems. Thanks [@aklyachkin](http://github.com/aklyachkin).

#### msu_package resource improvements

The `msu_package` resource has been improved to work better with Microsoft's cumulative update packages. Newer releases of these cumulative update packages will not correctly install over the previous versions. We also extended the default timeout for installing MSU packages to 60 minutes. Thanks for reporting the timeout issue [@danielfloyd](https://github.com/danielfloyd).

#### powershell_package

The `powershell_package` resource has been updated to use TLS 1.2 when communicating with the PowerShell Gallery on Windows Server 2012-2016. Previously, this resource used the system default cipher suite which did not include TLS 1.2. The PowerShell Gallery now requires TLS 1.2 for all communication, which caused failures on Windows Server 2012-2016. Thanks for reporting this issue [@Xorima](http://github.com/Xorima).

#### snap_package

Multiple issues with the `snap_package` resource have been resolved, including an infinite wait that occurred and issues with specifying the package version or channel. Thanks [@jaymzh](http://github.com/jaymzh).

#### zypper_repository

The `zypper_repository` resource has been updated to work with the newer release of GPG in openSUSE 15 and SLES 15. This prevents failures when importing GPG keys in the resource.

### Knife bootstrap updates

- Knife bootstrap will now warn when bootstrapping a system using a validation key. Users should instead use `validatorless bootstrapping` with `knife bootstrap` which generates node and client keys using the client key of the user bootstrapping the node. This method is far more secure as an org-wide validation key does not not need to be distributed or rotated. Users can switch to `validatorless bootstrapping` by removing any `validation_key` entries in their `config.rb (knife.rb)` file.
- Resolved an error bootstrapping Linux nodes from Windows hosts
- Improved information messages during the bootstrap process

### SSH Improvements

The `net-ssh` library used by the `knife ssh` and `knife bootstrap` commands has been updated bringing improvements to SSH connectivity:

- Support for additional key exchange and transport algorithms
- Support algorithm subtraction syntax in the `ssh_config` file
- Support empty lines and comments in `known_hosts` file

### Initial macOS Big Sur Support

Chef Infra Client now correctly detects macOS Big Sur (11.0) beta as being platform "mac_os_x". Chef Infra Client 15.12 has not been fully qualified for macOS Big Sur, but we will continue to validate against this release and provide any additional support updates.

### Platform Packages

- Debian 8 packages are no longer being produced as Debian 8 is now end-of-life.
- We now produce Windows 8 packages

## What's New In 15.11

### Bootstrapping Bugfixes

This release of Chef Infra Client resolves multiple issues when using `knife bootstrap` to bootstrap new nodes to a Chef Infra Server:

- Bootstrapping from a Windows host to a Linux host with an ED25519 ssh key no longer fails
- Resolved failures in the Windows bootstrap script
- Incorrect paths when bootstrapping Windows nodes have been resolved

### Chef InSpec 4.18.114

Chef InSpec was updated from 4.18.104 to 4.18.114 with the following improvements:

- Added new `--reporter_message_truncation` and `--reporter_backtrace_inclusion` reporter options to truncate messages and suppress backtraces.
- Fixed a warning when an input is provided
- Inputs and controls can now have the same name

### Resource Improvements

#### windows_firewall

The `windows_firewall` resource has been updated to support firewall rules that are associated with more than one profile. Thanks [@tecracer-theinen](https://github.com/tecracer-theinen).

#### chocolatey_package

The `chocolatey_package` resource has been updated to properly handle quotes within the `options` property. Thanks for reporting this issue [@dave-q](https://github.com/dave-q).

### Platform Support

#### Additional aarch64 Builds

Chef Infra Client is now tested on Debian 10, SLES 15, and Ubuntu 20.04 on the aarch64 architecture with packages available on the [Chef Downloads Page](https://downloads.chef.io/chef).

### Security Updates

#### openSSL

openSSL has been updated from 1.0.2u to 1.0.2v which does not address any particular CVEs, but includes multiple security hardening updates.

## What's New in 15.10

### Improvements

- The `systemd_unit` resource now respects the `sensitive` property and will no longer output the contents of the unit file to logs if this is set.
- A new `arm?` helper has been added which can be used in recipes and resources to determine if a system is on the ARM architecture.

### Bug Fixes

- Resolved a bug that prevented users from bootstrapping nodes using knife when specifying the `--use_sudo_password`.
- Resolved a bug that prevented the `--bootstrap-version` flag from being honored when bootstrapping in knife.

### Chef InSpec 4.18.104

- Resolved a regression that prevented the `service` resource from working correctly on Windows. Thanks [@Axuba](https://github.com/Axuba)
- Implemented VMware and Hyper-V detection on Linux systems
- Implemented VMware, Hyper-V, Virtualbox, KVM and Xen detection on Windows systems
- Added helpers `virtual_system?` and `physical_system?`. Thanks [@tecracer-theinen](https://github.com/tecracer-theinen)

### Ohai 15.9

- Improve the resiliency of the `Shard` plugin when `dmidecode` cannot be found on a system. Thanks [@jaymzh](https://github.com/jaymzh)
- Fixed detection of Openstack guests via DMI data. Thanks [@ramereth](https://github.com/ramereth)

### Platform Support

#### Amazon Linux 2

Chef Infra Client is now tested on Amazon Linux 2 running on x86_64 and aarch64 with packages available on the [Chef Downloads Page](https://downloads.chef.io/chef).

## What's New in 15.9

### Chef InSpec 4.18.100

Chef InSpec has been updated from 4.18.85 to 4.18.100:

- Resolved several failures in executing resources
- Fixed `auditd` resource processing of action and list
- Fixed platform detection when running in Habitat
- "inspec schema" has been revised to be in the JSON Schema draft 7 format
- Improved the functionality of the `oracledb_session` resource

### Ohai 15.8

Ohai has been updated to 15.8.0 which includes a fix for failures that occurred in the OpenStack plugin (thanks [@sawanoboly](https://github.com/sawanoboly/)) and improved parsing of data in the `optional_plugins` config option (thanks [@salzig](https://github.com/salzig/)).

### Resource Improvements

#### build_essential

The `build_essential` resource has been updated to better detect if the Xcode CLI Tools package needs to be installed on macOS. macOS 10.15 (Catalina) is now supported with this update. Thank you [@w0de](https://github.com/w0de/) for kicking this work off, [@jazaval](https://github.com/jazaval/) for advice on macOS package parsing, and Microsoft for their work in the macOS cookbook.

#### rhsm_errata / rhsm_errata_level

The `rhsm_errata` and `rhsm_errata_level` resources have been updated to properly function on RHEL 8 systems.

#### rhsm_register

The `rhsm_register` resource has a new property `https_for_ca_consumer` that enables using https connections during registration. Thanks for this improvement [@jasonwbarnett](https://github.com/jasonwbarnett/). This resource has also been updated to properly function on RHEL 8.

#### windows_share

Resolved failures in the `windows_share` resource when setting the `path` property. Thanks for reporting this issue [@Kundan22](https://github.com/Kundan22/).

### Platform Support

#### Ubuntu 20.04

Chef Infra Client is now tested on Ubuntu 20.04 (AMD64) with packages available on the [Chef Downloads Page](https://downloads.chef.io/chef).

#### Ubuntu 18.04 aarch64

Chef Infra Client is now tested on Ubuntu 18.04 aarch64 with packages available on the [Chef Downloads Page](https://downloads.chef.io/chef).

#### Windows 10

Our Windows 10 Chef Infra Client packages now receive an additional layer of testing to ensure they function as expected.

### Security Updates

#### Ruby

Ruby has been updated from 2.6.5 to 2.6.6 to resolve the following CVEs:

  - [CVE-2020-16255](https://www.ruby-lang.org/en/news/2020/03/19/json-dos-cve-2020-10663/): Unsafe Object Creation Vulnerability in JSON (Additional fix)
  - [CVE-2020-10933](https://www.ruby-lang.org/en/news/2020/03/31/heap-exposure-in-socket-cve-2020-10933/): Heap exposure vulnerability in the socket library

#### libarchive

libarchive has been updated from 3.4.0 to 3.4.2 to resolve multiple security vulnerabilities including the following CVEs:

  - [CVE-2019-19221](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-19221): archive_wstring_append_from_mbs in archive_string.c has an out-of-bounds read because of an incorrect mbrtowc or mbtowc call
  - [CVE-2020-9308](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-9308): archive_read_support_format_rar5.c in libarchive before 3.4.2 attempts to unpack a RAR5 file with an invalid or corrupted header

## What's New in 15.8

### New notify_group functionality

Chef Infra Client now includes a new `notify_group` feature that can be used to extract multiple common notifies out of individual resources to reduce duplicate code in your cookbooks and custom resources. Previously cookbook authors would often use a `log` resource to achieve a similar outcome, but using the log resource results in unnecessary Chef Infra Client log output. The `notify_group` method produces no additional logging, but fires all defined notifications when the `:run` action is set.

Example notify_group that stops, sleeps, and then starts service when a service config is updated:

``` ruby
service "crude" do
  action [ :enable, :start ]
end

chef_sleep "60" do
  action :nothing
end

notify_group "crude_stop_and_start" do
  notifies :stop, "service[crude]", :immediately
  notifies :sleep, "chef_sleep[60]", :immediately
  notifies :start, "service[crude]", :immediately
end

template "/etc/crude/crude.conf" do
  source "crude.conf.erb"
  variables node["crude"]
  notifies :run, "notify_group[crude_stop_and_start]", :immediately
end
```

### Chef InSpec 4.18.85

Chef InSpec has been updated from 4.18.39 to 4.18.85. This release includes a large number of bug fixes in addition to some great resource enhancements:

* The service resource features new support for yocto-based linux distributions. Thank you to [@michaellihs](https://github.com/michaellihs) for this addition!
* The package resource now includes support for FreeBSD. Thank you to [@fzipi](https://github.com/fzipi) for this work!
* We standardized the platform for the etc_hosts, virtualization, ini, and xml resources.
* The oracledb_session resource works again due to a missing quote fix.
* The groups resource on macOS no longer reports duplicates anymore.
command.exist? now conforms to POSIX standards. Thanks to [@PiQuer](https://github.com/PiQuer)!
* Changed the postfix_conf resource's supported platform to the broader unix. Thank you to [@fzipi](https://github.com/fzipi) for this fix!

### New Cookbook Helpers

New helpers have been added to make writing cookbooks easier.

#### Platform Version Helpers

New helpers for checking platform versions have been added. These helpers return parsed version strings so there's no need to convert the returned values to Integers or Floats before comparing them. Additionally, comparisons with version objects properly understand the order of versions so `5.11` will compare as larger than `5.9`, whereas converting those values to Floats would result in `5.9` being larger than `5.11`.

* `windows_nt_version` returns the NT kernel version which often differs from Microsoft's marketing versions. This helper offers a good way to find desktop and server releases that are based on the same codebase. For example, NT 6.3 is both Windows 8.1 and Windows 2012 R2.
* `powershell_version` returns the version of PowerShell installed on the system.
* `platform_version` returns the value of node['platform_version'].

Example comparison using windows_nt_version:

```ruby
if windows_nt_version >= 10
  some_modern_windows_things
end
```

#### Cloud Helpers

The cloud helpers from chef-sugar have been ported to Chef Infra Client:

* `cloud?` - if the node is running in any cloud, including internal clouds
* `ec2?` - if the node is running in ec2
* `gce?` - if the node is running in gce
* `rackspace?` - if the node is running in rackspace
* `eucalyptus?` - if the node is running under eucalyptus
* `linode?` - if the node is running in linode
* `openstack?` - if the node is running under openstack
* `azure?` - if the node is running in azure
* `digital_ocean?` - if the node is running in digital ocean
* `softlayer?` - if the node is running in softlayer

#### Virtualization Helpers

The virtualization helpers from chef-sugar have been ported to Chef Infra Client and extended with helpers to detect hypervisor hosts, physical, and guest systems.

* `kvm?` - if the node is a kvm guest
* `kvm_host?` - if the node is a kvm host
* `lxc?` - if the node is an lxc guest
* `lxc_host?` - if the node is an lxc host
* `parallels?`- if the node is a parallels guest
* `parallels_host?`- if the node is a parallels host
* `vbox?` - if the node is a virtualbox guest
* `vbox_host?` - if the node is a virtualbox host
* `vmware?` - if the node is a vmware guest
* `vmware_host?` - if the node is a vmware host
* `openvz?` - if the node is an openvz guest
* `openvz_host?` - if the node is an openvz host
* `guest?` - if the node is detected as any kind of guest
* `hypervisor?` - if the node is detected as being any kind of hypervisor
* `physical?` - the node is not running as a guest (may be a hypervisor or may be bare-metal)
* `vagrant?` - attempts to identify the node as a vagrant guest (this check may be error-prone)

#### include_recipe? helper

chef-sugar's `include_recipe?` has been added to Chef Infra Client providing a simple way to see if a recipe has been included on a node already.

Example usage in a not_if conditional:

```ruby
execute 'install my_app'
  command '/tmp/my_app_install.sh'
  not_if { include_recipe?('my_app::install') }
end
```

### Updated Resources

#### ifconfig

The `ifconfig` resource now supports the newer `ifconfig` release that ships in Debian 10.

#### mac_user

The `mac_user` resource, used when creating a user on Mac systems, has been improved to work better with macOS Catalina (10.15). The resource now properly looks up the numeric GID when creating a user, once again supports the `system` property, and includes a new `hidden` property which prevents the user from showing on the login screen. Thanks [@chilcote](https://github.com/chilcote) for these fixes and improvements.

#### sysctl

The `sysctl` resource has been updated to allow the inclusion of descriptive comments. Comments may be passed as an array or as a string. Any comments provided are prefixed with '#' signs and precede the `sysctl` setting in generated files.

An example:

```ruby
sysctl 'vm.swappiness' do
  value 10
  comment [
     "define how aggressively the kernel will swap memory pages.",
     "Higher values will increase aggressiveness",
     "lower values decrease the amount of swap.",
     "A value of 0 instructs the kernel not to initiate swap",
     "until the amount of free and file-backed pages is less",
     "than the high water mark in a zone.",
     "The default value is 60."
    ]
end
```

which results in `/etc/sysctl.d/99-chef-vm.swappiness.conf` as follows:

```
# define how aggressively the kernel will swap memory pages.
# Higher values will increase aggressiveness
# lower values decrease the amount of swap.
# A value of 0 instructs the kernel not to initiate swap
# until the amount of free and file-backed pages is less
# than the high water mark in a zone.
# The default value is 60.
vm.swappiness = 10
```

### Platform Support

- Chef Infra Clients packages are now validated for Debian 10.

### macOS Binary Signing

Each binary in the macOS Chef Infra Client installation is now signed to improve the integrity of the installation and ensure compatibility with macOS Catalina security requirements.

## What's New in 15.7

### Updated Resources

#### archive_file

The `archive_file` resource will now only change ownership on files and directories that were part of the archive itself. This prevents changing permissions on important high level directories such as /etc or /bin when you extract a file into those directories. Thanks for this fix, [@bobchaos](https://github.com/bobchaos/).

#### cron and cron_d

The `cron` and `cron_d` resources now include a `timeout` property, which allows you to configure actions to perform when a job times out. This property accepts a hash of timeout configuration options:

- `preserve-status`: `true`/`false` with a default of `false`
- `foreground`: `true`/`false` with a default of `false`
- `kill-after`: `Integer` for the timeout in seconds
- `signal`: `String` or `Integer` to send to the process such as `HUP`

#### launchd

The `launchd` resource has been updated to properly capitalize `HardResourceLimits`. Thanks for this fix, [@rb2k](https://github.com/rb2k/).

#### sudo

The `sudo` resource no longer fails on the second Chef Infra Client run when using a `Cmnd_Alias`. Thanks for reporting this issue, [@Rudikza](https://github.com/Rudikza).

#### user

The `user` resource on AIX no longer forces the user to change the password after Chef Infra Client modifies the password. Thanks for this fix, [@Triodes](https://github.com/Triodes).

The `user` resource on macOS 10.15 has received several important fixes to improve logging and prevent failures.

#### windows_task

The `windows_task` resource is now idempotent when a system is joined to a domain and the job runs under a local user account.

#### x509_certificate

The `x509_certificate` resource now includes a new `renew_before_expiry` property that allows you to auto renew certificates a specified number of days before they expire. Thanks [@julienhuon](https://github.com/julienhuon/) for this improvement.

### Additional Recipe Helpers

We have added new helpers for identifying Windows releases that can be used in any part of your cookbooks.

#### windows_workstation?

Returns `true` if the system is a Windows Workstation edition.

#### windows_server?

Returns `true` if the system is a Windows Server edition.

#### windows_server_core?

Returns `true` if the system is a Windows Server Core edition.

### Notable Changes and Fixes

- `knife upload` and `knife cookbook upload` will now generate a metadata.json file from metadata.rb when uploading a cookbook to the Chef Infra Server.
- A bug in `knife bootstrap` behavior that caused failures when bootstrapping Windows hosts from non-Windows hosts and vice versa has been resolved.
- The existing system path is now preserved when bootstrapping Windows nodes. Thanks for this fix, [@Xorima](https://github.com/Xorima/).
- Ohai now properly returns the drive name on Windows and includes new drive_type fields to allow you to determine the type of attached disk. Thanks for this improvement [@sshock](https://github.com/sshock/).
- Ohai has been updated to properly return DMI data to Chef Infra Client. Thanks for troubleshooting this, [@zmscwx](https://github.com/zmscwx) and [@Sliim](https://github.com/Sliim).

### Platform Support

- Chef Infra Clients packages are no longer produced for Windows 2008 R2 as this release reached its end of life on Jan 14th, 2020.
- Chef Infra Client packages are no longer produced for RHEL 6 on the s390x platform. Builds will continue to be published for RHEL 7 on the s390x platform.

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2u to resolve [CVE-2019-1551](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1551)

## What's New in 15.6

### Updated Resources

### apt_repository

The `apt_repository` resource now properly escapes repository URIs instead of quoting them. This prevents failures when using the `apt-file` command, which was unable to parse the quoted URIs. Thanks for reporting this [@Seb-Solon](https://github.com/Seb-Solon)

### file

The `file` resource now shows the output of any failures when running commands specified in the `verify` property. This means you can more easily validate config files before potentially writing an incorrect file to disk. Chef Infra Client will shellout to any specified command and will show the results of failures for further troubleshooting.

### user

The `user` resource on Linux systems now continues successfully when `usermod` returns an exit code of 12. Exit code 12 occurs when a user's home directory is changed and the underlying directory already exists. Thanks [@skippyj](https://github.com/skippyj) for this fix.

### yum_repository

The `yum_repository` now properly formats the repository configuration when multiple `baseurl` values are present. Thanks [@bugok](https://github.com/bugok) for this fix.

### Performance Improvements

This release of Chef Infra Client ships with several optimizations to our Ruby installation to improve the performance of loading the chef-client and knife commands. These improvements are particularly noticeable on non-SSD hosts and on Windows.

### Smaller Install Footprint

We've further optimized our install footprint and reduced the size of `/opt/chef` by ~7% by removing unnecessary test files and libraries that shipped in previous releases.

### filesystem2 Ohai Data on Windows

Ohai 15.6 includes new `node['filesystem2']` data on Windows hosts. Fileystem2 presents filesystem data by both mountpoint and by device name. This data structure matches that of the filesystem plugin on Linux and other *nix operating systems. Thanks [@jaymzh](https://github.com/jaymzh) for this new data structure.

## What's New in 15.5.15

The Chef Infra Client 15.5.15 release includes fixes for two regressions. A regression in the `build_essential` resource caused failures on `rhel` platforms and a second regression caused Chef Infra Client to fail when starting with `enforce_path_sanity` enabled. As part of this fix we've added a new property, `raise_if_unsupported`, to the `build-essential` resource. Instead of silently continuing, this property will fail a Chef Infra Client run if an unknown platform is encountered.

We've also updated the `windows_package` resource. The resource will now provide better error messages if invalid options are passed to the `installer_type` property and the `checksum` property will now accept uppercase SHA256 checksums.

## What's New in 15.5.9

### New Cookbook Helpers

Chef Infra Client now includes a new `chef-utils` gem, which ships with a large number of helpers to make writing cookbooks easier. Many of these helpers existed previously in the `chef-sugar` gem. We have renamed many of the named helpers for consistency, while providing backwards compatibility with existing `chef-sugar` names. Existing cookbooks written with `chef-sugar` should work unmodified with any of these new helpers. Expect a Cookstyle rule in the near future to help you update existing `chef-sugar` code to use the newer built-in helpers.

For more information all all of the new helpers available, see the [chef-utils readme](https://github.com/chef/chef/blob/master/chef-utils/README.md)

### Chefignore Improvements

We've reworked how chefignore files are handled in `knife`, which has allowed us to close out a large number of long outstanding bugs. `knife` will now traverse all the way up the directory structure looking for a chefignore file. This means you can place a chefignore file in each cookbook or any parent directory in your repository structure. Additionally, we have made fixes that ensure that commands like `knife diff` and `knife cookbook upload` always honor your chefignore files.

### Windows Habitat Plan

Official Habitat packages of Chef Infra Client are now available for Windows. It has all the executables of the traditional omnibus packages, but in Habitat form. You can find it in the Habitat Builder under [chef/chef-infra-client](https://bldr.habitat.sh/#/pkgs/chef/chef-infra-client/latest/windows).

### Performance Improvements

This release of Chef Infra Client ships with several optimizations to our Ruby installation that improve the performance of the chef-client and knife commands, especially on Windows systems. Expect to see more here in future releases.

### Chef InSpec 4.18.39

Chef InSpec has been updated from 4.17.17 to 4.18.38. This release includes a large number of bug fixes in addition to some great resource enhancements:

- Inputs can now be used within a `describe.one` block
- The `service` resource now includes a `startname` property for Windows and systemd services
- The `interface` resource now includes a `name` property
- The `user` resource now better supports Windows with the addition of `passwordage`, `maxbadpasswords`, and `badpasswordattempts` properties
- The `nginx` resource now includes parsing support for wildcard, dot prefix, and regex
- The `iis_app_pool` resource now handles empty app pools
- The `filesystem` resource now supports devices with very long names
- The `apt` better handles URIs and supports repos with an `arch`
- The `oracledb_session` has received multiple fixes to make it work better
- The `npm` resource now works under sudo on Unix and on Windows with a custom PATH

### New Resources

#### chef_sleep

The `chef_sleep` resource can be used to sleep for a specified number of seconds during a Chef Infra Client run. This may be helpful to use with other commands that return a completed status before they are actually ready. In general, do not use this resource unless you truly need it.

Using with a Windows service that starts, but is not immediately ready:

```ruby
service 'Service that is slow to start and reports as started' do
  service_name 'my_database'
  action :start
  notifies :sleep, chef_sleep['wait for service start']
end

chef_sleep 'wait for service start' do
  seconds 30
  action :nothing
end
```

### Updated Resources

### systemd_unit / service

The `systemd_unit` and `service` resources (when on systemd) have been updated to not re-enable services with an indirect status. Thanks [@jaymzh](https://github.com/jaymzh) for this fix.

### windows_firewall

The `windows_firewall` resource has been updated to support passing in an array of profiles in the `profile` property. Thanks [@Happycoil](https://github.com/Happycoil) for this improvement.

### Security Updates

#### libxslt

libxslt has been updated to 1.1.34 to resolve [CVE-2019-13118](https://nvd.nist.gov/vuln/detail/CVE-2019-13118).

## What's New in 15.4

### converge_if_changed Improvements

Chef Infra Client will now take into account any `default` values specified in custom resources when making converge determinations with the `converge_if_changed` helper. Previously, default values would be ignored, which caused necessary changes to be skipped. Note: This change may cause behavior changes for some users, but we believe this original behavior is an impacting bug for enough users to make it outside of a major release. Thanks [@ jakauppila](https://github.com/jakauppila) for reporting this.

### Bootstrap Improvements

Several improvements have been made to the `knife bootstrap` command to make it more reliable and secure:

- File creation is now wrapped in a umask to avoid potential race conditions
- `NameError` and `RuntimeError` failures during bootstrap have been resolved
- `Undefined method 'empty?' for nil:NilClass` during bootstrap have been resolved
- Single quotes in attributes during bootstrap no longer result in bootstrap failures
- The bootstrap command no longer appears in PS on the host while bootstrapping is running

### knife supermarket list Improvements

The `knife supermarket list` command now includes two new options:

- `--sort-by [recently_updated recently_added most_downloaded most_followed]`: Sort cookbooks returned from the Supermarket API
- `--owned_by`: Limit returned cookbooks to a particular owner

### Updated Resources

#### chocolatey_package

The `chocolatey_package` resource no longer fails when passing options with the `options` property. Thanks for reporting this issue [@kenmacleod](https://github.com/kenmacleod).

#### kernel_module

The `kernel_module` resource includes a new `options` property, which allows users to set module specific parameters and settings. Thanks [@ramereth](https://github.com/ramereth) for this new feature.

Example of a kernel_module resource using the new options property:

```ruby
  kernel_module 'loop' do
  options [ 'max_loop=4', 'max_part=8' ]
  end
```

#### remote_file

The `remote_file` resource has been updated to better display progress when using the `show_progress` resource. Thanks for reporting this issue [@isuftin](https://github.com/isuftin).

#### sudo

The `sudo` resource now runs sudo config validation against all of the sudo configuration files on the system instead of only the file being written. This allows us to detect configuration errors that occur when configs conflict with each other. Thanks for reporting this issue [@drzewiec](https://github.com/drzewiec).

#### windows_ad_join

The `windows_ad_join` has a new `:leave` action for leaving an Active Directory domain and rejoining a workgroup. This new action also has a new `workgroup_name` property for specifying the workgroup to join upon leaving the domain. Thanks [@jasonwbarnett](https://github.com/jasonwbarnett) for adding this new action.

Example of leaving a domain

```ruby
windows_ad_join 'Leave the domain' do
  workgroup_name 'local'
  action :leave
end
```

#### windows_package

The `windows_package` resource no longer updates environmental variables before installing the package. This prevents potential modifications that may cause a package installation to fail. Thanks [@jeremyhage](https://github.com/jeremyhage) for this fix.

#### windows_service

The `windows_service` resource no longer updates the service and triggers notifications if the case of the `run_as_user` property does not match the user set on the service. Thanks [@jasonwbarnett](https://github.com/jasonwbarnett) for this fix.

#### windows_share

The `windows_share` resource is now fully idempotent by better validating the provided `path` property from the user. Thanks [@Happycoil](https://github.com/Happycoil) for this fix.

### Security Updates

#### Ruby

Ruby has been updated from 2.6.4 to 2.6.5 in order to resolve the following CVEs:

- [CVE-2019-16255](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16255): A code injection vulnerability of Shell#[] and Shell#test
- [CVE-2019-16254](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16254): HTTP response splitting in WEBrick (Additional fix)
- [CVE-2019-15845](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-15845): A NUL injection vulnerability of File.fnmatch and File.fnmatch?
- [CVE-2019-16201](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16201): Regular Expression Denial of Service vulnerability of WEBrick's Digest access authentication

## What's New in 15.3

### Custom Resource Unified Mode

Chef Infra Client 15.3 introduces an exciting new way to easily write custom resources that mix built-in Chef Infra resources with Ruby code. Previously custom resources would use Chef Infra's standard compile and converge phases, which meant that Ruby would be evaluated first and then the resources would be converged. This often results in confusing and undesirable behavior when you are trying to mix resources with Ruby logic. Many custom resource authors would attempt to get around this by forcing resources to run at compile time so that all the code in their resource would execute during the compile phase.

An example of forcing a resource to run at compile time:

```ruby
resource_name 'foo' do
  action :nothing
end.run_action(:some_action)
```

With unified mode, you opt in to a single phase per resource where all Ruby and Chef Infra resources are executed at once. This makes it far easier to determine how your code will be evaluated and run. Additionally, you no longer need to force any resources to run at compile time, as all code is run in the compile phase. To enable this new mode just add `unified_mode true` to your resources like this:

```ruby
property :Some_property, String

unified_mode true

action :create do
  # some code
end
```

### Interval Mode Now Fails on Windows

Chef Infra Client 15.3 will now raise an error if you attempt to keep the chef-client process running long-term by enabling interval runs. Interval runs have already raised failures on non-Windows platforms and we've suggested that users move away from them on Windows for many years. The long-running chef-client process on Windows will load and reload cookbooks over each other in memory. This could produce a running state which is not a representation of the cookbook code that the authors wrote or tested, and behavior that may be wildly different depending on how long the chef-client process has been running and on the sequence that the cookbooks were uploaded.

### Updated Resources

#### ifconfig

The `ifconfig` resource has been updated to properly support interfaces with a hyphen in their name. This is most commonly encountered with bridge interfaces that are named `br-1234`.

#### archive_file

The `archive_file` resource now supports archives in the RAR 5.0 format as well as zip files compressed using xz, lzma, ppmd8 and bzip2 compression.

#### user

**macOS 10.14 / 10.15 support**

The `user` resource now supports the creation of users on macOS 10.14 and 10.15 systems. The updated resource now complies with macOS TCC policies by using a user with admin privileges to create and modify users. The following new properties have been added for macOS user creation:

- `admin` sets a user to be an admin.

- `admin_username` and `admin_password` define the admin user credentials required for toggling SecureToken for a user. The value of 'admin_username' must correspond to a system user that is part of the 'admin' with SecureToken enabled in order to toggle SecureToken.

- `secure_token` is a boolean property that sets the desired state for SecureToken. FileVault requires a SecureToken for full disk encryption.

- `secure_token_password` is the plaintext password required to enable or disable `secure_token` for a user. If no salt is specified we assume the 'password' property corresponds to a plaintext password and will attempt to use it in place of secure_token_password if it is not set.

**Password property is now sensitive**

The `password` property is now set to sensitive to prevent the password from being shown in debug or failure logs.

**gid property can now be a string**

The `gid` property now allows specifying the user's gid as a string. For example:

```ruby
user 'tim' do
  gid '123'
end
```

### Platform Support Updates

#### macOS 10.15 Support

Chef Infra Client is now validated against macOS 10.15 (Catalina) with packages now available at [downloads.chef.io](https://downloads.chef.io/) and via the [Omnitruck API](https://docs.chef.io/api_omnitruck/). Additionally, Chef Infra Client will no longer be validated against macOS 10.12.

#### AIX 7.2

Chef Infra Client is now validated against AIX 7.2 with packages now available at [downloads.chef.io](https://downloads.chef.io/) and via the [Omnitruck API](https://docs.chef.io/api_omnitruck/).

### Chef InSpec 4.16

Chef InSpec has been updated from 4.10.4 to 4.16.0 with the following changes:

- A new `postfix_conf` has been added for inspecting Postfix configuration files.
- A new `plugins` section has been added to the InSpec configuration file which can be used to pass secrets or other configurations into Chef InSpec plugins.
- The `service` resource now includes a new `startname` property for determining which user is starting the Windows services.
- The `groups` resource now properly gathers membership information on macOS hosts.

### Security Updates

#### Ruby

Ruby has been updated from 2.6.3 to 2.6.4 in order to resolve [CVE-2012-6708](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2012-6708) and [CVE-2015-9251](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-9251).

#### openssl

openssl has been updated from 1.0.2s to 1.0.2t in order to resolve [CVE-2019-1563](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1563) and [CVE-2019-1547](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1547).

#### nokogiri

nokogiri has been updated from 1.10.2 to 1.10.4 in order to resolve [CVE-2019-5477](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-5477)

## What's New in 15.2

### Updated Resources

#### dnf_package

The `dnf_package` resource has been updated to fully support RHEL 8.

#### kernel_module

The `kernel_module` now supports a `:disable` action. Thanks [@tomdoherty](https://github.com/tomdoherty).

#### rhsm_repo

The `rhsm_repo` resource has been updated to support passing a repo name of `*` in the `:disable` action. Thanks for reporting this issue [@erinn](https://github.com/erinn).

#### windows_task

The `windows_task` resource has been updated to allow the `day` property to accept an `Integer` value.

#### zypper_package

The `zypper_package` package has been updated to properly upgrade packages if necessary based on the version specified in the resource block. Thanks [@foobarbam](https://github.com/foobarbam) for this fix.

### Platform Support Updates

#### RHEL 8 Support Added

Chef Infra Client 15.2 now includes native packages for RHEL 8 with all builds now validated on RHEL 8 hosts.

#### SLES 11 EOL

Packages will no longer be built for SUSE Linux Enterprise Server (SLES) 11 as SLES 11 exited the 'General Support' phase on March 31, 2019. See Chef's [Platform End-of-Life Policy](https://docs.chef.io/platforms/#platform-end-of-life-policy) for more information on when Chef ends support for an OS release.

#### Ubuntu 14.04 EOL

Packages will no longer be built for Ubuntu 14.04 as Canonical ended maintenance updates on April 30, 2019. See Chef's [Platform End-of-Life Policy](https://docs.chef.io/platforms/#platform-end-of-life-policy) for more information on when Chef ends support for an OS release.

### Ohai 15.2

Ohai has been updated to 15.2 with the following changes:

- Improved detection of Openstack including proper detection of Windows nodes running on Openstack when fetching metadata. Thanks [@jjustice6](https://github.com/jjustice6).
- A new `other_versions` field has been added to the Packages plugin when the node is using RPM. This allows you to see all installed versions of packages, not just the latest version. Thanks [@jjustice6](https://github.com/jjustice6).
- The Linux Network plugin has been improved to not mark interfaces down if `stp_state` is marked as down. Thanks [@josephmilla](https://github.com/josephmilla).
- Arch running on ARM processors is now detected as the `arm` platform. Thanks [@BackSlasher](https://github.com/BackSlasher).

### Chef InSpec 4.10.4

Chef InSpec has been updated from 4.6.4 to 4.10.4 with the following changes:

- Fix handling multiple triggers in the `windows_task` resource
- Fix exceptions when resources are used with incompatible transports
- Un-deprecate the `be_running` matcher on the `service` resource
- Add resource `sys_info.manufacturer` and `sys_info.model`
- Add `ip6tables` resource

### Security Updates

#### bzip2

bzip2 has been updated from 1.0.6 to 1.0.8 to resolve [CVE-2016-3189](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-3189) and [CVE-2019-12900](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-12900).

## What's New in 15.1

### New Resources

#### chocolatey_feature

The `chocolatey_feature` resource allows you to enable and disable Chocolatey features. See the [chocolatey_feature documentation](https://docs.chef.io/resources/chocolatey_feature/) for full usage information. Thanks [@gep13](https://github.com/gep13) for this new resource.

### Updated Resources

#### chocolatey_source

The `chocolatey_source` resource has been updated with new `enable` and `disable` actions, as well as `admin_only` and `allow_self_service` properties. Thanks [@gep13](https://github.com/gep13) for this enhancement.

#### launchd

The `launchd` resource has been updated with a new `launch_events` property, which allows you to specify higher-level event types to be used as launch-on-demand event sources. Thanks [@chilcote](https://github.com/chilcote) for this enhancement.

#### yum_package

The `yum_package` resource's helper for interacting with the yum subsystem has been updated to always close out the rpmdb lock, even during failures. This may prevent the rpmdb becoming locked in some rare conditions. Thanks for reporting this issue, [@lytao](https://github.com/lytao).

#### template

The `template` resource now provides additional information on failures, which is especially useful in ChefSpec tests. Thanks [@brodock](https://github.com/brodock) for this enhancement.

### Target Mode Improvements

Our experimental Target Mode received a large number of updates in Chef Infra Client 15.1. Target Mode now reuses the connection to the remote system, which greatly speeds up the remote Chef Infra run. There is also now support for Target Mode in the `systemd_unit`, `log`, `ruby_block`, and `breakpoint` resources. Keep in mind that when using `ruby_block` with Target Mode that the Ruby code in the block will execute locally as there is not necessarily a Ruby runtime on the remote host.

### Ohai 15.1

Ohai has been updated to 15.1 with the following changes:

- The `Shard` plugin properly uses the machine's `machinename`, `serial`, and `uuid` attributes to generate the shard value. The plugin also no longer throws an exception on macOS hosts. Thanks [@michel-slm](https://github.com/michel-slm) for these fixes.
- The `Virtualbox` plugin has been enhanced to gather information on running guests, storage, and networks when VirtualBox is installed on a node. Thanks [@freakinhippie](https://github.com/freakinhippie) for this new capability.
- Ohai no longer fails to gather interface information on Solaris in some rare conditions. Thanks [@devoptimist](https://github.com/devoptimist) for this fix.

### Chef InSpec 4.6.4

Chef InSpec has been updated from 4.3.2 to 4.6.4 with the following changes:

- InSpec `Attributes` have now been renamed to `Inputs` to avoid confusion with Chef Infra attributes.
- A new InSpec plugin type of `Input` has been added for defining new input types. See the [InSpec Plugins documentation](https://github.com/inspec/inspec/blob/master/docs/dev/plugins.md#implementing-input-plugins) for more information on writing these plugins.
- InSpec no longer prints errors to the stdout when passing `--format json`.
- When fetching profiles from GitHub, the URL can now include periods.
- The performance of InSpec startup has been improved.

## What's New in 15.0.300

This release includes critical bugfixes for the 15.0 release:
- Fix `knife bootstrap` over SSH when `requiretty` is configured on the host.
- Added the `--chef-license` CLI flag to `chef-apply` and `chef-solo` commands.

## What's New in 15.0.298

This release includes critical bugfixes for the 15.0 release:
- Allow accepting the license on non-interactive Windows sessions
- Resolve license acceptance failures on Windows 2012 R2
- Improve some `knife` and `chef-client` help text
- Properly handle session_timeout default value in `knife bootstrap`
- Avoid failures due to Train::Transports::SSHFailed class not being loaded in `knife bootstrap`
- Resolve failures using the ca_trust_file option with `knife bootstrap`

## What's New in 15.0.293

### Chef Client is now Chef Infra Client

Chef Client has a new name, but don't worry, it's the same Chef Client you've grown used to. You'll notice new branding throughout the application, help, and documentation but the command line name of `chef-client` remains the same.

### Chef EULA

Chef Infra Client requires an EULA to be accepted by users before it can run. Users can accept the EULA in a variety of ways:

- `chef-client --chef-license accept`
- `chef-client --chef-license accept-no-persist`
- `CHEF_LICENSE="accept" chef-client`
- `CHEF_LICENSE="accept-no-persist" chef-client`

Finally, if users run `chef-client` without any of these options, they will receive an interactive prompt asking for license acceptance. If the license is accepted, a marker file will be written to the filesystem unless `accept-no-persist` is specified. Once this marker file is persisted, users no longer need to set any of these flags.

See our [Frequently Asked Questions document](https://www.chef.io/bmc-faq/) for more information on the EULA and license acceptance.

### New Features / Functionality

#### Target Mode Prototype

Chef Infra Client 15 adds a prototype for a new method of executing resources called Target Mode. Target Mode allows a Chef Infra Client run to manage a remote system over SSH or another protocol supported by the Train library. This support includes platforms that we currently support like Ubuntu Linux, but also allows for configuring other architectures and platforms, such as switches that do not have native builds of Chef Infra Client. Target Mode maintains a separate node object for each target and allows you to manage that node using existing patterns that you currently use.

As of this release, only the `execute` resource and guards are supported, but modifying existing resources or writing new resources to support Target Mode is relatively easy. Using Target Mode is as easy as running `chef-client --target hostname`. The authentication credentials should be stored in your local `~/.chef/credentials` file with the hostname of the target node as the profile name. Each key/value pair is passed to Train for authentication.

#### Data Collection Ground-Up Refactor

Chef Infra Client's Data Collection subsystem is used to report node changes during client runs to Chef Automate or other reporting systems. For Chef Infra Client 15, we performed a ground-up rewrite of this subsystem, which greatly improves the data reported to Chef Automate and ensures data is delivered even in the toughest of failure conditions.

#### copy_properties_from in Custom Resources

A new `copy_properties_from` method for custom resources allows you copy properties from your custom resource into other resources you are calling, so you can avoid unnecessarily repeating code.

To inherit all the properties of another resource:
```ruby
resource_name :my_resource

property :mode, String, default: '777'
property :owner, String, default: 'app_user'
property :group, String, default: 'admins'

directory '/etc/myapp' do
  copy_properties_from new_resource
  recursive true
end
```

To selectively inherit certain properties from a resource:

```ruby
resource_name :my_resource

property :mode, String, default: '777'
property :owner, String, default: 'app_user'
property :group, String, default: 'admins'

directory '/etc/myapp' do
  copy_properties_from(new_resource, :owner, :group, :mode)
  mode '755'
  recursive true
end
```

#### ed25519 SSH key support

Our underlying SSH implementation has been updated to support the new ed25519 SSH key format. This means you will be able to use `knife bootstrap` and `knife ssh` on hosts that only support this new key format.

#### Allow Using --delete-entire-chef-repo in Chef Local Mode

Chef Solo's `--delete-entire-chef-repo` option has been extended to work in Local Mode as well. Be warned that this flag does exactly what it states, and when used incorrectly, can result in loss of work.

### New Resources

#### archive_file resource

Use the `archive_file` resource to decompress multiple archive formats without the need for compression tools on the host.

See the [archive_file](https://docs.chef.io/resources/archive_file/) documentation for more information.

#### windows_uac resource

Use the `windows_uac` resource to configure UAC settings on Windows hosts.

See the [windows_uac](https://docs.chef.io/resources/windows_uac) documentation for more information.

#### windows_dfs_folder resource

Use the `windows_dfs_folder` resource to create and delete Windows DFS folders.

See the [windows_dfs_folder](https://docs.chef.io/resources/windows_dfs_folder) documentation for more information.

#### windows_dfs_namespace resources

Use the `windows_dfs_namespace` resource to create and delete Windows DFS namespaces.

See the [windows_dfs_namespace](https://docs.chef.io/resources/windows_dfs_namespace) documentation for more information.

#### windows_dfs_server resources

Use the `windows_dfs_server` resource to configure Windows DFS server settings.

See the [windows_dfs_server](https://docs.chef.io/resources/windows_dfs_server) documentation for more information.

#### windows_dns_record resource

Use the `windows_dns_record` resource to create or delete DNS records.

See the [windows_dns_record](https://docs.chef.io/resources/windows_dns_record) documentation for more information.

#### windows_dns_zone resource

Use the `windows_dns_zone` resource to create or delete DNS zones.

See the [windows_dns_zone](https://docs.chef.io/resources/windows_dns_zone) documentation for more information.

#### snap_package resource

Use the `snap_package` resource to install snap packages on Ubuntu hosts.

See the [snap_package](https://docs.chef.io/resources/snap_package) documentation for more information.

### Resource Improvements

#### windows_task

The `windows_task` resource now supports the Start When Available option with a new `start_when_available` property.

#### locale

The `locale` resource now allows setting all possible LC_* environmental variables.

#### directory

The `directory` resource now property supports passing `deny_rights :write` on Windows nodes.

#### windows_service

The `windows_service` resource has been improved to prevent accidentally reverting a service back to default settings in a subsequent definition.

This example will no longer result in the MyApp service reverting to default RunAsUser:
```ruby
windows_service 'MyApp' do
  run_as_user 'MyAppsUser'
  run_as_password 'MyAppsUserPassword'
  startup_type :automatic
  delayed_start true
  action [:configure, :start]
end

...

windows_service 'MyApp' do
  startup_type :automatic
  action [:configure, :start]
end
```

#### Ruby 2.6.3

Chef now ships with Ruby 2.6.3. This new version of Ruby improves performance and includes many new features to make more advanced Chef usage easier. See <https://www.rubyguides.com/2018/11/ruby-2-6-new-features/> for a list of some of the new functionality.

### Ohai Improvements

#### Improved Linux Platform / Platform Family Detection

`Platform` and `platform_family` detection on Linux has been rewritten to utilize the latest config files on modern Linux distributions before falling back to slower and fragile legacy detection methods. Ohai will now begin by parsing the contents of `/etc/os-release` for OS information if available. This feature improves the reliability of detection on modern distros and allows detection of new distros as they are released.

With this change, we now detect `sles_sap` as a member of the `suse` `platform_family`. Additionally, this change corrects our detection of the `platform_version` on Cisco Nexus switches where previously the build number was incorrectly appended to the version string.

#### Improved Virtualization Detection

Hypervisor detection on multiple platforms has been updated to use DMI data and a single set of hypervisors. This greatly improves the detection of hypervisors on Windows, BSD and Solaris platforms. It also means that as new hypervisor detection is added in the future, we will automatically support the majority of platforms.

#### Fix Windows 2016 FQDN Detection

Ohai 14 incorrectly detected a Windows 2016 node's `fqdn` as the node's `hostname`. Ohai 15 now correctly reports the FQDN value.

#### Improved Memory Usage

Ohai now uses less memory due to internal optimization of how we track plugin information.

#### FIPS Detection Improvements

The FIPS plugin now uses the built-in FIPS detection in Ruby for improved detection.

### New Deprecations

#### knife cookbook site deprecated in favor of knife supermarket

The `knife cookbook site` command has been deprecated in favor of the `knife supermarket` command. `knife cookbook site` will now produce a warning message. In Chef Infra Client 16, we will remove the `knife cookbook site` command entirely.

#### locale LC_ALL property

The `LC_ALL` property in the `locale` resource has been deprecated as the usage of this environmental variable is not recommended by distribution maintainers.

### Breaking Changes

#### Knife Bootstrap

Knife bootstrap has been entirely rewritten. Native support for Windows bootstrapping is now a part of the main `knife bootstrap` command. This marks the deprecation of the `knife-windows` plugin's `bootstrap` behavior. This change also addresses [CVE-2015-8559](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-8559): *The `knife bootstrap` command in chef leaks the validator.pem private RSA key to /var/log/messages*.

**Important**: `knife bootstrap` can bootstrap all supported versions of Chef Infra Client. Older versions may continue to work as far back as 12.20.

In order to accommodate a combined bootstrap that supports both SSH and WinRM, some CLI flags have been added, removed, or changed. Using the changed options will result in deprecation warnings, but `knife bootstrap` will accept those options unless otherwise noted. Using removed options will cause the command to fail.

**New Flags**

| Flag | Description |
|-----:|:------------|
| --max-wait SECONDS | Maximum time to wait for initial connection to be established. |
| --winrm-basic-auth-only | Perform only Basic Authentication to the target WinRM node. |
| --connection-protocol PROTOCOL| Connection protocol to use. Valid values are 'winrm' and 'ssh'. Default is 'ssh'. |
| --connection-user | User to authenticate as, regardless of protocol. |
| --connection-password| Password to authenticate as, regardless of protocol. |
| --connection-port | Port to connect to, regardless of protocol. |
| --ssh-verify-host-key VALUE | Verify host key. Default is 'always'. Valid values are 'accept', 'accept\_new', 'accept\_new\_or\_local\_tunnel', and 'never'. |

**Changed Flags**

| Flag | New Option | Notes |
|-----:|:-----------|:------|
| --[no-]host-key-verify |--ssh-verify-host-key VALUE | See above for valid values. |
| --forward-agent | --ssh-forward-agent| |
| --session-timeout MINUTES | --session-timeout SECONDS|New for ssh, existing for winrm. The unit has changed from MINUTES to SECONDS for consistency with other timeouts. |
| --ssh-password | --connection-password | |
| --ssh-port | --connection-port | `knife[:ssh_port]` config setting remains available.
| --ssh-user | --connection-user | `knife[:ssh_user]` config setting remains available.
| --ssl-peer-fingerprint | --winrm-ssl-peer-fingerprint | |
| --prerelease |--channel CHANNEL | This now allows you to specify the channel that Chef Infra Client gets installed from. Valid values are _stable_, _current_, and _unstable_. 'current' has the same effect as using the old --prerelease. |
| --winrm-authentication-protocol=PROTO | --winrm-auth-method=AUTH-METHOD | Valid values: _plaintext_, _kerberos_, _ssl_, _negotiate_ |
| --winrm-password| --connection-password | |
| --winrm-port| --connection-port | `knife[:winrm_port]` config setting remains available.|
| --winrm-ssl-verify-mode MODE | --winrm-no-verify-cert | Mode is not accepted. When flag is present, SSL cert will not be verified. Same as original mode of 'verify\_none'. [1] |
| --winrm-transport TRANSPORT | --winrm-ssl | Use this flag if the target host is accepts WinRM connections over SSL. [1] |
| --winrm-user | --connection-user | `knife[:winrm_user]` config setting remains available.|
| --winrm-session-timeout | --session-timeout | Now available for bootstrapping over SSH as well |

[1] These flags do not have an automatic mapping of old flag -> new flag. The new flag must be used.

**Removed Flags**

| Flag | Notes |
|-----:|:------|
|--kerberos-keytab-file| This option existed but was not implemented. |
|--winrm-codepage| This was used under `knife-windows` because bootstrapping was performed over a `cmd` shell. It is now invoked from `powershell`, so this option is no longer used. |
|--winrm-shell| This option was ignored for bootstrap. |
|--install-as-service| Installing Chef Client as a service is not supported. |

**Usage Changes**

Instead of specifying protocol with `-o`, it is also possible to prefix the target hostname with the protocol in URL format. For example:

```
knife bootstrap example.com -o ssh
knife bootstrap ssh://example.com
knife bootstrap example.com -o winrm
knife bootstrap winrm://example.com
```

#### Chef Infra Client packages remove /opt/chef before installation

Upon upgrading Chef Infra Client packages, the `/opt/chef` directory is removed. This ensures any `chef_gem` installed gem versions and other modifications to `/opt/chef` will removed to prevent upgrade issues. Due to technical details with rpm script execution order, the implementation involves a a pre-installation script that wipes `/opt/chef` before every install, and is done consistently this way on every package manager.

Users who are properly managing customizations to `/opt/chef` through Chef recipes would not be affected, because their customizations will still be installed by the new package.

You will see a warning that the `/opt/chef` directory will be removed during the package installation process.

#### powershell_script now allows overriding the default flags

We now append `powershell_script` user flags to the default flags rather than the other way around, which made user flags override the defaults. This is the correct behavior, but it may cause scripts to execute differently than in previous Chef Client releases.

#### Package provider allow_downgrade is now true by default

We reversed the default behavior to `allow_downgrade true` for our package providers. To override this setting to prevent downgrades, use the `allow_downgrade false` flag. This behavior change will mostly affect users of the rpm and zypper package providers.

In this example, the code below should now read as asserting that the package `foo` must be version `1.2.3` after that resource is run.:

```
package "foo" do
  version "1.2.3"
end
```

The code below is now what is necessary to specify that `foo` must be version `1.2.3` or higher. Note that the yum provider supports syntax like `package "foo > 1.2.3"`, which should be used and is preferred over using allow_downgrade.

```
package "foo" do
  allow_downgrade false
  version "1.2.3"
end
```

#### Node Attributes deep merge nil values

Writing a `nil` to a precedence level in the node object now acts like any other value and can be used to override values back to `nil`.

For example:

```
chef (15.0.53)> node.default["foo"] = "bar"
 => "bar"
chef (15.0.53)> node.override["foo"] = nil
 => nil
chef (15.0.53)> node["foo"]
 => nil
```

In prior versions of `chef-client`, the `nil` set in the override level would be completely ignored and the value of `node["foo"]` would have been "bar".

#### http_disable_auth_on_redirect now enabled

The Chef config ``http_disable_auth_on_redirect`` has been changed from `false` to `true`. In Chef Infra Client 16, this config option will be removed altogether and Chef Infra Client will always disable auth on redirect.

#### knife cookbook test removal

The `knife cookbook test` command has been removed. This command would often report non-functional cookbooks as functional, and has been superseded by functionality in other testing tools such as `cookstyle`, `foodcritic`, and `chefspec`.

#### ohai resource's ohai_name property removal

The `ohai` resource contained a non-functional `ohai_name` property, which has been removed.

#### knife status --hide-healthy flag removal

The `knife status --hide-healthy` flag has been removed. Users should run `knife status --hide-by-mins MINS` instead.

#### Cookbook shadowing in Chef Solo Legacy Mode Removed

Previously, if a user provided multiple cookbook paths to Chef Solo that contained cookbooks with the same name, Chef Solo would combine these into a single cookbook. This merging of two cookbooks often caused unexpected outcomes and has been removed.

#### Removal of unused route resource properties

The `route` resource contained multiple unused properties that have been removed. If you previously set `networking`, `networking_ipv6`, `hostname`, `domainname`, or `domain`, they would be ignored. In Chef Infra Client 15, setting these properties will throw an error.

#### FreeBSD pkg provider removal

Support for the FreeBSD `pkg` package system in the `freebsd_package` resource has been removed. FreeBSD 10 replaced the `pkg` system with `pkg-ng` system, so this removal only impacts users of EOL FreeBSD releases.

#### require_recipe removal

The legacy `require_recipe` method in recipes has been removed. This method was replaced with `include_recipe` in Chef Client 10, and a FoodCritic rule has been warning to update cookbooks for multiple years.

#### Legacy shell_out methods removed

In Chef Client 14, many of the more obscure `shell_out` methods used in LWRPs and custom resources were combined into the standard `shell_out` and `shell_out!` methods. The legacy methods were infrequently used and Chef Client 14/Foodcritic both contained deprecation warnings for these methods. The following methods will now throw an error: `shell_out_compact`, `shell_out_compact!`, `shell_out_compact_timeout`, `shell_out_compact_timeout!`, `shell_out_with_systems_locale`, and `shell_out_with_systems_locale!`.

#### knife bootstrap --identity_file removal

The `knife bootstrap --identity_file` flag has been removed. This flag was deprecated in Chef Client 12, and users should now use the `--ssh-identity-file` flag instead.

### knife user support for Chef Infra Server < 12 removed

The `knife user` command no longer supports the open source Chef Infra Server version prior to 12.

#### attributes in metadata.rb

Chef Infra Client no longer processes attributes in the `metadata.rb` file. Attributes could be defined in the `metadata.rb` file as a form of documentation, which would be shown when running `knife cookbook show COOKBOOK_NAME`. Often, these attribute definitions would become out of sync with the attributes in the actual attributes files. Chef Infra Client 15 will no longer show these attributes when running `knife cookbook show COOKBOOK_NAME` and will instead throw a warning message upon upload. Foodcritic has warned against the use of attributes in the `metadata.rb` file since April 2017.

#### Node attributes array bugfix

Chef Infra Client 15 includes a bugfix for incorrect node attribute behavior involving a rare usage of arrays, which may impact users who depend on the incorrect behavior.

Previously, you could set an attribute like this:

```
node.default["foo"] = []
node.default["foo"] << { "bar" => "baz }
```

This would result in a Hash, instead of a VividMash, inserted into the AttrArray, so that:

```
node.default["foo"][0]["bar"] # gives the correct result
node.default["foo"][0][:bar]  # does not work due to the sub-Hash not
                              # converting keys
```

The new behavior uses a Mash so that the attributes will work as expected.

#### Ohai's system_profile plugin for macOS removed

We removed the `system_profile` plugin because it incorrectly returned data on modern macOS systems. If you relied on this plugin, you'll want to update recipes to use `node['hardware']` instead, which correctly returns the same data, but in a more easily consumed format. Removing this plugin speeds up Ohai and Chef Infra Client by ~3 seconds, and dramatically reduces the size of the node object on the Chef Infra Server.

#### Ohai's Ohai::Util::Win32::GroupHelper class has been removed

We removed the `Ohai::Util::Win32::GroupHelper` helper class from Ohai. This class was intended for use internally in several Windows plugins, but it was never marked private in the codebase. If any of your Ohai plugins rely on this helper class, you will need to update your plugins for Ohai 15.

#### Audit Mode

Chef Client's Audit mode was introduced in 2015 as a beta that needed to be enabled via `client.rb`. Its functionality has been superseded by Chef InSpec and has been removed.

#### Ohai system_profiler plugin removal

The `system_profiler` plugin, which ran on macOS systems, has been removed. This plugin took longer to run than all other plugins on macOS combined, and no longer produced usable information on modern macOS releases. If you're looking for similar information, it can now be found in the `hardware` plugin.

#### Ohai::Util::Win32::GroupHelper helper removal

The deprecated `Ohai::Util::Win32::GroupHelper` helper has been removed from Ohai. Any custom Ohai plugins using this helper will need to be updated.

#### Ohai::System.refresh_plugins method removal

The `refresh_plugins` method in the `Ohai::System` class has been removed as it has been unused for multiple major Ohai releases. If you are programmatically using Ohai in your own Ruby application, you will need to update your code to use the `load_plugins` method instead.

#### Ohai Microsoft VirtualPC / VirtualServer detection removal

The `Virtualization` plugin will no longer detect systems running on the circa ~2005 VirtualPC or VirtualServer hypervisors. These hypervisors were long ago deprecated by Microsoft and support can no longer be tested.

## What's New in 14.15

### Updated Resources

#### ifconfig

The `ifconfig` resource has been updated to properly support interfaces with a hyphen in their name. This is most commonly encountered with bridge interfaces that are named `br-1234`. Additionally, the `ifconfig` resource now supports the latest ifconfig binaries found in OS releases such as Debian 10.

#### windows_task

The `windows_task` resource now supports the Start When Available option with a new `start_when_available` property. Issues that prevented the resource from being idempotent on Windows 2016 and 2019 hosts have also been resolved.

### Platform Support

#### New Platforms

Chef Infra Client is now tested against the following platforms with packages available on [downloads.chef.io](https://downloads.chef.io):

- Ubuntu 20.04
- Ubuntu 18.04 aarch64
- Debian 10

#### Retired Platforms

- Chef Infra Clients packages are no longer produced for Windows 2008 R2 as this release reached its end of life on Jan 14th, 2020.
- Chef Infra Client packages are no longer produced for RHEL 6 on the s390x platform.

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2u to resolve [CVE-2019-1551](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1551)

#### Ruby

Ruby has been updated from 2.5.7 to 2.5.8 to resolve the following CVEs:

- [CVE-2020-16255](https://www.ruby-lang.org/en/news/2020/03/19/json-dos-cve-2020-10663/): Unsafe Object Creation Vulnerability in JSON (Additional fix)
- [CVE-2020-10933](https://www.ruby-lang.org/en/news/2020/03/31/heap-exposure-in-socket-cve-2020-10933/): Heap exposure vulnerability in the socket library

## What's New in 14.14.29

### Bug Fixes

 - Fixed an error with the `service` and `systemd_unit` resources which would try to re-enable services with an indirect status.
 - The `systemd_unit` resource now logs at the info level.
 - Fixed knife config when it returned a `TypeError: no implicit conversion of nil into String` error.

### Security Updates

#### libxslt

libxslt has been updated to 1.1.34 to resolve [CVE-2019-13118](https://nvd.nist.gov/vuln/detail/CVE-2019-13118).

## What's New in 14.14.25

### Bug Fixes

- Resolved a regression introduced in Chef Infra Client 14.14.14 that broke installation of gems in some scenarios
- Fixed Habitat packaging of `chef-client` artifacts
- Fixed crash in knife when displaying a missing profile error message
- Fixed knife subcommand --help not working as intended for some commands
- Fixed knife ssh interactive mode exit error
- Fixed for `:day` option not accepting integer value in the `windows_task` resource
- Fixed for `user` resource not handling a GID if it is specified as a string
- Fixed the `ifconfig` resource to support interfaces with a `-` in the name

## What's New in 14.14.14

### Platform Updates

#### Newly Supported Platforms

The following platforms are now packaged and tested for Chef Infra Client:

- Red Hat 8
- FreeBSD 12
- macOS 10.15
- Windows 2019
- AIX 7.2

#### Deprecated Platforms

The following platforms have reached EOL status and are no longer packaged or tested for Chef Infra Client:

- FreeBSD 10
- macOS 10.12
- SUSE Linux Enterprise Server (SLES) 11
- Ubuntu 14.04

See Chef's [Platform End-of-Life Policy](/platforms/#platform-end-of-life-policy) for more information on when Chef ends support for an OS release.

### Updated Resources

#### dnf_package

The `dnf_package` resource has been updated to fully support RHEL 8.

#### zypper_package

The `zypper_package` resource has been updated to properly update packages when using the `:upgrade` action.

#### remote_file

The `remote_file` resource now properly shows download progress when the `show_progress` property is set to true.

### Improvements

### Custom Resource Unified Mode

Chef Infra Client 14.14 introduces an exciting new way to easily write custom resources that mix built-in Chef Infra resources with Ruby code. Previously, custom resources would use Chef Infra's standard compile and converge phases, which meant that Ruby would be evaluated first and then the resources would be converged. This often results in confusing and undesirable behavior when you are trying to mix resources with Ruby logic. Many custom resource authors would attempt to get around this by forcing resources to run at compile time so that all the code in their resource would execute during the compile phase.

An example of forcing a resource to run at compile time:

```ruby
resource_name 'foo' do
  action :nothing
end.run_action(:some_action)
```

With unified mode, you opt in to a single phase per resource where all Ruby and Chef Infra resources are executed at once. This makes it far easier to determine how your code will be evaluated and run. Additionally, you no longer need to force any resources to run at compile time, as all code is run in the compile phase. To enable this new mode just add `unified_mode true` to your resources like this:

```ruby
property :Some_property, String

unified_mode true

action :create do
  # some code
end
```

#### New Options for installing Ruby Gems From metadata.rb

Chef Infra Client allows gems to be specified in the cookbook metadata.rb, which can be problematic in some environments. When a cookbook is running in an airgapped environment, Chef Infra Client attempts to connect to rubygems.org even if the gem is already on the system. There are now two additional configuration options that can be set in your `client.rb` config:
    - `gem_installer_bundler_options`: This allows setting additional bundler options for the install such as  --local to install from local cache. Example: ["--local", "--clean"].
    - `skip_gem_metadata_installation`: If set to true skip gem metadata installation if all gems are already installed.

#### SLES / openSUSE 15 detection

Ohai now properly detects SLES and openSUSE 15.x. Thanks for this fix [@balasankarc](https://gitlab.com/balasankarc).

#### Performance Improvements

We have improved the performance of Chef Infra Client by resolving bundler errors in our packaging.

#### Bootstrapping Chef Infra Client 15 will no fail

Knife now fails with a descriptive error message when attempting to bootstrap nodes with Chef Infra Client 15. You will need to bootstrap these nodes using Knife from Chef Infra Client 15.x. We recommend performing this bootstrap from Chef Workstation, which includes the Knife CLI in addition to other useful tools for managing your infrastructure with Chef Infra.

### Security Updates

#### Ruby

Ruby has been updated from 2.5.5 to 2.5.7 in order to resolve the following CVEs:

- [CVE-2012-6708](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2012-6708)
- [CVE-2015-9251](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-9251).
- [CVE-2019-16201](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-15845).
- [CVE-2019-15845](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-9251).
- [CVE-2019-16254](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16254).
- [CVE-2019-16255](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16255).

#### openssl

openssl has been updated from 1.0.2s to 1.0.2t in order to resolve [CVE-2019-1563](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1563) and [CVE-2019-1547](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1547).

#### nokogiri

nokogiri has been updated from 1.10.2 to 1.10.4 in order to resolve [CVE-2019-5477](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-5477).

## What's New in 14.13

### Updated Resources

#### directory

The `directory` has been updated to properly set the `deny_rights` permission on Windows. Thanks [@merlinjim](https://github.com/merlinjim) for reporting this issue.

#### service

The `service` resource is now idempotent on SLES 11 systems. Thanks [@gsingla294](https://github.com/gsingla294) for reporting this issue.

#### cron

The `cron` resource has been updated to advise users to use the specify properties rather than passing values in as part of the `environment` property. This avoids a situation where a user could pass the differing values in both locations and receive unexpected results.

#### link

The `link` resource includes improved logging upon failure to help you debug what has failed. Thanks [@jaymzh](https://github.com/jaymzh) for this improvement.

#### template

The `template` resource now includes additional information when templating failures, which is particularly useful in ChefSpec. Thanks [@brodock](https://github.com/brodock) for this improvement.

### delete_resource Fix

The `delete_resource` helper now works properly when the resource you are attempting to delete has multiple providers. Thanks [@artem-sidorenko](https://github.com/artem-sidorenko) for this fix.

### Helpers Help Everywhere

Various helpers have been moved into Chef Infra Client's `universal` class, which makes them available anywhere in your cookbook, not just recipes. If you've ever been confused why something like `search`, `powershell_out`, or `data_bag_item` didn't work somewhere in your code, that should be resolved now.

### Deprecations

The `CHEF-25` deprecation for resource collisions between cookbooks and resources in Chef Infra Client has been removed. Instead you will see a log warning that a collision has occurred, which advises you to update your run_list or cookbooks.

### Updated Components

- openssl 1.0.2r -> 1.0.2s (bugfix only release)
- cacerts 2019-01-23 -> 2019-05-15

## What's New in 14.12.9

### License Acceptance Placeholder Flag

In preparation for Chef Infra Client 15.0 we've added a placeholder `--chef-license` flag to the chef-client command. This allows you to use the new `--chef-license` flag on both Chef Infra Client 14.12.9+ and 15+ notes without producing errors on Chef Infra Client 14.

### Important Bug Fixes

- Blacklisting and whitelisting default and override level attributes is once again possible.
- You may now encrypt a previously unencrypted data bag.
- Resolved a regression introduced in Chef Infra Client 14.12.3 that resulted in errors when managing Windows services

## What's New in 14.12

### Updated Resources

#### windows_service

The windows_service resource no longer resets credentials on a service when using the :start action without the :configure action. Thanks [@jasonwbarnett](https://github.com/jasonwbarnett) for fixing this.

#### windows_certificate

The windows_certificate resource now imports nested certificates while importing P7B certs.

### Updated Components

- nokogiri 1.10.1 -> 1.10.2
- ruby 2.5.3 -> 2.5.5
- InSpec 3.7.1 -> 3.9.0
- The unused windows-api gem is no longer bundled with Chef on Windows hosts

## What's New in 14.11

### Updated Resources

#### chocolatey_package

The chocolatey_package resource now uses the provided options to fetch information on available packages, which allows installation packages from private sources. Thanks [@astoltz](https://github.com/astoltz) for reporting this issue.

#### openssl_dhparam

The openssl_dhparam resource now supports updating the dhparam file's mode on subsequent chef-client runs. Thanks [@anewb](https://github.com/anewb) for the initial work on this fix.

#### mount

The mount resource now properly adds a blank line between entries in fstab to prevent mount failures on AIX.

#### windows_certificate

The windows_certificate resource now supports importing Base64 encoded CER certificates and nested P7B certificates. Additionally, private keys in PFX certificates are now imported along with the certificate.

#### windows_share

The windows_share resource has improved logic to compare the desired share path vs. the current path, which prevents the resource from incorrectly converging during each Chef run. Thanks [@Xorima](https://github.com/xorima) for this fix.

#### windows_task

The windows_task resource now properly clears out arguments that are no longer present when updating a task. Thanks [@nmcspadden](https://github.com/nmcspadden) for reporting this.

### InSpec 3.7.1

InSpec has been updated from 3.4.1 to 3.7.1. This new release contains improvements to the plugin system, a new config file system, and improvements to multiple resources. Additionally, profile attributes have also been renamed to inputs to prevent confusion with Chef attributes, which weren't actually related in any way.

### Updated Components

- bundler 1.16.1 -> 1.17.3
- libxml2 2.9.7 -> 2.9.9
- ca-certs updated to 2019-01-22 for new roots

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2r in order to resolve [CVE-2019-1559](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1559)

#### RubyGems

RubyGems has been updated to 2.7.9 in order to resolve the following CVEs:

- [CVE-2019-8320](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8320): Delete directory using symlink when decompressing tar
- [CVE-2019-8321](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8321): Escape sequence injection vulnerability in verbose
- [CVE-2019-8322](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8322): Escape sequence injection vulnerability in gem owner
- [CVE-2019-8323](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8323): Escape sequence injection vulnerability in API response handling
- [CVE-2019-8324](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8324): Installing a malicious gem may lead to arbitrary code execution
- [CVE-2019-8325](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8325): Escape sequence injection vulnerability in errors

## What's New in 14.10

### Updated Resources

#### windows_certificate

The windows_certificate resource is now fully idempotent and properly imports private keys. Thanks [@Xorima](https://github.com/Xorima) for reporting these issues.

#### apt_repository

The apt_repository resource no longer creates .gpg directory in the user's home directory owned by root when installing repository keys. Thanks [@omry](http://github.com/omry) for reporting this issue.

#### git

The git resource no longer displays the URL of the repository if the `sensitive` property is set.

### InSpec 3.4.1

InSpec has been updated from 3.2.6 to 3.4.1. This new release adds new `aws_billing_report` / `aws_billing_reports` resources, resolves multiple bugs, and includes tons of under the hood improvements.

### New Deprecations

#### knife cookbook site

Since Chef 13, `knife cookbook site` has actually called the `knife supermarket` command under the hood. In Chef 16 (April 2020), we will remove the `knife cookbook site` command in favor of `knife supermarket`.

#### Audit Mode

Chef's Audit mode was introduced in 2015 as a beta that needed to be enabled via client.rb. Its functionality has been superceded by InSpec and we will be removing this beta feature in Chef Infra Client 15 (April 2019).

#### Cookbook Shadowing

Cookbook shadowing was deprecated in 0.10 and will be removed in Chef Infra Client 15 (April 2019). Cookbook shadowing allowed combining cookbooks within a mono-repo, so long as the cookbooks in question had the same name and were present in both the cookbooks directory and the site-cookbooks directory.

## What's New in 14.9

### Updated Resources

#### group

On Windows hosts, the group resource now supports setting the comment field via a new `comment` property.

#### homebrew_cask

Two issues, which caused homebrew_cask to converge on each Chef run, have been resolved. Thanks [@jeroenj](https://github.com/jeroenj) for this fix. Additionally, the resource will no longer fail if the `cask_name` property is specified.

#### homebrew_tap

The homebrew_tap resource no longer fails if the `tap_name` property is specified.

#### openssl_x509_request

The openssl_x509_request resource now properly writes out the CSR file if the `path` property is specified. Thank you [@cpjones](https://github.com/cpjones) for reporting this issue.

#### powershell_package_source

powershell_package_source now suppresses warnings, which prevented properly loading the resource state, and resolves idempotency issues when both the `name` and `source_name` properties were specified. Thanks [@Happycoil](https://github.com/Happycoil) for this fix.

#### sysctl

The sysctl resource now allows slashes in the key or block name. This allows keys such as `net/ipv4/conf/ens256.401/rp_filter` to be used with this resource.

#### windows_ad_join

Errors joining the domain are now properly suppressed from the console and logs if the `sensitive` property is set to true. Thanks [@Happycoil](https://github.com/Happycoil) for this improvement.

#### windows_certificate

The delete action now longer fails if a certificate does not exist on the system. Additionally, certificates with special characters in their passwords will no longer fail. Thank you for reporting this [@chadmccune](https://github.com/chadmccune).

#### windows_printer

The windows_printer resource no longer fails when creating or deleting a printer if the `device_id` property is specified.

#### windows_task

Non-system users can now run tasks without a password being specified.

### Minimal Ohai Improvements

The ohai `init_package` plugin is now included as part of the `minimal_ohai` plugins set, which allows resources such as timezone to continue to function if Chef is running with the minimal number of ohai plugins.

### Ruby 2.6 Support

Chef 14.9 now supports Ruby 2.6.

### InSpec 3.2.6

InSpec has been updated from 3.0.64 to 3.2.6 with improved resources for auditing. See the [InSpec changelog](https://github.com/inspec/inspec/blob/master/CHANGELOG.md#v326-2018-12-20) for additional details on this new version.

### powershell_exec Runtimes Bundled

The necessary VC++ runtimes for the powershell_exec helper are now bundled with Chef to prevent failures on hosts that lacked the runtimes.

## What's New in 14.8

### Updated Resources

#### apt_package

The apt_package resource now supports using the `allow_downgrade` property to enable downgrading of packages on a node in order to meet a specified version. Thank you [@whiteley](https://github.com/whiteley) for requesting this enhancement.

#### apt_repository

An issue was resolved in the apt_repository resource that caused the resource to fail when importing GPG keys on newer Debian releases. Thank you [@EugenMayer](https://github.com/EugenMayer) for this fix.

#### dnf_package / yum_package

Initial support has been added for Red Hat Enterprise Linux 8. Thank you [@pixdrift](https://github.com/pixdrift) for this fix.

#### gem_package

gem_package now supports installing gems into Ruby 2.6 or later installations.

#### windows_ad_join

windows_ad_join now uses the UPN format for usernames, which prevents some failures authenticating to the domain.

#### windows_certificate

An issue was resolved in the :acl_add action of the windows_certificate resource, which caused the resource to fail. Thank you [@shoekstra](https://github.com/shoekstra) for reporting this issue.

#### windows_feature

The windows_feature resource now allows for the installation of DISM features that have been fully removed from a system. Thank you [@zanecodes](https://github.com/zanecodes) for requesting this enhancement.

#### windows_share

Multiple issues were resolved in windows_share, which caused the resource to either fail or update the share state on every Chef Client run. Thank you [@chadmccune](https://github.com/chadmccune) for reporting several of these issues and [@derekgroh](https://github.com/derekgroh) for one of the fixes.

#### windows_task

A regression was resolved that prevented ChefSpec from testing the windows_task resource in Chef Client 14.7. Thank you [@jjustice6](https://github.com/jjustice6) for reporting this issue.

### Ohai 14.8

#### Improved Virtualization Detection

**Hyper-V Hypervisor Detection**

Detection of Linux guests running on Hyper-V has been improved. In addition, Linux guests on Hyper-V hypervisors will also now detect their hypervisor's hostname. Thank you [@safematix](https://github.com/safematix) for contributing this enhancement.

Example `node['virtualization']` data:

```json
{
  "systems": {
    "hyperv": "guest"
  },
  "system": "hyperv",
  "role": "guest",
  "hypervisor_host": "hyper_v.example.com"
}
```

**LXC / LXD Detection**

On Linux systems running lxc or lxd containers, the lxc/lxd virtualization system will now properly populate the `node['virtualization']['systems']` attribute.

**BSD Hypervisor Detection**

BSD-based systems can now detect guests running on KVM and Amazon's hypervisor without the need for the dmidecode package.

#### New Platform Support

- Ohai now properly detects the openSUSE 15.X platform. Thank you [@megamorf](https://github.com/megamorf) for reporting this issue.
- SUSE Linux Enterprise Desktop now identified as platform_family 'suse'
- XCP-NG is now identified as platform 'xcp' and platform_family 'rhel'. Thank you [@heyjodom](http://github.com/heyjodom) for submitting this enhancement.
- Mangeia Linux is now identified as platform 'mangeia' and platform_family 'mandriva'
- Antergos Linux now identified as platform_family 'arch'
- Manjaro Linux now identified as platform_family 'arch'

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2q in order to resolve:

- Microarchitecture timing vulnerability in ECC scalar multiplication [CVE-2018-5407](https://nvd.nist.gov/vuln/detail/CVE-2018-5407)
- Timing vulnerability in DSA signature generation ([CVE-2018-0734](https://nvd.nist.gov/vuln/detail/CVE-2018-0734))

## What's New in 14.7

### New Resources

#### windows_firewall_rule

Use the `windows_firewall_rule` resource create or delete Windows Firewall rules.

See the [windows_firewall_rule](https://docs.chef.io/resources/windows_firewall_rule) documentation for more information.

Thank you [Schuberg Philis](https://schubergphilis.com/) for transferring us the [windows_firewall cookbook](https://supermarket.chef.io/cookbooks/windows_firewall) and to [@Happycoil](https://github.com/Happycoil) for porting it to chef-client with a significant refactoring.

#### windows_share

Use the `windows_share` resource create or delete Windows file shares.

See the [windows_share](https://docs.chef.io/resources/windows_share) documentation for more information.

#### windows_certificate

Use the `windows_certificate` resource add, remove, or verify certificates in the system or user certificate stores.

See the [windows_certificate](https://docs.chef.io/resources/windows_certificate) documentation for more information.

### Updated Resources

#### dmg_package

The dmg_package resource has been refactored to improve idempotency and properly support accepting a DMG's EULA with the `accept_eula` property.

#### kernel_module

Kernel_module now only runs the `initramfs` update once per Chef run to greatly speed up chef-client runs when multiple kernel_module resources are used. Thank you [@tomdoherty](https://github.com/tomdoherty) for this improvement.

#### mount

The `supports` property once again allows passing supports data as an array. This matches the behavior present in Chef 12.

#### timezone

macOS support has been added to the timezone resource.

#### windows_task

A regression in Chef 14.6's windows_task resource which resulted in tasks being created with the "Run only when user is logged on" option being set when created with a specific user other than SYSTEM, has been resolved.

## What's New in 14.6

### Smaller Package and Install Size

Both Chef packages and on disk installations have been greatly reduced in size by trimming unnecessary installation files. This has reduced our package size on macOS/Linux by ~50% and Windows by ~12%. With this change Chef 14 is now smaller than a legacy Chef 10 package.

### New Resources

#### timezone

Chef now includes the `timezone` resource from [@dragonsmith](http://github.com/dragonsmith)'s `timezone_lwrp` cookbook. This resource supports setting a Linux node's timezone. Thank you [@dragonsmith](http://github.com/dragonsmith) for allowing us to include this out of the box in Chef.

Example:

```ruby
timezone 'UTC'
```

### Updated Resources

#### windows_task

The `windows_task` resource has been updated to support localized system users and groups on non-English nodes. Thanks [@jugatsu](http://github.com/jugatsu) for making this possible.

#### user

The `user` resource now includes a new `full_name` property for Windows hosts, which allows specifying a user's full name.

Example:

```ruby
user 'jdoe' do
  full_name 'John Doe'
end
```

#### zypper_package

The `zypper_package` resource now includes a new `global_options` property. This property can be used to specify one or more options for the zypper command line that are global in context.

Example:

```ruby
package 'sssd' do
   global_options '-D /tmp/repos.d/'
end
```

### InSpec 3.0

Inspec has been updated to version 3.0 with addition resources, exception handling, and a new plugin system. See <https://blog.chef.io/2018/10/16/announcing-inspec-3-0/> for details.

### macOS Mojave (10.14)

Chef is now tested against macOS Mojave, and packages are now available at downloads.chef.io.

### Important Bugfixes

- Multiple bugfixes in Chef Vault have been resolved by updating chef-vault to 3.4.2
- Invalid yum package names now gracefully fail
- `windows_ad_join` now properly executes. Thank you [@cpjones01](https://github.com/cpjones01) for reporting this.
- `rhsm_errata_level` now properly executes. Thank you [@freakinhippie](https://github.com/freakinhippie) for this fix.
- `registry_key` now properly writes out the correct value when `sensitive` is specified. Thank you [@josh-barker](https://github.com/josh-barker) for this fix.
- `locale` now properly executes on RHEL 6 and Amazon Linux 201X.

### Ohai 14.6

#### Filesystem Plugin on AIX and Solaris

AIX and Solaris now ship with a filesystem2 plugin that updates the filesystem data to match that of Linux, macOS, and BSD hosts. This new data structure makes accessing filesystem data in recipes easier and especially improves the layout and depth of data on ZFS filesystems. In Chef Infra Client 15 (April 2019) we will begin writing this same format of data to the existing `node['filesystem']` namespace. In Chef 16 (April 2020) we will remove the `node['filesystem2']` namespace, completing the transition to the new format. Thank you [@jaymzh](https://github.com/jaymzh) for continuing the updates to our filesystem plugins with this change.

#### macOS Improvements

The system_profile plugin has been improved to skip over unnecessary data, which reduces macOS node sizes on the Chef Server. Additionally the CPU plugin has been updated to limit what sysctl values it polls, which prevents hanging on some system configurations.

#### SLES 15 Detection

SLES 15 is now correctly detected as the platform "suse" instead of "sles". This matches the behavior of SLES 11 and 12 hosts.

### New Deprecations

#### system_profile Ohai plugin removal

The system_profile plugin will be removed from Chef/Ohai 15 in April 2019. This plugin does not correctly return data on modern Mac systems. Additionally the same data is provided by the hardware plugin, which has a format that is simpler to consume. Removing this plugin will reduce Ohai return by ~3 seconds and greatly reduce the size of the node object on the Chef server.

### Security Updates

#### Ruby 2.5.3

Ruby has been updated to from 2.5.1 to 2.5.3 to resolve multiple CVEs and bugs:

- [CVE-2018-16396](https://www.ruby-lang.org/en/news/2018/10/17/not-propagated-taint-flag-in-some-formats-of-pack-cve-2018-16396/)
- [CVE-2018-16395](https://www.ruby-lang.org/en/news/2018/10/17/openssl-x509-name-equality-check-does-not-work-correctly-cve-2018-16395/)

## What's New in 14.5.33

This release resolves a regression that caused the ``windows_ad_join`` resource to fail to run. It also makes the following additional fixes:

  - The ``ohai`` resource's unused ``ohai_name`` property has been deprecated. This will be removed in Chef Infra Client 15.0.
  - Error messages in the ``windows_feature`` resources have been improved.
  - The ``windows_service`` resource will no longer log potentially sensitive information if the ``sensitive`` property is used.

Thanks to @cpjones01, @kitforbes, and @dgreeninger for their help with this release.

## What's New in 14.5.27

### New Resources

We've added new resources to Chef 14.5. Cookbooks using these resources will continue to take precedent until the Chef Infra Client 15.0 release

#### windows_workgroup

Use the `windows_workgroup` resource to join or change a Windows host workgroup.

See the [windows_workgroup](https://docs.chef.io/resources/windows_workgroup) documentation for more information.

Thanks [@derekgroh](https://github.com/derekgroh) for contributing this new resource.

#### locale

Use the `locale` resource to set the system's locale.

See the [locale](https://docs.chef.io/resources/locale) documentation for more information.

Thanks [@vincentaubert](https://github.com/vincentaubert) for contributing this new resource.

### Updated Resources

#### windows_ad_join

`windows_ad_join` now includes a `new_hostname` property for setting the hostname for the node upon joining the domain.

Thanks [@derekgroh](https://github.com/derekgroh) for contributing this new property.

### InSpec 2.2.102

InSpec has been updated from 2.2.70 to 2.2.102. This new version includes the following improvements:

  - Support for using ERB templating within the .yml files
  - HTTP basic auth support for fetching dependent profiles
  - A new global attributes concept
  - Better error handling with Automate reporting
  - Vendor command now vendors profiles when using path://

### Ohai 14.5

#### Windows Improvements

Detection for the `root_group` attribute on Windows has been simplified and improved to properly support non-English systems. With this change, we've also deprecated the `Ohai::Util::Win32::GroupHelper` helper, which is no longer necessary. Thanks to [@jugatsu](https://github.com/jugatsu) for putting this together.

We've also added a new `encryption_status` attribute to volumes on Windows. Thanks to [@kmf](https://github.com/kmf) for suggesting this new feature.

#### Configuration Improvements

The timeout period for communicating with OpenStack metadata servers can now be configured with the `openstack_metadata_timeout` config option. Thanks to [@sawanoboly](https://github.com/sawanoboly) for this improvement.

Ohai now properly handles relative paths to config files when running on the command line. This means commands like `ohai -c ../client.rb` will now properly use your config values.

### Security updates

#### Rubyzip

The rubyzip gem has been updated to 1.2.2 to resolve [CVE-2018-1000544](https://www.cvedetails.com/cve/CVE-2018-1000544/)

## What's New in 14.4

### Knife configuration profile management commands

Several new commands have been added under `knife config` to help manage multiple
profiles in your `credentials` file.

`knife config get-profile` displays the active profile.

`knife config use-profile PROFILE` sets the workstation-level default
profile. You can still override this setting with the `--profile` command line
option or the `$CHEF_PROFILE` environment variable.

`knife config list-profiles` displays all your available profiles along with
summary information on each.

```bash
$ knife config get-profile
staging
$ knife config use-profile prod
Set default profile to prod
$ knife config list-profiles
 Profile  Client  Key               Server
-----------------------------------------------------------------------------
 staging  myuser  ~/.chef/user.pem  https://example.com/organizations/staging
*prod     myuser  ~/.chef/user.pem  https://example.com/organizations/prod
```

Thank you [@coderanger](https://github.com/coderanger) for this contribution.

### New Resources

The following new previous resources were added to Chef 14.4. Cookbooks with the same resources will continue to take precedent until the Chef Infra Client 15.0 release

#### cron_d

Use the [cron_d](https://docs.chef.io/resources/cron_d) resource to manage cron definitions in /etc/cron.d. This is similar to the `cron` resource, but it does not use the monolithic `/etc/crontab`. file.

#### cron_access

Use the [cron_access](https://docs.chef.io/resources/cron_access) resource to manage the `/etc/cron.allow` and `/etc/cron.deny` files. This resource previously shipped in the `cron` community cookbook and has fully backwards compatibility with the previous `cron_manage` definition in that cookbook.

#### openssl_x509_certificate

Use the [openssl_x509_certificate](https://docs.chef.io/resources/openssl_x509_certificate) resource to generate signed or self-signed, PEM-formatted x509 certificates. If no existing key is specified, the resource automatically generates a passwordless key with the certificate. If a CA private key and certificate are provided, the certificate will be signed with them. This resource previously shipped in the `openssl` cookbook as `openssl_x509` and is fully backwards compatible with the legacy resource name.

Thank you [@juju482](https://github.com/juju482) for updating this resource!

#### openssl_x509_request

Use the [openssl_x509_request](https://docs.chef.io/resources/openssl_x509_request) resource to generate PEM-formatted x509 certificates requests. If no existing key is specified, the resource automatically generates a passwordless key with the certificate.

Thank you [@juju482](https://github.com/juju482) for contributing this resource.

#### openssl_x509_crl

Use the [openssl_x509_crl](https://docs.chef.io/resources/openssl_x509_crl)l resource to generate PEM-formatted x509 certificate revocation list (CRL) files.

Thank you [@juju482](https://github.com/juju482) for contributing this resource.

#### openssl_ec_private_key

Use the [openssl_ec_private_key](https://docs.chef.io/resources/openssl_ec_private_key) resource to generate ec private key files. If a valid ec key file can be opened at the specified location, no new file will be created.

Thank you [@juju482](https://github.com/juju482) for contributing this resource.

#### openssl_ec_public_key

Use the [openssl_ec_public_key](https://docs.chef.io/resources/openssl_ec_public_key) resource to generate ec public key files given a private key.

Thank you [@juju482](https://github.com/juju482) for contributing this resource.

### Resource improvements

#### windows_package

The windows_package resource now supports setting the `sensitive` property to avoid showing errors if a package install fails.

#### sysctl

The sysctl resource will now update the on-disk `sysctl.d` file even if the current sysctl value matches the desired value.

#### windows_task

The windows_task resource now supports setting the task priority of the scheduled task with a new `priority` property. Additionally windows_task now supports managing the behavior of task execution when a system is on battery using new `disallow_start_if_on_batteries` and `stop_if_going_on_batteries` properties.

#### ifconfig

The ifconfig resource now supports setting the interface's VLAN via a new `vlan` property on RHEL `platform_family` and setting the interface's gateway via a new `gateway` property on RHEL/Debian `platform_family`.

Thank you [@tomdoherty](https://github.com/tomdoherty) for this contribution.

#### route

The route resource now supports additional RHEL platform_family systems as well as Amazon Linux.

#### systemd_unit

The [systemd_unit](https://docs.chef.io/resources/systemd_unit) resource now supports specifying options multiple times in the content hash. Instead of setting the value to a string you can now set it to an array of strings.

Thank you [@dbresson](https://github.com/dbresson) for this contribution.

### Security Updates

#### OpenSSL

OpenSSL updated to 1.0.2p to resolve:
- Client DoS due to large DH parameter ([CVE-2018-0732](https://nvd.nist.gov/vuln/detail/CVE-2018-0732))
- Cache timing vulnerability in RSA Key Generation ([CVE-2018-0737](https://nvd.nist.gov/vuln/detail/CVE-2018-0737))

## What's New in 14.3

### New Preview Resources Concept

This release of Chef introduces the concept of Preview Resources. Preview resources behave the same as a standard resource built into Chef, except Chef will load a resource with the same name from a cookbook instead of the built-in preview resource.

What does this mean for you? It means we can introduce new resources in Chef without breaking existing behavior in your infrastructure. For instance if you have a cookbook with a resource named `manage_everything` and a future version of Chef introduced a preview resource named `manage_everything` you will continue to receive the resource from your cookbook. That way outside of a major release your won't experience a potentially breaking behavior change from the newly included resource.

Then when we perform our yearly major release we'll remove the preview designation from all resources, and the built in resources will take precedence over resources with the same names in cookbooks.

### New Resources

#### chocolatey_config

Use the chocolatey_config resource to add or remove Chocolatey configuration keys."

**Actions**

- `set` - Sets a Chocolatey config value.
- `unset` - Unsets a Chocolatey config value.

**Properties**

- `config_key` - The name of the config. We'll use the resource's name if this isn't provided.
- `value` - The value to set.

#### chocolatey_source

Use the chocolatey_source resource to add or remove Chocolatey sources.

**Actions**

- `add` - Adds a Chocolatey source.
- `remove` - Removes a Chocolatey source.

**Properties**

- `source_name` - The name of the source to add. We'll use the resource's name if this isn't provided.
- `source` - The source URL.
- `bypass_proxy` - Whether or not to bypass the system's proxy settings to access the source.
- `priority` - The priority level of the source.

#### powershell_package_source

Use the `powershell_package_source` resource to register a PowerShell package repository.

#### Actions

- `register` - Registers and updates the PowerShell package source.
- `unregister` - Unregisters the PowerShell package source.

**Properties**

- `source_name` - The name of the package source.
- `url` - The url to the package source.
- `trusted` - Whether or not to trust packages from this source.
- `provider_name` - The package management provider for the source. It supports the following providers: 'Programs', 'msi', 'NuGet', 'msu', 'PowerShellGet', 'psl' and 'chocolatey'.
- `publish_location` - The url where modules will be published to for this source. Only valid if the provider is 'PowerShellGet'.
- `script_source_location` - The url where scripts are located for this source. Only valid if the provider is 'PowerShellGet'.
- `script_publish_location` - The location where scripts will be published to for this source. Only valid if the provider is 'PowerShellGet'.

#### kernel_module

Use the kernel_module resource to manage kernel modules on Linux systems. This resource can load, unload, blacklist, install, and uninstall modules.

**Actions**

- `install` - Load kernel module, and ensure it loads on reboot.
- `uninstall` - Unload a kernel module and remove module config, so it doesn't load on reboot.
- `blacklist` - Blacklist a kernel module.
- `load` - Load a kernel module.
- `unload` - Unload kernel module

**Properties**

- `modname` - The name of the kernel module.
- `load_dir` - The directory to load modules from.
- `unload_dir` - The modprobe.d directory.

#### ssh_known_hosts_entry

Use the ssh_known_hosts_entry resource to add an entry for the specified host in /etc/ssh/ssh_known_hosts or a user's known hosts file if specified.

**Actions**

- `create` - Create an entry in the ssh_known_hosts file.
- `flush` - Immediately flush the entries to the config file. Without this the actual writing of the file is delayed in the Chef run so all entries can be accumulated before writing the file out.

**Properties**

- `host` - The host to add to the known hosts file.
- `key` - An optional key for the host. If not provided this will be automatically determined.
- `key_type` - The type of key to store.
- `port` - The server port that the ssh-keyscan command will use to gather the public key.
- `timeout` - The timeout in seconds for ssh-keyscan.
- `mode` - The file mode for the ssh_known_hosts file.
- `owner`- The file owner for the ssh_known_hosts file.
- `group` - The file group for the ssh_known_hosts file.
- `hash_entries` - Hash the hostname and addresses in the ssh_known_hosts file for privacy.
- `file_location` - The location of the ssh known hosts file. Change this to set a known host file for a particular user.

### New `knife config get` command

The `knife config get` command has been added to help with debugging configuration issues with `knife` and other tools that use the `knife.rb` file.

With no arguments, it will display all options you've set:

```bash
$ knife config get
Loading from configuration file /Users/.../.chef/knife.rb
chef_server_url: https://...
client_key:      /Users/.../.chef/user.pem
config_file:     /Users/.../.chef/knife.rb
log_level:       warn
log_location:    STDERR
node_name:       ...
validation_key:
```

You can also pass specific keys to only display those `knife config get node_name client_key`, or use `--all` to display everything (including options that are using the default value).

### Simplification of `shell_out` APIs

The following helper methods have been deprecated in favor of the single shell_out helper:

- `shell_out_with_systems_locale`
- `shell_out_with_timeout`
- `shell_out_compact`
- `shell_out_compact_timeout`
- `shell_out_with_systems_locale!`
- `shell_out_with_timeout!`
- `shell_out_compact!`
- `shell_out_compact_timeout!`

The functionality of `shell_out_with_systems_locale` has been implemented using the `default_env: false` option that removes the PATH and locale mangling that has been the default behavior of `shell_out`.

The functionality of `shell_out_compact` has been folded into `shell_out`. The `shell_out` API when called with varargs has its arguments flatted, compacted and coerced to strings. This style of calling is encouraged over using strings and building up commands using `join(" ")` since it avoids shell interpolation and edge conditions in the construction of spaces between arguments. The varargs form is still not supported on Windows.

The functionality of `shell_out*timeout` has also been folded into `shell_out`. Users writing Custom Resources should be explicit for Chef-14: `shell_out!("whatever", timeout: new_resource.timeout)` which will become automatic in Chef-15.

### Silencing deprecation warnings

While deprecation warnings have been great for the Chef community to ensure cookbooks are kept up-to-date and to prepare for major version upgrades, sometimes you just can't fix a deprecation right now. This is often compounded by the recommendation to enable `treat_deprecation_warnings_as_errors` mode in your Test Kitchen integration tests, which doesn't understand the difference between deprecations from community cookbooks and those from your own code.

Two new options are provided for silencing deprecation warnings: `silence_deprecation_warnings` and inline `chef:silence_deprecation` comments.

The `silence_deprecation_warnings` configuration value can be set in your `client.rb` or `solo.rb` config file, either to `true` to silence all deprecation warnings or to an array of deprecations to silence. You can specify which to silence either by the deprecation key name (e.g. `"internal_api"`), the numeric deprecation ID (e.g. `25` or `"CHEF-25"`), or by specifying the filename and line number where the deprecation is being raised from (e.g. `"default.rb:67"`).

An example of setting the `silence_deprecation_warnings` option in your `client.rb` or `solo.rb`:

```ruby
silence_deprecation_warnings %w{deploy_resource chef-23 recipes/install.rb:22}
```

or in your `kitchen.yml`:

```yaml
provisioner:
  name: chef_solo
  solo_rb:
    treat_deprecation_warnings_as_errors: true
    silence_deprecation_warnings:
    - deploy_resource
    - chef-23
    - recipes/install.rb:22
```

You can also silence deprecations using a comment on the line that is raising the warning:

```ruby
erl_call 'something' do # chef:silence_deprecation
```

We advise caution in the use of this feature, as excessive or prolonged silencing can lead to difficulty upgrading when the next major release of Chef comes out.

### Misc Windows improvements

- A new `skip_publisher_check` property has been added to the `powershell_package` resource
- `windows_feature_powershell` now supports Windows 2008 R2
- The `mount` resource now supports the `mount_point` property on Windows
- `windows_feature_dism` no longer errors when specifying the source
- Resolved idempotency issues in the `windows_task` resource and prevented setting up a task with bad credentials
- `windows_service` no longer throws Ruby deprecation warnings

### Newly Introduced Deprecations

#### CHEF-26: Deprecation of old shell_out APIs

As noted above, this release of Chef unifies our shell_out helpers into just shell_out and shell_out!. Previous helpers are now deprecated and will be removed in Chef Infra Client 15.

See [CHEF-26 Deprecation Page](https://docs.chef.io/deprecations_shell_out) for details.

#### Legacy FreeBSD pkg provider

Chef Infra Client 15 will remove support for the legacy FreeBSD pkg format. We will continue to support the pkgng format introduced in FreeBSD 10.

## What's New in 14.2

### `ssh-agent` support for user keys

You can now use `ssh-agent` to hold your user key when using knife. This allows storing your user key in an encrypted form as well as using `ssh -A` agent forwarding for running knife commands from remote devices.

You can enable this by adding `ssh_agent_signing true` to your `knife.rb` or `ssh_agent_signing = true` in your `credentials` file.

To encrypt your existing user key, you can use OpenSSL:

```
( openssl rsa -in user.pem -pubout && openssl rsa -in user.pem -aes256 ) > user_enc.pem
chmod 600 user_enc.pem
```

This will prompt you for a passphrase for to use to encrypt the key. You can then load the key into your `ssh-agent` by running `ssh-add user_enc.pem`. Make sure you add the `ssh_agent_signing` to your configuration, and update your `client_key` to point at the new, encrypted key (and once you've verified things are working, remember to delete your unencrypted key file).

### default_env Property in Execute Resource

The shell_out helper has been extended with a new option `default_env` to allow disabling Chef from modifying PATH and LOCALE environmental variables as it shells out. This new option defaults to true (modify the env), preserving the previous behavior of the helper.

The execute resource has also been updated with a new property `default_env` that allows utilizing this the ENV sanity functionality in shell_out. The new property defaults to false, but it can be set to true in order to ensure a sane PATH and LOCALE when shelling out. If you find that binaries cannot be found when using the execute resource, `default_env` set to true may resolve those issues.

### Small Size on Disk

Chef now bundles the inspec-core and train-core gems, which omit many cloud dependencies not needed within the Chef client. This change reduces the install size of a typical system by ~22% and the number of files within that installation by ~20% compared to Chef 14.1\. Enjoy the extra disk space.

### Virtualization detection on AWS

Ohai now detects the virtualization hypervisor `amazonec2` when running on Amazon's new C5/M5 instances.

## What's New in 14.1.12

This release resolves a number of regressions in 14.1.1:

- `git` resource: don't use `--prune-tags` as it's really new.
- `rhsm_repo` resource: now works
- `apt_repository` resource: use the `repo_name` property to name files
- `windows_task` resource: properly handle commands with arguments
- `windows_task` resource: handle creating tasks as the SYSTEM user
- `remote_directory` resource: restore the default for the `overwrite` property

### Ohai 14.1.3

- Properly detect FIPS environments
- `shard` plugin: work in FIPS compliant environments
- `filesystem` plugin: Handle BSD platforms

## What's New in 14.1.1

### Platform Additions

Enable Ubuntu-18.04 and Debian-9 tested chef-client packages.

## What's New in 14.1

### Windows Task

The `windows_task` resource has been entirely rewritten. This resolves a large number of bugs, including being able to correctly set the start time of tasks, proper creation and deletion of tasks, and improves Chef's validation of tasks. The rewrite will also solve the idempotency problems that users have reported.

### build_essential

The `build_essential` resource no longer requires a name, similar to the `apt_update` resource.

### Ignore Failure

The `ignore_failure` property takes a new argument, `:quiet`, to suppress the error output when the resource does in fact fail.

### This release of Chef Client 14 resolves a number of regressions in 14.0

- On Windows, the installer now correctly re-extracts files during repair mode
- Fix a number of issues relating to use with Red Hat Satellite
- Git fetch now prunes remotes before running
- Fix locking and unlocking packages with apt and zypper
- Ensure we don't request every remote file when running with lazy loading enabled
- The sysctl resource correctly handles missing keys when used with `ignore_error`
- --recipe-url apparently never worked on Windows. Now it does.

### Security Updates

#### ffi Gem

- CVE-2018-1000201: DLL loading issue which can be hijacked on Windows OS

## Ohai Release Notes 14.1

### Configurable DMI Whitelist

The whitelist of DMI IDs is now user configurable using the `additional_dmi_ids` configuration setting, which takes an Array.

### Shard plugin

The Shard plugin has been returned to a default plugin rather than an optional one. To ensure we work in FIPS environments, the plugin will use SHA256 rather than MD5 in those environments.

### SCSI plugin

A new plugin to enumerate SCSI devices has been added. This plugin is optional.

## What's New in 14.0.202

This release of Chef 14 resolves several regressions in the Chef 14.0 release.

- Resources contained in cookbooks would be used instead of built-in Chef client resources causing older resources to run
- Resources failed due to a missing `property_is_set?` and `resources` methods
- `yum_package` changed the order of `disablerepo` and `enablerepo` options
- Depsolving large numbers of cookbooks with chef zero/local took a very long time

## What's New in 14.0

### New Resources

Chef 14 includes a large number of resources ported from community cookbooks. These resources have been tested, improved, and had their functionality expanded. With these new resources in the Chef Client itself, the need for external cookbook dependencies and dependency management has been greatly reduced.

#### build_essential

Use the build_essential resource to install packages required for compiling C software from source. This resource was ported from the `build-essential` community cookbook.

`Note`: This resource no longer configures msys2 on Windows systems.

#### chef_handler

Use the chef_handler resource to install or uninstall Chef reporting/exception handlers. This resource was ported from the `chef_handler` community cookbook.

#### dmg_package

Use the dmg_package resource to install a dmg 'package'. The resource will retrieve the dmg file from a remote URL, mount it using hdiutil, copy the application (.app directory) to the specified destination (/Applications), and detach the image using hdiutil. The dmg file will be stored in the Chef::Config[:file_cache_path]. This resource was ported from the `dmg` community cookbook.

#### homebrew_cask

Use the homebrew_cask resource to install binaries distributed via the Homebrew package manager. This resource was ported from the `homebrew` community cookbook.

#### homebrew_tap

Use the homebrew_tap resource to add additional formula repositories to the Homebrew package manager. This resource was ported from the `homebrew` community cookbook.

#### hostname

Use the hostname resource to set the system's hostname, configure hostname and hosts config file, and re-run the Ohai hostname plugin so the hostname will be available in subsequent cookbooks. This resource was ported from the `chef_hostname` community cookbook.

#### macos_userdefaults

Use the macos_userdefaults resource to manage the macOS user defaults system. The properties of this resource are passed to the defaults command, and the parameters follow the convention of that command. See the defaults(1) man page for details on how the tool works. This resource was ported from the `mac_os_x` community cookbook.

#### ohai_hint

Use the ohai_hint resource to pass hint data to Ohai to aid in configuration detection. This resource was ported from the `ohai` community cookbook.

#### openssl_dhparam

Use the openssl_dhparam resource to generate dhparam.pem files. If a valid dhparam.pem file is found at the specified location, no new file will be created. If a file is found at the specified location but it is not a valid dhparam file, it will be overwritten. This resource was ported from the `openssl` community cookbook.

#### openssl_rsa_private_key

Use the openssl_rsa_private_key resource to generate RSA private key files. If a valid RSA key file can be opened at the specified location, no new file will be created. If the RSA key file cannot be opened, either because it does not exist or because the password to the RSA key file does not match the password in the recipe, it will be overwritten. This resource was ported from the `openssl` community cookbook.

#### openssl_rsa_public_key

Use the openssl_rsa_public_key resource to generate RSA public key files given a RSA private key. This resource was ported from the `openssl` community cookbook.

#### rhsm_errata

Use the rhsm_errata resource to install packages associated with a given Red Hat Subscription Manager Errata ID. This is helpful if packages to mitigate a single vulnerability must be installed on your hosts. This resource was ported from the `redhat_subscription_manager` community cookbook.

#### rhsm_errata_level

Use the rhsm_errata_level resource to install all packages of a specified errata level from the Red Hat Subscription Manager. For example, you can ensure that all packages associated with errata marked at a 'Critical' security level are installed. This resource was ported from the `redhat_subscription_manager` community cookbook.

#### rhsm_register

Use the rhsm_register resource to register a node with the Red Hat Subscription Manager or a local Red Hat Satellite server. This resource was ported from the `redhat_subscription_manager` community cookbook.

#### rhsm_repo

Use the rhsm_repo resource to enable or disable Red Hat Subscription Manager repositories that are made available via attached subscriptions. This resource was ported from the `redhat_subscription_manager` community cookbook.

#### rhsm_subscription

Use the rhsm_subscription resource to add or remove Red Hat Subscription Manager subscriptions for your host. This can be used when a host's activation_key does not attach all necessary subscriptions to your host. This resource was ported from the `redhat_subscription_manager` community cookbook.

#### sudo

Use the sudo resource to add or remove individual sudo entries using `sudoers.d` files. Sudo version 1.7.2 or newer is required to use the sudo resource, as it relies on the `#includedir` directive introduced in version 1.7.2\. This resource does not enforce installation of the required sudo version. Supported releases of Ubuntu, Debian, SuSE, and RHEL (6+) all support this feature. This resource was ported from the `sudo` community cookbook.

#### swap_file

Use the swap_file resource to create or delete swap files on Linux systems, and optionally to manage the swappiness configuration for a host. This resource was ported from the `swap` community cookbook.

#### sysctl

Use the sysctl resource to set or remove kernel parameters using the sysctl command line tool and configuration files in the system's `sysctl.d` directory. Configuration files managed by this resource are named 99-chef-KEYNAME.conf. If an existing value was already set for the value it will be backed up to the node and restored if the :remove action is used later. This resource was ported from the `sysctl` community cookbook.

`Note`: This resource no longer backs up existing key values to the node when changing values as we have done in the sysctl cookbook previously. The resource has also been renamed from `sysctl_param` to `sysctl` with backwards compatibility for the previous name.

#### windows_ad_join

Use the windows_ad_join resource to join a Windows Active Directory domain and reboot the node. This resource is based on the `win_ad_client` resource in the `win_ad` community cookbook, but is not backwards compatible with that resource.

#### windows_auto_run

Use the windows_auto_run resource to set applications to run at logon. This resource was ported from the `windows` community cookbook.

#### windows_feature

Use the windows_feature resource to add, remove or delete Windows features and roles. This resource calls the `windows_feature_dism` or `windows_feature_powershell` resources depending on the specified installation method and defaults to dism, which is available on both Workstation and Server editions of Windows. This resource was ported from the `windows` community cookbook.

`Note`: These resources received significant refactoring in the 4.0 version of the windows cookbook (March 2018). windows_feature resources now fail if the installation of invalid features is requested and support for installation via server `servermanagercmd.exe` has been removed. If you are using a windows cookbook version less than 4.0 you may need to update cookbooks for Chef 14.

#### windows_font

Use the windows_font resource to install or remove font files on Windows. By default, the font is sourced from the cookbook using the resource, but a URI source can be specified as well. This resource was ported from the `windows` community cookbook.

#### windows_printer

Use the windows_printer resource to setup Windows printers. Note that this doesn't currently install a printer driver. You must already have the driver installed on the system. This resource was ported from the `windows` community cookbook.

#### windows_printer_port

Use the windows_printer_port resource to create and delete TCP/IPv4 printer ports on Windows. This resource was ported from the `windows` community cookbook.

#### windows_shortcut

Use the windows_shortcut resource to create shortcut files on Windows. This resource was ported from the `windows` community cookbook.

#### windows_workgroup

Use the windows_workgroup resource to join a Windows Workgroup and reboot the node. This resource is based on the `windows_ad_join` resource.

### Custom Resource Improvements

We've expanded the DSL for custom resources with new functionality to better document your resources and help users with errors and upgrades. Many resources in Chef itself are now using this new functionality, and you'll see more updated to take advantage of this it in the future.

#### Deprecations in Cookbook Resources

Chef 14 provides new primitives that allow you to deprecate resources or properties with the same functionality used for deprecations in Chef Client resources. This allows you make breaking changes to enterprise or community cookbooks with friendly notifications to downstream cookbook consumers directly in the Chef run.

Deprecate the foo_bar resource in a cookbook:

```ruby
deprecated "The foo_bar resource has been deprecated and will be removed in the next major release of this cookbook scheduled for 12/25/2018!"

property :thing, String, name_property: true

action :create do
 # you'd probably have some actual chef code here
end
```

Deprecate the thing2 property in a resource

```ruby
property :thing2, String, deprecated: 'The thing2 property has been deprecated and will be removed in the next major release of this cookbook scheduled for 12/25/2018!'
```

Rename a property with a deprecation warning for users of the old property name

```ruby
deprecated_property_alias 'thing2', 'the_second_thing', 'The thing2 property was renamed the_second_thing in the 2.0 release of this cookbook. Please update your cookbooks to use the new property name.'
```

#### Platform Deprecations

chef-client no longer is built or tested on OS X 10.10 in accordance with Chef's EOL policy.

#### validation_message

Validation messages allow you give the user a friendly error message when any validation on a property fails.

Provide a friendly message when a regex fails:

```ruby
property :repo_name, String, regex: [/^[^\/]+$/], validation_message: "The repo_name property cannot contain a forward slash '/'",
```

#### Resource Documentation

You can now include documentation that describes how a resource is to be used. Expect this data to be consumed by Chef and other tooling in future releases.

A resource which includes description and introduced values in the resource, actions, and properties:

```ruby
description 'The apparmor_policy resource is used to add or remove policy files from a cookbook file'
introduced '14.1'

property :source_cookbook, String,
         description: 'The cookbook to source the policy file from'
property :source_filename, String,
         description: 'The name of the source file if it differs from the apparmor.d file being created'

action :add do
  description 'Adds an apparmor policy'

  # you'd probably have some actual chef code here
end
```

### Improved Resources

Many existing resources now include new actions and properties that expand their functionality.

#### apt_package

`apt_package` includes a new `overwrite_config_files` property. Setting this new property to true is equivalent to passing `-o Dpkg::Options::="--force-confnew"` to apt, and allows you to install packages that prompt the user to overwrite config files. Thanks @ccope for this new property.

#### env

The `env` resource has been renamed to `windows_env` as it only supports the Windows platform. Existing cookbooks using `env` will continue to function, but should be updated to use the new name.

#### ifconfig

`ifconfig` includes a new `family` property for setting the network family on Debian systems. Thanks @martinisoft for this new property.

#### registry_key

The `sensitive` property can now be used in `registry_key` to suppress the output of the key's data from logs and error messages. Thanks @shoekstra for implementing this.

#### powershell_package

`powershell_package` includes a new `source` property to allow specifying the source of the package. Thanks @Happycoil for this new property.

#### systemd_unit

`systemd_unit` includes the following new actions:

- `preset` - Restore the preset enable/disable configuration for a unit
- `revert` - Revert to a vendor's version of a unit file
- `reenable` - Reenable a unit file

Thanks @nathwill for these new actions.

#### windows_service

`windows_service` now includes actions for fully managing services on Windows, in addition to the previous actions for starting/stopping/enabling services.

- `create` - Create a new service
- `delete` - Delete an existing service
- `configure` - Reconfigure an existing service

Thanks @jasonwbarnett for these new actions

#### route

`route` includes a new `comment` property.

Thanks Thomas Doherty for adding this new property.

### Expanded Configuration Detection

Ohai has been expanded to collect more information than ever. This should make writing cross-platform and cross cloud cookbooks simpler.

#### Windows Kernel information

The kernel plugin now reports the following information on Windows:

- `node['kernel']['product_type']` - Workstation vs. Server editions of Windows
- `node['kernel']['system_type']` - What kind of hardware are we installed on (Desktop, Mobile, Workstation, Enterprise Server, etc.)
- `node['kernel']['server_core']` - Are we on Windows Server Core edition?

#### Cloud Detection

Ohai now detects the Scaleway cloud and provides additional configuration information for systems running on Azure.

#### Virtualization / Container Detection

In addition to detecting if a system is a Docker host, we now provide a large amount of Docker configuration information available at `node['docker']`. This includes the release of Docker, installed plugins, network config, and the number of running containers.

Ohai also now properly detects LXD containers and macOS guests running on VirtualBox / VMware. This data is available in `node['virtualization']['systems']`.

#### Optional Ohai Plugins

Ohai now includes the ability to mark plugins as optional, which skips those plugins by default. This allows us to ship additional plugins, which some users may find useful, but not all users want that data collected in the node object on a Chef server. The change introduces two new configuration options; `run_all_plugins` which runs everything including optional plugins, and `optional_plugins` which allows you to run plugins marked as optional.

By default we will now be marking the `lspci`, `sessions` `shard` and `passwd` plugins as optional. Passwd has been particularly problematic for nodes attached to LDAP or AD where it attempts to write the entire directory's contents to the node. If you previously disabled this plugin via Ohai config, you no longer need to. Hurray!

### Other Changes

#### Ruby 2.5

Ruby has been updated to version 2.5 bringing a 10% performance improvement and improved functionality.

#### InSpec 2.0

InSpec has been updated to the 2.0 release. InSpec 2.0 brings compliance automation to the cloud, with new resource types specifically built for AWS and Azure clouds. Along with these changes are major speed improvements and quality of life updates. Please visit <https://docs.chef.io/inspec/> for more information.

#### Policyfile Hoisting

Many users of Policyfiles rely on "hoisting" to provide group specific attributes. This approach was formalized in the poise-hoist extension, and is now included in Chef 14.

To hoist an attribute, the user provides a default attribute structure in their Policyfile similar to:

```ruby
default['staging']['myapp']['title'] = "My Staging App" default['production']['myapp']['title'] = "My App"
```

and then accesses the node attribute in their cookbook as:

```ruby
node['myapp']['title']
```

The correct attribute is then provided based on the policy_group of the node, so with a policy_group of staging the attribute would contain "My Staging App".

#### yum_package rewrite

yum_package received a ground up rewrite that greatly improves both the performance and functionality while also resolving a dozen existing issues. It introduces a new caching method that runs for the duration of the chef-client process. This caching method speeds up each package install and takes 1/2 the memory of the previous `yum-dump.py` process.

yum_package should now take any argument that `yum install` does and operate the same way, including version constraints "foo < 1.2.3" and globs "foo-1.2*" along with arches "foo.i386" and in combinations

Package with a version constraint:

```ruby
yum_package "foo < 1.2.3"
```

Installing a package via what it provides:

```ruby
yum_package "perl(Git)"
```

#### powershell_exec Mixin

Since our supported Windows platforms can all run .NET Framework 4.0 and PowerShell 4.0 we have taken time to add a new helper that will allow for faster and safer interactions with the system PowerShell. You will be able to use the powershell_exec mixin in most places where you would have previously used powershell_out. For comparison, a basic benchmark test to return the $PSVersionTable 100 times completed 7.3X faster compared to the powershell_out method. The majority of the time difference is because of less time spent in invocation. So we believe it has big future potential where multiple calls to PowerShell are required inside (for example) a custom resource. Many core Chef resources will be updated to use this new mixin in future releases.

#### Logging Improvements

Chef now includes a new log level of `:trace` in addition to the existing `:info`, `:warn`, and `:debug` levels. With the introduction of `trace` level logging we've moved a large amount of logging that is more useful for Chef developers from `debug` to `trace`. This makes it easier for Chef Cookbook developers to use `debug` level to get useful information.

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2o to resolve [CVE-2018-0739](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-0739)

#### Ruby

Ruby has been updated to 2.5.1 to resolve the following vulnerabilities:

- [cve-2017-17742](https://www.ruby-lang.org/en/news/2018/03/28/http-response-splitting-in-webrick-cve-2017-17742/)
- [cve-2018-6914](https://www.ruby-lang.org/en/news/2018/03/28/unintentional-file-and-directory-creation-with-directory-traversal-cve-2018-6914/)
- [cve-2018-8777](https://www.ruby-lang.org/en/news/2018/03/28/large-request-dos-in-webrick-cve-2018-8777/)
- [cve-2018-8778](https://www.ruby-lang.org/en/news/2018/03/28/buffer-under-read-unpack-cve-2018-8778/)
- [cve-2018-8779](https://www.ruby-lang.org/en/news/2018/03/28/poisoned-nul-byte-unixsocket-cve-2018-8779/)
- [cve-2018-8780](https://www.ruby-lang.org/en/news/2018/03/28/poisoned-nul-byte-dir-cve-2018-8780/)
- [Multiple vulnerabilities in rubygems](https://www.ruby-lang.org/en/news/2018/02/17/multiple-vulnerabilities-in-rubygems/)

### Breaking Changes

This release completes the deprecation process for many of the deprecations that were warnings throughout the Chef 12 and Chef 13 releases.

#### erl_call Resource

The erl_call resource was deprecated in Chef 13.7 and has been removed.

#### deploy Resource

The deploy resource was deprecated in Chef 13.6 and been removed. If you still require this resource, it is available in the new `deploy_resource` cookbook at <https://supermarket.chef.io/cookbooks/deploy_resource>

#### Windows 2003 Support

Support for Windows 2003 has been removed from both Chef and Ohai, improving the performance of Chef on Windows hosts.

#### knife deprecations

- `knife bootstrap` options `--distro` and `--template_file` flags were deprecated in Chef 12 and have now been removed.
- `knife help` functionality that read legacy Chef manpages has been removed as the manpages had not been updated and were often quite wrong. Running knife help will now simply show the help menu.
- `knife index rebuild` has been removed as reindexing Chef Server was only necessary on releases prior to Chef Server 11.
- The `knife ssh --identity-file` flag was deprecated and has been removed. Users should use the `--ssh_identity_file` flag instead.
- `knife ssh csshx` was deprecated in Chef 10 and has been removed. Users should use `knife ssh cssh` instead.

#### Chef Solo `-r` flag

The Chef Solo `-r` flag has been removed as it was deprecated and replaced with the `--recipe-url` flag in Chef 12.

#### node.set and node.set_unless attribute levels removal

`node.set` and `node.set_unless` were deprecated in Chef 12 and have been removed in Chef 14\. To replicate this same functionality users should use `node.normal` and `node.normal_unless`, although we highly recommend reading our [attribute documentation](https://docs.chef.io/attributes) to make sure `normal` is in fact the your desired attribute level.

#### chocolatey_package :uninstall Action

The chocolatey_package resource in the chocolatey cookbook supported an `:uninstall` action. When this resource was moved into the Chef Client we allowed this action with a deprecation warning. This action is now removed.

#### Property names not using new_resource.NAME

Previously if a user wrote a custom resource with a property named `foo` they could reference it throughout the resource using the name `foo`. This caused multiple edge cases where the property name could conflict with resources or methods in Chef. Properties now must be referenced as `new_resource.foo`. This was already the case when writing LWRPs.

#### epic_fail

The original name for the `ignore_failure` property in resource was `epic_fail`. The legacy name has been removed.

#### Legacy Mixins

Several legacy mixins mostly used in older HWRPs have been removed. Usage of these mixins has resulted in deprecation warnings for several years and they are rarely used in cookbooks available on the Supermarket.

- Chef::Mixin::LanguageIncludeAttribute
- Chef::Mixin::RecipeDefinitionDSLCore
- Chef::Mixin::LanguageIncludeRecipe
- Chef::Mixin::Language
- Chef::DSL::Recipe::FullDSL

#### cloud_v2 and filesystem2 Ohai Plugins

In Chef 13 the `cloud_v2` plugin replaced data at `node['cloud']` and `filesystem2` replaced data at `node['filesystem']`. For compatibility with cookbooks that were previously using the "v2" data we continued to write data to both locations (ie: both node['filesystem'] and node['filesystem2']). We now no longer write data to the "v2" locations which greatly reduces the amount of data we need to store on the Chef server.

#### Ipscopes Ohai Plugin Removed

The ipscopes plugin has been removed as it duplicated data already present in the network plugins and required the user to install an additional gem into the Chef installation.

#### Ohai libvirt attributes moved

The libvirt Ohai plugin now writes data to `node['libvirt']` instead of writing to various locations in `node['virtualization']`. This plugin required installing an additional gem into the Chef installation and thus was infrequently used.

#### Ohai Plugin V6 Support Removed

In 2014 we introduced Ohai v7 with a greatly improved plugin format. With Chef 14 we no longer support loading of the legacy "v6" plugin format.

#### Newly-disabled Ohai Plugins

As mentioned above we now support an `optional` flag for Ohai plugins and have marked the `sessions`, `lspci`, and `passwd` plugins as optional, which disables them by default. If you need one of these plugins you can include them using `optional_plugins`.

optional_plugins in the client.rb file:

```ruby
optional_plugins [ "lspci", "passwd" ]
```

## What's New in 13.12.14

### Bugfixes

- The mount provider now properly adds blank lines between fstab entries on AIX
- Ohai now reports itself as Ohai well communicating with GCE metadata endpoints
- Property deprecations in custom resources no longer result in an error. Thanks for reporting this [martinisoft](https://github.com/martinisoft)
- mixlib-archive has been updated to prevent corruption of archives on Windows systems

### Updated Components

- libxml2 2.9.7 -> 2.9.9
- ca-certs updated to 2019-01-22 for new roots
- nokogiri 1.8.5 -> 1.10.1

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2r in order to resolve [CVE-2019-1559](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1559) and [CVE-2018-5407](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-5407)

#### RubyGems

RubyGems has been updated to 2.7.9 in order to resolve the following CVEs:
- [CVE-2019-8320](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8320): Delete directory using symlink when decompressing tar
- [CVE-2019-8321](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8321): Escape sequence injection vulnerability in verbose
- [CVE-2019-8322](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8322): Escape sequence injection vulnerability in gem owner
- [CVE-2019-8323](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8323): Escape sequence injection vulnerability in API response handling
- [CVE-2019-8324](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8324): Installing a malicious gem may lead to arbitrary code execution
- [CVE-2019-8325](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-8325): Escape sequence injection vulnerability in errors

## What's New in 13.12.3

### Smaller Package and Install Size

We trimmed unnecessary installation files, greatly reducing the sizes of both Chef packages and on disk installations. MacOS/Linux/FreeBSD packages are ~50% smaller and Windows are ~12% smaller. Chef 13 is now smaller than a legacy Chef 10 package.

### macOS Mojave (10.14)

Chef is now tested against macOS Mojave and packages are now available at downloads.chef.io.

### SUSE Linux Enterprise Server 15

- Ohai now properly detects SLES 15
- The Chef package will no longer remove symlinks to chef-client and ohai when upgrading on SLES 15

### Updated Chef-Vault

Updating chef-vault to 3.4.2 resolved multiple bugs.

### Faster Windows Installations

Improved Windows installation speed by skipping unnecessary steps when Windows Installer 5.0 or later is available.

### Ohai Release Notes 13.12

#### macOS Improvements

- sysctl commands have been modified to gather only the bare minimum required data, which prevents sysctl hanging in some scenarios
- Extra data has been removed from the system_profile plugin, reducing the amount of data stored on the chef-server for each node

### New Deprecations

#### system_profile Ohai plugin removal

The system_profile plugin will be removed from Chef/Ohai 15 in April, 2019. This plugin incorrectly returns data on modern Mac systems. Further, the hardware plugin returns the same data in a more readily consumable format. Removing this plugin reduces the speed of the Ohai return by ~3 seconds and also greatly reduces the node object size on the Chef server

#### ohai_name property in ohai resource

The ``ohai`` resource's unused ``ohai_name`` property has been deprecated. This will be removed in Chef Infra Client 15.0.

### Security Updates

#### Ruby 2.4.5

Ruby has been updated to from 2.4.4 to 2.4.5 to resolve multiple CVEs as well as bugs:
- [CVE-2018-16396](https://www.ruby-lang.org/en/news/2018/10/17/not-propagated-taint-flag-in-some-formats-of-pack-cve-2018-16396/)
- [CVE-2018-16395](https://www.ruby-lang.org/en/news/2018/10/17/openssl-x509-name-equality-check-does-not-work-correctly-cve-2018-16395/)

## What's New in 13.11

-   **Sensitive Properties on Windows**
    -   windows_service no longer logs potentially sensitive
        information when a service is setup
    -   windows_package now respects the sensitive property to avoid
        logging sensitive data in the event of a package installation
        failure
-   **Bugfixes**
    -   `remote_directory` now properly loads files in the root of a
        cookbook's files directory
    -   `osx_profile` now uses the full path the profiles CLI tool to
        avoid running other binaries of the same name in a users path
    -   `package` resources that don't support the `allow_downgrade`
        property will no longer fail
    -   `knife bootstrap windows` error messages have been improved
-   **Security Updates**
    -   [CVE-2018-0732](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-0732):
        Fixes handshake violation in OpenSSL
    -   [CVE-2018-0737](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-0737):
        OpenSSL RSA Key generation algorithm has been shown to be
        vulnerable to a cache timing side channel attack
    -   [CVE-2018-1000544](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-1000544):
        rubyzip gem rubyzip version 1.2.1 and earlier contains a
        Directory Traversal vulnerability

## What's New in 13.10

-   **Bugfixes**
    -   Resolves a duplicate logging getting created when redirecting
        stdout
    -   Using `--recipe-url` with a local file on Windows no longer
        fails
    -   `Service` resource no longer throws Ruby deprecation warnings on
        Windows
-   **Ohai 13.10 Improvements**
    -   Correctly identifies the `platform_version` on the final release
        of Amazon Linux 2.0
    -   Detects nodes with the DMI data of "OpenStack Compute" as
        OpenStack nodes
-   **Security Updates**
    -   [CVE-2018-1000201](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-1000201):
        DLL loading issue which can be hijacked on Windows OS resolved
        by updating FFI gem

## What's New in 13.9.4

-   **Platform Updates**

    As Debian 7 is now end of life we will no longer produce Debian 7
    chef-client packages.

-   **Ifconfig on Ubuntu 18.04**

    Incompatibilities with Ubuntu 18.04 in the ifconfig resource have
    been resolved.

### Ohai 13.9.2

-   **Virtualization detection on AWS**

    Ohai now detects the virtualization hypervisor amazonec2 when
    running on Amazon's new C5/M5 instances.

-   **Configurable DMI Whitelist**

    The whitelist of DMI IDs is now user configurable using the
    additional_dmi_ids configuration setting, which takes an Array.

-   **Filesystem2 on BSD**

    The Filesystem2 functionality has been backported to BSD systems to
    provide a consistent filesystem format.

### Security Updates

-   **Ruby has been updated to 2.4.4**
    -   [CVE-2017-17742](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-17742):
        HTTP response splitting in WEBrick
    -   [CVE-2018-6914](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-6914):
        Unintentional file and directory creation with directory
        traversal in tempfile and tmpdir
    -   [CVE-2018-8777](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8777):
        DoS by large request in WEBrick
    -   [CVE-2018-8778](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8778):
        Buffer under-read in String\#unpack
    -   [CVE-2018-8779](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8779):
        Unintentional socket creation by poisoned NUL byte in UNIXServer
        and UNIXSocket
    -   [CVE-2018-8780](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8780):
        Unintentional directory traversal by poisoned NUL byte in Dir
    -   Multiple vulnerabilities in RubyGems
-   **OpenSSL has been updated to 1.0.2o**
    -   CVE-2018-0739: Constructed ASN.1 types with a recursive
        definition could exceed the stack.

## What's New in 13.9.1

-   On Windows, the installer now correctly re-extracts files during
    repair mode
-   The [mount](/resources/mount/) resource will not create duplicate
    entries when the device type differs
-   Chef no longer requests every remote file when running with lazy
    loading enabled
-   Fixes a bug that caused Chef to crash when retrieving access rights
    for Windows system accounts

This release also includes the [custom resource
improvements](/release_notes/#custom-resource-improvements) that
were introduced in Chef 14.

### Ohai 13.9

-   Fixes uptime parsing on AIX
-   Fixes Softlayer cloud detection
-   Uses the current Azure metadata endpoint
-   Correctly detects macOS guests on VMware and VirtualBox

## What's New in 13.9

-   On Windows, the installer now correctly re-extracts files during
    repair mode
-   The [mount](/resources/mount/) resource will now not create
    duplicate entries when the device type differs
-   Ensure we don't request every remote file when running with lazy
    loading enabled
-   Don't crash when getting the access rights for Windows system
    accounts

### Custom Resource Improvements

We've expanded the DSL for custom resources with new functionality to
better document your resources and help users with errors and upgrades.
Many resources in Chef itself are now using this new functionality, and
you'll see more updated to take advantage of this it in the future.

### Deprecations in Cookbook Resources

-   Chef 13 provides new primitives that allow you to deprecate
    resources or properties with the same functionality used for
    deprecations in Chef Client resources. This allows you make breaking
    changes to enterprise or community cookbooks with friendly
    notifications to downstream cookbook consumers directly in the Chef
    run.
-   Provide a friendly message when a regex fails:

### Resource Documentation

You can now include documentation that describes how a resource is to be
used. Expect this data to be consumed by Chef and other tooling in
future releases.

A resource which includes description and introduced values in the
resource, actions, and properties:

``` ruby
description 'The apparmor_policy resource is used to add or remove policy files from a cookbook file'
 introduced '14.1'

 property :source_cookbook, String,
         description: 'The cookbook to source the policy file from'
 property :source_filename, String,
         description: 'The name of the source file if it differs from the apparmor.d file being created'

 action :add do
   description 'Adds an apparmor policy'

   # you'd probably have some actual chef code here
 end
```

### Ohai Release Notes 13.9

-   Fix uptime parsing on AIX
-   Fix Softlayer cloud detection
-   Use the current Azure metadata endpoint
-   Correctly detect macOS guests on VMware and VirtualBox
-   Please see the CHANGELOG for the complete list of changes.

## What's New in 13.8.5

This is a small bug fix release to resolve two issues we found in the
13.8 release:

-   chef-client run failures due to a failure in a newer version of the
    FFI gem on RHEL 6.x and 7.x
-   knife failures when running `knife cookbook site install` to install
    a deprecated cookbook that has no replacement

## What's New in 13.8.3

This is a small bug fix release that updates Ohai to properly detect and
poll SoftLayer metadata now that SoftLayer no longer supports TLS
1.0/1.1. This update is only necessary if you're running on Softlayer.

## What's New in 13.8

-   **Fixes regression from 13.7.16**

    This release fixes the
    [regression](https://discourse.chef.io/t/regression-in-chef-client-13-7-16/12518)
    in how arrays and hashes were handled in Chef 13.7. Version 13.8 has
    reverted to the same code that was used in Chef 13.6.

-   **Continued windows_task Improvements**

    Chef 13.8 has better validation for the `idle_time` property when
    using the `on_idle` frequency option.

-   **Security Updates**

    Libxml2 has been updated to version 2.9.7 as a fix for
    [CVE-2017-15412](https://access.redhat.com/security/cve/cve-2017-15412).

See the detailed [change
log](https://github.com/chef/chef/blob/chef-13/CHANGELOG.md#v1380-2018-02-27)
for more information.

## What's New in 13.7.16

-   **The windows_task Resource should be better behaved**

    We've spent a considerable amount of time testing and fixing the
    [windows_task](/resources/windows_task/) resource to ensure that
    it is properly idempotent and correct in more situations.

-   **Credentials Handling**

    Previously, ChefDK workstations used `knife.rb` or `config.rb` to
    handle credentials. This didn't do a great job of interacting with
    multiple Chef servers, which lead to the need for tools like
    [knife_block](https://github.com/knife-block/knife-block). We've
    added support for a credentials file that contains configuration
    information for many Chef servers / organizations, and we've made it
    easy to indicate which account you mean to use.

-   **Bug Fixes**

    -   Resolved a bug where knife commands that resulted in a prompt on
        Windows would never display the prompt
    -   Fixed a bug that affected the hiding of sensitive resources when
        [converge_if_changed](/dsl_custom_resource/#converge-if-changed)
        was used
    -   Fixes to certain scenarios that would result in services failing
        to start on Solaris

-   **Security Updates**

    -   OpenSSL has been upgraded to 1.0.2n to resolve
        [CVE-2017-3738](https://nvd.nist.gov/vuln/detail/CVE-2017-3738),
        [CVE-2017-3737](https://nvd.nist.gov/vuln/detail/CVE-2017-3737),
        [CVE-2017-3736](https://nvd.nist.gov/vuln/detail/CVE-2017-3736),
        and
        [CVE-2017-3735](https://nvd.nist.gov/vuln/detail/CVE-2017-3735)
    -   Ruby has been upgraded to 2.4.3 to resolve
        [CVE-2017-17405](https://nvd.nist.gov/vuln/detail/CVE-2017-17405)

### Deprecations

-   **erl_call Resource**

    We introduced the `erl_call` resource to help us to manage CouchDB
    servers back in the olden times of Chef. Since then we've noticed
    that no one uses it, and so `erl_call` will be removed in Chef 14.
    Foodcritic rule [FC105](http://www.foodcritic.io/#FC105) has been
    introduced to detect usage of `erl_call`.

-   **epic_fail**

    The original name for the `ignore_failure` property in resources was
    `epic_fail`. Our documentation hasn't referred to `epic_fail` for
    years and out of the 3500 cookbooks on the Supermarket only one uses
    `epic_fail`. In Chef 14 we will remove the `epic_fail` property
    entirely. Foodcritic rule [FC107](http://www.foodcritic.io/#FC107)
    has been introduced to detect usage of `epic_fail`.

-   **Legacy Mixins**

    In Chef 14 several legacy mixins will be removed. Usage of these
    mixins has resulted in deprecation warnings for several years. They
    were traditionally used in some HWRPs, but are rarely found in code
    available on the Supermarket. Foodcritic rules
    [FC097](http://www.foodcritic.io/#FC097),
    [FC098](http://www.foodcritic.io/#FC098),
    [FC099](http://www.foodcritic.io/#FC099),
    [FC100](http://www.foodcritic.io/#FC100), and
    [FC102](http://www.foodcritic.io/#FC102) have been introduced to
    detect these mixins:

    -   `Chef::Mixin::LanguageIncludeAttribute`
    -   `Chef::Mixin::RecipeDefinitionDSLCore`
    -   `Chef::Mixin::LanguageIncludeRecipe`
    -   `Chef::Mixin::Language`
    -   `Chef::DSL::Recipe::FullDSL`

-   **:uninstall Action in chocolatey_package**

    The chocolatey cookbook's `chocolatey_package` resource originally
    contained an `:uninstall` action. When
    [chocolatey_package](/resources/chocolatey_package/) was moved
    into core Chef we made `:uninstall` an alias for `:remove`. In Chef
    14, `:uninstall` will no longer be a valid action. Foodcritic rule
    [FC103](http://www.foodcritic.io/#FC103) has been introduced to
    detect usage of the `:uninstall` action.

### Ohai 13.7

-   **Network Tunnel Information**

    The Network plugin on Linux hosts now gathers additional information
    on tunnels.

-   **LsPci Plugin**

    The new LsPci plugin provides a `node['pci']` hash with information
    about the PCI bus based on lspci. Only runs on Linux.

-   **EC2 C5 Detection**

    The EC2 plugin has been updated to properly detect the new AWS
    hypervisor used in the C5 instance types.

-   **mdadm**

    The mdadm plugin has been updated to properly handle arrays with
    more than 10 disks, and to properly handle journal and spare drives
    in the disk counts.

## What's New in 13.6.4

-   **Resolved Debian / Ubuntu regression**

    This release resolves a regression in 13.6.0 that prevented the
    upgrading of packages on Debian or Ubuntu when the package name
    contained a tilde (`~`).

-   **Security Updates**

    -   OpenSSL has been upgraded to 1.0.2m to resolve
        [CVE-2017-3735](https://nvd.nist.gov/vuln/detail/CVE-2017-3735)
        and
        [CVE-2017-3736](https://nvd.nist.gov/vuln/detail/CVE-2017-3736)
    -   RubyGems has been upgraded to 2.6.14 to resolve
        [CVE-2017-0903](https://nvd.nist.gov/vuln/detail/CVE-2017-0903)

See the full [change
log](https://github.com/chef/chef/blob/master/CHANGELOG.md#v1364-2017-11-06)
for additional details.

## What's New in 13.6.0

-   **The deploy resource is deprecated**

    The `deploy` and `deploy_revision` resources have been deprecated,
    to be removed in Chef 14. This is being done because this resource
    is considered overcomplicated and error-prone in the modern Chef
    ecosystem. A compatibility cookbook will be available to help users
    migrate during the Chef 14 release cycle. See the [deprecation
    documentation](/deprecations_deploy_resource/)
    for more information.

-   **zypper_package supports package downgrades**

    `zypper_package` now supports downgrading installed packages with
    the `allow_downgrade` property.

-   **InSpec has been updated to 1.42.3**

-   **Reserve certain Data Bag names**

    It's no longer possible to create data bags named `node`, `role`,
    `client`, or `environment`. Existing data bags will continue to work
    as they did previously.

-   **Properly use YUM on RHEL and CentOS 7**

    On systems with both DNF and YUM installed, there were instances
    where the `yum` provider would choose to run `dnf` instead. It now
    only runs `yum`.

### Ohai 13.6

-   **Critical Plugins**

    Users can now specify a list of plugins which are `critical`.
    Critical plugins will cause Ohai to fail if they do not run
    successfully, and thus cause a Chef run using Ohai to fail. The
    syntax for this is:

    ```ruby
    ohai.critical_plugins << :Filesystem
    ```

-   **Filesystem now has an \`allow_partial_data\` configuration
    option**

    The Filesystem plugin now has an `allow_partial_data` configuration
    option. When set, the filesystem will return whatever data it can,
    even if some of its attempted commands fail to execute.

-   **Rackspace detection on Windows**

    Windows nodes running on Rackspace will now properly detect
    themselves as running on Rackspace, without a hint file.

-   **Package data on Amazon Linux**

    The Packages plugin now supports gathering package data on Amazon
    Linux

#### Deprecation Updates

In Ohai 13 we replaced the `filesystem` and `cloud` plugins with the
`filesystem2` and `cloud_v2` plugins. In order to maintain compatibility
with users of the previous V2 plugins, we write data to both locations.
We had originally planned to continue writing data to both locations
until Chef 15. Instead, due to the large amount of duplicate node data
this introduces, we are updating the
[OHAI-11](/deprecations_ohai_cloud_v2/) and
[OHAI-12](/deprecations_ohai_filesystem_v2/) deprecations to remove
`node['cloud_v2']` and `node['filesystem2']` with the release of Chef 14
in April 2018.

## What's New in 13.5.3

-   **The mount resource's password property is now marked as
    sensitive** Passwords passed to mount won't show up in logs.
-   **The windows_task resource now correctly handles start_day**
    Previously, the resource would accept any date that was formatted
    correctly in the local locale, unlike the Windows cookbook and
    Windows itself. We now support only the MM/DD/YYYY format, in
    keeping with the Windows cookbook.
-   **InSpec updated to 1.39.1**

See the detailed [change
log](https://github.com/chef/chef/blob/master/CHANGELOG.md#v1353-2017-10-03)
for additonal information.

### Ohai 13.5

-   **Correctly detect IPv6 routes ending in ::** Previously, Ohai would
    ignore routes that ended with `::`, but now they can be detected
    properly.
-   **Plugin run time is now measured** Debug logs will show the length
    of time each plugin takes to run, which makes it easier to debug
    long Ohai runs.

## What's New in 13.4.24

This release includes Ruby 2.4.2 to fix the following CVEs:

-   [CVE-2017-0898](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-0898)
-   [CVE-2017-10784](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CxVE-2017-10784)
-   CVE-2017-14033
-   [CVE-2017-14064](https://nvd.nist.gov/vuln/detail/CVE-2017-14064)

It contains no other changes from version 13.4.19.

{{< note >}}

Due to issues beyond our control, this release is only built for Linux
(on x86, x86_64 and s390x), FreeBSD, and Windows. We'll release a new
build with support for our other platforms (AIX, Solaris, and macOS) as
soon as possible.

{{< /note >}}

## What's New in 13.4.19

-   **Security release of RubyGems** RubyGems has been upgraded to
    2.6.13 to address the following:
    -   [CVE-2017-0899](https://nvd.nist.gov/vuln/detail/CVE-2017-0899)
    -   [CVE-2017-0900](https://nvd.nist.gov/vuln/detail/CVE-2017-0900)
    -   [CVE-2017-0901](https://nvd.nist.gov/vuln/detail/CVE-2017-0901)
    -   [CVE-2017-0902](https://nvd.nist.gov/vuln/detail/CVE-2017-0902)
-   **Additional ifconfig options on RHEL and CentOS** The
    `ethtool_opts`, `bonding_opts`, `master`, and `slave` properties
    have been added. See the [ifconfig resource
    documentation](/resources/ifconfig/) for additional details.
-   **Chef vault now included by default** Chef client 13.4 includes the
    `chef-vault` gem, so users can more easily work with encrypted
    items.
-   **Windows remote_file resource now supports alternative
    credentials** The `remote_user`, `remote_domain`, and
    `remote_password` options have been added to allow access to a file
    even if the Chef client process identity does not have permission to
    access it. This is mainly intended to be used for accessing files
    between two nodes on different domains. See the [remote_file
    documentation](/resources/remote_file/) for more information.
-   **New windows_path resource** `windows_path` has been moved from
    the Windows cookbook to core Chef. The `windows_path` resource is
    used to manage the path environment variable on Windows. See the
    [windows_path documentation](/resources/windows_path/) for
    additional details.

### Ohai 13.4

-   **Windows EC2 Detection** Detection of nodes running in EC2 has been
    greatly improved, and Ohai should now detect nodes 100% of the time,
    including nodes that have been migrated to EC2 or were built with
    custom AMIs.

-   **Package plugin supports Arch Linux** The Package plugin has been
    updated to include package information on Arch Linux systems.

-   **Azure Metadata Endpoint Detection** Ohai now polls the new Azure
    metadata endpoint, providing additional configuration details on
    nodes running in Azure. Sample data now available under Azure:

    ``` none
    {
      "metadata": {
        "compute": {
          "location": "westus",
          "name": "timtest",
          "offer": "UbuntuServer",
          "osType": "Linux",
          "platformFaultDomain": "0",
          "platformUpdateDomain": "0",
          "publisher": "Canonical",
          "sku": "17.04",
          "version": "17.04.201706191",
          "vmId": "8d523242-71cf-4dff-94c3-1bf660878743",
          "vmSize": "Standard_DS1_v2"
        },
        "network": {
          "interfaces": {
            "000D3A33AF03": {
              "mac": "000D3A33AF03",
              "public_ipv6": [

              ],
              "public_ipv4": [
                "52.160.95.99",
                "23.99.10.211"
              ],
              "local_ipv6": [

              ],
              "local_ipv4": [
                "10.0.1.5",
                "10.0.1.4",
                "10.0.1.7"
              ]
            }
          },
          "public_ipv4": [
            "52.160.95.99",
            "23.99.10.211"
          ],
          "local_ipv4": [
            "10.0.1.5",
            "10.0.1.4",
            "10.0.1.7"
          ],
          "public_ipv6": [

          ],
          "local_ipv6": [

          ]
        }
      }
    }
    ```

## What's New in 13.3

-   **Unprivileged symlink creation on Windows** Chef can now create
    symlinks without privilege escalation, which allows for the creation
    of symlinks on Windows 10 Creator Update.

-   **nokogiri Gem** The nokogiri gem is once again bundled with the
    omnibus install of Chef.

-   **New resources** This release introduces the
    [apt_preference](/resources/apt_preference/) and
    [zypper_repository](/resources/zypper_repository/) resources.

-   **windows_task Improvements** The `windows_task` resource now
    allows updating the configuration of a scheduled task when using the
    `:create` action. The `:change` action from the windows cookbook has
    been aliased to `:create` to provide backward compatibility.

-   **zypper_package Options** It is now possible to pass additional
    options to Zypper in the `zypper_package` resource. For example:

    ``` ruby
    zypper_package 'foo' do
      options '--user-provided'
    end
    ```

-   **Ohai support for F5 Big-IP** Ohai now detects the [F5
    Big-IP](https://www.f5.com/) platform and platform_version:

    -   platform: bigip
    -   platform_family: rhel

## What's New in 13.2

-   **Properly send PolicyFile data** When sending events back to the
    Chef Server, Chef client now correctly expands the run_list for
    nodes that use PolicyFiles. This allows Automate to correctly report
    the node.

-   **Reconfigure between runs when daemonized** When Chef performs a
    reconfigure, it rereads the configuration files. It also reopens its
    log files, which facilitates log file rotation.

    Normally, Chef will reconfigure when sent a HUP signal. As of this
    release, if you send a HUP signal while it is converging, the
    reconfigure happens at the end of the run. This is avoids the
    potential Ruby issues that occur when the configuration file
    contains additional Ruby code that is executed. While the daemon is
    sleeping between runs, sending a SIGHUP will still cause an
    immediate reconfigure.

    When daemonized, Chef now performs a reconfigure after every run.

### New deprecations

-   [Explicit property methods](/deprecations_namespace_collisions/)
    In Chef 14, custom resources will no longer assume property methods
    are being called on `new_resource`, and will instead require the
    resource author to be explicit.

## What's New in 13.0/13.1

-   **Blacklist attributes**
-   **RubyGems sources**
-   **windows_task resource**
-   **Chef client will now exit using the rfc062 defined exit codes**
-   **New default encrypted databag format**
-   **Backwards compatibility breaks**

### It is now possible to blacklist node attributes

#### Blacklist Attributes

{{< warning >}}

When attribute blacklist settings are used, any attribute defined in a
blacklist will not be saved and any attribute that is not defined in a
blacklist will be saved. Each attribute type is blacklisted
independently of the other attribute types. For example, if
`automatic_attribute_blacklist` defines attributes that will not be
saved, but `normal_attribute_blacklist`, `default_attribute_blacklist`,
and `override_attribute_blacklist` are not defined, then all normal
attributes, default attributes, and override attributes will be saved,
as well as the automatic attributes that were not specifically excluded
through blacklisting.

{{< /warning >}}

Attributes that should not be saved by a node may be blacklisted in the
client.rb file. The blacklist is a Hash of keys that specify each
attribute to be filtered out.

Attributes are blacklisted by attribute type, with each attribute type
being blacklisted independently. Each attribute type---`automatic`,
`default`, `normal`, and `override`---may define blacklists by using the
following settings in the client.rb file:

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
<td><code>automatic_attribute_blacklist</code></td>
<td>A hash that blacklists <code>automatic</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['network/interfaces/eth0']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
<tr class="even">
<td><code>default_attribute_blacklist</code></td>
<td>A hash that blacklists <code>default</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['filesystem/dev/disk0s2/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
<tr class="odd">
<td><code>normal_attribute_blacklist</code></td>
<td>A hash that blacklists <code>normal</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['filesystem/dev/disk0s2/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
<tr class="even">
<td><code>override_attribute_blacklist</code></td>
<td>A hash that blacklists <code>override</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['map - autohome/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
</tbody>
</table>

{{< warning >}}

The recommended practice is to use only `automatic_attribute_blacklist`
for blacklisting attributes. This is primarily because automatic
attributes generate the most data, but also that normal, default, and
override attributes are typically much more important attributes and are
more likely to cause issues if they are blacklisted incorrectly.

{{< /warning >}}

For example, automatic attribute data similar to:

``` javascript
{
  "filesystem" => {
    "/dev/disk0s2" => {
      "size" => "10mb"
    },
    "map - autohome" => {
      "size" => "10mb"
    }
  },
  "network" => {
    "interfaces" => {
      "eth0" => {...},
      "eth1" => {...},
    }
  }
}
```

To blacklist the `filesystem` attributes and allow the other attributes
to be saved, update the client.rb file:

``` ruby
automatic_attribute_blacklist ['filesystem']
```

When a blacklist is defined, any attribute of that type that is not
specified in that attribute blacklist **will** be saved. So based on the
previous blacklist for automatic attributes, the `filesystem` and
`map - autohome` attributes will not be saved, but the `network`
attributes will.

For attributes that contain slashes (`/`) within the attribute value,
such as the `filesystem` attribute `'/dev/diskos2'`, use an array. For
example:

``` ruby
automatic_attribute_blacklist [['filesystem','/dev/diskos2']]
```

### RubyGems provider sources behavior changed.

The behavior of `gem_package` and `chef_gem` is now to always apply the
`Chef::Config[:rubygems_uri]` sources, which may be a string uri or an
array of strings. If additional sources are put on the resource with the
`source` property those are added to the configured `:rubygems_uri`
sources.

This should enable easier setup of rubygems mirrors particularly in
"airgapped" environments through the use of the global config variable.
It also means that an admin may force all rubygems.org traffic to an
internal mirror, while still being able to consume external cookbooks
which have resources which add other mirrors unchanged (in a
non-airgapped environment).

In the case where a resource must force the use of only the specified
source(s), then the `include_default_source` property has been added --
setting it to false will remove the `Chef::Config[:rubygems_url]`
setting from the list of sources for that resource.

The behavior of the `clear_sources` property is now to only add
`--clear-sources` and has no magic side effects on the source options.

### Ruby version upgraded to 2.4.1

We've upgraded to the latest stable release of the Ruby programming
language. See the Ruby [2.4.0 Release
Notes](https://www.ruby-lang.org/en/news/2016/12/25/ruby-2-4-0-released/)
for an overview of what's new in the language.

### Resource can now declare a default name

The core `apt_update` resource can now be declared without any name
argument, no need for `apt_update STING`.

This can be used by any other resource by just overriding the name
property and supplying a default:

``` ruby
property :name, String, default: ""
```

Notifications to resources with empty strings as their name is also
supported via either the bare resource name (`apt_update` -- matches
what the user types in the DSL) or with empty brackets (`apt_update[]`
-- matches the resource notification pattern).

### The knife ssh command applies the same fuzzifier as knife search node

A bare name to knife search node will search for the name in `tags`,
`roles`, `fqdn`, `addresses`, `policy_name` or `policy_group` fields and
will match when given partial strings (available since Chef 11). The
`knife ssh` search term has been similarly extended so that the search
API matches in both cases. The node search fuzzifier has also been
extracted out to a `fuzz` option to Chef::Search::Query for re-use
elsewhere.

### Cookbook root aliases

Rather than `attributes/default.rb`, cookbooks can now use
`attributes.rb` in the root of the cookbook. Similarly for a single
default recipe, cookbooks can use `recipe.rb` in the root of the
cookbook.

### knife ssh connects gateways with ssh key authentication

The new `gateway_identity_file` option allows the operator to specify
the key to access ssh gateways with.

### Windows Task resource added

The `windows_task` resource has been ported from the windows cookbook.
Use the **windows_task** resource to create, delete or run a Windows
scheduled task. Requires Windows Server 2008 due to API usage.

**Note**: `:change` action has been removed from `windows_task`
resource. `:create` action can be used to update an existing task.

### Solaris SMF services can now be started recursively

It is now possible to load Solaris services recursively, by ensuring the
new `options` property of the `service` resource contains `-r`.

### The guard interpreter for `powershell_script` is PowerShell, again

When writing `not_if` or `only_if` statements, by default we now run
those statements using powershell, rather than forcing the user to set
`guard_interpreter` each time.

### Zypper GPG checks by default

Zypper now defaults to performing gpg checks of packages.

### The InSpec gem is now shipped by default

The `inspec` and `train` gems are shipped by default in the chef omnibus
package, making it easier for users in airgapped environments to use
InSpec.

### Properly support managing Sys-V services on Debian systemd hosts

Chef now properly supports managing sys-v services on hosts running
systemd. Previously Chef would incorrectly attempt to fallback to
Upstart even if upstart was not installed.

### New default encrypted databag format

The default encrypted databag format is now 3.0, which was introduced in
Chef 12.0. Encrypted databag version 3.0 format uses the `aes-256-gcm`
cipher for enhanced security. All nodes using encrypted data bags in
your environment will need to be upgraded to Chef 12.0 or later before
creating encrypted data bags in this new format.

### Backwards Compatibility Breaks

#### Resource Cloning has been removed

When Chef compiles resources, it will no longer attempt to merge the
properties of previously compiled resources with the same name and type
in to the new resource. See [the deprecation
page](/deprecations_resource_cloning/) for
further information.

#### It is an error to specify both `default` and `name_property` on a property

Chef 12 made this work by picking the first option it found, but it was
always an error and has now been disallowed.

#### The path property of the execute resource has been removed

It was never implemented in the provider, so it was always a no-op to
use it, the remediation is to simply delete it.

#### Using the command property on any script resource (including bash, etc) is now a hard error

This was always a usage mistake. The command property was used
internally by the script resource and was not intended to be exposed to
users. Users should use the code property instead (or use the command
property on an execute resource to execute a single command).

#### Omitting the code property on any script resource (including bash, etc) is now a hard error

It is possible that this was being used as a no-op resource, but the log
resource is a better choice for that until we get a null resource added.
Omitting the code property or mixing up the code property with the
command property are also common usage mistakes that we need to catch
and error on.

#### The chef_gem resource defaults to not run at compile time

The `compile_time true` flag may still be used to force compile time.

#### The Chef::Config\[:chef_gem_compile_time\] config option has been removed

In order to for community cookbooks to behave consistently across all
users this optional flag has been removed.

#### The `supports[:manage_home]` and `supports[:non_unique]` API has been removed

The remediation is to set the manage_home and non_unique properties
directly.

#### `creates` without `cwd` is a hard error

Using relative paths in the `creates` property of an execute resource
with specifying a `cwd` is now a hard error Without a declared cwd the
relative path was (most likely?) relative to wherever chef-client
happened to be invoked which is not deterministic or easy to intuit
behavior.

#### Chef::PolicyBuilder::ExpandNodeObject\#load_node has been removed

This change is most likely to only affect internals of tooling like
chefspec if it affects anything at all.

#### PolicyFile failback

PolicyFile failback to create non-policyfile nodes on Chef Server \<
12.3 has been removed PolicyFile users on Chef-13 should be using Chef
Server 12.3 or higher.

#### Cookbooks with self dependencies are no longer allowed

The remediation is removing the self-dependency `depends` line in the
metadata.

#### Removed `supports` API from Chef::Resource

Retained only for the service resource (where it makes some sense) and
for the mount resource.

#### Removed retrying of non-StandardError exceptions for Chef::Resource

Exceptions not descending from StandardError (e.g. LoadError,
SecurityError, SystemExit) will no longer trigger a retry if they are
raised during the execution of a resources with a non-zero retries
setting.

#### Removed deprecated `method_missing` access from the Chef::Node object

Previously, the syntax `node.foo.bar` could be used to mean
`node["foo"]["bar"]`, but this API had sharp edges where methods
collided with the core ruby Object class (e.g. `node.class`) and where
it collided with our own ability to extend the `Chef::Node` API. This
method access has been deprecated for some time, and has been removed in
Chef-13.

#### Changed `declare_resource` API

Dropped the `create_if_missing` parameter that was immediately
supplanted by the `edit_resource` API (most likely nobody ever used
this) and converted the `created_at` parameter from an optional
positional parameter to a named parameter. These changes are unlikely to
affect any cookbook code.

#### Node deep-duping fixes

The `` node.to_hash`/`node.to_h `` and `node.dup` APIs have been fixed
so that they correctly deep-dup the node data structure including every
string value. This results in a mutable copy of the immutable merged
node structure. This is correct behavior, but is now more expensive and
may break some poor code (which would have been buggy and difficult to
follow code with odd side effects before).

For example:

``` ruby
node.default["foo"] = "fizz"
n = node.to_hash   # or node.dup
n["foo"] << "buzz"
```

before this would have mutated the original string in-place so that
`node["foo"]` and `node.default["foo"]` would have changed to "fizzbuzz"
while now they remain "fizz" and only the mutable `n["foo"]` copy is
changed to "fizzbuzz".

#### Freezing immutable merged attributes

Since Chef 11 merged node attributes have been intended to be immutable
but the merged strings have not been frozen. In Chef 13, in the process
of merging the node attributes strings and other simple objects are
dup'd and frozen. In order to get a mutable copy, you can now correctly
use the `node.dup` or `node.to_hash` methods, or you should mutate the
object correctly through its precedence level like <span
class="title-ref">node.default\["some_string"\] \<\<
"appending_this"</span>.

#### The Chef::REST API has been removed

It has been fully replaced with `Chef::ServerAPI` in chef-client code.

#### Properties overriding methods now raise an error

Defining a property that overrides methods defined on the base ruby
`Object` or on `Chef::Resource` itself can cause large amounts of
confusion. A simple example is `property :hash` which overrides the
Object\#hash method which will confuse ruby when the Custom Resource is
placed into the Chef::ResourceCollection which uses a hash internally
which expects to call Object\#hash to get a unique id for the object.
Attempting to create `property :action` would also override the
Chef::Resource\#action method which is unlikely to end well for the
user. Overriding inherited properties is still supported.

#### `chef-shell` now supports solo and legacy solo modes

Running `chef-shell -s` or `chef-shell --solo` will give you an
experience consistent with `chef-solo`. `chef-shell --solo-legacy-mode`
will give you an experience consistent with `chef-solo --legacy-mode`.

#### Chef::Platform.set and related methods have been removed

The deprecated code has been removed. All providers and resources should
now be using Chef \>= 12.0 `provides` syntax.

#### Remove `sort` option for the Search API

This option has been unimplemented on the server side for years, so any
use of it has been pointless.

#### Remove Chef::ShellOut

This was deprecated and replaced a long time ago with mixlib-shellout
and the shell_out mixin.

#### Remove `method_missing` from the Recipe DSL

The core of chef hasn't used this to implement the Recipe DSL since
12.5.1 and its unlikely that any external code depended upon it.

#### Simplify Recipe DSL wiring

Support for actions with spaces and hyphens in the action name has been
dropped. Resources and property names with spaces and hyphens most
likely never worked in Chef-12. UTF-8 characters have always been
supported and still are.

#### `easy_install` resource has been removed

The Python `easy_install` package installer has been deprecated for many
years, so we have removed support for it. No specific replacement for
`pip` is being included with Chef at this time, but a `pip`-based
`python_package` resource is available in the
[poise-python](https://github.com/poise/poise-python) cookbooks.

#### Removal of run_command and popen4 APIs

All the APIs in chef/mixlib/command have been removed. They were
deprecated by mixlib-shellout and the shell_out mixin API.

#### Iconv has been removed from the ruby libraries and chef omnibus build

The ruby Iconv library was replaced by the Encoding library in ruby
1.9.x and since the deprecation of ruby 1.8.7 there has been no need for
the Iconv library but we have carried it forwards as a dependency since
removing it might break some chef code out there which used this
library. It has now been removed from the ruby build. This also removes
LGPLv3 code from the omnibus build and reduces build headaches from
porting iconv to every platform we ship chef-client on.

This will also affect nokogiri, but that gem natively supports UTF-8,
UTF-16LE/BE, ISO-8851-1(Latin-1), ASCII and "HTML" encodings. Users who
really need to write something like Shift-JIS inside of XML will need to
either maintain their own nokogiri installs or will need to convert to
using UTF-8.

#### Deprecated cookbook metadata has been removed

The `recommends`, `suggests`, `conflicts`, `replaces` and `grouping`
metadata fields are no longer supported, and have been removed. Chef
will ignore them in existing `metadata.rb` files, but we recommend that
you remove them.

#### All unignored cookbook files will now be uploaded.

We now treat every file under a cookbook directory as belonging to a
cookbook, unless that file is ignored with a `chefignore` file. This is
a change from the previous behavior where only files in certain
directories, such as `recipes` or `templates`, were treated as special.
This change allows chef to support new classes of files, such as Ohai
plugins or InSpec tests, without having to make changes to the cookbook
format to support them.

#### DSL-based custom resources and providers no longer get module constants

Up until now, creating a `mycook/resources/thing.rb` would create a
`Chef::Resources::MycookThing` name to access the resource class object.
This const is no longer created for resources and providers. You can
access resource classes through the resolver API like:

``` ruby
Chef::Resource.resource_for_node(:mycook_thing, node)
```

Accessing a provider class is a bit more complex, as you need a resource
against which to run a resolution like so:

``` ruby
Chef::ProviderResolver.new(node, find_resource!("mycook_thing[name]"), :nothing).resolve
```

#### Default values for resource properties are frozen

A resource declaring something like:

``` ruby
property :x, default: {}
```

will now see the default value set to be immutable. This prevents cases
of modifying the default in one resource affecting others. If you want a
per-resource mutable default value, define it inside a `lazy{}` helper
like:

``` ruby
property :x, default: lazy { {} }
```

#### ResourceCollection and notifications

Resources which later modify their name during creation will have their
name changed on the ResourceCollection and notifications

``` ruby
some_resource "name_one" do
  name "name_two"
```

The fix for sending notifications to multipackage resources involved
changing the API so that it no longer directly takes the string that is
typed into the DSL but reads the (possibly coerced) name off of the
resource after it is built. The result is that the above resource will
be named `some_resource[name_two]` instead of `some_resource[name_one]`.
Note that setting the name (*not* the `name_property`, but actually
renaming the resource) is very uncommon. The fix is to name the resource
correctly in the first place (`some_resource name_two do`).

#### `use_inline_resources` is always enabled

The `use_inline_resources` provider mode is always enabled when using
the `action :name do ... end` syntax. You can remove the
`use_inline_resources` line.

#### `knife cookbook site vendor` has been removed

Please use `knife cookbook site install` instead.

#### `knife cookbook create` has been removed

Please use `chef generate cookbook` from ChefDK instead.

#### Verify commands no longer support "%{file}"

Chef has always recommended `%{path}`, and `%{file}` has now been
removed.

#### The `partial_search` recipe method has been removed

The `partial_search` method has been fully replaced by the
`filter_result` argument to `search`, and has now been removed.

#### The logger and formatter settings are more predictable

The default now is the formatter. There is no more automatic switching
to the logger when logging or when output is sent to a pipe. The logger
needs to be specifically requested with `--force-logger` or it will not
show up.

The `--force-formatter` option does still exist, although it will
probably be deprecated in the future.

If your logfiles switch to the formatter, you need to include
`--force-logger` for your daemonized runs.

Redirecting output to a file with `chef-client > /tmp/chef.out` now
captures the same output as invoking it directly on the command line
with no redirection.

#### Path Sanity disabled by default and modified

The chef client itself no long modifies its `ENV['PATH']` variable
directly. When using the `shell_out` API now, in addition to setting up
LANG/LANGUAGE/LC_ALL variables that API will also inject certain system
paths and the ruby bindir and gemdirs into the PATH (or Path on
Windows). The `shell_out_with_systems_locale` API still does not mangle
any environment variables. During the Chef-13 lifecycle changes will be
made to prep Chef-14 to switch so that `shell_out` by default behaves
like `shell_out_with_systems_locale`. A new flag will get introduced to
call `shell_out(..., internal: [true|false])` to either get the forced
locale and path settings ("internal") or not. When that is introduced in
Chef 13.x the default will be `true` (backwards-compat with 13.0) and
that default will change in 14.0 to `false`.

The PATH changes have also been tweaked so that the ruby bindir and
gemdir PATHS are prepended instead of appended to the PATH. Some system
directories are still appended.

Some examples of changes:

-   `which ruby` in 12.x will return any system ruby and fall back to
    the embedded ruby if using omnibus
-   `which ruby` in 13.x will return any system ruby and will not find
    the embedded ruby if using omnibus
-   `shell_out_with_systems_locale("which ruby")` behaves the same as
    `which ruby` above
-   `shell_out("which ruby")` in 12.x will return any system ruby and
    fall back to the embedded ruby if using omnibus
-   `shell_out("which ruby")` in 13.x will always return the omnibus
    ruby first (but will find the system ruby if not using omnibus)

The PATH in `shell_out` can also be overridden:

-   `shell_out("which ruby", env: { "PATH" => nil })` - behaves like
    shell_out_with_systems_locale()
-   `shell_out("which ruby", env: { "PATH" => [...include PATH string here...] })` -
    set it arbitrarily however you need

Since most providers which launch custom user commands use
`shell_out_with_systems_locale` (service, execute, script, etc) the
behavior will be that those commands that used to be having embedded
omnibus paths injected into them no longer will. Generally this will fix
more problems than it solves, but may causes issues for some use cases.

#### Default guard clauses (not_if/only_if) do not change the PATH or other env vars

The implementation switched to `shell_out_with_systems_locale` to match
`execute` resource, etc.

#### Chef Client exits the RFC062 defined exit codes

Chef Client will only exit with exit codes defined in RFC 062. This
allows other tooling to respond to how a Chef run completes. Attempting
to exit Chef Client with an unsupported exit code (either via
`Chef::Application.fatal!` or `Chef::Application.exit!`) will result in
an exit code of 1 (GENERIC_FAILURE) and a warning in the event log.

When Chef Client is running as a forked process on unix systems, the
standardized exit codes are used by the child process. To actually have
Chef Client return the standard exit code, `client_fork false` will need
to be set in Chef Client's configuration file.

### New deprecations

-   [Removal of support for Ohai version 6
    plugins](/deprecations_ohai_v6_plugins/)

## What's New in 12.22.3

This release fixes an issue in our Windows security support code that
would occasionally cause heap corruption on Windows. This would manifest
as Chef Client runs that terminated without any logging or errors. Since
the issue was located within the common `get_account_right` method, this
could affect a number of different recipes, but was most often seen when
using the windows_service resource.

This issue is also fixed in the recent Chef 14.0.190 release, and will
be included in the next Chef 13 release expected by the end of the
month.

This is the final planned Chef 12 release, which is currently deprecated
and will become End of Life on April 30th. For additional information on
that process, please see our [Chef 12 and ChefDK 1 EOL
information](https://www.chef.io/eol-chef12-and-chefdk1).

## What's New in 12.22.1

-   **Security Updates**
    -   Ruby has been updated to 2.3.6 to resolve
        [CVE-2017-17405](https://nvd.nist.gov/vuln/detail/CVE-2017-17405)
    -   Libxml2 has been updated to 2.9.7 to resolve
        [CVE-2017-15412](https://access.redhat.com/security/cve/cve-2017-15412)
-   **Ohai 8.26.1**
    -   Ohai now provides EC2 metadata configuration information on the
        new C5/M5 instance types running on Amazon's new hypervisor
    -   The new LsPci plugin provides a `node['pci']` hash with
        information about the PCI bus based on `lspci`. Only runs on
        Linux.
    -   The virtualization plugin has been updated to properly detect
        Docker CE

## What's New in 12.21.31

-   **Support for AArch64 platform on Red Hat Enterprise Linux**
-   **mdadm support for arrays with more than 10 disks**
-   **OpenSSL updated to version 1.0.2**

## What's New in 12.21.26

-   **Security release of libxml2** libxml2 has been upgraded to 2.9.5
    to resolve the following CVEs:
    -   [CVE-2017-9050](https://www.cvedetails.com/cve/CVE-2017-9050/)
    -   [CVE-2017-9049](https://www.cvedetails.com/cve/CVE-2017-9049/)
    -   [CVE-2017-9048](https://www.cvedetails.com/cve/CVE-2017-9048/)
    -   [CVE-2017-9047](https://www.cvedetails.com/cve/CVE-2017-9047/)
    -   [CVE-2017-8872](https://www.cvedetails.com/cve/CVE-2017-8872/)
    -   [CVE-2017-5969](https://www.cvedetails.com/cve/CVE-2017-5969/)
    -   [CVE-2016-9318](https://www.cvedetails.com/cve/CVE-2016-9318/)
    -   [CVE-2016-5131](https://www.cvedetails.com/cve/CVE-2016-5131/)
-   **Security release of libxlst** libxlst has been upgraded to 1.1.30
    to resolve the following CVEs:
    -   [CVE-2017-5029](http://www.cvedetails.com/cve/CVE-2017-5029/)
    -   [CVE-2015-9019](http://www.cvedetails.com/cve/CVE-2015-9019/)
-   **Security release of zlib** zlib has been upgraded to 1.2.11 to
    resolve the following CVEs:
    -   [CVE-2016-9840](https://www.cvedetails.com/cve/CVE-2016-9840/)
    -   [CVE-2016-9841](https://www.cvedetails.com/cve/CVE-2016-9841/)
    -   [CVE-2016-9842](https://www.cvedetails.com/cve/CVE-2016-9842/)
    -   [CVE-2016-9843](https://www.cvedetails.com/cve/CVE-2016-9843/)
-   **Security release of openssl** openssl has been upgraded to 1.0.2j
    to resolve the following CVEs:
    -   [CVE-2017-3731](http://www.cvedetails.com/cve/CVE-2017-3731)
    -   [CVE-2017-3732](http://www.cvedetails.com/cve/CVE-2017-3732)
    -   [CVE-2016-7055](http://www.cvedetails.com/cve/CVE-2016-7055)
-   **Security release of rubygems** rubygems has been upgraded to
    2.6.14 to resolve the following CVEs:
    -   [CVE-2017-0903](http://www.cvedetails.com/cve/CVE-2017-0903)
-   **Ruby 2.2 compatibility** a regression in the 12.21.20 release has
    been corrected to restore full compatibility with Ruby 2.2 and later
-   **Ohai Critical Plugins** Ohai has been upgraded to 8.25 with
    support for Ohai critical plugins.

### Ohai Critical Plugins Functionality

Users can now specify a list of plugins which are critical for the Chef
run. Critical plugins will cause Ohai to fail if they do not run
successfully (and thus cause a Chef run using Ohai to fail). The syntax
for this is:

``` ruby
ohai.critical_plugins << :Filesystem
```

## What's New in 12.21.20

-   **Improved dsc_script logging** Converge logging in `dsc_script`
    has been improved
-   **DNF Improvements** Accuracy in determining when to use the
    `dnf_package` resource has been improved. DNF will no longer be used
    on RHEL 7 systems that have it installed, and the determination
    logic performance has been greatly improved.

## What's New in 12.21.14

-   **apt_repository APT key fingerprint fixes** `apt_repository` now
    correctly checks APT key fingerprints on newer systems

## What's New in 12.21.12

-   **DSC Windows Management Framework 5** DSC has been updated to work
    properly with Windows Management Framework 5
-   **Security release of Ruby** RubyGems has been upgraded to 2.3.5 to
    address the following CVEs:
    -   [CVE-2017-0898](https://nvd.nist.gov/vuln/detail/CVE-2017-0898)
    -   [CVE-2017-10784](https://nvd.nist.gov/vuln/detail/CVE-2017-10784)
    -   [CVE-2017-14033](https://nvd.nist.gov/vuln/detail/CVE-2017-14033)
    -   [CVE-2017-14064](https://nvd.nist.gov/vuln/detail/CVE-2017-14064)

## What's New in 12.21.10

-   **Security release of RubyGems** RubyGems has been upgraded to
    2.6.13 to address the following:
    -   [CVE-2017-0899](https://nvd.nist.gov/vuln/detail/CVE-2017-0899)
    -   [CVE-2017-0900](https://nvd.nist.gov/vuln/detail/CVE-2017-0900)
    -   [CVE-2017-0901](https://nvd.nist.gov/vuln/detail/CVE-2017-0901)
    -   [CVE-2017-0902](https://nvd.nist.gov/vuln/detail/CVE-2017-0902)
-   **Attribute Performance** Attribute performance has been improved
    when utilizing large numbers of merged attributes

## What's New in 12.21.4

-   **Improved Resource Reporting** Resource reporting for Chef Automate
    has been improved
-   **Ruby Upgrade** Ruby has been updated to 2.3.4
-   **RubyGems Upgrade** RubyGems has been updated to 2.6.12 to prevent
    a segfault on Windows
-   **Policyfile fix** Chef client now properly sends expanded run list
    events for policy file nodes

## What's New in 12.21.1

### zlib Security Update

zlib has been updated to resolve the following CVEs:

-   [CVE-2016-98406](https://nvd.nist.gov/vuln/detail/CVE-2016-98406)
-   [CVE-2016-98414](https://nvd.nist.gov/vuln/detail/CVE-2016-98414)
-   [CVE-2016-98423](https://nvd.nist.gov/vuln/detail/CVE-2016-98423)
-   [CVE-2016-98434](https://nvd.nist.gov/vuln/detail/CVE-2016-98434)

### On Debian prefer Systemd to Upstart

On Debian systems, packages that support systemd will often ship both an
old style init script and a systemd unit file. When this happened, Chef
would incorrectly choose Upstart rather than systemd as the service
provider. Chef will now prefer systemd where available.

### Handle the 'supports' property better

Chef 13 removed the `supports` property from core resources. Chef 12 was
incorrectly giving a deprecation notice for another propeerty called
`support`, which prevented users from properly testing their cookbooks
for upgrades.

### Don't crash if downgrading from Chef 13 to 12

On systems where Chef 13 had been run, Chef 12 would crash as the
on-disk cookbook format has changed. Chef 12 now correctly ignores the
unexpected files.

### Provide better information during failures

When Chef client fails, the output now includes details about the
platform and version of Chef that was running, so that a bug report has
more detail.

## What's New in 12.20

The following items are new for chef-client 12.20, or introduce changes
from previous versions:

### Server Enforced Recipe

This release adds support for Server Enforced Recipe, as described in
[RFC
896](https://github.com/chef/chef-rfc/blob/master/rfc089-server-enforced-recipe.md)
and implemented in Chef server 12.15. Full release documentation of this
feature will be coming soon.

### Bugfixes

Fixes issue where the [apt_repository](/resources/apt_repository/)
resource couldn't identify key fingerprints when gnupg 2.1.x was used.

## What's New in 12.19

The following items are new for chef-client 12.19 and/or are changes
from previous versions. The short version:

-   **Systemd unit files are now verified before being installed.**
-   **Added support for windows alternate user identity in execute
    resources.**
-   **Added ed25519 key support for ssh connections.**

### Windows alternate user identity execute support

The `execute` resource and similar resources such as `script`, `batch`,
and `powershell_script` now support the specification of credentials on
Windows so that the resulting process is created with the security
identity that corresponds to those credentials.

**Note**: When Chef is running as a service, this feature requires that
the user that Chef runs as has 'SeAssignPrimaryTokenPrivilege' (aka
'SE_ASSIGNPRIMARYTOKEN_NAME') user right. By default only LocalSystem
and NetworkService have this right when running as a service. This is
necessary even if the user is an Administrator.

This right can be added and checked in a recipe using this example:

``` ruby
# Add 'SeAssignPrimaryTokenPrivilege' for the user
Chef::ReservedNames::Win32::Security.add_account_right('<user>', 'SeAssignPrimaryTokenPrivilege')

# Check if the user has 'SeAssignPrimaryTokenPrivilege' rights
Chef::ReservedNames::Win32::Security.get_account_right('<user>').include?('SeAssignPrimaryTokenPrivilege')
```

### Properties

The following properties are new or updated for the `execute`, `script`,
`batch`, and `powershell_script` resources and any resources derived
from them:

`user`

:   **Ruby Type:** String The user name of the user identity with which
    to launch the new process. The user name may optionally be specified
    with a domain, i.e. `domain\user` or `user@my.dns.domain.com` via
    Universal Principal Name (UPN) format. It can also be specified
    without a domain simply as `user` if the domain is instead specified
    using the `domain` attribute. On Windows only, if this property is
    specified, the `password` property **must** be specified.

`password`

:   **Ruby types** String _Windows only:_ The password of the user
    specified by the `user` property. This property is mandatory if
    `user` is specified on Windows and may only be specified if `user`
    is specified. The `sensitive` property for this resource will
    automatically be set to `true` if `password` is specified.

`domain`

:   **Ruby types** String _Windows only:_ The domain of the user
    specified by the `user` property. If not specified, the user name
    and password specified by the `user` and `password` properties will
    be used to resolve that user against the domain in which the system
    running Chef client is joined, or if that system is not joined to a
    domain it will resolve the user as a local account on that system.
    An alternative way to specify the domain is to leave this property
    unspecified and specify the domain as part of the `user` property.

### Usage

The following examples explain how alternate user identity properties
can be used in the execute resources:

``` ruby
powershell_script 'create powershell-test file' do
  code <<-EOH
  $stream = [System.IO.StreamWriter] "#{Chef::Config[:file_cache_path]}/powershell-test.txt"
  $stream.WriteLine("In #{Chef::Config[:file_cache_path]}...word.")
  $stream.close()
  EOH
  user 'username'
  password 'password'
end

execute 'mkdir test_dir' do
  cwd Chef::Config[:file_cache_path]
  domain "domain-name"
  user "user"
  password "password"
end

script 'create test_dir' do
  interpreter "bash"
  code  "mkdir test_dir"
  cwd Chef::Config[:file_cache_path]
  user "domain-name\\username"
  password "password"
end

batch 'create test_dir' do
  code "mkdir test_dir"
  cwd Chef::Config[:file_cache_path]
  user "username@domain-name"
  password "password"
end
```

### Highlighted bug fixes for this release:

-   **Ensure that the Windows Administrator group can access the
    chef-solo nodes directory**
-   **When loading a cookbook in Chef Solo, use \`\`metadata.json\`\` in
    preference to \`\`metadata.rb\`\`.**

## What's New in 12.18

The following items are new for chef-client 12.18 and/or are changes
from previous versions. The short version:

-   **Can now specify the acceptable return codes from the
    chocolatey_package resource using the returns property**
-   **Can now enable chef-client to run as a scheduled task directly
    from the client MSI on Windows hosts**
-   **Package provider now supports DNF packages for Fedora and upcoming
    RHEL releases**

### New deprecations

-   [Chef::Platform helper
    methods](/deprecations_chef_platform_methods/)
-   [run_command helper method](/deprecations_run_command/)
-   [DNF package allow_downgrade
    property](/deprecations_dnf_package_allow_downgrade/)

## What's New in 12.17

The following items are new for chef-client 12.17 and/or are changes
from previous versions. The short version:

-   **Added msu_package resource and provider**
-   **Added alias unmount to umount action for mount resource**
-   **Can now delete multiple nodes/clients in knife**
-   **Haskell language plugin added to Ohai**

### msu_package resource

The **msu_package** resource installs or removes Microsoft Update(MSU)
packages on Microsoft Windows machines. Here are some examples:

**Using local path in source**

``` ruby
msu_package 'Install Windows 2012R2 Update KB2959977' do
  source 'C:\Users\xyz\AppData\Local\Temp\Windows8.1-KB2959977-x64.msu'
  action :install
end
```

``` ruby
msu_package 'Remove Windows 2012R2 Update KB2959977' do
  source 'C:\Users\xyz\AppData\Local\Temp\Windows8.1-KB2959977-x64.msu'
  action :remove
end
```

**Using URL in source**

``` ruby
msu_package 'Install Windows 2012R2 Update KB2959977' do
  source 'https://s3.amazonaws.com/my_bucket/Windows8.1-KB2959977-x64.msu'
  action :install
end
```

``` ruby
msu_package 'Remove Windows 2012R2 Update KB2959977' do
  source 'https://s3.amazonaws.com/my_bucket/Windows8.1-KB2959977-x64.msu'
  action :remove
end
```

### `unmount` alias for `umount` action

Now you can use `action :unmount` to unmout a mount point through the
mount resource. For example:

``` ruby
mount '/mount/tmp' do
  action :unmount
end
```

### Multiple client/node deletion in knife

You can now pass multiple nodes/clients to `knife node delete` or
`knife client delete` subcommands.

``` bash
knife client delete client1,client2,client3
```

### Ohai Enhancements

**Haskell Language plugin**

Haskell is now detected in a new haskell language plugin:

``` javascript
"languages": {
  "haskell": {
    "stack": {
      "version": "1.2.0",
      "description": "Version 1.2.0 x86_64 hpack-0.14.0"
    }
  }
}
```

**LSB Release Detection**

The lsb_release command line tool is now preferred to the contents of
`/etc/lsb-release` for release detection. This resolves an issue where a
distro can be upgraded, but `/etc/lsb-release` is not upgraded to
reflect the change.

## What's New in 12.16

The following items are new for chef-client 12.16 and/or are changes
from previous versions. The short version:

-   **Added new attribute_changed event hook**
-   **Automatic connection to Chef Automate's data collector through
    Chef server**
-   **Added new --field-separator flag to knife show commands**

### `attribute_changed` event hook

In a cookbook library file, you can add this in order to print out all
attribute changes in cookbooks:

``` ruby
Chef.event_handler do
  on :attribute_changed do |precedence, key, value|
    puts "setting attribute #{precedence}#{key.map {|n| "[\"#{n}\"]" }.join} = #{value}"
  end
end
```

If you want to setup a policy that override attributes should never be
used:

``` ruby
Chef.event_handler do
  on :attribute_changed do |precedence, key, value|
    raise "override policy violation" if precedence == :override
  end
end
```

### Automatic connection to Chef Automate's data collector with supported Chef server

Chef client will automatically attempt to connect to the Chef server
authenticated data collector proxy. If you have a supported version of
Chef server with this feature enabled, Chef client run data will
automatically be forwarded to Chef Automate without additional Chef
client configuration. If you do not have Chef Automate, or the feature
is disabled on the Chef server, Chef client will detect this and disable
data collection.

{{< note >}}

Chef Server 12.11.0 or newer is required for this feature.

{{< /note >}}

### RFC018 Partially Implemented: Specify `--field-separator` for attribute filtering

If you have periods (`.`) in your Chef Node attribute keys, you can now
pass the `--field-separator` (or `-S`) flag along with your
`--attribute` (or `-a`) flag to specify a custom nesting character other
than `.`.

In a situation where the *webapp* node has the following node data:

``` javascript
{
  "foo.bar": "baz",
  "alpha": {
    "beta": "omega"
  }
}
```

Running `knife node show` with the default field separator (`.`) won't
show us the data we're expecting for the `foo.bar` attribute because of
the period:

``` bash
knife node show webapp -a foo.bar
webapp:
  foo.bar:

knife node show webapp -a alpha.beta
webapp:
  alpha.beta: omega
```

However, by specifying a field separator other than `.` we are now able
to show the data.

``` bash
knife node show webapp -S: -a foo.bar
webapp:
  foo.bar: baz

knife node show webapp -S: -a alpha:beta
webapp:
  alpha:beta: omega
```

### Package locking for Apt, Yum, and Zypper

To allow for more fine grain control of package installation the
`apt_package`, `yum_package`, and `zypper_package` resources now support
the `:lock` and `:unlock` actions.

``` ruby
package "httpd" do
  action :lock
end

package "httpd" do
  action :unlock
end
```

## What's New in 12.15

The following items are new for chef-client 12.15 and/or are changes
from previous versions. The short version:

-   **Omnibus packages are now available for Ubuntu 16.04**
-   **New cab_package resource** Supports the installation of cabinet
    packages on Microsoft Windows.
-   **Added new Chef client exit code (213)** New exit code when Chef
    client exits during upgrade.
-   **Default for gpgcheck on yum_repository resource is set to true**
-   **Allow deletion of registry_key without the need for users to pass
    data key in values hash**
-   **If provided, knife ssh will pass the -P option on the command line
    as the sudo password and will bypass prompting**

### cab_package

Supports the installation of cabinet packages on Microsoft Windows. For
example:

``` ruby
cab_package 'Install .NET 3.5 sp1 via KB958488' do
  source 'C:\Users\xyz\AppData\Local\Temp\Windows6.1-KB958488-x64.cab'
  action :install
end
```

``` ruby
cab_package 'Remove .NET 3.5 sp1 via KB958488' do
  source 'C:\Users\xyz\AppData\Local\Temp\Windows6.1-KB958488-x64.cab'
  action :remove
end
```

{{< note >}}

The `cab_package` resource does not support URL strings in the source
property.

{{< /note >}}

### exit code 213

This new exit code signals Chef has exited during a client upgrade. This
allows for easier testing of chef client upgrades in Test Kitchen. See
[Chef
Killing](https://github.com/chef-cookbooks/omnibus_updater#chef-killing)
in the omnibus_updater cookbook for more information.

## What's New in 12.14

The following items are new for chef-client 12.14 and/or are changes
from previous versions. The short version:

-   **Upgraded Ruby version from 2.1.9 to 2.3.1** Adds several
    performance and functionality enhancements.
-   **Now support for Chef client runs on Windows Nano Server** A small
    patch to Ruby 2.3.1 and improvements to the Ohai network plugin now
    allow you to do chef client runs on Windows Nano Server.
-   **New yum_repository resource** Use the **yum_repository**
    resource to manage a yum repository configuration file.
-   **Added the ability to mark a property of a custom resource as
    sensitive** This will suppress the property's value when it's used
    in other outputs, such as messages used by the data collector.

### yum_repository

Use the **yum_repository** resource to manage a Yum repository
configuration file located at `/etc/yum.repos.d/repositoryid.repo` on
the local machine. This configuration file specifies which repositories
to reference, how to handle cached data, etc.

For syntax, a list of properties and actions, see
[yum_repository](/resources/yum_repository/).

### sensitive: true

Some properties in custom resources may include sensitive data, such as
a password for a database server. When the resource's state is built for
use by data collector or a similar auditing tool, a hash is built of all
state properties for that resource and their values. This leads to
sensitive data being transmitted and potentially stored in the clear.

Individual properties can now be marked as sensitive and then have the
value of that property suppressed when exporting the resource's state.
To do this, add `sensitive: true` when defining the property, such as in
the following example:

``` ruby
property :db_password, String, sensitive: true
```

## What's New in 12.13

The following items are new for chef-client 12.13 and/or are changes
from previous versions. The short version:

-   **Ohai 8.18 includes new plugin for gathering available user
    shells** Other additions include a new hardware plugin for macOS
    that gathers system information and detection of VMWare and
    VirtualBox installations.
-   **New Chef client option to override any config key/value pair** Use
    `chef-client --config-option` to override any config setting from
    the command line.

### --config-option

Use the `--config-option` option to override a single configuration
option when calling a command on `chef-client`. To override multiple
configuration options, simply add additional `--config-option` options
like in the following example:

``` bash
chef-client --config-option chef_server_url=http://example --config-option policy_name=web"
```

### Updated Dependencies

-   ruby - 2.1.9 (from 2.1.8)

#### Updated Gems

-   chef-zero - 4.8.0 (from 4.7.0)
-   cheffish - 2.0.5 (from 2.0.4)
-   compat_resource - 12.10.7 (from 12.10.6)
-   ffi - 1.9.14 (from 1.9.10)
-   ffi-yajl - 2.3.0 (from 2.2.3)
-   fuzzyurl - 0.9.0 (from 0.8.0)
-   mixlib-cli - 1.7.0 (from 1.6.0)
-   mixlib-log - 1.7.0 (from 1.6.0)
-   ohai - 8.18.0 (from 8.17.1)
-   pry - 0.10.4 (from 0.10.3)
-   rspec - 3.5.0 (from 3.4.0)
-   rspec-core - 3.5.2 (from 3.4.4)
-   rspec-expectations - 3.5.0 (from 3.4.0)
-   rspec-mocks - 3.5.0 (from 3.4.1)
-   rspec-support - 3.5.0 (from 3.4.1)
-   simplecov - 0.12.0 (from 0.11.2)
-   specinfra - 2.60.3 (from 2.59.4)
-   mixlib-archive - 0.2.0 (added to package)

## What's New in 12.12

The following items are new for chef-client 12.12 and/or are changes
from previous versions. The short version:

-   **New node attribute APIs** Common set of methods to read, write,
    delete, and check if node attributes exist.
-   **Data collector updates** Minor enhancements to data that the data
    collector reports on.
-   **knife cookbook create has been deprecated** You should use [chef
    generate cookbook](/ctl_chef/#chef-generate-cookbook) instead.

### New node attribute read, write, unlink, and exist? APIs

The four methods `read`, `write`, `unlink`, and `exist?` (and their
corresponding unsafe versions) can be used on node objects to set,
retrieve, delete, and validate existance of attributes.

#### read/read!

Use the `read` method to retrieve an attribute value on a node object.
It is a safe, non-autovivifying reader that returns `nil` if the
attribute does not exist.

`node.read("foo", "bar", "baz")` is equivalent to
`node["foo"]["bar"]["baz"]` but returns `nil` instead of raising an
exception when no value is set.

The `read!` method is a non-autovivifying reader that also retrieves an
attribute value on a node object; however, it will throw a NoMethodError
exception if the attribute does not exist.

On the node level, `node.default.read/read!("foo")` behaves similarly to
`node.read("foo")`, but only on the default level.

#### write/write!

Use the `write` method set an attribute value on a node object. It is a
safe, autovivifying writer that replaces intermediate non-hash objects.

`node.write(:default, "foo", "bar", "baz")` is equivalent to
`node.default["foo"]["bar"] = "baz"`.

The `write!` method is also an autovivifying method to set an attribute
value on a node object; however, it will throw an NoSuchAttribute
exception if there is a non-hash on an intermediate key.

{{< note >}}

There is currently no non-autovivifying writer method for attributes.

{{< /note >}}

On the node level, `node.default.write/write!("foo", "bar")` is
equivalent to `node.write/write!(:default, "foo", "bar")`.

#### unlink/unlink!

Use the `unlink` method to delete an attribute on a node object. `nil`
will be returned if the value is not a valid Hash or Array.

The `unlink!` method also deletes an attribute on a node object;
however, it will throw a NoSuchAttribute exception if the attribute does
not exist.

On the node level, `node.default.unlink/unlink!("foo")` is equivalent to
`node.unlink/unlink!(:default, "foo")`.

#### exist?

Use the `exist?` method to check whether the attribute exists. For
example, `node.exist?("foo", "bar")` can be used to see if
`node["foo"]["bar"]` exists.

On the node level, `node.default.exist?("foo", "bar")` can be used to
see if `node.default["foo"]["bar"]` exists.

### Depreciated node attribute methods

The following methods have been deprecated in this release:

-   `node.set`
-   `node.set_unless`

### data_collector updates

-   Adds `node` to the data_collector message.
-   `data_collector` reports on all resources and not just those that
    have been processed.

## What's New in 12.11

The following items are new for chef-client 12.11 and/or are changes
from previous versions. The short version:

-   **Support for standard exit codes in Chef client** Standard exit
    codes are now used by Chef client and should be identical across all
    OS platforms. New configuration setting `exit_status` has been added
    to specify how Chef client reports non-standard exit codes.
-   **New data collector functionality for run statistics** New feature
    that provides a unified method for sharing statistics about your
    Chef runs in webhook-like manner.
-   **Default chef-solo behavior is equivalent to chef-client local
    mode** chef-solo now uses chef-client local mode. To use the
    previous `chef-solo` behavior, run in `chef-solo --legacy-mode`.
-   **New systemd_unit resource** Use the **systemd_unit** to manage
    systemd units.

### exit_status

When set to `:enabled`, chef-client will use [standardized exit
codes](https://github.com/chef/chef-rfc/blob/master/rfc062-exit-status.md#exit-codes-in-use)
for Chef client run status, and any non-standard exit codes will be
converted to `1` or `GENERIC_FAILURE`. This setting can also be set to
`:disabled` which preserves the old behavior of using non-standardized
exit codes and skips the deprecation warnings. Default value: `nil`.

{{< note >}}

The behavior with the default value consists of a warning on the use of
deprecated and non-standard exit codes. In a future release of Chef
client, using standardized exit codes will be the default behavior.

{{< /note >}}

### Data collector

The data collector feature is new to Chef 12.11. It provides a unified
method for sharing statistics about your Chef runs in a webhook-like
manner. The data collector supports Chef in all its modes: Chef client,
Chef solo (commonly referred to as "Chef client local mode"), and Chef
solo legacy mode.

To enable the data collector, specify the following settings in your
client configuration file:

-   `data_collector.server_url`: Required. The URL to which the Chef
    client will POST the data collector messages
-   `data_collector.token`: Optional. An token which will be sent in a
    x-data-collector-token HTTP header which can be used to authenticate
    the message.
-   `data_collector.mode`: The Chef mode in which the data collector
    should run. Chef client mode is chef client configured to use Chef
    server to provide Chef client its resources and artifacts. Chef solo
    mode is Chef client configured to use a local Chef zero server
    (`chef-client --local-mode`). This setting also allows you to only
    enable data collector in Chef solo mode but not Chef client mode.
    Available options are `:solo`, `:client`, or `:both`. Default is
    `:both`.
-   `data_collector.raise_on_failure`: If enabled, Chef will raise an
    exception and fail to run if the data collector cannot be reached at
    the start of the Chef run. Defaults to false.
-   `data_collector.organization`: Optional. In Chef solo mode, the
    organization field in the messages will be set to this value.
    Default is `chef_solo`. This field does not apply to Chef client
    mode.

### Replace previous Chef-solo behavior with Chef client local mode

The default operation of chef-solo is now the equivalent to
`chef-client -z` or `chef-client --local-mode`, but you can use the
previous chef-solo behavior by running in `chef-solo --legacy-mode`. As
part of this change, environment and role files written in ruby are now
fully supported by `knife upload`.

### systemd_unit

Use the **systemd_unit** resource to create, manage, and run [systemd
units](https://www.freedesktop.org/software/systemd/man/systemd.html#Concepts).

#### Syntax

A **systemd_unit** resource describes the configuration behavior for
systemd units. For example:

``` ruby
systemd_unit 'sysstat-collect.timer' do
  content({
    'Unit' => {
      'Description' => 'Run system activity accounting tool every 10 minutes'
    },
    'Timer' => {
      'OnCalendar' => '*:00/10'
    },
    'Install' => {
      'WantedBy' => 'sysstat.service'
    }
  })
  action [:create, :enable, :start]
end
```

The full syntax for all of the properties that are available to the
**systemd_unit** resource is:

``` ruby
systemd_unit 'name' do
  user                   String
  content                String or Hash
  triggers_reload        Boolean
end
```

where

-   `name` is the name of the unit
-   `user` is the user account that systemd units run under. If not
    specified, systemd units will run under the system account.
-   `content` describes the behavior of the unit
-   `triggers_reload` controls if a <span
    class="title-ref">daemon-reload</span> is executed to load the unit

#### Actions

This resource has the following actions:

`:create`

:   Create a unit file, if it does not already exist.

`:delete`

:   Delete a unit file, if it exists.

`:enable`

:   Ensure the unit will be started after the next system boot.

`:disable`

:   Ensure the unit will not be started after the next system boot.

`:nothing`

:   Default. Do nothing with the unit.

`:mask`

:   Ensure the unit will not start, even to satisfy dependencies.

`:unmask`

:   Stop the unit from being masked and cause it to start as specified.

`:start`

:   Start a unit based in its systemd unit file.

`:stop`

:   Stop a running unit.

`:restart`

:   Restart a unit.

`:reload`

:   Reload the configuration file for a unit.

`:try_restart`

:   Try to restart a unit if the unit is running.

`:reload_or_restart`

:   For units that are services, this action reloads the configuration
    of the service without restarting, if possible; otherwise, it will
    restart the service so the new configuration is applied.

`:reload_or_try_restart`

:   For units that are services, this action reloads the configuration
    of the service without restarting, if possible; otherwise, it will
    try to restart the service so the new configuration is applied.

#### Properties

This resource has the following properties:

`user`

:   **Ruby Type:** String

    The user account that the systemd unit process is run under. The
    path to the unit for that user would be something like
    `/etc/systemd/user/sshd.service`. If no user account is specified,
    the systemd unit will run under a `system` account, with the path to
    the unit being something like `/etc/systemd/system/sshd.service`.

`content`

:   **Ruby Type:** String, Hash

    A string or hash that contains a systemd [unit
    file](https://www.freedesktop.org/software/systemd/man/systemd.unit.html)
    definition that describes the properties of systemd-managed
    entities, such as services, sockets, devices, and so on.

`triggers_reload`

:   **Ruby Type:** true, false | **Default Value:** `true`

    Specifies whether to trigger a daemon reload when creating or
    deleting a unit.

`verify`

:   **Ruby Type:** true, false | **Default Value:** `true`

    Specifies if the unit will be verified before installation. Systemd
    can be overly strict when verifying units, so in certain cases it is
    preferable not to verify the unit.

## What's New in 12.10

The following items are new for chef-client 12.10 and/or are changes
from previous versions. The short version:

-   **New layout property for mdadm resource** Use the `layout` property
    to set the RAID5 parity algorithm. Possible values:
    `left-asymmetric` (or `la`), `left-symmetric` (or `ls`),
    `right-asymmetric` (or `ra`), or `right-symmetric` (or `rs`).
-   **New with_run_context for the Recipe DSL** Use `with_run_context`
    to run resource blocks as part of the root or parent run context.
-   **New Recipe DSL methods for declaring, deleting, editing, and
    finding resources** Use the `declare_resource`, `delete_resource`,
    `edit_resource`, and `find_resource` methods to interact with
    resources in the resource collection. Use the `delete_resource!`,
    `edit_resource!`, or `find_resource!` methods to trigger an
    exception when the resource is not found in the collection.

### with_run_context

Use the `with_run_context` method to define a block that has a pointer
to a location in the `run_context` hierarchy. Resources in recipes
always run at the root of the `run_context` hierarchy, whereas custom
resources and notification blocks always build a child `run_context`
which contains their sub-resources.

The syntax for the `with_run_context` method is as follows:

``` ruby
with_run_context :type do
  # some arbitrary pure Ruby stuff goes here
end
```

where `:type` may be one of the following:

-   `:root` runs the block as part of the root `run_context` hierarchy
-   `:parent` runs the block as part of the parent process in the
    `run_context` hierarchy

For example:

``` ruby
action :run do
  with_run_context :root do
    edit_resource(:my_thing, "accumulated state") do
      action :nothing
      my_array_property << accumulate_some_stuff
    end
  end
  log "kick it off" do
    notifies :run, "my_thing[accumulated state]", :delayed
  end
end
```

### declare_resource

Use the `declare_resource` method to instantiate a resource and then add
it to the resource collection.

The syntax for the `declare_resource` method is as follows:

``` ruby
declare_resource(:resource_type, 'resource_name', resource_attrs_block)
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.
-   `resource_attrs_block` is a block in which properties of the
    instantiated resource are declared.

For example:

``` ruby
declare_resource(:file, '/x/y.txy', caller[0]) do
  action :delete
end
```

is equivalent to:

``` ruby
file '/x/y.txt' do
  action :delete
end
```

### delete_resource

Use the `delete_resource` method to find a resource in the resource
collection, and then delete it.

The syntax for the `delete_resource` method is as follows:

``` ruby
delete_resource(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
delete_resource(:template, '/x/y.erb')
```

### delete_resource!

Use the `delete_resource!` method to find a resource in the resource
collection, and then delete it. If the resource is not found, an
exception is returned.

The syntax for the `delete_resource!` method is as follows:

``` ruby
delete_resource!(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
delete_resource!(:file, '/x/file.txt')
```

### edit_resource

Use the `edit_resource` method to:

-   Find a resource in the resource collection, and then edit it.
-   Define a resource block. If a resource block with the same name
    exists in the resource collection, it will be updated with the
    contents of the resource block defined by the `edit_resource`
    method. If a resource block does not exist in the resource
    collection, it will be created.

The syntax for the `edit_resource` method is as follows:

``` ruby
edit_resource(:resource_type, 'resource_name', resource_attrs_block)
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.
-   `resource_attrs_block` is a block in which properties of the
    instantiated resource are declared.

For example:

``` ruby
edit_resource(:template, '/x/y.txy') do
  cookbook 'cookbook_name'
end
```

and a resource block:

``` ruby
edit_resource(:template, '/etc/aliases') do
  source 'aliases.erb'
  cookbook 'aliases'
  variables({:aliases => {} })
  notifies :run, 'execute[newaliases]'
end
```

### edit_resource!

Use the `edit_resource!` method to:

-   Find a resource in the resource collection, and then edit it.
-   Define a resource block. If a resource with the same name exists in
    the resource collection, its properties will be updated with the
    contents of the resource block defined by the `edit_resource`
    method.

In both cases, if the resource is not found, an exception is returned.

The syntax for the `edit_resource!` method is as follows:

``` ruby
edit_resource!(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.
-   `resource_attrs_block` is a block in which properties of the
    instantiated resource are declared.

For example:

``` ruby
edit_resource!(:file, '/x/y.rst')
```

### find_resource

Use the `find_resource` method to:

-   Find a resource in the resource collection.
-   Define a resource block. If a resource block with the same name
    exists in the resource collection, it will be returned. If a
    resource block does not exist in the resource collection, it will be
    created.

The syntax for the `find_resource` method is as follows:

``` ruby
find_resource(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
find_resource(:template, '/x/y.txy')
```

and a resource block:

``` ruby
find_resource(:template, '/etc/seapower') do
  source 'seapower.erb'
  cookbook 'seapower'
  variables({:seapower => {} })
  notifies :run, 'execute[newseapower]'
end
```

### find_resource!

Use the `find_resource!` method to find a resource in the resource
collection. If the resource is not found, an exception is returned.

The syntax for the `find_resource!` method is as follows:

``` ruby
find_resource!(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
find_resource!(:template, '/x/y.erb')
```

## What's New in 12.9

The following items are new for chef-client 12.9 and/or are changes from
previous versions. The short version:

-   **New apt_repository resource**
-   **64-bit chef-client for Microsoft Windows** Starting with
    chef-client 12.9, 64-bit
-   **New property for the mdadm resource** Use the `mdadm_defaults`
    property to set the default values for `chunk` and `metadata` to
    `nil`, which allows mdadm to apply its own default values.
-   **File redirection in Windows for 32-bit applications** Files on
    Microsoft Windows that are managed by the **file** and **directory**
    resources are subject to file redirection, depending if Chef Client
    is 64-bit or 32-bit.
-   **Registry key redirection in Windows for 32-bit applications**
    Registry keys on Microsoft Windows that are managed by the
    **registry_key** resource are subject to key redirection, depending
    if Chef Client is 64-bit or 32-bit.
-   **New values for log_location** Use `:win_evt` to write log output
    to the (Windows Event Logger and `:syslog` to write log output to
    the syslog daemon facility with the originator set as `chef-client`.
-   **New timeout setting for knife ssh** Set the `--ssh-timeout`
    setting to an integer (in seconds) as part of a `knife ssh` command.
    The `ssh_timeout` setting may also be configured (as seconds) in the
    knife.rb file.
-   **New "seconds to wait before first chef-client run" setting** The
    `-daemonized` option for Chef Client now allows the seconds to wait
    before starting Chef Client run to be specified. For example, if
    `--daemonize 10` is specified, Chef Client will wait ten seconds.

### apt_repository resource

The apt_repository resource, previously available in the apt cookbook,
is now included in chef-client. With this change you will no longer need
to depend on the apt cookbook to use the apt_repository resource.

### 64-bit chef-client

Chef Client now runs on 64-bit Microsoft Windows operating systems.

-   Support for file redirection
-   Support for key redirection

#### File Redirection

64-bit versions of Microsoft Windows have a 32-bit compatibility layer
that redirects attempts by 32-bit application to access the `System32`
directory to a different location. Starting with chef-client version
12.9, the 32-bit version of Chef Client is subject to the file
redirection policy.

For example, consider the following script:

``` ruby
process_type = ENV['PROCESSOR_ARCHITECTURE'] == 'AMD64' ? '64-bit' : '32-bit'
system32_dir = ::File.join(ENV['SYSTEMROOT'], 'system32')
test_dir = ::File.join(system32_dir, 'cheftest')
test_file = ::File.join(test_dir, 'chef_architecture.txt')

directory test_dir do
  # some directory
end

file test_file do
  content "Chef made me, I come from a #{process_type} process."
end
```

When running a 32-bit version of chef-client, the script will write the
`chef_architecture` file to the `C:\Windows\SysWow64` directory.
However, when running a native 64-bit version of the chef-client, the
script will write a file to the `C:\Windows\System32` directory, as
expected.

For more information, see: [File System
Redirector](https://msdn.microsoft.com/en-us/library/windows/desktop/aa384187(v=vs.85).aspx).

#### Key Redirection

64-bit versions of Microsoft Windows have a 32-bit compatibility layer
in the registry that reflects and redirects certain keys (and their
values) into specific locations (or logical views) of the registry hive.

Chef Client can access any reflected or redirected registry key. The
machine architecture of the system on which Chef Client is running is
used as the default (non-redirected) location. Access to the `SysWow64`
location is redirected must be specified. Typically, this is only
necessary to ensure compatibility with 32-bit applications that are
running on a 64-bit operating system.

32-bit versions of Chef Client (12.8 and earlier) and 64-bit versions of
Chef Client (12.9 and later) generally behave the same in this
situation, with one exception: it is only possible to read and write
from a redirected registry location using chef-client version 12.9 (and
later).

For more information, see: [Registry
Reflection](https://msdn.microsoft.com/en-us/library/windows/desktop/aa384235(v=vs.85).aspx).

## What's New in 12.8

The following items are new for chef-client 12.8 and/or are changes from
previous versions. The short version:

-   **Support for OpenSSL validation of FIPS** Chef Client can be
    configured to allow OpenSSL to enforce FIPS-validated security
    during a chef-client run.
-   **Support for multiple configuration files** Chef Client supports
    reading multiple configuration files by putting them inside a `.d`
    configuration directory.
-   **New launchd resource** Use the **launchd** resource to manage
    system-wide services (daemons) and per-user services (agents) on the
    macOS platform.
-   **chef-zero support for Chef Server API endpoints** chef-zero now
    supports using all Chef server API version 12 endpoints, with the
    exception of `/universe`.
-   **Updated support for OpenSSL** OpenSSL is updated to version 1.0.1.
-   **Ohai auto-detects hosts for Azure instances** Ohai will
    auto-detect hosts for instances that are hosted by Microsoft Azure.
-   **gem attribute added to metadata.rb** Specify a gem dependency to
    be installed via the **chef_gem** resource after all cookbooks are
    synchronized, but before any other cookbook loading is done.

### FIPS Mode

Federal Information Processing Standards (FIPS) is a United States
government computer security standard that specifies security
requirements for cryptography. The current version of the standard is
FIPS 140-2. Chef Client can be configured to allow OpenSSL to enforce
FIPS-validated security during a chef-client run. This will disable
cryptography that is explicitly disallowed in FIPS-validated software,
including certain ciphers and hashing algorithms. Any attempt to use any
disallowed cryptography will cause Chef Client to throw an exception
during a chef-client run.

{{< note >}}

Chef uses MD5 hashes to uniquely identify files that are stored on the
Chef server. MD5 is used only to generate a unique hash identifier and
is not used for any cryptographic purpose.

{{< /note >}}

Notes about FIPS:

-   May be enabled for nodes running on Microsoft Windows and Enterprise
    Linux platforms
-   Should only be enabled for environments that require FIPS 140-2
    compliance
-   May not be enabled for any version of Chef Client earlier than 12.8

#### Enable FIPS Mode

Allowing OpenSSL to enforce FIPS-validated security may be enabled by
using any of the following ways:

-   Set the `fips` configuration setting to `true` in the client.rb or
    knife.rb files
-   Set the `--fips` command-line option when running any knife command
    or Chef Client executable
-   Set the `--fips` command-line option when bootstrapping a node using
    the `knife bootstrap` command

#### Command Option

The following command-line option may be used to with a knife or
chef-client executable command:

`--[no-]fips`

:   Allows OpenSSL to enforce FIPS-validated security during Chef Client
    run.

**Bootstrap a node using FIPS**

``` bash
knife bootstrap 192.0.2.0 -P vanilla -x root -r 'recipe[apt],recipe[xfs],recipe[vim]' --fips
```

which shows something similar to:

``` none
OpenSSL FIPS 140 mode enabled
...
192.0.2.0 Chef Client finished, 12/12 resources updated in 78.942455583 seconds
```

#### Configuration Setting

The following configuration setting may be set in the knife.rb,
client.rb, or config.rb files:

`fips`

:   Allows OpenSSL to enforce FIPS-validated security during Chef Client
    run. Set to `true` to enable FIPS-validated security.

### .d Directories

Chef Client supports reading multiple configuration files by putting
them inside a `.d` configuration directory. For example:
`/etc/chef/client.d`. All files that end in `.rb` in the `.d` directory
are loaded; other non-`.rb` files are ignored.

`.d` directories may exist in any location where the `client.rb`,
`config.rb`, or `solo.rb` files are present, such as:

-   `/etc/chef/client.d`
-   `/etc/chef/config.d`
-   `~/chef/solo.d`

(There is no support for a `knife.d` directory; use `config.d` instead.)

For example, when using knife, the following configuration files would
be loaded:

-   `~/.chef/config.rb`
-   `~/.chef/config.d/company_settings.rb`
-   `~/.chef/config.d/ec2_configuration.rb`
-   `~/.chef/config.d/old_settings.rb.bak`

The `old_settings.rb.bak` file is ignored because it's not a
configuration file. The `config.rb`, `company_settings.rb`, and
`ec2_configuration` files are merged together as if they are a single
configuration file.

{{< note >}}

If multiple configuration files exists in a `.d` directory, ensure that
the same setting has the same value in all files.

{{< /note >}}

### launchd

Use the **launchd** resource to manage system-wide services (daemons)
and per-user services (agents) on the macOS platform.

#### Syntax

A **launchd** resource manages system-wide services (daemons) and
per-user services (agents) on the macOS platform:

``` ruby
launchd 'call.mom.weekly' do
  program '/Library/scripts/call_mom.sh'
  start_calendar_interval 'Weekday' => 7, 'Hourly' => 10
  time_out 300
end
```

The full syntax for all of the properties that are available to the
**launchd** resource is:

``` ruby
launchd 'name' do
  abandon_process_group      true, false
  backup                     Integer, false
  cookbook                   String
  debug                      true, false
  disabled                   true, false
  enable_globbing            true, false
  enable_transactions        true, false
  environment_variables      Hash
  exit_timeout               Integer
  group                      String, Integer
  hard_resource_limits       Hash
  hash                       Hash
  ignore_failure             true, false
  inetd_compatibility        Hash
  init_groups                true, false
  keep_alive                 true, false
  label                      String
  launch_only_once           true, false
  limit_load_from_hosts      Array
  limit_load_to_hosts        Array
  limit_load_to_session_type String
  low_priority_io            true, false
  mach_services              Hash
  mode                       Integer, String
  nice                       Integer
  notifies                   # see description
  on_demand                  true, false
  owner                      Integer, String
  path                       String
  process_type               String
  program                    String
  program_arguments          Array
  provider                   Chef::Provider::Launchd
  queue_directories          Array
  retries                    Integer
  retry_delay                Integer
  root_directory             String
  run_at_load                true, false
  sockets                    Hash
  soft_resource_limits       Array
  standard_error_path        String
  standard_in_path           String
  standard_out_path          String
  start_calendar_interval    Hash
  start_interval             Integer
  start_on_mount             true, false
  subscribes                 # see description
  throttle_interval          Integer
  time_out                   Integer
  type                       String
  umask                      Integer
  username                   String
  wait_for_debugger          true, false
  watch_paths                Array
  working_directory          String
  action                     Symbol # defaults to :create if not specified
end
```

where

-   `launchd` is the resource
-   `name` is the name of the resource block
-   `action` identifies the steps Chef Client will take to bring the
    node into the desired state
-   `abandon_process_group`, `backup`, `cookbook`, `debug`, `disabled`,
    `enable_globbing`, `enable_transactions`, `environment_variables`,
    `exit_timeout`, `group`, `hard_resource_limits`, `hash`,
    `inetd_compatibility`, `init_groups`, `keep_alive`, `label`,
    `launch_only_once`, `limit_load_from_hosts`, `limit_load_to_hosts`,
    `limit_load_to_session_type`, `low_priority_io`, `mach_services`,
    `mode`, `nice`, `on_demand`, `owner`, `path`, `process_type`,
    `program`, `program_arguments`, `queue_directories`, `retries`,
    `retry_delay`, `root_directory`, `run_at_load`, `sockets`,
    `soft_resource_limits`, `standard_error_path`, `standard_in_path`,
    `standard_out_path`, `start_calendar_interval`, `start_interval`,
    `start_on_mount`, `throttle_interval`, `time_out`, `type`, `umask`,
    `username`, `wait_for_debugger`, `watch_paths`, and
    `working_directory` are properties of this resource, with the Ruby
    type shown. See "Properties" section below for more information
    about all of the properties that may be used with this resource.

#### Actions

The launchd resource has the following actions:

`:create`

:   Default. Create a launchd property list.

`:create_if_missing`

:   Create a launchd property list, if it does not already exist.

`:delete`

:   Delete a launchd property list. This will unload a daemon or agent,
    if loaded.

`:disable`

:   Disable a launchd property list.

`:enable`

:   Create a launchd property list, and then ensure that it is enabled.
    If a launchd property list already exists, but does not match,
    updates the property list to match, and then restarts the daemon or
    agent.

`:restart`

:   Restart a launchd managed daemon or agent.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

#### Properties

This resource has the following properties:

`backup`

:   **Ruby Type:** Integer, false

    The number of backups to be kept in `/var/chef/backup`. Set to
    `false` to prevent backups from being kept.

`cookbook`

:   **Ruby Type:** String

    The name of the cookbook in which the source files are located.

`group`

:   **Ruby Type:** String, Integer

    When launchd is run as the root user, the group to run the job as.
    If the `username` property is specified and this property is not,
    this value is set to the default group for the user.

`hash`

:   **Ruby Type:** Hash

    A Hash of key value pairs used to create the launchd property list.

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`label`

:   **Ruby Type:** String

    The unique identifier for the job.

`mode`

:   **Ruby Type:** Integer, String

    A quoted 3-5 character string that defines the octal mode. For
    example: `'755'`, `'0755'`, or `00755`. If `mode` is not specified
    and if the directory already exists, the existing mode on the
    directory is used. If `mode` is not specified, the directory does
    not exist, and the `:create` action is specified, Chef Client
    assumes a mask value of `'0777'`, and then applies the umask for the
    system on which the directory is to be created to the `mask` value.
    For example, if the umask on a system is `'022'`, Chef Client uses
    the default value of `'0755'`.

    The behavior is different depending on the platform.

    UNIX- and Linux-based systems: A quoted 3-5 character string that
    defines the octal mode that is passed to chmod. For example:
    `'755'`, `'0755'`, or `00755`. If the value is specified as a quoted
    string, it works exactly as if the `chmod` command was passed. If
    the value is specified as an integer, prepend a zero (`0`) to the
    value to ensure that it is interpreted as an octal number. For
    example, to assign read, write, and execute rights for all users,
    use `'0777'` or `'777'`; for the same rights, plus the sticky bit,
    use `01777` or `'1777'`.

    Microsoft Windows: A quoted 3-5 character string that defines the
    octal mode that is translated into rights for Microsoft Windows
    security. For example: `'755'`, `'0755'`, or `00755`. Values up to
    `'0777'` are allowed (no sticky bits) and mean the same in Microsoft
    Windows as they do in UNIX, where `4` equals `GENERIC_READ`, `2`
    equals `GENERIC_WRITE`, and `1` equals `GENERIC_EXECUTE`. This
    property cannot be used to set `:full_control`. This property has no
    effect if not specified, but when it and `rights` are both
    specified, the effects are cumulative.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during the Chef Client run at which a
    notification is run. The following timers are available:

    `:before`

    :   Specifies that the action on a notified resource should be run
        before processing the resource block in which the notification
        is located.

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the end of the Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`owner`

:   **Ruby Type:** Integer, String

    A string or ID that identifies the group owner by user name,
    including fully qualified user names such as `domain\user` or
    `user@domain`. If this value is not specified, existing owners
    remain unchanged and new owner assignments use the current user
    (when necessary).

`path`

:   **Ruby Type:** String

    The path to the directory. Using a fully qualified path is
    recommended, but is not always required. Default value: the `name`
    of the resource block. See "Syntax" section above for more
    information.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`session_type`

:   **Ruby Type:** String

    The type of launchd plist to be created. Possible values: `system`
    (default) or `user`.

`source`

:   **Ruby Type:** String

    The path to the launchd property list.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during the Chef Client run at which a
    notification is run. The following timers are available:

    `:before`

    :   Specifies that the action on a notified resource should be run
        before processing the resource block in which the notification
        is located.

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the end of the Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`supports`

:   **Ruby Type:** Array

    An array of options for supported mount features. Default value:
    `{ :remount => false }`.

`type`

:   **Ruby Type:** String

    The type of resource. Possible values: `daemon` (default), `agent`.

The following resource properties may be used to define keys in the XML
property list for a daemon or agent. Please refer to the Apple man page
documentation for launchd for more information about these keys:

`abandon_process_group`

:   **Ruby Type:** true, false

    If a job dies, all remaining processes with the same process ID may
    be kept running. Set to `true` to kill all remaining processes.

`debug`

:   **Ruby Type:** true, false

    Sets the log mask to `LOG_DEBUG` for this job.

`disabled`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Hints to `launchctl` to not submit this job to launchd.

`enable_globbing`

:   **Ruby Type:** true, false

    Update program arguments before invocation.

`enable_transactions`

:   **Ruby Type:** true, false

    Track in-progress transactions; if none, then send the `SIGKILL`
    signal.

`environment_variables`

:   **Ruby Type:** Hash

    Additional environment variables to set before running a job.

`exit_timeout`

:   **Ruby Type:** Integer | **Default Value:** `20`

    The amount of time (in seconds) launchd waits before sending a
    `SIGKILL` signal.

`hard_resource_limits`

:   **Ruby Type:** Hash

    A Hash of resource limits to be imposed on a job.

`inetd_compatibility`

:   **Ruby Type:** Hash

    Specifies if a daemon expects to be run as if it were launched from
    `inetd`. Set to `wait => true` to pass standard input, output, and
    error file descriptors. Set to `wait => false` to call the `accept`
    system call on behalf of the job, and then pass standard input,
    output, and error file descriptors.

`init_groups`

:   **Ruby Type:** true, false

    Specify if `initgroups` is called before running a job. Default
    value: `true` (starting with macOS 10.5).

`keep_alive`

:   **Ruby Type:** true, false, Hash | **Default Value:** `false`

    Keep a job running continuously (`true`) or allow demand and
    conditions on the node to determine if the job keeps running
    (`false`).

    Hash type was added in Chef client 12.14.

`launch_only_once`

:   **Ruby Type:** true, false

    Specify if a job can be run only one time. Set this value to `true`
    if a job cannot be restarted without a full machine reboot.

`limit_load_from_hosts`

:   **Ruby Type:** Array

    An array of hosts to which this configuration file does not apply,
    i.e. "apply this configuration file to all hosts not specified in
    this array".

`limit_load_to_hosts`

:   **Ruby Type:** Array

    An array of hosts to which this configuration file applies.

`limit_load_to_session_type`

:   **Ruby Type:** String

    The session type to which this configuration file applies.

`low_priority_io`

:   **Ruby Type:** true, false

    Specify if the kernel on the node should consider this daemon to be
    low priority during file system I/O.

`mach_services`

:   **Ruby Type:** Hash

    Specify services to be registered with the bootstrap subsystem.

`nice`

:   **Ruby Type:** Integer

    The program scheduling priority value in the range `-20` to `20`.

`on_demand`

:   **Ruby Type:** true, false

    Keep a job alive. Only applies to macOS version 10.4 (and earlier);
    use `keep_alive` instead for newer versions.

`process_type`

:   **Ruby Type:** String

    The intended purpose of the job: `Adaptive`, `Background`,
    `Interactive`, or `Standard`.

`program`

:   **Ruby Type:** String

    The first argument of `execvp`, typically the file name associated
    with the file to be executed. This value must be specified if
    `program_arguments` is not specified, and vice-versa.

`program_arguments`

:   **Ruby Type:** Array

    The second argument of `execvp`. If `program` is not specified, this
    property must be specified and will be handled as if it were the
    first argument.

`queue_directories`

:   **Ruby Type:** Array

    An array of non-empty directories which, if any are modified, will
    cause a job to be started.

`root_directory`

:   **Ruby Type:** String

    `chroot` to this directory, and then run the job.

`run_at_load`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Launch a job once (at the time it is loaded).

`sockets`

:   **Ruby Type:** Hash

    A Hash of on-demand sockets that notify launchd when a job should be
    run.

`soft_resource_limits`

:   **Ruby Type:** Array

    A Hash of resource limits to be imposed on a job.

`standard_error_path`

:   **Ruby Type:** String

    The file to which standard error (`stderr`) is sent.

`standard_in_path`

:   **Ruby Type:** String

    The file to which standard input (`stdin`) is sent.

`standard_out_path`

:   **Ruby Type:** String

    The file to which standard output (`stdout`) is sent.

`start_calendar_interval`

:   **Ruby Type:** Hash

    A Hash (similar to `crontab`) that defines the calendar frequency at
    which a job is started. For example:
    `{ Minute => "0", Hour => "20", Day => "*", Weekday => "1-5", Month => "*" }`
    will run a job at 8:00 PM every day, Monday through Friday, every
    month of the year.

`start_interval`

:   **Ruby Type:** Integer

    The frequency (in seconds) at which a job is started.

`start_on_mount`

:   **Ruby Type:** true, false

    Start a job every time a file system is mounted.

`throttle_interval`

:   **Ruby Type:** Integer | **Default Value:** `10`

    The frequency (in seconds) at which jobs are allowed to spawn.

`time_out`

:   **Ruby Type:** Integer

    The amount of time (in seconds) a job may be idle before it times
    out. If no value is specified, the default timeout value for launchd
    will be used.

`umask`

:   **Ruby Type:** Integer

    A decimal value to pass to `umask` before running a job.

`username`

:   **Ruby Type:** String

    When launchd is run as the root user, the user to run the job as.

`wait_for_debugger`

:   **Ruby Type:** true, false

    Specify if launchd has a job wait for a debugger to attach before
    executing code.

`watch_paths`

:   **Ruby Type:** Array

    An array of paths which, if any are modified, will cause a job to be
    started.

`working_directory`

:   **Ruby Type:** String

    `chdir` to this directory, and then run the job.

#### Examples

**Create a Launch Daemon from a cookbook file**

``` ruby
launchd 'com.chef.every15' do
  source 'com.chef.every15.plist'
end
```

**Create a Launch Daemon using keys**

``` ruby
launchd 'call.mom.weekly' do
  program '/Library/scripts/call_mom.sh'
  start_calendar_interval 'Weekday' => 7, 'Hourly' => 10
  time_out 300
end
```

**Remove a Launch Daemon**

``` ruby
launchd 'com.chef.every15' do
  action :delete
end
```

### gem, metadata.rb

Specifies a gem dependency for installation into Chef Client through
bundler. The gem installation occurs after all cookbooks are
synchronized but before loading any other cookbooks. Use this attribute
one time for each gem dependency. For example:

``` ruby
gem "poise"
gem "chef-sugar"
```

## What's New in 12.7

The following items are new for chef-client 12.7 and/or are changes from
previous versions. The short version:

-   **Chef::REST =\> require 'chef/rest'** Internal API calls are moved
    from `Chef::REST` to `Chef::ServerAPI`. Any code that uses
    `Chef::REST` must use `require 'chef/rest'`.
-   **New chocolatey_package resource** Use the **chocolatey_package**
    resource to manage packages using Chocolatey for the Microsoft
    Windows platform.
-   **New osx_profile resource** Use the **osx_profile** resource to
    manage configuration profiles (`.mobileconfig` files) on the macOS
    platform.
-   **New apt_update resource** Use the **apt_update** resource to
    manage Apt repository updates on Debian and Ubuntu platforms.
-   **Improved support for UTF-8** Chef Client 12.7 release fixes a
    UTF-8 handling bug present in chef-client versions 12.4, 12.5, and
    12.6.
-   **New options for the chef-client** Chef Client has a new option:
    `--delete-entire-chef-repo`.
-   **Multi-package support for Chocolatey and Zypper** A resource may
    specify multiple packages and/or versions for platforms that use
    Zypper or Chocolatey package managers (in addition to the existing
    support for specifying multiple packages for Yum and Apt packages).

### Chef::REST =\> require 'chef/rest'

Internal API calls are moved from `Chef::REST` to `Chef::ServerAPI`. As
a result of this move, `Chef::REST` is no longer globally required. Any
code that uses `Chef::REST` must be required as follows:

``` ruby
require 'chef/rest'
```

For code that is run using knife or chef command line interfaces,
consider using `Chef::ServerAPI` instead.

### chocolatey_package

Use the **chocolatey_package** resource to manage packages using
Chocolatey on the Microsoft Windows platform.

#### Syntax

A **chocolatey_package** resource block manages packages using
Chocolatey for the Microsoft Windows platform. The simplest use of the
**chocolatey_package** resource is:

``` ruby
chocolatey_package 'package_name'
```

which will install the named package using all of the default options
and the default action (`:install`).

The full syntax for all of the properties that are available to the
**chocolatey_package** resource is:

``` ruby
chocolatey_package 'name' do
  notifies                   # see description
  options                    String
  package_name               String, Array # defaults to 'name' if not specified
  source                     String
  subscribes                 # see description
  timeout                    String, Integer
  version                    String, Array
  action                     Symbol # defaults to :install if not specified
end
```

where

-   `chocolatey_package` tells Chef Client to manage a package
-   `'name'` is the name of the package
-   `action` identifies which steps Chef Client will take to bring the
    node into the desired state
-   `options`, `package_name`, `source`, `timeout`, and `version` are
    properties of this resource, with the Ruby type shown. See
    "Properties" section below for more information about all of the
    properties that may be used with this resource.

#### Actions

This resource has the following actions:

`:install`

:   Default. Install a package. If a version is specified, install the
    specified version of the package.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:purge`

:   Purge a package. This action typically removes the configuration
    files as well as the package.

`:reconfig`

:   Reconfigure a package. This action requires a response file.

`:remove`

:   Remove a package.

`:uninstall`

:   Uninstall a package.

`:upgrade`

:   Install a package and/or ensure that a package is the latest
    version.

#### Properties

This resource has the following properties:

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during the Chef Client run at which a
    notification is run. The following timers are available:

    `:before`

    :   Specifies that the action on a notified resource should be run
        before processing the resource block in which the notification
        is located.

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the end of the Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`options`

:   **Ruby Type:** String

    One (or more) additional options that are passed to the command.

`package_name`

:   **Ruby Type:** String, Array

    The name of the package. Default value: the `name` of the resource
    block. See "Syntax" section above for more information.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`source`

:   **Ruby Type:** String

    Optional. The path to a package in the local file system.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during the Chef Client run at which a
    notification is run. The following timers are available:

    `:before`

    :   Specifies that the action on a notified resource should be run
        before processing the resource block in which the notification
        is located.

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the end of the Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`timeout`

:   **Ruby Type:** String, Integer

    The amount of time (in seconds) to wait before timing out.

`version`

:   **Ruby Type:** String, Array

    The version of a package to be installed or upgraded.

#### Examples

**Install a package**

``` ruby
chocolatey_package 'name of package' do
  action :install
end
```

**Install a package with options**

This example uses Chocolatey's `--checksum` option:

``` ruby
chocolatey_package 'name of package' do
  options '--checksum 1234567890'
  action :install
end
```

### osx_profile

Use the **osx_profile** resource to manage configuration profiles
(`.mobileconfig` files) on the macOS platform. The **osx_profile**
resource installs profiles by using the `uuidgen` library to generate a
unique `ProfileUUID`, and then using the `profiles` command to install
the profile on the system.

#### Syntax

A **osx_profile** resource block manages configuration profiles on the
macOS platform:

``` ruby
osx_profile 'Install screensaver profile' do
  profile 'com.company.screensaver.mobileconfig'
end
```

The full syntax for all of the properties that are available to the
**osx_profile** resource is:

``` ruby
osx_profile 'name' do
  path                       # set automatically
  profile                    String, Hash
  profile_name               String # defaults to 'name' if not specified
  identifier                 String
  action                     Symbol # defaults to :install if not specified
end
```

where

-   `osx_profile` is the resource
-   `name` is the name of the resource block
-   `action` identifies the steps Chef Client will take to bring the
    node into the desired state
-   `profile`, `profile_name`, and `identifier` are properties of this
    resource, with the Ruby type shown. See "Properties" section below
    for more information about all of the properties that may be used
    with this resource.

#### Actions

The osx_profile resource has the following actions:

`:install`

:   Default. Install the specified configuration profile.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:remove`

:   Remove the specified configuration profile.

#### Properties

The osx_profile resource has the following properties:

`identifier`

:   **Ruby Type:** String

    Use to specify the identifier for the profile, such as
    `com.company.screensaver`.

`path`

:   **Ruby Type:** String

    The path to write the profile to disk before loading it.

`profile`

:   **Ruby Type:** String, Hash

    Use to specify a profile. This may be the name of a profile
    contained in a cookbook or a Hash that contains the contents of the
    profile.

`profile_name`

:   **Ruby Type:** String | **Default Value:**
    `The resource block's name`

    Use to specify the name of the profile, if different from the name
    of the resource block.

#### Examples

**One liner to install profile from cookbook file**

The `profiles` command will be used to install the specified
configuration profile.

``` ruby
osx_profile 'com.company.screensaver.mobileconfig'
```

**Install profile from cookbook file**

The `profiles` command will be used to install the specified
configuration profile. It can be in sub-directory within a cookbook.

``` ruby
osx_profile 'Install screensaver profile' do
  profile 'screensaver/com.company.screensaver.mobileconfig'
end
```

**Install profile from a hash**

The `profiles` command will be used to install the configuration
profile, which is provided as a hash.

``` ruby
profile_hash = {
  'PayloadIdentifier' => 'com.company.screensaver',
  'PayloadRemovalDisallowed' => false,
  'PayloadScope' => 'System',
  'PayloadType' => 'Configuration',
  'PayloadUUID' => '1781fbec-3325-565f-9022-8aa28135c3cc',
  'PayloadOrganization' => 'Chef',
  'PayloadVersion' => 1,
  'PayloadDisplayName' => 'Screensaver Settings',
  'PayloadContent'=> [
    {
      'PayloadType' => 'com.apple.ManagedClient.preferences',
      'PayloadVersion' => 1,
      'PayloadIdentifier' => 'com.company.screensaver',
      'PayloadUUID' => '73fc30e0-1e57-0131-c32d-000c2944c108',
      'PayloadEnabled' => true,
      'PayloadDisplayName' => 'com.apple.screensaver',
      'PayloadContent' => {
        'com.apple.screensaver' => {
          'Forced' => [
            {
              'mcx_preference_settings' => {
                'idleTime' => 0,
              }
            }
          ]
        }
      }
    }
  ]
}

osx_profile 'Install screensaver profile' do
  profile profile_hash
end
```

**Remove profile using identifier in resource name**

The `profiles` command will be used to remove the configuration profile
specified by the provided `identifier` property.

``` ruby
osx_profile 'com.company.screensaver' do
  action :remove
end
```

**Remove profile by identifier and user friendly resource name**

The `profiles` command will be used to remove the configuration profile
specified by the provided `identifier` property.

``` ruby
osx_profile 'Remove screensaver profile' do
  identifier 'com.company.screensaver'
  action :remove
end
```

### apt_update

Use the **apt_update** resource to manage APT repository updates on
Debian and Ubuntu platforms.

#### Syntax

An **apt_update** resource block defines the update frequency for APT
repositories:

``` ruby
apt_update 'name' do
  frequency                  Integer
  action                     Symbol # defaults to :periodic if not specified
end
```

where

-   `apt_update` is the resource
-   `name` is the name of the resource block
-   `action` identifies the steps Chef Client will take to bring the
    node into the desired state
-   `frequency` is a property of this resource, with the Ruby type
    shown. See "Properties" section below for more information about all
    of the properties that may be used with this resource.

#### Actions

The apt_update resource has the following actions:

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:periodic`

:   Update the Apt repository at the interval specified by the
    `frequency` property.

`:update`

:   Update the Apt repository at the start of Chef Client run.

#### Properties

The apt_update resource has the following properties:

`frequency`

:   **Ruby Type:** Integer | **Default Value:** `86400`

    Determines how frequently (in seconds) APT repository updates are
    made. Use this property when the `:periodic` action is specified.

#### Examples

**Update the Apt repository at a specified interval**

``` ruby
apt_update 'all platforms' do
  frequency 86400
  action :periodic
end
```

**Update the Apt repository at the start of a chef-client run**

``` ruby
apt_update 'update'
```

### New chef-client options

Chef Client has the following new options:

`--delete-entire-chef-repo`

:   Delete the entire chef-repo. This option may only be used when
    running Chef Client in local mode (`--local-mode`) mode. This
    options requires `--recipe-url` to be specified.

## What's New in 12.6

The following items are new for chef-client 12.6 and/or are changes from
previous versions. The short version:

-   **New timer for resource notifications** Use the `:before` timer
    with the `notifies` and `subscribes` properties to specify that the
    action on a notified resource should be run before processing the
    resource block in which the notification is located.
-   **New ksh resource** The **ksh** resource is added and is based on
    the **script** resource.
-   **New metadata.rb settings** The metadata.rb file has settings for
    `chef_version` and `ohai_version` that allow ranges to be specified
    that declare the supported versions of Chef Client and Ohai.
-   **dsc_resource supports reboots** The **dsc_resource** resource
    supports immediate and queued reboots. This uses the **reboot**
    resource and its `:reboot_now` or `:request_reboot` actions.
-   **New and changed knife bootstrap options** The `--identify-file`
    option for the `knife bootstrap` subcommand is renamed to
    `--ssh-identity-file`; the `--sudo-preserve-home` is new.
-   **New installer types for the windows_package resource** The
    **windows_package** resource now supports the following installer
    types: `:custom`, Inno Setup (`:inno`), InstallShield
    (`:installshield`), Microsoft Installer Package (MSI) (`:msi`),
    Nullsoft Scriptable Install System (NSIS) (`:nsis`), Wise (`:wise`).
    Prior versions of Chef supported only `:msi`.
-   **dsc_resource resource may be run in non-disabled refresh mode**
    The latest version of Windows Management Framework (WMF) 5 has
    relaxed the limitation that prevented Chef Client from running in
    non-disabled refresh mode. Requires Windows PowerShell 5.0.10586.0
    or higher.
-   **dsc_script and dsc_resource resources may be in the same
    run-list** The latest version of Windows Management Framework (WMF)
    5 has relaxed the limitation that prevented Chef Client from running
    in non-disabled refresh mode, which allows the Local Configuration
    Manager to be set to `Push`. Requires Windows PowerShell 5.0.10586.0
    or higher.
-   **New --profile-ruby option** Use the `--profile-ruby` option to
    dump a (large) profiling graph into
    `/var/chef/cache/graph_profile.out`.
-   **New live_stream property for the execute resource** Set the
    `live_stream` property to `true` to send the output of a command run
    by the **execute** resource to Chef Client event stream.

### Notification Timers

A timer specifies the point during the Chef Client run at which a
notification is run. The following timers are available:

`:before`

:   Specifies that the action on a notified resource should be run
    before processing the resource block in which the notification is
    located.

`:delayed`

:   Default. Specifies that a notification should be queued up, and then
    executed at the end of the Chef Client run.

`:immediate`, `:immediately`

:   Specifies that a notification should be run immediately, per
    resource notified.

### ksh

Use the **ksh** resource to execute scripts using the Korn shell (ksh)
interpreter. This resource may also use any of the actions and
properties that are available to the **execute** resource. Commands that
are executed with this resource are (by their nature) not idempotent, as
they are typically unique to the environment in which they are run. Use
`not_if` and `only_if` to guard this resource for idempotence.

{{< note >}}

The **ksh** script resource (which is based on the **script** resource)
is different from the **ruby_block** resource because Ruby code that is
run with this resource is created as a temporary file and executed like
other script resources, rather than run inline.

{{< /note >}}

#### Syntax

A **ksh** resource block executes scripts using ksh:

``` ruby
ksh 'hello world' do
  code <<-EOH
    echo "Hello world!"
    echo "Current directory: " $cwd
    EOH
end
```

where

-   `code` specifies the command to run

The full syntax for all of the properties that are available to the
**ksh** resource is:

``` ruby
ksh 'name' do
  code                       String
  creates                    String
  cwd                        String
  environment                Hash
  flags                      String
  group                      String, Integer
  notifies                   # see description
  path                       Array
  returns                    Integer, Array
  subscribes                 # see description
  timeout                    Integer, Float
  user                       String, Integer
  umask                      String, Integer
  action                     Symbol # defaults to :run if not specified
end
```

where

-   `ksh` is the resource
-   `name` is the name of the resource block
-   `action` identifies the steps Chef Client will take to bring the
    node into the desired state
-   `code`, `creates`, `cwd`, `environment`, `flags`, `group`, `path`,
    `returns`, `timeout`, `user`, and `umask` are properties of this
    resource, with the Ruby type shown. See "Properties" section below
    for more information about all of the properties that may be used
    with this resource.

#### Actions

The ksh resource has the following actions:

`:nothing`

:   Prevent a command from running. This action is used to specify that
    a command is run only when another resource notifies it.

`:run`

:   Default. Run a script.

#### Properties

The ksh resource has the following properties:

`code`

:   **Ruby Type:** String

    A quoted (" ") string of code to be executed.

`creates`

:   **Ruby Type:** String

    Prevent a command from creating a file when that file already
    exists.

`cwd`

:   **Ruby Type:** String

    The current working directory from which the command will be run.

`environment`

:   **Ruby Type:** Hash

    A Hash of environment variables in the form of
    `({"ENV_VARIABLE" => "VALUE"})`. (These variables must exist for a
    command to be run successfully.)

`flags`

:   **Ruby Type:** String

    One or more command line flags that are passed to the interpreter
    when a command is invoked.

`group`

:   **Ruby Type:** String, Integer

    The group name or group ID that must be changed before running a
    command.

`path`

:   {{< warning >}}

    The `path` property has been deprecated and will throw an exception
    in Chef Client 12 or later. We recommend you use the `environment`
    property instead.

    {{< /warning >}}

    **Ruby Type:** Array

    An array of paths to use when searching for a command. These paths
    are not added to the command's environment \$PATH. The default value
    uses the system path.

    For example:

    ``` ruby
    ksh 'mycommand' do
      environment 'PATH' => "/my/path/to/bin:#{ENV['PATH']}"
    end
    ```

`returns`

:   **Ruby Type:** Integer, Array | **Default Value:** `0`

    The return value for a command. This may be an array of accepted
    values. An exception is raised when the return value(s) do not
    match.

`timeout`

:   **Ruby Type:** Integer, Float | **Default Value:** `3600`

    The amount of time (in seconds) a command is to wait before timing
    out.

`user`

:   **Ruby Type:** String, Integer

    The user name or user ID that should be changed before running a
    command.

`umask`

:   **Ruby Type:** String, Integer

    The file mode creation mask, or umask.

### Changes for PowerShell 5.0.10586.0

Using the **dsc_resource** has the following requirements:

-   Windows Management Framework (WMF) 5.0 February Preview (or higher),
    which includes Windows PowerShell 5.0.10018.0 (or higher).

-   The `RefreshMode` configuration setting in the Local Configuration
    Manager must be set to `Disabled`.

    {{< note spaces=4 >}}

    Starting with Chef Client 12.6 release, this requirement applies
    only for versions of Windows PowerShell earlier than 5.0.10586.0.
    The latest version of Windows Management Framework (WMF) 5 has
    relaxed the limitation that prevented Chef Client from running in
    non-disabled refresh mode.

    {{< /note >}}

-   The **dsc_script** resource may not be used in the same run-list
    with the **dsc_resource**. This is because the **dsc_script**
    resource requires that `RefreshMode` in the Local Configuration
    Manager be set to `Push`, whereas the **dsc_resource** resource
    requires it to be set to `Disabled`.

    {{< note spaces=4 >}}

    Starting with Chef Client 12.6 release, this requirement applies
    only for versions of Windows PowerShell earlier than 5.0.10586.0.
    The latest version of Windows Management Framework (WMF) 5 has
    relaxed the limitation that prevented Chef Client from running in
    non-disabled refresh mode, which allows the Local Configuration
    Manager to be set to `Push`.

    {{< /note >}}

-   The **dsc_resource** resource can only use binary- or script-based
    resources. Composite DSC resources may not be used.

    This is because composite resources aren't "real" resources from the
    perspective of the Local Configuration Manager (LCM). Composite
    resources are used by the "configuration" keyword from the
    `PSDesiredStateConfiguration` module, and then evaluated in that
    context. When using DSC to create the configuration document (the
    Managed Object Framework (MOF) file) from the configuration command,
    the composite resource is evaluated. Any individual resources from
    that composite resource are written into the Managed Object
    Framework (MOF) document. As far as the Local Configuration Manager
    (LCM) is concerned, there is no such thing as a composite resource.
    Unless that changes, the **dsc_resource** resource and/or
    `Invoke-DscResource` command cannot directly use them.

### New metadata.rb Settings

The following settings are new for metadata.rb:

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
<td><p><code>chef_version</code></p></td>
<td><p>A range of chef-client versions that are supported by this cookbook.</p>
<p>For example, to match any 12.x version of the chef-client, but not 11.x or 13.x:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>chef_version <span class="st">&#39;~&gt; 12&#39;</span></span></code></pre></div>
<p>A more complex example where you set both a lower and upper bound of Chef Client version:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>chef_version <span class="st">&quot;&gt;= 14.2.1&quot;</span>, <span class="st">&quot;&lt; 14.5.1&quot;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>ohai_version</code></p></td>
<td><p>A range of chef-client versions that are supported by this cookbook.</p>
<p>For example, to match any 8.x version of Ohai, but not 7.x or 9.x:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>ohai_version <span class="st">&quot;~&gt; 8&quot;</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

{{< note >}}

These settings are not visible in Chef Supermarket.

{{< /note >}}

### knife bootstrap Options

The following option is new for `knife bootstrap`:

`--sudo-preserve-home`

:   Use to preserve the non-root user's `HOME` environment.

The `--identify-file` option is now `--ssh-identify-file`.

### --profile-ruby Option

Use the `--profile-ruby` option to dump a (large) profiling graph into
`/var/chef/cache/graph_profile.out`. Use the graph output to help
identify, and then resolve performance bottlenecks in a chef-client run.
This option:

-   Generates a large amount of data about the chef-client run.
-   Has a dependency on the `ruby-prof` gem, which is packaged as part
    of Chef and the ChefDK.
-   Increases the amount of time required to complete the chef-client
    run.
-   Should not be used in a production environment.

## What's New in 12.5

The following items are new for chef-client 12.5 and/or are changes from
previous versions. The short version:

-   **New way to build custom resources** The process for extending the
    collection of resources that are built into Chef has been
    simplified. It is defined only in the `/resources` directory using a
    simplified syntax that easily leverages the built-in collection of
    resources. (All of the ways you used to build custom resources still
    work.)
-   **"resource attributes" are now known as "resource properties"** In
    previous releases of Chef, resource properties are referred to as
    attributes, but this is confusing for users because nodes also have
    attributes. Starting with chef-client 12.5 release---and
    retroactively updated for all previous releases of the
    documentation---"resource attributes" are now referred to as
    "resource properties" and the word "attribute" now refers
    specifically to "node attributes".
-   **ps_credential helper to embed usernames and passwords** Use the
    `ps_credential` helper on Microsoft Windows to create a
    `PSCredential` object---security credentials, such as a user name or
    password---that can be used in the **dsc_script** resource.
-   **New Handler DSL** A new DSL exists to make it easier to use events
    that occur during Chef Client run from recipes. The `on` method is
    easily associated with events. The action Chef Client takes as a
    result of that event (when it occurs) is up to you.
-   **The -j / --json-attributes supports policy revisions and
    environments** The JSON file used by the `--json-attributes` option
    for Chef Client may now contain the policy name and policy group
    associated with a policy revision or may contain the name of the
    environment to which the node is associated.
-   **verify property now uses path, not file** The `verify` property,
    used by file-based resources such as **remote_file** and **file**,
    runs user-defined correctness checks against the proposed new file
    before making the change. For versions of Chef Client prior to 12.5,
    the name of the temporary file was stored as `file`; starting with
    chef-client 12.5, use `path`. This change is documented as a warning
    across all versions in any topic in which the `version` attribute is
    documented.
-   **depth property added to deploy resource** The `depth` property
    allows the depth of a git repository to be truncated to the
    specified number of versions.
-   **The knife ssl check subcommand supports SNI** Support for Server
    Name Indication (SNI) is added to the `knife ssl check` subcommand.
-   **Chef Policy group and name can now be part of the node object**
    Chef policy is a beta feature of Chef Client that will eventually
    replace roles, environments or manually specifying the run_list.
    Policy group and name can now be stored as part of the node object
    rather than in the client.rb file. A recent version of the Chef
    server, such as 12.2.0 or higher, is needed to fully utilize this
    feature.

### Custom Resources

A custom resource:

-   Is a simple extension of Chef that adds your own resources
-   Is implemented and shipped as part of a cookbook
-   Follows easy, repeatable syntax patterns
-   Effectively leverages resources that are built into Chef and/or
    custom Ruby code
-   Is reusable in the same way as resources that are built into Chef

For example, Chef includes built-in resources to manage files, packages,
templates, and services, but it does not include a resource that manages
websites.

{{< note >}}

See /custom_resources.html for more information about custom resources,
including a scenario that shows how to build a `website` resource.

{{< /note >}}

#### Syntax

A custom resource is defined as a Ruby file and is located in a
cookbook's `/resources` directory. This file

-   Declares the properties of the custom resource
-   Loads current state of properties, if the resource already exists
-   Defines each action the custom resource may take

The syntax for a custom resource is. For example:

``` ruby
property :property_name, RubyType, default: 'value'

load_current_value do
  # some Ruby for loading the current state of the resource
end

action :action_name do
 # a mix of built-in Chef resources and Ruby
end

action :another_action_name do
 # a mix of built-in Chef resources and Ruby
end
```

where the first action listed is the default action.

{{< warning >}}

Do not use existing keywords from Chef Client resource system in a
custom resource, like "name". For example, `property :property_name` in
the following invalid syntax:
`property :name, String, default: 'thename'`.

{{< /warning >}}

This example `site` utilizes Chef's built in `file`, `service` and
`package` resources, and includes `:create` and `:delete` actions. Since
it uses built in Chef resources, besides defining the property and
actions, the code is very similar to that of a recipe.

``` ruby
property :homepage, String, default: '<h1>Hello world!</h1>'

action :create do
  package 'httpd'

  service 'httpd' do
    action [:enable, :start]
  end

  file '/var/www/html/index.html' do
    content homepage
  end
end

action :delete do
  package 'httpd' do
    action :delete
  end
end
```

where

-   `homepage` is a property that sets the default HTML for the
    `index.html` file with a default value of `'<h1>Hello world!</h1>'`
-   the `action` block uses the built-in collection of resources to tell
    Chef Client how to install Apache, start the service, and then
    create the contents of the file located at
    `/var/www/html/index.html`
-   `action :create` is the default resource, because it is listed
    first; `action :delete` must be called specifically (because it is
    not the default resource)

Once built, the custom resource may be used in a recipe just like any of
the resources that are built into Chef. The resource gets its name from
the cookbook and from the file name in the `/resources` directory, with
an underscore (`_`) separating them. For example, a cookbook named
`exampleco` with a custom resource named `site.rb` is used in a recipe
like this:

``` ruby
exampleco_site 'httpd' do
  homepage '<h1>Welcome to the Example Co. website!</h1>'
end
```

and to delete the exampleco website, do the following:

``` ruby
exampleco_site 'httpd' do
  action :delete
end
```

### Custom Resource DSL

Use the Custom Resource DSL to define property behaviors within custom
resources, such as:

-   Loading the value of a specific property
-   Comparing the current property value against a desired property
    value
-   Telling Chef Client when and how to make changes

#### action_class

Use the `action_class` block to make methods available to the actions in
the custom resource. Modules with helper methods created as files in the
cookbook library directory may be included. New action methods may also
be defined directly in the `action_class` block. Code in the
`action_class` block has access to the new_resource properties.

Assume a helper module has been created in the cookbook
`libraries/helper.rb` file.

``` ruby
module Sample
  module Helper
    def helper_method
      # code
    end
  end
end
```

Methods may be made available to the custom resource actions by using an
`action_class` block.

``` ruby
property file, String

action :delete do
  helper_method
  FileUtils.rm(new_resource.file) if file_ex
end

action_class do

  def file_exist
    ::File.exist?(new_resource.file)
  end

  def file_ex
    ::File.exist?(new_resource.file)
  end

  require 'fileutils'

  include Sample::Helper

end
```

#### converge_if_changed

Use the `converge_if_changed` method inside an `action` block in a
custom resource to compare the desired property values against the
current property values (as loaded by the `load_current_value` method).
Use the `converge_if_changed` method to ensure that updates only occur
when property values on the system are not the desired property values
and to otherwise prevent a resource from being converged.

To use the `converge_if_changed` method, wrap it around the part of a
recipe or custom resource that should only be converged when the current
state is not the desired state:

``` ruby
action :some_action do

  converge_if_changed do
    # some property
  end

end
```

For example, a custom resource defines two properties (`content` and
`path`) and a single action (`:create`). Use the `load_current_value`
method to load the property value to be compared, and then use the
`converge_if_changed` method to tell Chef Client what to do if that
value is not the desired value:

``` ruby
property :content, String
property :path, String, name_property: true

load_current_value do
  if ::File.exist?(new_resource.path)
    content IO.read(new_resource.path)
  end
end

action :create do
  converge_if_changed do
    IO.write(new_resource.path, new_resource.content)
  end
end
```

When the file does not exist, the `IO.write(path, content)` code is
executed and Chef Client output will print something similar to:

``` bash
Recipe: recipe_name::block
  * resource_name[blah] action create
    - update my_file[blah]
    -   set content to "hola mundo" (was "hello world")
```

**Multiple Properties**

The `converge_if_changed` method may be used multiple times. The
following example shows how to use the `converge_if_changed` method to
compare the multiple desired property values against the current
property values (as loaded by the `load_current_value` method).

``` ruby
property :path, String, name_property: true
property :content, String
property :mode, String

load_current_value do
  if ::File.exist?(new_resource.path)
    content IO.read(new_resource.path)
    mode ::File.stat(new_resource.path).mode
  end
end

action :create do
  converge_if_changed :content do
    IO.write(new_resource.path, new_resource.content)
  end
  converge_if_changed :mode do
    ::File.chmod(new_resource.mode, new_resource.path)
  end
end
```

where

-   `load_current_value` loads the property values for both `content`
    and `mode`
-   A `converge_if_changed` block tests only `content`
-   A `converge_if_changed` block tests only `mode`

Chef Client will only update the property values that require updates
and will not make changes when the property values are already in the
desired state

#### default_action

The default action in a custom resource is, by default, the first action
listed in the custom resource. For example, action `aaaaa` is the
default resource:

``` ruby
property :property_name, RubyType, default: 'value'

...

action :aaaaa do
 # the first action listed in the custom resource
end

action :bbbbb do
 # the second action listed in the custom resource
end
```

The `default_action` method may also be used to specify the default
action. For example:

``` ruby
property :property_name, RubyType, default: 'value'

default_action :aaaaa

action :aaaaa do
 # the first action listed in the custom resource
end

action :bbbbb do
 # the second action listed in the custom resource
end
```

defines action `aaaaa` as the default action. If `default_action :bbbbb`
is specified, then action `bbbbb` is the default action. Use this method
for clarity in custom resources, if deliberately stating the default
resource is desired, or to specify a default action that is not listed
first in the custom resource.

#### load_current_value

Use the `load_current_value` method to load the specified property
values from the node, and then use those values when the resource is
converged. This method may take a block argument.

Use the `load_current_value` method to guard against property values
being replaced. For example:

``` ruby
load_current_value do
  if ::File.exist?('/var/www/html/index.html')
    homepage IO.read('/var/www/html/index.html')
  end

  if ::File.exist?('/var/www/html/404.html')
    page_not_found IO.read('/var/www/html/404.html')
  end
end
```

This ensures the values for `homepage` and `page_not_found` are not
changed to the default values when Chef Client configures the node.

#### new_resource.property

Custom resources are designed to use core resources that are built into
Chef. In some cases, it may be necessary to specify a property in the
custom resource that is the same as a property in a core resource, for
the purpose of overriding that property when used with the custom
resource. For example:

``` ruby
resource_name :node_execute

property :command, String, name_property: true
property :version, String

# Useful properties from the `execute` resource
property :cwd, String
property :environment, Hash, default: {}
property :user, [String, Integer]
property :sensitive, [true, false], default: false

prefix = '/opt/languages/node'

load_current_value do
  current_value_does_not_exist! if node.run_state['nodejs'].nil?
  version node.run_state['nodejs'][:version]
end

action :run do
  execute 'execute-node' do
    cwd cwd
    environment environment
    user user
    sensitive sensitive
    # gsub replaces 10+ spaces at the beginning of the line with nothing
    command <<-CODE.gsub(/^ {10}/, '')
      #{prefix}/#{new_resource.version}/#{command}
    CODE
  end
end
```

where the `property :cwd`, `property :environment`, `property :user`,
and `property :sensitive` are identical to properties in the **execute**
resource, embedded as part of the `action :run` action. Because both the
custom properties and the **execute** properties are identical, this
will result in an error message similar to:

``` ruby
## ArgumentError
wrong number of arguments (0 for 1)
```

To prevent this behavior, use `new_resource.` to tell Chef Client to
process the properties from the core resource instead of the properties
in the custom resource. For example:

``` ruby
resource_name :node_execute

property :command, String, name_property: true
property :version, String

# Useful properties from the `execute` resource
property :cwd, String
property :environment, Hash, default: {}
property :user, [String, Integer]
property :sensitive, [true, false], default: false

prefix = '/opt/languages/node'

load_current_value do
  current_value_does_not_exist! if node.run_state['nodejs'].nil?
  version node.run_state['nodejs'][:version]
end

action :run do
  execute 'execute-node' do
    cwd new_resource.cwd
    environment new_resource.environment
    user new_resource.user
    sensitive new_resource.sensitive
    # gsub replaces 10+ spaces at the beginning of the line with nothing
    command <<-CODE.gsub(/^ {10}/, '')
      #{prefix}/#{new_resource.version}/#{new_resource.command}
    CODE
  end
end
```

where `cwd new_resource.cwd`, `environment new_resource.environment`,
`user new_resource.user`, and `sensitive new_resource.sensitive`
correctly use the properties of the **execute** resource and not the
identically-named override properties of the custom resource.

#### property

Use the `property` method to define properties for the custom resource.
The syntax is:

``` ruby
property :property_name, ruby_type, default: 'value', parameter: 'value'
```

where

-   `:property_name` is the name of the property
-   `ruby_type` is the optional Ruby type or array of types, such as
    `String`, `Integer`, `true`, or `false`
-   `default: 'value'` is the optional default value loaded into the
    resource
-   `parameter: 'value'` optional parameters

For example, the following properties define `username` and `password`
properties with no default values specified:

``` ruby
property :username, String
property :password, String
```

**ruby_type**

The property ruby_type is a positional parameter. Use to ensure a
property value is of a particular ruby class, such as `true`, `false`,
`nil`, `String`, `Array`, `Hash`, `Integer`, `Symbol`. Use an array of
ruby classes to allow a value to be of more than one type. For example:

``` ruby
property :aaaa, String
```

``` ruby
property :bbbb, Integer
```

``` ruby
property :cccc, Hash
```

``` ruby
property :dddd, [true, false]
```

``` ruby
property :eeee, [String, nil]
```

``` ruby
property :ffff, [Class, String, Symbol]
```

``` ruby
property :gggg, [Array, Hash]
```

**validators**

A validation parameter is used to add zero (or more) validation
parameters to a property.

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>:callbacks</code></p></td>
<td><p>Use to define a collection of unique keys and values (a ruby hash) for which the key is the error message and the value is a lambda to validate the parameter. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a><span class="st">callbacks: </span>{</span>
<span id="cb1-2"><a href="#cb1-2"></a>             <span class="st">&#39;should be a valid non-system port&#39;</span> =&gt; lambda {</span>
<span id="cb1-3"><a href="#cb1-3"></a>               |p| p &gt; <span class="dv">1024</span> &amp;&amp; p &lt; <span class="dv">65535</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>             }</span>
<span id="cb1-5"><a href="#cb1-5"></a>           }</span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>:default</code></p></td>
<td><p>Use to specify the default value for a property. For example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a><span class="st">default: &#39;a_string_value&#39;</span></span></code></pre></div>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a><span class="st">default: </span><span class="dv">123456789</span></span></code></pre></div>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a><span class="st">default: </span>[]</span></code></pre></div>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a><span class="st">default: </span>()</span></code></pre></div>
<div class="sourceCode" id="cb6"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb6-1"><a href="#cb6-1"></a><span class="st">default: </span>{}</span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>:equal_to</code></p></td>
<td><p>Use to match a value with <code>==</code>. Use an array of values to match any of those values with <code>==</code>. For example:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb7-1"><a href="#cb7-1"></a><span class="st">equal_to: </span>[<span class="dv">true</span>, <span class="dv">false</span>]</span></code></pre></div>
<div class="sourceCode" id="cb8"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb8-1"><a href="#cb8-1"></a><span class="st">equal_to: </span>[<span class="st">&#39;php&#39;</span>, <span class="st">&#39;perl&#39;</span>]</span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>:regex</code></p></td>
<td><p>Use to match a value to a regular expression. For example:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb9-1"><a href="#cb9-1"></a><span class="st">regex: </span>[ <span class="ot">/^([a-z]|[A-Z]|[0-9]|_|-)+$/</span>, <span class="ot">/^\d+$/</span> ]</span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>:required</code></p></td>
<td><p>Indicates that a property is required. For example:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb10-1"><a href="#cb10-1"></a><span class="st">required: </span><span class="dv">true</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>:respond_to</code></p></td>
<td><p>Use to ensure that a value has a given method. This can be a single method name or an array of method names. For example:</p>
<div class="sourceCode" id="cb11"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb11-1"><a href="#cb11-1"></a><span class="st">respond_to: </span>valid_encoding?</span></code></pre></div></td>
</tr>
</tbody>
</table>

Some examples of combining validation parameters:

``` ruby
property :spool_name, String, regex: /$\w+/
```

``` ruby
property :enabled, equal_to: [true, false, 'true', 'false'], default: true
```

**desired_state**

Add `desired_state:` to set the desired state property for a resource.
This value may be `true` or `false`, and all properties default to true.

-   When `true`, the state of the property is determined by the state of
    the system
-   When `false`, the value of the property impacts how the resource
    executes, but it is not determined by the state of the system.

For example, if you were to write a resource to create volumes on a
cloud provider you would need define properties such as `volume_name`,
`volume_size`, and `volume_region`. The state of these properties would
determine if your resource needed to converge or not. For the resource
to function you would also need to define properties such as
`cloud_login` and `cloud_password`. These are necessary properties for
interacting with the cloud provider, but their state has no impact on
decision to converge the resource or not, so you would set
`desired_state` to `false` for these properties.

``` ruby
property :volume_name, String
property :volume_size, Integer
property :volume_region, String
property :cloud_login, String, desired_state: false
property :cloud_password, String, desired_state: false
```

**identity**

Add `identity:` to set a resource to a particular set of properties.
This value may be `true` or `false`.

-   When `true`, data for that property is returned as part of the
    resource data set and may be available to external applications,
    such as reporting
-   When `false`, no data for that property is returned.

If no properties are marked `true`, the property that defaults to the
`name` of the resource is marked `true`.

For example, the following properties define `username` and `password`
properties with no default values specified, but with `identity` set to
`true` for the user name:

``` ruby
property :username, String, identity: true
property :password, String
```

**Block Arguments**

Any properties that are marked `identity: true` or
`desired_state: false` will be available from `load_current_value`. If
access to other properties of a resource is needed, use a block argument
that contains all of the properties of the requested resource. For
example:

``` ruby
resource_name :file

load_current_value do |desired|
  puts "The user typed content = #{desired.content} in the resource"
end
```

#### property_is_set?

Use the `property_is_set?` method to check if the value for a property
is set. The syntax is:

``` ruby
property_is_set?(:property_name)
```

The `property_is_set?` method will return `true` if the property is set.

For example, the following custom resource creates and/or updates user
properties, but not their password. The `property_is_set?` method checks
if the user has specified a password and then tells Chef Client what to
do if the password is not identical:

``` ruby
action :create do
  converge_if_changed do
    shell_out!("rabbitmqctl create_or_update_user #{username} --prop1 #{prop1} ... ")
  end

  if property_is_set?(:password)
    if shell_out("rabbitmqctl authenticate_user #{username} #{password}").error?
      converge_by "Updating password for user #{username} ..." do
        shell_out!("rabbitmqctl update_user #{username} --password #{password}")
      end
    end
  end
end
```

#### provides

Use the `provides` method to associate a custom resource with the Recipe
DSL on different operating systems. When multiple custom resources use
the same DSL, specificity rules are applied to determine the priority,
from highest to lowest:

1.  provides :resource_name, platform_version: '0.1.2'
2.  provides :resource_name, platform: 'platform_name'
3.  provides :resource_name, platform_family: 'platform_family'
4.  provides :resource_name, os: 'operating_system'
5.  provides :resource_name

For example:

``` ruby
provides :my_custom_resource, platform: 'redhat' do |node|
  node['platform_version'].to_i >= 7
end

provides :my_custom_resource, platform: 'redhat'

provides :my_custom_resource, platform_family: 'rhel'

provides :my_custom_resource, os: 'linux'

provides :my_custom_resource
```

This allows you to use multiple custom resources files that provide the
same resource to the user, but for different operating systems or
operation system versions. With this you can eliminate the need for
platform or platform version logic within your resources.

**override**

Chef will warn you if the Recipe DSL is provided by another custom
resource or built-in resource. For example:

``` ruby
class X < Chef::Resource
  provides :file
end

class Y < Chef::Resource
  provides :file
end
```

This will emit a warning that `Y` is overriding `X`. To disable this
warning, use `override: true`:

``` ruby
class X < Chef::Resource
  provides :file
end

class Y < Chef::Resource
  provides :file, override: true
end
```

#### reset_property

Use the `reset_property` method to clear the value for a property as if
it had never been set, and then use the default value. For example, to
clear the value for a property named `password`:

``` ruby
reset_property(:password)
```

### Definition vs. Resource

The following examples show:

1.  A definition
2.  The same definition rewritten as a custom resource
3.  The same definition, rewritten again to use a [common resource
    property](/resource_common/)

#### As a Definition

The following definition processes unique hostnames and ports, passed on
as parameters:

``` ruby
define :host_porter, :port => 4000, :hostname => nil do
  params[:hostname] ||= params[:name]

  directory '/etc/#{params[:hostname]}' do
    recursive true
  end

  file '/etc/#{params[:hostname]}/#{params[:port]}' do
    content 'some content'
  end
end
```

#### As a Resource

The definition is improved by rewriting it as a custom resource:

``` ruby
property :port, Integer, default: 4000
property :hostname, String, name_property: true

action :create do

  directory "/etc/#{hostname}" do
    recursive true
  end

  file "/etc/#{hostname}/#{port}" do
    content 'some content'
  end

end
```

Once built, the custom resource may be used in a recipe just like the
any of the resources that are built into Chef. The resource gets its
name from the cookbook and from the file name in the `/resources`
directory, with an underscore (`_`) separating them. For example, a
cookbook named `host` with a custom resource in the `/resources`
directory named `porter.rb`. Use it in a recipe like this:

``` ruby
host_porter node['hostname'] do
  port 4000
end
```

or:

``` ruby
host_porter 'www1' do
  port 4001
end
```

#### Common Properties

Unlike definitions, custom resources are able to use [common resource
properties](/resource_common/). For example, `only_if`:

``` ruby
host_porter 'www1' do
  port 4001
  only_if '{ node['hostname'] == 'foo.bar.com' }'
end
```

### ps_credential Helper

{{< readFile_shortcode file="ps_credential_helper.md" >}}

### Handler DSL

Use the Handler DSL to attach a callback to an event. If the event
occurs during Chef Infra Client run, the associated callback is
executed. For example:

-   Sending email if a chef-client run fails
-   Sending a notification to chat application if an audit run fails
-   Aggregating statistics about resources updated during a chef-client
    runs to StatsD

#### on Method

Use the `on` method to associate an event type with a callback. The
callback defines what steps are taken if the event occurs during Chef
Client run and is defined using arbitrary Ruby code. The syntax is as
follows:

``` ruby
Chef.event_handler do
  on :event_type do
    # some Ruby
  end
end
```

where

-   `Chef.event_handler` declares a block of code within a recipe that
    is processed when the named event occurs during a chef-client run
-   `on` defines the block of code that will tell Chef Client how to
    handle the event
-   `:event_type` is a valid exception event type, such as `:run_start`,
    `:run_failed`, `:converge_failed`, `:resource_failed`, or
    `:recipe_not_found`

For example:

``` bash
Chef.event_handler do
  on :converge_start do
    puts "Ohai! I have started a converge."
  end
end
```

#### Example: Send Email

Use the `on` method to create an event handler that sends email when
Chef Client run fails. This will require:

-   A way to tell Chef Client how to send email
-   An event handler that describes what to do when the `:run_failed`
    event is triggered
-   A way to trigger the exception and test the behavior of the event
    handler

{{< note >}}

See /dsl_handler.html for more information about using event handlers
in recipes.

{{< /note >}}

**Define How Email is Sent**

Use a library to define the code that sends email when a chef-client run
fails. Name the file `helper.rb` and add it to a cookbook's `/libraries`
directory:

``` ruby
require 'net/smtp'

module HandlerSendEmail
  class Helper

    def send_email_on_run_failure(node_name)

      message = "From: Chef <chef@chef.io>\n"
      message << "To: Grant <grantmc@chef.io>\n"
      message << "Subject: Chef run failed\n"
      message << "Date: #{Time.now.rfc2822}\n\n"
      message << "Chef run failed on #{node_name}\n"
      Net::SMTP.start('localhost', 25) do |smtp|
        smtp.send_message message, 'chef@chef.io', 'grantmc@chef.io'
      end
    end
  end
end
```

**Add the Handler**

Invoke the library helper in a recipe:

``` ruby
Chef.event_handler do
  on :run_failed do
    HandlerSendEmail::Helper.new.send_email_on_run_failure(
      Chef.run_context.node.name
    )
  end
end
```

-   Use `Chef.event_handler` to define the event handler
-   Use the `on` method to specify the event type

Within the `on` block, tell Chef Client how to handle the event when
it's triggered.

**Test the Handler**

Use the following code block to trigger the exception and have Chef
Client send email to the specified email address:

``` ruby
ruby_block 'fail the run' do
  block do
    fail 'deliberately fail the run'
  end
end
```

### New Resource Properties

The following property is new for the **deploy** resource:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>depth</code></p></td>
<td><p><strong>Ruby Type:</strong> Integer</p>
<p>The depth of a git repository, truncated to the specified number of revisions.</p></td>
</tr>
</tbody>
</table>

### Specify Policy Revision

Use the following command to specify a policy revision:

``` bash
chef client -j JSON
```

where the JSON file is similar to:

``` javascript
{
  "policy_name": "appserver",
  "policy_group": "staging"
}
```

Or use the following settings to specify a policy revision in the
client.rb file:

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
<td><code>policy_group</code></td>
<td>The name of a policy group that exists on the Chef server.</td>
</tr>
<tr class="even">
<td><code>policy_name</code></td>
<td>The name of a policy, as identified by the <code>name</code> setting in a Policyfile.rb file.</td>
</tr>
</tbody>
</table>

### New Configuration Settings

The following settings are new for the client.rb file and enable the use
of policy files:

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
<td><code>named_run_list</code></td>
<td>The run-list associated with a policy file.</td>
</tr>
<tr class="even">
<td><code>policy_group</code></td>
<td>The name of a policy group that exists on the Chef server. (See "Specify Policy Revision" in this readme for more information.)</td>
</tr>
<tr class="odd">
<td><code>policy_name</code></td>
<td>The name of a policy, as identified by the <code>name</code> setting in a Policyfile.rb file. (See "Specify Policy Revision" in this readme for more information.)</td>
</tr>
</tbody>
</table>

### chef-client Options

The following options are new or updated for Chef Client executable and
enable the use of policy files:

`-n NAME`, `--named-run-list NAME`

:   The run-list associated with a policy file.

`-j PATH`, `--json-attributes PATH`

:   This option now supports using a JSON file to associate a policy
    revision.

    Use this option to use policy files by specifying a JSON file that
    contains the following settings:

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
    <td><code>policy_group</code></td>
    <td>The name of a policy group that exists on the Chef server.</td>
    </tr>
    <tr class="even">
    <td><code>policy_name</code></td>
    <td>The name of a policy, as identified by the <code>name</code> setting in a Policyfile.rb file.</td>
    </tr>
    </tbody>
    </table>

    For example:

    ``` javascript
    {
      "policy_name": "appserver",
      "policy_group": "staging"
    }
    ```

    This option also supports using a JSON file to associate an
    environment:

    Use this option to set the `chef_environment` value for a node.

    {{< note spaces=4 >}}

    Any environment specified for `chef_environment` by a JSON file will
    take precedence over an environment specified by the `--environment`
    option when both options are part of the same command.

    {{< /note >}}

    For example, run the following:

    ``` bash
    chef-client -j /path/to/file.json
    ```

    where `/path/to/file.json` is similar to:

    ``` javascript
    {
      "chef_environment": "pre-production"
    }
    ```

    This will set the environment for the node to `pre-production`.

## What's New in 12.4

The following items are new for chef-client 12.4 and/or are changes from
previous versions. The short version:

-   **Validatorless bootstrap now requires the node name** Use of the
    `-N node_name` option with a validatorless bootstrap is now
    required.
-   **remote_file resource supports Windows UNC paths for source
    location** A Microsoft Windows UNC path may be used to specify the
    location of a remote file.
-   **Run PowerShell commands without excessive quoting** Use the
    `Import-Module chef` module to run Windows PowerShell commands
    without excessive quotation.
-   **Logging may use the Windows Event Logger** Log files may be sent
    to the Windows Event Logger. Set the `log_location` setting in the
    client.rb file to `Chef::Log::WinEvt.new`.
-   **Logging may be configured to use daemon facility available to the
    chef-client** Log files may be sent to the syslog available to the
    chef-client. Set the `log_location` setting in the client.rb file to
    `Chef::Log::Syslog.new("chef-client", ::Syslog::LOG_DAEMON)`.
-   **Package locations on the Windows platform may be specified using a
    URL** The location of a package may be at URL when using the
    **windows_package** resource.
-   **Package locations on the Windows platform may be specified by
    passing attributes to the remote_file resource** Use the
    `remote_file_attributes` attribute to pass a Hash of attributes that
    modifies the **remote_file** resource.
-   **Public key management for users and clients** The `knife client`
    and `knife user` subcommands may now create, delete, edit, list, and
    show public keys.
-   **knife client create and knife user create options have changed**
    With the new key management subcommands, the options for
    `knife client create` and `knife user create` have changed.
-   **chef-client audit-mode is no longer marked as "experimental"** The
    recommended version of audit-mode is chef-client 12.4, where it is
    no longer marked as experimental. Chef Client will report audit
    failures independently of converge failures.

### UNC paths, **remote_file**

When using the **remote_file** resource, the location of a source file
may be specified using a Microsoft Windows UNC. For example:

``` ruby
source "\\\\path\\to\\img\\sketch.png"
```

### Import-Module chef

Chef Client version 12.4 release adds an optional feature to the
Microsoft Installer Package (MSI) for Chef. This feature enables the
ability to pass quoted strings from the Windows PowerShell command line
without the need for triple single quotes (`''' '''`). This feature
installs a Windows PowerShell module (typically in
`C:\opscode\chef\modules`) that is also appended to the `PSModulePath`
environment variable. This feature is not enabled by default. To
activate this feature, run the following command from within Windows
PowerShell:

``` bash
Import-Module chef
```

or add `Import-Module chef` to the profile for Windows PowerShell
located at:

``` bash
~\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1
```

This module exports cmdlets that have the same name as the command-line
tools---chef-client, knife, chef-apply---that are built into Chef.

For example:

``` bash
knife exec -E 'puts ARGV' """&s0meth1ng"""
```

is now:

``` bash
knife exec -E 'puts ARGV' '&s0meth1ng'
```

and:

``` bash
knife node run_list set test-node '''role[ssssssomething]'''
```

is now:

``` bash
knife node run_list set test-node 'role[ssssssomething]'
```

To remove this feature, run the following command from within Windows
PowerShell:

``` bash
Remove-Module chef
```

### client.rb Settings

The following settings have changed:

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
<td><code>log_location</code></td>
<td>The location of the log file. Possible values: <code>/path/to/log_location</code>, <code>STDOUT</code>, <code>STDERR</code>, <code>Chef::Log::WinEvt.new</code> (Windows Event Logger), or <code>Chef::Log::Syslog.new("chef-client", ::Syslog::LOG_DAEMON)</code> (writes to the syslog daemon facility with the originator set as <code>chef-client</code>). The application log will specify the source as <code>Chef</code>. Default value: <code>STDOUT</code>.</td>
</tr>
</tbody>
</table>

### **windows_package** Updates

The **windows_package** resource has two new attributes (`checksum` and
`remote_file_attributes`) and the `source` attribute now supports using
a URL:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Attribute</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>checksum</code></td>
<td>The SHA-256 checksum of the file. Use to prevent a file from being re-downloaded. When the local file matches the checksum, Chef Client does not download it. Use when a URL is specified by the <code>source</code> attribute.</td>
</tr>
<tr class="even">
<td><code>remote_file_attributes</code></td>
<td>A package at a remote location define as a Hash of properties that modifies the properties of the <strong>remote_file</strong> resource.</td>
</tr>
<tr class="odd">
<td><code>source</code></td>
<td>Optional. The path to a package in the local file system. The location of the package may be at a URL. Default value: the <code>name</code> of the resource block. See "Syntax" section above for more information.</td>
</tr>
</tbody>
</table>

Examples:

**Specify a URL for the source attribute**

``` ruby
windows_package '7zip' do
  source 'http://www.7-zip.org/a/7z938-x64.msi'
end
```

**Specify path and checksum**

``` ruby
windows_package '7zip' do
  source 'http://www.7-zip.org/a/7z938-x64.msi'
  checksum '7c8e873991c82ad9cfc123415254ea6101e9a645e12977dcd518979e50fdedf3'
end
```

**Modify remote_file resource attributes**

The **windows_package** resource may specify a package at a remote
location using the `remote_file_attributes` property. This uses the
**remote_file** resource to download the contents at the specified URL
and passes in a Hash that modifies the properties of the [remote_file
resource](/resources/remote_file/).

For example:

``` ruby
windows_package '7zip' do
  source 'http://www.7-zip.org/a/7z938-x64.msi'
  remote_file_attributes ({
    :path => 'C:\\7zip.msi',
    :checksum => '7c8e873991c82ad9cfc123415254ea6101e9a645e12977dcd518979e50fdedf3'
  })
end
```

### knife client key

Use the `knife client` subcommand to manage an API client list and their
associated RSA public key-pairs. This allows authentication requests to
be made to the Chef server by any entity that uses the Chef server API,
such as Chef Client and knife.

#### key create

Use the `key create` argument to create a public key.

**Syntax**

This argument has the following syntax:

``` bash
knife client key create CLIENT_NAME (options)
```

**Options**

This argument has the following options:

`-e DATE`, `--expiration-date DATE`

:   The expiration date for the public key, specified as an ISO 8601
    formatted string: `YYYY-MM-DDTHH:MM:SSZ`. If this option is not
    specified, the public key will not have an expiration date. For
    example: `2013-12-24T21:00:00Z`.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name. If the `--public-key`
    option is not specified the Chef server will generate a private key.

`-k NAME`, `--key-name NAME`

:   The name of the public key.

`-p FILE_NAME`, `--public-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, and only if `--key-name` is specified, the Chef
    server will generate a public/private key pair.

#### key delete

Use the `key delete` argument to delete a public key.

**Syntax**

This argument has the following syntax:

``` bash
knife client key delete CLIENT_NAME KEY_NAME
```

#### key edit

Use the `key edit` argument to modify or rename a public key.

**Syntax**

This argument has the following syntax:

``` bash
knife client key edit CLIENT_NAME KEY_NAME (options)
```

**Options**

This argument has the following options:

`-c`, `--create-key`

:   Generate a new public/private key pair and replace an existing
    public key with the newly-generated public key. To replace the
    public key with an existing public key, use `--public-key` instead.

`-e DATE`, `--expiration-date DATE`

:   The expiration date for the public key, specified as an ISO 8601
    formatted string: `YYYY-MM-DDTHH:MM:SSZ`. If this option is not
    specified, the public key will not have an expiration date. For
    example: `2013-12-24T21:00:00Z`.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name. If the `--public-key`
    option is not specified the Chef server will generate a private key.

`-k NAME`, `--key-name NAME`

:   The name of the public key.

`-p FILE_NAME`, `--public-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, and only if `--key-name` is specified, the Chef
    server will generate a public/private key pair.

#### key list

Use the `key list` argument to view a list of public keys for the named
client.

**Syntax**

This argument has the following syntax:

``` bash
knife client key list CLIENT_NAME (options)
```

**Options**

This argument has the following options:

`-e`, `--only-expired`

:   Show a list of public keys that have expired.

`-n`, `--only-non-expired`

:   Show a list of public keys that have not expired.

`-w`, `--with-details`

:   Show a list of public keys, including URIs and expiration status.

#### key show

Use the `key show` argument to view details for a specific public key.

**Syntax**

This argument has the following syntax:

``` bash
knife client key show CLIENT_NAME KEY_NAME
```

### knife user key

Use the `knife user` subcommand to manage the list of users and their
associated RSA public key-pairs.

#### key create

Use the `key create` argument to create a public key.

**Syntax**

This argument has the following syntax:

``` bash
knife user key create USER_NAME (options)
```

**Options**

This argument has the following options:

`-e DATE`, `--expiration-date DATE`

:   The expiration date for the public key, specified as an ISO 8601
    formatted string: `YYYY-MM-DDTHH:MM:SSZ`. If this option is not
    specified, the public key will not have an expiration date. For
    example: `2013-12-24T21:00:00Z`.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name.

`-k NAME`, `--key-name NAME`

:   The name of the public key.

`-p FILE_NAME`, `--public-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, and only if `--key-name` is specified, the Chef
    server will generate a public/private key pair.

#### key delete

Use the `key delete` argument to delete a public key.

**Syntax**

This argument has the following syntax:

``` bash
knife user key delete USER_NAME KEY_NAME
```

#### key edit

Use the `key edit` argument to modify or rename a public key.

**Syntax**

This argument has the following syntax:

``` bash
knife user key edit USER_NAME KEY_NAME (options)
```

**Options**

This argument has the following options:

`-c`, `--create-key`

:   Generate a new public/private key pair and replace an existing
    public key with the newly-generated public key. To replace the
    public key with an existing public key, use `--public-key` instead.

`-e DATE`, `--expiration-date DATE`

:   The expiration date for the public key, specified as an ISO 8601
    formatted string: `YYYY-MM-DDTHH:MM:SSZ`. If this option is not
    specified, the public key will not have an expiration date. For
    example: `2013-12-24T21:00:00Z`.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name. If the `--public-key`
    option is not specified the Chef server will generate a private key.

`-k NAME`, `--key-name NAME`

:   The name of the public key.

`-p FILE_NAME`, `--public-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, and only if `--key-name` is specified, the Chef
    server will generate a public/private key pair.

#### key list

Use the `key list` argument to view a list of public keys for the named
user.

**Syntax**

This argument has the following syntax:

``` bash
knife user key list USER_NAME (options)
```

**Options**

This argument has the following options:

`-e`, `--only-expired`

:   Show a list of public keys that have expired.

`-n`, `--only-non-expired`

:   Show a list of public keys that have not expired.

`-w`, `--with-details`

:   Show a list of public keys, including URIs and expiration status.

#### key show

Use the `key show` argument to view details for a specific public key.

**Syntax**

This argument has the following syntax:

``` bash
knife user key show USER_NAME KEY_NAME
```

### Updated knife Options

With the new key management subcommands, the options for
`knife client create` and `knife user create` have changed.

#### knife client create

This argument has the following options:

`-a`, `--admin`

:   Create a client as an admin client.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name.

`-k`, `--prevent-keygen`

:   Create a user without a public key. This key may be managed later by
    using the `knife user key` subcommands.

    {{< note spaces=4 >}}

    This option is valid only with Chef server API, version 1.0, which
    was released with Chef server 12.1. If this option or the
    `--user-key` option are not passed in the command, the Chef server
    will create a user with a public key named `default` and will return
    the private key. For the Chef server versions earlier than 12.1,
    this option will not work; a public key is always generated unless
    `--user-key` is passed in the command.

    {{< /note >}}

`-p FILE`, `--public-key FILE`

:   The path to a file that contains the public key. This option may not
    be passed in the same command with `--prevent-keygen`. When using
    Chef a default key is generated if this option is not passed in the
    command. For Chef server version 12.x, see the `--prevent-keygen`
    option.

`--validator`

:   Create the client as the chef-validator. Default value: `true`.

#### knife user create

This argument has the following options:

`-a`, `--admin`

:   Create a client as an admin client. This is required for any user to
    access Open Source Chef as an administrator. This option only works
    when used with the open source Chef server and will have no effect
    when used with Enterprise Chef or Chef server 12.x.

`-f FILE_NAME`, `--file FILE_NAME`

:   Save a private key to the specified file name.

`-k`, `--prevent-keygen`

:   Create a user without a public key. This key may be managed later by
    using the `knife user key` subcommands.

    {{< note spaces=4 >}}

    This option is valid only with Chef server API, version 1.0, which
    was released with Chef server 12.1. If this option or the
    `--user-key` option are not passed in the command, the Chef server
    will create a user with a public key named `default` and will return
    the private key. For the Chef server versions earlier than 12.1,
    this option will not work; a public key is always generated unless
    `--user-key` is passed in the command.

    {{< /note >}}

`-p PASSWORD`, `--password PASSWORD`

:   The user password. This option only works when used with the open
    source Chef server and will have no effect when used with Enterprise
    Chef or Chef server 12.x.

`--user-key FILE_NAME`

:   The path to a file that contains the public key. When using Open
    Source Chef a default key is generated if this option is not passed
    in the command. For Chef server version 12.x, see the
    `--prevent-keygen` option.

## What's New in 12.3

The following items are new for chef-client 12.3 and/or are changes from
previous versions. The short version:

-   **Socketless local mode with chef-zero** Port binding and HTTP
    requests on localhost may be disabled in favor of socketless mode.
-   **Minimal Ohai plugins** Run only the plugins required for name
    resolution and resource/provider detection.
-   **Dynamic resource and provider resolution** Four helper methods may
    be used in a library file to get resource and/or provider mapping
    details, and then set them per-resource or provider.
-   **New clear_sources attribute for the chef_gem and gem_package
    resources** Set to `true` to download a gem from the path specified
    by the `source` property (and not from RubyGems).

### Socketless Local Mode

Chef Client may disable port binding and HTTP requests on localhost by
making a socketless request to chef-zero. This may be done from the
command line or with a configuration setting.

Use the following command-line option:

`--[no-]listen`

:   Run chef-zero in socketless mode. Use `--no-listen` to disable port
    binding and HTTP requests on localhost.

Or add the following setting to the client.rb file:

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
<td><code>listen</code></td>
<td>Run chef-zero in socketless mode. Set to <code>false</code> to disable port binding and HTTP requests on localhost.</td>
</tr>
</tbody>
</table>

### Minimal Ohai

The following option may be used with chef-client, chef-solo, and
chef-apply to speed up testing intervals:

`--minimal-ohai`

:   Run the Ohai plugins for name detection and resource/provider
    selection and no other Ohai plugins. Set to `true` during
    integration testing to speed up test cycles.

This setting may be configured using the `minimal_ohai` setting in the
client.rb file.

### Dynamic Resolution

Resources and providers are resolved dynamically and can handle multiple
`provides` lines for a specific platform. When multiple `provides` lines
exist, such as `Homebrew` and `MacPorts` packages for the macOS
platform, then one is selected based on resource priority mapping
performed by Chef Client during Chef Client run.

Use the following helpers in a library file to get and/or set resource
and/or provider priority mapping before any recipes are compiled:

`Chef.get_provider_priority_array(resource_name)`

:   Get the priority mapping for a provider.

`Chef.get_resource_priority_array(resource_name)`

:   Get the priority mapping for a resource.

`Chef.set_provider_priority_array(resource_name, Array<Class>, *filter)`

:   Set the priority mapping for a provider.

`Chef.set_resource_priority_array(resource_name, Array<Class>, *filter)`

:   Set the priority mapping for a resource.

For example:

``` ruby
Chef.set_resource_priority_array(:package, [ Chef::Resource::MacportsPackage ], os: 'darwin')
```

## What's New in 12.2

The following items are new for chef-client 12.2 and/or are changes from
previous versions. The short version:

-   **New dsc_resource** Use the **dsc_resource** resource to use any
    DSC resource in a Chef recipe.
-   **New --exit-on-error option for knife-ssh** Use the
    `--exit-on-error` option to have the `knife ssh` subcommand exit on
    any error.

### dsc_resource

Windows PowerShell is a task-based command-line shell and scripting
language developed by Microsoft. Windows PowerShell uses a
document-oriented approach for managing Microsoft Windows-based
machines, similar to the approach that is used for managing Unix and
Linux-based machines. Windows PowerShell is [a tool-agnostic
platform](https://docs.microsoft.com/en-us/powershell/scripting/powershell-scripting)
that supports using Chef for configuration management.

Desired State Configuration (DSC) is a feature of Windows PowerShell
that provides [a set of language extensions, cmdlets, and
resources](https://docs.microsoft.com/en-us/powershell/dsc/overview)
that can be used to declaratively configure software. DSC is similar to
Chef, in that both tools are idempotent, take similar approaches to the
concept of resources, describe the configuration of a system, and then
take the steps required to do that configuration. The most important
difference between Chef and DSC is that Chef uses Ruby and DSC is
exposed as configuration data from within Windows PowerShell.

The **dsc_resource** resource allows any DSC resource to be used in a
Chef recipe, as well as any custom resources that have been added to
your Windows PowerShell environment. Microsoft [frequently adds new
resources](https://github.com/powershell/DscResources) to the DSC
resource collection.

Using the **dsc_resource** has the following requirements:

-   Windows Management Framework (WMF) 5.0 February Preview (or higher),
    which includes Windows PowerShell 5.0.10018.0 (or higher).

-   The `RefreshMode` configuration setting in the Local Configuration
    Manager must be set to `Disabled`.

    {{< note spaces=4 >}}

    Starting with Chef Client 12.6 release, the `RefreshMode: Disabled`
    requirement applies only for versions of Windows PowerShell earlier
    than 5.0.10586.0. The latest version of Windows Management Framework
    (WMF) 5 has relaxed the limitation that prevented Chef Client from
    running in non-disabled refresh mode.

    {{< /note >}}

-   The **dsc_script** resource may not be used in the same run-list
    with the **dsc_resource**. This is because the **dsc_script**
    resource requires that `RefreshMode` in the Local Configuration
    Manager be set to `Push`, whereas the **dsc_resource** resource
    requires it to be set to `Disabled`.

-   The **dsc_resource** resource can only use binary- or script-based
    resources. Composite DSC resources may not be used. This is because
    composite resources aren't "real" resources from the perspective of
    the Local Configuration Manager (LCM). Composite resources are used
    by the "configuration" keyword from the
    `PSDesiredStateConfiguration` module, and then evaluated in that
    context. When using DSC to create the configuration document (the
    Managed Object Framework (MOF) file) from the configuration command,
    the composite resource is evaluated. Any individual resources from
    that composite resource are written into the Managed Object
    Framework (MOF) document. As far as the Local Configuration Manager
    (LCM) is concerned, there is no such thing as a composite resource.
    Unless that changes, the **dsc_resource** resource and/or
    `Invoke-DscResource` command cannot directly use them.

#### Syntax

A **dsc_resource** resource block allows DSC resources to be used in a
Chef recipe. For example, the DSC `Archive` resource:

``` powershell
Archive ExampleArchive {
  Ensure = "Present"
  Path = "C:\Users\Public\Documents\example.zip"
  Destination = "C:\Users\Public\Documents\ExtractionPath"
}
```

and then the same **dsc_resource** with Chef:

``` ruby
dsc_resource 'example' do
   resource :archive
   property :ensure, 'Present'
   property :path, "C:\Users\Public\Documents\example.zip"
   property :destination, "C:\Users\Public\Documents\ExtractionPath"
 end
```

The full syntax for all of the properties that are available to the
**dsc_resource** resource is:

``` ruby
dsc_resource 'name' do
  module_name                String
  notifies                   # see description
  property                   Symbol
  resource                   String
  subscribes                 # see description
end
```

where

-   `dsc_resource` is the resource
-   `name` is the name of the resource block
-   `property` is zero (or more) properties in the DSC resource, where
    each property is entered on a separate line, `:dsc_property_name` is
    the case-insensitive name of that property, and `"property_value"`
    is a Ruby value to be applied by the chef-client
-   `module_name`, `property`, and `resource` are properties of this
    resource, with the Ruby type shown. See "Properties" section below
    for more information about all of the properties that may be used
    with this resource.

#### Attributes

This resource has the following properties:

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`module_name`

:   **Ruby Type:** String

    The name of the module from which a DSC resource originates. If this
    property is not specified, it will be inferred.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`property`

:   **Ruby Type:** Symbol

    A property from a Desired State Configuration (DSC) resource. Use
    this property multiple times, one for each property in the Desired
    State Configuration (DSC) resource. The format for this property
    must follow `property :dsc_property_name, "property_value"` for each
    DSC property added to the resource block.

    The `:dsc_property_name` must be a symbol.

    Use the following Ruby types to define `property_value`:

    <table>
    <colgroup>
    <col style="width: 50%" />
    <col style="width: 50%" />
    </colgroup>
    <thead>
    <tr class="header">
    <th>Ruby</th>
    <th>Windows PowerShell</th>
    </tr>
    </thead>
    <tbody>
    <tr class="odd">
    <td><code>Array</code></td>
    <td><code>Object[]</code></td>
    </tr>
    <tr class="even">
    <td><code>Chef::Util::Powershell:PSCredential</code></td>
    <td><code>PSCredential</code></td>
    </tr>
    <tr class="odd">
    <td><code>False</code></td>
    <td><code>bool($false)</code></td>
    </tr>
    <tr class="even">
    <td><code>Fixnum</code></td>
    <td><code>Integer</code></td>
    </tr>
    <tr class="odd">
    <td><code>Float</code></td>
    <td><code>Double</code></td>
    </tr>
    <tr class="even">
    <td><code>Hash</code></td>
    <td><code>Hashtable</code></td>
    </tr>
    <tr class="odd">
    <td><code>True</code></td>
    <td><code>bool($true)</code></td>
    </tr>
    </tbody>
    </table>

    These are converted into the corresponding Windows PowerShell type
    during Chef Client run.

`resource`

:   **Ruby Type:** String

    The name of the DSC resource. This value is case-insensitive and
    must be a symbol that matches the name of the DSC resource.

    For built-in DSC resources, use the following values:

    <table>
    <colgroup>
    <col style="width: 50%" />
    <col style="width: 50%" />
    </colgroup>
    <thead>
    <tr class="header">
    <th>Value</th>
    <th>Description</th>
    </tr>
    </thead>
    <tbody>
    <tr class="odd">
    <td><code>:archive</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/archiveresource">unpack archive (.zip) files</a>.</td>
    </tr>
    <tr class="even">
    <td><code>:environment</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/environmentresource">manage system environment variables</a>.</td>
    </tr>
    <tr class="odd">
    <td><code>:file</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/fileresource">manage files and directories</a>.</td>
    </tr>
    <tr class="even">
    <td><code>:group</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/groupresource">manage local groups</a>.</td>
    </tr>
    <tr class="odd">
    <td><code>:log</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/logresource">log configuration messages</a>.</td>
    </tr>
    <tr class="even">
    <td><code>:package</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/packageresource">install and manage packages</a>.</td>
    </tr>
    <tr class="odd">
    <td><code>:registry</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/registryresource">manage registry keys and registry key values</a>.</td>
    </tr>
    <tr class="even">
    <td><code>:script</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/scriptresource">run PowerShell script blocks</a>.</td>
    </tr>
    <tr class="odd">
    <td><code>:service</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/serviceresource">manage services</a>.</td>
    </tr>
    <tr class="even">
    <td><code>:user</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/userresource">manage local user accounts</a>.</td>
    </tr>
    <tr class="odd">
    <td><code>:windowsfeature</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/windowsfeatureresource">add or remove Windows features and roles</a>.</td>
    </tr>
    <tr class="even">
    <td><code>:windowsoptionalfeature</code></td>
    <td>Use to configure Microsoft Windows optional features.</td>
    </tr>
    <tr class="odd">
    <td><code>:windowsprocess</code></td>
    <td>Use to <a href="https://msdn.microsoft.com/en-us/powershell/dsc/windowsprocessresource">configure Windows processes</a>.</td>
    </tr>
    </tbody>
    </table>

    Any DSC resource may be used in a Chef recipe. For example, the DSC
    Resource Kit contains resources for [configuring Active Directory
    components](http://www.powershellgallery.com/packages/xActiveDirectory/2.8.0.0),
    such as `xADDomain`, `xADDomainController`, and `xADUser`. Assuming
    that these resources are available to the chef-client, the
    corresponding values for the `resource` attribute would be:
    `:xADDomain`, `:xADDomainController`, and `xADUser`.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

#### Examples

**Open a Zip file**

``` ruby
dsc_resource 'example' do
   resource :archive
   property :ensure, 'Present'
   property :path, 'C:\Users\Public\Documents\example.zip'
   property :destination, 'C:\Users\Public\Documents\ExtractionPath'
 end
```

**Manage users and groups**

``` ruby
dsc_resource 'demogroupadd' do
  resource :group
  property :groupname, 'demo1'
  property :ensure, 'present'
end

dsc_resource 'useradd' do
  resource :user
  property :username, 'Foobar1'
  property :fullname, 'Foobar1'
  property :password, ps_credential('P@assword!')
  property :ensure, 'present'
end

dsc_resource 'AddFoobar1ToUsers' do
  resource :Group
  property :GroupName, 'demo1'
  property :MembersToInclude, ['Foobar1']
end
```

## What's New in 12.1

The following items are new for chef-client 12.1 and/or are changes from
previous versions. The short version:

-   **chef-client may be run in audit-mode** Use audit-mode to run audit
    tests against a node.
-   **control method added to Recipe DSL** Use the `control` method to
    define specific tests that match directories, files, packages,
    ports, and services. A `control` method must be contained within a
    `control_group` block.
-   **control_group method added to Recipe DSL** Use the
    `control_group` method to group one (or more) `control` methods into
    a single audit.
-   **Bootstrap nodes without using the ORGANIZATION-validator.key
    file** A node may now be bootstrapped using the USER.pem file,
    instead of the ORGANIZATION-validator.pem file. Also known as a
    "validatorless bootstrap".
-   **New options for knife-bootstrap** Use the
    `--bootstrap-vault-file`, `--bootstrap-vault-item`, and
    `--bootstrap-vault-json` options with `knife bootstrap` to specify
    items that are stored in chef-vault.
-   **New verify attribute for cookbook_file, file, remote_file, and
    template resources** Use the `verify` attribute to test a file using
    a block of code or a string.
-   **New imports attribute for dsc_script resource** Use the `imports`
    attribute to import DSC resources from modules.
-   **New attribute for chef_gem resource** Use the `compile_time`
    attribute to disable compile-time installation of gems.
-   **New openbsd_package resource** Use the **openbsd_package**
    resource to install packages on the OpenBSD platform.
-   **New --proxy-auth option for knife raw subcommand** Enable proxy
    authentication to the Chef server web user interface..
-   **New watchdog_timeout setting for the Windows platform** Use the
    `windows_service.watchdog_timeout` setting in the client.rb file to
    specify the maximum amount of time allowed for a chef-client run on
    the Microsoft Windows platform.
-   **Support for multiple packages and versions** Multiple packages and
    versions may be specified for platforms that use Yum or Apt.
-   **New attributes for windows_service resource** Use the
    `run_as_user` and `run_as_password` attributes to specify the user
    under which a Microsoft Windows service should run.

### chef-client, audit-mode

Chef Client may be run in audit-mode. Use audit-mode to evaluate custom
rules---also referred to as audits---that are defined in recipes.
audit-mode may be run in the following ways:

-   By itself (i.e. a chef-client run that does not build the resource
    collection or converge the node)
-   As part of Chef Client run, where audit-mode runs after all
    resources have been converged on the node

Each audit is authored within a recipe using the `control_group` and
`control` methods that are part of the Recipe DSL. Recipes that contain
audits are added to the run-list, after which they can be processed by
the chef-client. Output will appear in the same location as the regular
chef-client run (as specified by the `log_location` setting in the
client.rb file).

Finished audits are reported back to the Chef server. From there, audits
are sent to the Chef Analytics platform for further analysis, such as
rules processing and visibility from the actions web user interface.

Use following option to run Chef Client in audit-mode mode:

`--audit-mode MODE`

:   Enable audit-mode. Set to `audit-only` to skip the converge phase of
    Chef Client run and only perform audits. Possible values:
    `audit-only`, `disabled`, and `enabled`. Default value: `disabled`.

#### The Audit Run

The following diagram shows the stages of the audit-mode phase of Chef
Client run, and then the list below the diagram describes in greater
detail each of those stages.

![image](/images/audit_run.png)

When Chef Client is run in audit-mode, the following happens:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Stages</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><strong>chef-client Run ID</strong></td>
<td>Chef Client run identifier is associated with each audit.</td>
</tr>
<tr class="even">
<td><strong>Configure the Node</strong></td>
<td>If audit-mode is run as part of the full chef-client run, audit-mode occurs after Chef Client has finished converging all resources in the resource collection.</td>
</tr>
<tr class="odd">
<td><strong>Audit node based on controls in cookbooks</strong></td>
<td>Each <code>control_group</code> and <code>control</code> block found in any recipe that was part of the run-list of for the node is evaluated, with each expression in each <code>control</code> block verified against the state of the node.</td>
</tr>
<tr class="even">
<td><strong>Upload audit data to the Chef server</strong></td>
<td>When audit-mode mode is complete, the data is uploaded to the Chef server.</td>
</tr>
<tr class="odd">
<td><strong>Send to Chef Analytics</strong></td>
<td>Most of this data is passed to the Chef Analytics platform for further analysis, such as rules processing (for notification events triggered by expected or unexpected audit outcomes) and visibility from the actions web user interface.</td>
</tr>
</tbody>
</table>

### control

A control is an automated test that is built into a cookbook, and then
used to test the state of the system for compliance. Compliance can be
many things. For example, ensuring that file and directory management
meets specific internal IT policies---"Does the file exist?", "Do the
correct users or groups have access to this directory?". Compliance may
also be complex, such as helping to ensure goals defined by large-scale
compliance frameworks such as PCI, HIPAA, and Sarbanes-Oxley can be met.

Use the `control` method to define a specific series of tests that
comprise an individual audit. A `control` method MUST be contained
within a `control_group` block. A `control_group` block may contain
multiple `control` methods.

The syntax for the `control` method is as follows:

``` ruby
control_group 'audit name' do
  control 'name' do
    it 'should do something' do
      expect(something).to/.to_not be_something
    end
  end
end
```

where:

-   `control_group` groups one (or more) `control` blocks
-   `control 'name' do` defines an individual audit
-   Each `control` block must define at least one validation
-   Each `it` statement defines a single validation. `it` statements are
    processed individually when Chef Client is run in audit-mode
-   An `expect(something).to/.to_not be_something` is a statement that
    represents the individual test. In other words, this statement tests
    if something is expected to be (or not be) something. For example, a
    test that expects the PostgreSQL package to not be installed would
    be similar to `expect(package('postgresql')).to_not be_installed`
    and a test that ensures a service is enabled would be similar to
    `expect(service('init')).to be_enabled`
-   An `it` statement may contain multiple `expect` statements

#### directory Matcher

Matchers are available for directories. Use this matcher to define
audits for directories that test if the directory exists, is mounted,
and if it is linked to. This matcher uses the same matching
syntax---`expect(file('foo'))`---as the files. The following matchers
are available for directories:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Matcher</th>
<th>Description, Example</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>be_directory</code></p></td>
<td><p>Use to test if directory exists. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>it <span class="st">&#39;should be a directory&#39;</span> <span class="kw">do</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>  expect(file(<span class="st">&#39;/var/directory&#39;</span>)).to be_directory</span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_linked_to</code></p></td>
<td><p>Use to test if a subject is linked to the named directory. For example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>it <span class="st">&#39;should be linked to the named directory&#39;</span> <span class="kw">do</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>  expect(file(<span class="st">&#39;/etc/directory&#39;</span>)).to be_linked_to(<span class="st">&#39;/etc/some/other/directory&#39;</span>)</span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_mounted</code></p></td>
<td><p>Use to test if a directory is mounted. For example:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>it <span class="st">&#39;should be mounted&#39;</span> <span class="kw">do</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>  expect(file(<span class="ch">&#39;/&#39;</span>)).to be_mounted</span>
<span id="cb3-3"><a href="#cb3-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For directories with a single attribute that requires testing:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>it <span class="st">&#39;should be mounted with an ext4 partition&#39;</span> <span class="kw">do</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>  expect(file(<span class="ch">&#39;/&#39;</span>)).to be_mounted.with( <span class="st">:type</span> =&gt; <span class="st">&#39;ext4&#39;</span> )</span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For directories with multiple attributes that require testing:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a>it <span class="st">&#39;should be mounted only with certain attributes&#39;</span> <span class="kw">do</span></span>
<span id="cb5-2"><a href="#cb5-2"></a>  expect(file(<span class="ch">&#39;/&#39;</span>)).to be_mounted.only_with(</span>
<span id="cb5-3"><a href="#cb5-3"></a>    <span class="st">:attribute</span> =&gt; <span class="st">&#39;value&#39;</span>,</span>
<span id="cb5-4"><a href="#cb5-4"></a>    <span class="st">:attribute</span> =&gt; <span class="st">&#39;value&#39;</span>,</span>
<span id="cb5-5"><a href="#cb5-5"></a>)</span>
<span id="cb5-6"><a href="#cb5-6"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### file Matcher

Matchers are available for files and directories. Use this matcher to
define audits for files that test if the file exists, its version, if it
is executable, writable, or readable, who owns it, verify checksums
(both MD5 and SHA-256) and so on. The following matchers are available
for files:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Matcher</th>
<th>Description, Example</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>be_executable</code></p></td>
<td><p>Use to test if a file is executable. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>it <span class="st">&#39;should be executable&#39;</span> <span class="kw">do</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_executable</span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is executable by its owner:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>it <span class="st">&#39;should be executable by owner&#39;</span> <span class="kw">do</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_executable.by(<span class="st">&#39;owner&#39;</span>)</span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is executable by a group:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>it <span class="st">&#39;should be executable by group members&#39;</span> <span class="kw">do</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_executable.by(<span class="st">&#39;group&#39;</span>)</span>
<span id="cb3-3"><a href="#cb3-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is executable by a specific user:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>it <span class="st">&#39;should be executable by user foo&#39;</span> <span class="kw">do</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_executable.by_user(<span class="st">&#39;foo&#39;</span>)</span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_file</code></p></td>
<td><p>Use to test if a file exists. For example:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a>it <span class="st">&#39;should be a file&#39;</span> <span class="kw">do</span></span>
<span id="cb5-2"><a href="#cb5-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_file</span>
<span id="cb5-3"><a href="#cb5-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_grouped_into</code></p></td>
<td><p>Use to test if a file is grouped into the named group. For example:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb6-1"><a href="#cb6-1"></a>it <span class="st">&#39;should be grouped into foo&#39;</span> <span class="kw">do</span></span>
<span id="cb6-2"><a href="#cb6-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_grouped_into(<span class="st">&#39;foo&#39;</span>)</span>
<span id="cb6-3"><a href="#cb6-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_linked_to</code></p></td>
<td><p>Use to test if a subject is linked to the named file. For example:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb7-1"><a href="#cb7-1"></a>it <span class="st">&#39;should be linked to the named file&#39;</span> <span class="kw">do</span></span>
<span id="cb7-2"><a href="#cb7-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_linked_to(<span class="st">&#39;/etc/some/other/file&#39;</span>)</span>
<span id="cb7-3"><a href="#cb7-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_mode</code></p></td>
<td><p>Use to test if a file is set to the specified mode. For example:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb8-1"><a href="#cb8-1"></a>it <span class="st">&#39;should be mode 440&#39;</span> <span class="kw">do</span></span>
<span id="cb8-2"><a href="#cb8-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_mode(<span class="dv">440</span>)</span>
<span id="cb8-3"><a href="#cb8-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_owned_by</code></p></td>
<td><p>Use to test if a file is owned by the named owner. For example:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb9-1"><a href="#cb9-1"></a>it <span class="st">&#39;should be owned by the root user&#39;</span> <span class="kw">do</span></span>
<span id="cb9-2"><a href="#cb9-2"></a>  expect(file(<span class="st">&#39;/etc/sudoers&#39;</span>)).to be_owned_by(<span class="st">&#39;root&#39;</span>)</span>
<span id="cb9-3"><a href="#cb9-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_readable</code></p></td>
<td><p>Use to test if a file is readable. For example:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb10-1"><a href="#cb10-1"></a>it <span class="st">&#39;should be readable&#39;</span> <span class="kw">do</span></span>
<span id="cb10-2"><a href="#cb10-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_readable</span>
<span id="cb10-3"><a href="#cb10-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is readable by its owner:</p>
<div class="sourceCode" id="cb11"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb11-1"><a href="#cb11-1"></a>it <span class="st">&#39;should be readable by owner&#39;</span> <span class="kw">do</span></span>
<span id="cb11-2"><a href="#cb11-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_readable.by(<span class="st">&#39;owner&#39;</span>)</span>
<span id="cb11-3"><a href="#cb11-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is readable by a group:</p>
<div class="sourceCode" id="cb12"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb12-1"><a href="#cb12-1"></a>it <span class="st">&#39;should be readable by group members&#39;</span> <span class="kw">do</span></span>
<span id="cb12-2"><a href="#cb12-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_readable.by(<span class="st">&#39;group&#39;</span>)</span>
<span id="cb12-3"><a href="#cb12-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is readable by a specific user:</p>
<div class="sourceCode" id="cb13"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb13-1"><a href="#cb13-1"></a>it <span class="st">&#39;should be readable by user foo&#39;</span> <span class="kw">do</span></span>
<span id="cb13-2"><a href="#cb13-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_readable.by_user(<span class="st">&#39;foo&#39;</span>)</span>
<span id="cb13-3"><a href="#cb13-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_socket</code></p></td>
<td><p>Use to test if a file exists as a socket. For example:</p>
<div class="sourceCode" id="cb14"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb14-1"><a href="#cb14-1"></a>it <span class="st">&#39;should be a socket&#39;</span> <span class="kw">do</span></span>
<span id="cb14-2"><a href="#cb14-2"></a>  expect(file(<span class="st">&#39;/var/file.sock&#39;</span>)).to be_socket</span>
<span id="cb14-3"><a href="#cb14-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_symlink</code></p></td>
<td><p>Use to test if a file exists as a symbolic link. For example:</p>
<div class="sourceCode" id="cb15"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb15-1"><a href="#cb15-1"></a>it <span class="st">&#39;should be a symlink&#39;</span> <span class="kw">do</span></span>
<span id="cb15-2"><a href="#cb15-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_symlink</span>
<span id="cb15-3"><a href="#cb15-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_version</code></p></td>
<td><p>Microsoft Windows only. Use to test if a file is the specified version. For example:</p>
<div class="sourceCode" id="cb16"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb16-1"><a href="#cb16-1"></a>it <span class="st">&#39;should be version 1.2&#39;</span> <span class="kw">do</span></span>
<span id="cb16-2"><a href="#cb16-2"></a>  expect(file(<span class="st">&#39;C:\\Windows\\path\\to\\file&#39;</span>)).to be_version(<span class="st">&#39;1.2&#39;</span>)</span>
<span id="cb16-3"><a href="#cb16-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_writable</code></p></td>
<td><p>Use to test if a file is writable. For example:</p>
<div class="sourceCode" id="cb17"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb17-1"><a href="#cb17-1"></a>it <span class="st">&#39;should be writable&#39;</span> <span class="kw">do</span></span>
<span id="cb17-2"><a href="#cb17-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_writable</span>
<span id="cb17-3"><a href="#cb17-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is writable by its owner:</p>
<div class="sourceCode" id="cb18"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb18-1"><a href="#cb18-1"></a>it <span class="st">&#39;should be writable by owner&#39;</span> <span class="kw">do</span></span>
<span id="cb18-2"><a href="#cb18-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_writable.by(<span class="st">&#39;owner&#39;</span>)</span>
<span id="cb18-3"><a href="#cb18-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is writable by a group:</p>
<div class="sourceCode" id="cb19"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb19-1"><a href="#cb19-1"></a>it <span class="st">&#39;should be writable by group members&#39;</span> <span class="kw">do</span></span>
<span id="cb19-2"><a href="#cb19-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_writable.by(<span class="st">&#39;group&#39;</span>)</span>
<span id="cb19-3"><a href="#cb19-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a file that is writable by a specific user:</p>
<div class="sourceCode" id="cb20"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb20-1"><a href="#cb20-1"></a>it <span class="st">&#39;should be writable by user foo&#39;</span> <span class="kw">do</span></span>
<span id="cb20-2"><a href="#cb20-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to be_writable.by_user(<span class="st">&#39;foo&#39;</span>)</span>
<span id="cb20-3"><a href="#cb20-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>contain</code></p></td>
<td><p>Use to test if a file contains specific contents. For example:</p>
<div class="sourceCode" id="cb21"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb21-1"><a href="#cb21-1"></a>it <span class="st">&#39;should contain docs.chef.io&#39;</span> <span class="kw">do</span></span>
<span id="cb21-2"><a href="#cb21-2"></a>  expect(file(<span class="st">&#39;/etc/file&#39;</span>)).to contain(<span class="st">&#39;docs.chef.io&#39;</span>)</span>
<span id="cb21-3"><a href="#cb21-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### package Matcher

Matchers are available for packages and may be used to define audits
that test if a package or a package version is installed. The following
matchers are available:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Matcher</th>
<th>Description, Example</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>be_installed</code></p></td>
<td><p>Use to test if the named package is installed. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>it <span class="st">&#39;should be installed&#39;</span> <span class="kw">do</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>  expect(package(<span class="st">&#39;httpd&#39;</span>)).to be_installed</span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a specific package version:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>it <span class="st">&#39;should be installed&#39;</span> <span class="kw">do</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>  expect(package(<span class="st">&#39;httpd&#39;</span>)).to be_installed.with_version(<span class="st">&#39;0.1.2&#39;</span>)</span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### port Matcher

Matchers are available for ports and may be used to define audits that
test if a port is listening. The following matchers are available:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Matcher</th>
<th>Description, Example</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>be_listening</code></p></td>
<td><p>Use to test if the named port is listening. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>it <span class="st">&#39;should be listening&#39;</span> <span class="kw">do</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>  expect(port(<span class="dv">23</span>)).to be_listening</span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a named port that is not listening:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>it <span class="st">&#39;should not be listening&#39;</span> <span class="kw">do</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>  expect(port(<span class="dv">23</span>)).to_not be_listening</span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a specific port type use <code>.with('port_type')</code>. For example, UDP:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>it <span class="st">&#39;should be listening with UDP&#39;</span> <span class="kw">do</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>  expect(port(<span class="dv">23</span>)).to_not be_listening.with(<span class="st">&#39;udp&#39;</span>)</span>
<span id="cb3-3"><a href="#cb3-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For UDP, version 6:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>it <span class="st">&#39;should be listening with UDP6&#39;</span> <span class="kw">do</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>  expect(port(<span class="dv">23</span>)).to_not be_listening.with(<span class="st">&#39;udp6&#39;</span>)</span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For TCP/IP:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a>it <span class="st">&#39;should be listening with TCP&#39;</span> <span class="kw">do</span></span>
<span id="cb5-2"><a href="#cb5-2"></a>  expect(port(<span class="dv">23</span>)).to_not be_listening.with(<span class="st">&#39;tcp&#39;</span>)</span>
<span id="cb5-3"><a href="#cb5-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For TCP/IP, version 6:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb6-1"><a href="#cb6-1"></a>it <span class="st">&#39;should be listening with TCP6&#39;</span> <span class="kw">do</span></span>
<span id="cb6-2"><a href="#cb6-2"></a>  expect(port(<span class="dv">23</span>)).to_not be_listening.with(<span class="st">&#39;tcp6&#39;</span>)</span>
<span id="cb6-3"><a href="#cb6-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### service Matcher

Matchers are available for services and may be used to define audits
that test for conditions related to services, such as if they are
enabled, running, have the correct startup mode, and so on. The
following matchers are available:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Matcher</th>
<th>Description, Example</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>be_enabled</code></p></td>
<td><p>Use to test if the named service is enabled (i.e. will start up automatically). For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>it <span class="st">&#39;should be enabled&#39;</span> <span class="kw">do</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_enabled</span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a service that is enabled at a given run level:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>it <span class="st">&#39;should be enabled at the specified run level&#39;</span> <span class="kw">do</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_enabled.with_level(<span class="dv">3</span>)</span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_installed</code></p></td>
<td><p>Microsoft Windows only. Use to test if the named service is installed on the Microsoft Windows platform. For example:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>it <span class="st">&#39;should be installed&#39;</span> <span class="kw">do</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>  expect(service(<span class="st">&#39;DNS Client&#39;</span>)).to be_installed</span>
<span id="cb3-3"><a href="#cb3-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>be_running</code></p></td>
<td><p>Use to test if the named service is running. For example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>it <span class="st">&#39;should be running&#39;</span> <span class="kw">do</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_running</span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="kw">end</span></span></code></pre></div>
<p>For a service that is running under supervisor:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a>it <span class="st">&#39;should be running under supervisor&#39;</span> <span class="kw">do</span></span>
<span id="cb5-2"><a href="#cb5-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_running.under(<span class="st">&#39;supervisor&#39;</span>)</span>
<span id="cb5-3"><a href="#cb5-3"></a><span class="kw">end</span></span></code></pre></div>
<p>or daemontools:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb6-1"><a href="#cb6-1"></a>it <span class="st">&#39;should be running under daemontools&#39;</span> <span class="kw">do</span></span>
<span id="cb6-2"><a href="#cb6-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_running.under(<span class="st">&#39;daemontools&#39;</span>)</span>
<span id="cb6-3"><a href="#cb6-3"></a><span class="kw">end</span></span></code></pre></div>
<p>or Upstart:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb7-1"><a href="#cb7-1"></a>it <span class="st">&#39;should be running under upstart&#39;</span> <span class="kw">do</span></span>
<span id="cb7-2"><a href="#cb7-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_running.under(<span class="st">&#39;upstart&#39;</span>)</span>
<span id="cb7-3"><a href="#cb7-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>be_monitored_by</code></p></td>
<td><p>Use to test if the named service is being monitored by the named monitoring application. For example:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb8-1"><a href="#cb8-1"></a>it <span class="st">&#39;should be monitored by&#39;</span> <span class="kw">do</span></span>
<span id="cb8-2"><a href="#cb8-2"></a>  expect(service(<span class="st">&#39;ntpd&#39;</span>)).to be_monitored_by(<span class="st">&#39;monit&#39;</span>)</span>
<span id="cb8-3"><a href="#cb8-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>have_start_mode</code></p></td>
<td><p>Microsoft Windows only. Use to test if the named service's startup mode is correct on the Microsoft Windows platform. For example:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb9-1"><a href="#cb9-1"></a>it <span class="st">&#39;should start manually&#39;</span> <span class="kw">do</span></span>
<span id="cb9-2"><a href="#cb9-2"></a>  expect(service(<span class="st">&#39;DNS Client&#39;</span>)).to have_start_mode.<span class="dt">Manual</span></span>
<span id="cb9-3"><a href="#cb9-3"></a><span class="kw">end</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### Examples

**A package is installed**

For example, a package is installed:

``` ruby
control_group 'audit name' do
  control 'mysql package' do
    it 'should be installed' do
      expect(package('mysql')).to be_installed
    end
  end
end
```

The `control_group` block is processed when Chef Client run is run in
audit-mode. If the audit was successful, Chef Client will return output
similar to:

``` bash
Audit Mode
  mysql package
    should be installed
```

If an audit was unsuccessful, Chef Client will return output similar to:

``` bash
Starting audit phase

Audit Mode
  mysql package
  should be installed (FAILED - 1)

Failures:

1) Audit Mode mysql package should be installed
  Failure/Error: expect(package('mysql')).to be_installed.with_version('5.6')
    expected Package 'mysql' to be installed
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:22:in 'block (3 levels) in from_file'

Finished in 0.5745 seconds (files took 0.46481 seconds to load)
1 examples, 1 failures

Failed examples:

rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:21 # Audit Mode mysql package should be installed
```

**A package version is installed**

A package that is installed with a specific version:

``` ruby
control_group 'audit name' do
  control 'mysql package' do
    it 'should be installed' do
      expect(package('mysql')).to be_installed.with_version('5.6')
    end
  end
end
```

**A package is not installed**

A package that is not installed:

``` ruby
control_group 'audit name' do
  control 'postgres package' do
    it 'should not be installed' do
      expect(package('postgresql')).to_not be_installed
    end
  end
end
```

If the audit was successful, Chef Client will return output similar to:

``` bash
Audit Mode
  postgres audit
    postgres package
      is not installed
```

**A service is enabled**

A service that is enabled and running:

``` ruby
control_group 'audit name' do
  control 'mysql service' do
    let(:mysql_service) { service('mysql') }
    it 'should be enabled' do
      expect(mysql_service).to be_enabled
    end
    it 'should be running' do
      expect(mysql_service).to be_running
    end
  end
end
```

If the audit was successful, Chef Client will return output similar to:

``` bash
Audit Mode
  mysql service audit
    mysql service
      is enabled
      is running
```

**A configuration file contains specific settings**

The following example shows how to verify `sshd` configuration, including
whether it's installed, what the permissions are, and how it can be
accessed:

``` ruby
control_group 'check sshd configuration' do

  control 'sshd package' do
    it 'should be installed' do
      expect(package('openssh-server')).to be_installed
    end
  end

  control 'sshd configuration' do
    let(:config_file) { file('/etc/ssh/sshd_config') }
    it 'should exist with the right permissions' do
      expect(config_file).to be_file
      expect(config_file).to be_mode(644)
      expect(config_file).to be_owned_by('root')
      expect(config_file).to be_grouped_into('root')
    end
    it 'should not permit RootLogin' do
      expect(config_file.content).to_not match(/^PermitRootLogin yes/)
    end
    it 'should explicitly not permit PasswordAuthentication' do
      expect(config_file.content).to match(/^PasswordAuthentication no/)
    end
    it 'should force privilege separation' do
      expect(config_file.content).to match(/^UsePrivilegeSeparation sandbox/)
    end
  end
end
```

where

-   `let(:config_file) { file('/etc/ssh/sshd_config') }` uses the `file`
    matcher to test specific settings within the `sshd` configuration
    file

**A file contains desired permissions and contents**

The following example shows how to verify that a file has the desired
permissions and contents:

``` ruby
controls 'mysql config' do
  control 'mysql config file' do
    let(:config_file) { file('/etc/mysql/my.cnf') }
    it 'exists with correct permissions' do
      expect(config_file).to be_file
      expect(config_file).to be_mode(0400)
    end
    it 'contains required configuration' do
      expect(its('contents')).to match(/default-time-zone='UTC'/)
    end
  end
end
```

If the audit was successful, Chef Client will return output similar to:

``` bash
Audit Mode
  mysql config
    mysql config file
      exists with correct permissions
      contains required configuration
```

### control_group

Use the `control_group` method to define a group of `control` methods
that comprise a single audit. The name of each `control_group` must be
unique within the organization.

The syntax for the `control_group` method is as follows:

``` ruby
control_group 'name' do
  control 'name' do
    it 'should do something' do
      expect(something).to/.to_not be_something
    end
  end
  control 'name' do
    ...
  end
  ...
end
```

where:

-   `control_group` groups one (or more) `control` blocks
-   `'name'` is the unique name for the `control_group`; Chef Client
    will raise an exception if duplicate `control_group` names are
    present
-   `control` defines each individual audit within the `control_group`
    block. There is no limit to the number of `control` blocks that may
    defined within a `control_group` block

#### Examples

**control_group block with multiple control blocks**

The following `control_group` ensures that MySQL is installed, that
PostgreSQL is not installed, and that the services and configuration
files associated with MySQL are configured correctly:

``` ruby
control_group 'Audit Mode' do

  control 'mysql package' do
    it 'should be installed' do
      expect(package('mysql')).to be_installed.with_version('5.6')
    end
  end

  control 'postgres package' do
    it 'should not be installed' do
      expect(package('postgresql')).to_not be_installed
    end
  end

  control 'mysql service' do
    let(:mysql_service) { service('mysql') }
    it 'should be enabled' do
      expect(mysql_service).to be_enabled
    end
    it 'should be running' do
      expect(mysql_service).to be_running
    end
  end

  control 'mysql config directory' do
    let(:config_dir) { file('/etc/mysql') }
    it 'should exist with correct permissions' do
      expect(config_dir).to be_directory
      expect(config_dir).to be_mode(0700)
    end
    it 'should be owned by the db user' do
      expect(config_dir).to be_owned_by('db_service_user')
    end
  end

  control 'mysql config file' do
    let(:config_file) { file('/etc/mysql/my.cnf') }
    it 'should exist with correct permissions' do
      expect(config_file).to be_file
      expect(config_file).to be_mode(0400)
    end
    it 'should contain required configuration' do
      expect(config_file.content).to match(/default-time-zone='UTC'/)
    end
  end

end
```

The `control_group` block is processed when Chef Client is run in
audit-mode. If Chef Client run was successful, Chef Client will return
output similar to:

``` bash
Audit Mode
  mysql package
    should be installed
  postgres package
    should not be installed
  mysql service
    should be enabled
    should be running
  mysql config directory
    should exist with correct permissions
    should be owned by the db user
  mysql config file
    should exist with correct permissions
    should contain required configuration
```

If an audit was unsuccessful, Chef Client will return output similar to:

``` bash
Starting audit phase

Audit Mode
  mysql package
  should be installed (FAILED - 1)
postgres package
  should not be installed
mysql service
  should be enabled (FAILED - 2)
  should be running (FAILED - 3)
mysql config directory
  should exist with correct permissions (FAILED - 4)
  should be owned by the db user (FAILED - 5)
mysql config file
  should exist with correct permissions (FAILED - 6)
  should contain required configuration (FAILED - 7)

Failures:

1) Audit Mode mysql package should be installed
  Failure/Error: expect(package('mysql')).to be_installed.with_version('5.6')
    expected Package 'mysql' to be installed
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:22:in 'block (3 levels) in from_file'

2) Audit Mode mysql service should be enabled
  Failure/Error: expect(mysql_service).to be_enabled
    expected Service 'mysql' to be enabled
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:35:in 'block (3 levels) in from_file'

3) Audit Mode mysql service should be running
   Failure/Error: expect(mysql_service).to be_running
    expected Service 'mysql' to be running
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:38:in 'block (3 levels) in from_file'

4) Audit Mode mysql config directory should exist with correct permissions
  Failure/Error: expect(config_dir).to be_directory
    expected `File '/etc/mysql'.directory?` to return true, got false
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:45:in 'block (3 levels) in from_file'

5) Audit Mode mysql config directory should be owned by the db user
  Failure/Error: expect(config_dir).to be_owned_by('db_service_user')
    expected `File '/etc/mysql'.owned_by?('db_service_user')` to return true, got false
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:49:in 'block (3 levels) in from_file'

6) Audit Mode mysql config file should exist with correct permissions
  Failure/Error: expect(config_file).to be_file
    expected `File '/etc/mysql/my.cnf'.file?` to return true, got false
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:56:in 'block (3 levels) in from_file'

7) Audit Mode mysql config file should contain required configuration
  Failure/Error: expect(config_file.content).to match(/default-time-zone='UTC'/)
    expected '-n\n' to match /default-time-zone='UTC'/
    Diff:
    @@ -1,2 +1,2 @@
    -/default-time-zone='UTC'/
    +-n
  # /var/chef/cache/cookbooks/grantmc/recipes/default.rb:60:in 'block (3 levels) in from_file'

Finished in 0.5745 seconds (files took 0.46481 seconds to load)
8 examples, 7 failures

Failed examples:

rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:21 # Audit Mode mysql package should be installed
rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:34 # Audit Mode mysql service should be enabled
rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:37 # Audit Mode mysql service should be running
rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:44 # Audit Mode mysql config directory should exist with correct permissions
rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:48 # Audit Mode mysql config directory should be owned by the db user
rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:55 # Audit Mode mysql config file should exist with correct permissions
rspec /var/chef/cache/cookbooks/grantmc/recipes/default.rb:59 # Audit Mode mysql config file should contain required configuration
Auditing complete
```

**Duplicate control_group names**

If two `control_group` blocks have the same name, Chef Client will raise
an exception. For example, the following `control_group` blocks exist in
different cookbooks:

``` ruby
control_group 'basic control group' do
  it 'should pass' do
    expect(2 - 2).to eq(0)
  end
end
```

``` ruby
control_group 'basic control group' do
  it 'should pass' do
    expect(3 - 2).to eq(1)
  end
end
```

Because the two `control_group` block names are identical, Chef Client
will return an exception similar to:

``` ruby
Synchronizing Cookbooks:
  - audit_test
Compiling Cookbooks...

================================================================================
Recipe Compile Error in /Users/grantmc/.cache/chef/cache/cookbooks
                        /audit_test/recipes/error_duplicate_control_groups.rb
================================================================================

Chef::Exceptions::AuditControlGroupDuplicate
--------------------------------------------
Audit control group with name 'basic control group' has already been defined

Cookbook Trace:
---------------
/Users/grantmc/.cache/chef/cache/cookbooks
/audit_test/recipes/error_duplicate_control_groups.rb:13:in 'from_file'

Relevant File Content:
----------------------
/Users/grantmc/.cache/chef/cache/cookbooks/audit_test/recipes/error_duplicate_control_groups.rb:

control_group 'basic control group' do
  it 'should pass' do
    expect(2 - 2).to eq(0)
  end
end

control_group 'basic control group' do
  it 'should pass' do
    expect(3 - 2).to eq(1)
  end
end

Running handlers:
[2015-01-15T09:36:14-08:00] ERROR: Running exception handlers
Running handlers complete
```

**Verify a package is installed**

The following `control_group` verifies that the `git` package has been
installed:

``` ruby
package 'git' do
  action :install
end

execute 'list packages' do
  command 'dpkg -l'
end

execute 'list directory' do
  command 'ls -R ~'
end

control_group 'my audits' do
  control 'check git' do
    it 'should be installed' do
      expect(package('git')).to be_installed
    end
  end
end
```

### Validatorless Bootstrap

The ORGANIZATION-validator.pem is typically added to the .chef directory
on the workstation. When a node is bootstrapped from that workstation,
the ORGANIZATION-validator.pem is used to authenticate the newly-created
node to the Chef server during the initial chef-client run. Starting
with Chef client 12.1, it is possible to bootstrap a node using the
USER.pem file instead of the ORGANIZATION-validator.pem file. This is
known as a "validatorless bootstrap".

To create a node via the USER.pem file, simply delete the
ORGANIZATION-validator.pem file on the workstation. For example:

``` bash
rm -f /home/lamont/.chef/myorg-validator.pem
```

and then make the following changes in the config.rb file:

-   Remove the `validation_client_name` setting
-   Edit the `validation_key` setting to be something that isn't a path
    to an existent ORGANIZATION-validator.pem file. For example:
    `/nonexist`.

As long as a USER.pem is also present on the workstation from which the
validatorless bootstrap operation will be initiated, the bootstrap
operation will run and will use the USER.pem file instead of the
ORGANIZATION-validator.pem file.

When running a validatorless `knife bootstrap` operation, the output is
similar to:

``` bash
desktop% knife bootstrap 10.1.1.1 -N foo01.acme.org \
  -E dev -r 'role[base]' -j '{ "foo": "bar" }' \
  --ssh-user vagrant --sudo
Node foo01.acme.org exists, overwrite it? (Y/N)
Client foo01.acme.org exists, overwrite it? (Y/N)
Creating new client for foo01.acme.org
Creating new node for foo01.acme.org
Connecting to 10.1.1.1
10.1.1.1 Starting first Chef Client run...
[....etc...]
```

#### knife bootstrap Options

Use the following options to specify items that are stored in
chef-vault:

`--bootstrap-vault-file VAULT_FILE`

:   The path to a JSON file that contains a list of vaults and items to
    be updated.

`--bootstrap-vault-item VAULT_ITEM`

:   A single vault and item to update as `vault:item`.

`--bootstrap-vault-json VAULT_JSON`

:   A JSON string that contains a list of vaults and items to be
    updated.

    For example:

    ``` none
    --bootstrap-vault-json '{ "vault1": ["item1", "item2"], "vault2": "item2" }'
    ```

### New Resource Attributes

The following attributes are new for chef-client 12.1.

#### verify

The `verify` attribute may be used with the **cookbook_file**,
**file**, **remote_file**, and **template** resources.

`verify`

:   A block or a string that returns `true` or `false`. A string, when
    `true` is executed as a system command.

    The following examples show how the `verify` attribute is used with
    the **template** resource. The same approach (but with different
    resource names) is true for the **cookbook_file**, **file**, and
    **remote_file** resources:

    A block is arbitrary Ruby defined within the resource block by using
    the `verify` property. When a block is `true`, Chef Client will
    continue to update the file as appropriate.

    For example, this should return `true`:

    ``` ruby
    template '/tmp/baz' do
      verify { 1 == 1 }
    end
    ```

    This should return `true`:

    ``` ruby
    template '/etc/nginx.conf' do
      verify 'nginx -t -c %{path}'
    end
    ```

    {{< warning spaces =4 >}}

    For releases of Chef Client prior to 12.5 (chef-client 12.4 and
    earlier) the correct syntax is:

    ``` ruby
    template '/etc/nginx.conf' do
      verify 'nginx -t -c %{file}'
    end
    ```

    See GitHub issues <https://github.com/chef/chef/issues/3232> and
    <https://github.com/chef/chef/pull/3693> for more information about
    these differences.

    {{< /warning >}}

    This should return `true`:

    ``` ruby
    template '/tmp/bar' do
      verify { 1 == 1}
    end
    ```

    And this should return `true`:

    ``` ruby
    template '/tmp/foo' do
      verify do |path|
        true
      end
    end
    ```

    Whereas, this should return `false`:

    ``` ruby
    template '/tmp/turtle' do
      verify '/usr/bin/false'
    end
    ```

    If a string or a block return `false`, Chef Client run will stop and
    an error is returned.

#### imports

The following attribute is new for the **dsc_script** resource:

`imports`

:   Use to import DSC resources from a module. To import all resources
    from a module, specify only the module name:

    ``` ruby
    imports "module_name"
    ```

    To import specific resources, specify the module name and then the
    name for each resource in that module to import:

    ``` ruby
    imports "module_name", "resource_name_a", "resource_name_b", ...
    ```

    For example, to import all resources from a module named
    `cRDPEnabled`:

    ``` ruby
    imports "cRDPEnabled"
    ```

    And to import only the `PSHOrg_cRDPEnabled` resource:

    ``` ruby
    imports "cRDPEnabled", "PSHOrg_cRDPEnabled"
    ```

#### compile_time

The following attribute is new for the **chef_gem** resource:

`compile_time`

:   Controls the phase during which a gem is installed on a node. Set to
    `true` to install a gem while the resource collection is being built
    (the "compile phase"). Set to `false` to install a gem while Chef
    Client is configuring the node (the "converge phase"). Possible
    values: `nil` (for verbose warnings), `true` (to warn once per
    chef-client run), or `false` (to remove all warnings). Recommended
    value: `false`.

    {{< note spaces=4 >}}

    This topic is hooked into client.rb topics, starting with 12.1, in
    addition to the resource reference pages.

    {{< /note >}}

    To suppress warnings for cookbooks authored prior to chef-client
    12.1, use a `respond_to?` check to ensure backward compatibility.
    For example:

    ``` ruby
    chef_gem 'aws-sdk' do
      compile_time false if respond_to?(:compile_time)
    end
    ```

### [run_as](#run_as)

The following attributes are new for the **windows_service** resource:

`run_as_password`

:   The password for the user specified by `run_as_user`.

`run_as_user`

:   The user under which a Microsoft Windows service runs.

### paludis_package

Use the **paludis_package** resource to manage packages for the Paludis
platform.

{{< note >}}

In many cases, it is better to use the package resource instead of this
one. This is because when the package resource is used in a recipe, Chef
Client will use details that are collected by Ohai at the start of Chef
Client run to determine the correct package application. Using the
package resource allows a recipe to be authored in a way that allows it
to be used across many platforms.

{{< /note >}}

#### Syntax

A **paludis_package** resource block manages a package on a node,
typically by installing it. The simplest use of the **paludis_package**
resource is:

``` ruby
paludis_package 'package_name'
```

which will install the named package using all of the default options
and the default action (`:install`).

The full syntax for all of the properties that are available to the
**paludis_package** resource is:

``` ruby
paludis_package 'name' do
  notifies                   # see description
  options                    String
  package_name               String, Array # defaults to 'name' if not specified
  source                     String
  subscribes                 # see description
  timeout                    String, Integer
  version                    String, Array
  action                     Symbol # defaults to :install if not specified
end
```

where:

-   `paludis_package` is the resource.
-   `name` is the name given to the resource block.
-   `action` identifies which steps Chef Client will take to bring the
    node into the desired state.
-   `options`, `package_name`, `source`, `recursive`, `timeout`, and
    `version` are properties of this resource, with the Ruby type shown.
    See "Properties" section below for more information about all of the
    properties that may be used with this resource.

#### Actions

This resource has the following actions:

`:install`

:   Default. Install a package. If a version is specified, install the
    specified version of the package.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:remove`

:   Remove a package.

`:upgrade`

:   Install a package and/or ensure that a package is the latest
    version.

#### Attributes

This resource has the following properties:

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`options`

:   **Ruby Type:** String

    One (or more) additional options that are passed to the command.

`package_name`

:   **Ruby Type:** String, Array

    The name of the package. Default value: the `name` of the resource
    block. See "Syntax" section above for more information.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`source`

:   **Ruby Type:** String

    Optional. The path to a package in the local file system.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`timeout`

:   **Ruby Type:** String, Integer

    The amount of time (in seconds) to wait before timing out.

`version`

:   **Ruby Type:** String, Array

    The version of a package to be installed or upgraded.

#### Examples

**Install a package**

``` ruby
paludis_package 'name of package' do
  action :install
end
```

### openbsd_package

Use the **openbsd_package** resource to manage packages for the OpenBSD
platform.

{{< note >}}

In many cases, it is better to use the package resource instead of this
one. This is because when the package resource is used in a recipe, Chef
Client will use details that are collected by Ohai at the start of Chef
Client run to determine the correct package application. Using the
package resource allows a recipe to be authored in a way that allows it
to be used across many platforms.

{{< /note >}}

#### Syntax

A **openbsd_package** resource block manages a package on a node,
typically by installing it. The simplest use of the **openbsd_package**
resource is:

``` ruby
openbsd_package 'package_name'
```

which will install the named package using all of the default options
and the default action (`:install`).

The full syntax for all of the properties that are available to the
**openbsd_package** resource is:

``` ruby
openbsd_package 'name' do
  options                    String
  package_name               String, Array # defaults to 'name' if not specified
  source                     String
  timeout                    String, Integer
  version                    String, Array
  action                     Symbol # defaults to :install if not specified
end
```

where:

-   `openbsd_package` is the resource.
-   `name` is the name given to the resource block.
-   `action` identifies which steps Chef Client will take to bring the
    node into the desired state
-   `options`, `package_name`, `source`, `timeout`, and `version` are
    properties of this resource, with the Ruby type shown. See
    "Properties" section below for more information about all of the
    properties that may be used with this resource.

#### Actions

The openbsd_package resource has the following actions:

`:install`

:   Default. Install a package. If a version is specified, install the
    specified version of the package.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:remove`

:   Remove a package.

#### Attributes

This resource has the following properties:

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`options`

:   **Ruby Type:** String

    One (or more) additional options that are passed to the command.

`package_name`

:   **Ruby Type:** String, Array

    The name of the package. Default value: the `name` of the resource
    block. See "Syntax" section above for more information.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`source`

:   **Ruby Type:** String

    Optional. The path to a package in the local file system.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`timeout`

:   **Ruby Type:** String, Integer

    The amount of time (in seconds) to wait before timing out.

`version`

:   **Ruby Type:** String, Array

    The version of a package to be installed or upgraded.

#### Examples

**Install a package**

``` ruby
openbsd_package 'name of package' do
  action :install
end
```

### New client.rb Settings

The following client.rb settings are new:

`chef_gem_compile_time`

:   Controls the phase during which a gem is installed on a node. Set to
    `true` to install a gem while the resource collection is being built
    (the "compile phase"). Set to `false` to install a gem while Chef
    Client is configuring the node (the "converge phase"). Recommended
    value: `false`.

    {{< note spaces=4 >}}

    This topic is hooked into client.rb topics, starting with 12.1, in
    addition to the resource reference pages.

    {{< /note >}}

    To suppress warnings for cookbooks authored prior to chef-client
    12.1, use a `respond_to?` check to ensure backward compatibility.
    For example:

    ``` ruby
    chef_gem 'aws-sdk' do
      compile_time false if respond_to?(:compile_time)
    end
    ```

`windows_service.watchdog_timeout`

:   The maximum amount of time (in seconds) available to Chef Client run
    when Chef Client is run as a service on the Microsoft Windows
    platform. If Chef Client run does not complete within the specified
    timeframe, Chef Client run is terminated. Default value:
    `2 * (60 * 60)`.

### Multiple Packages and Versions

A resource may specify multiple packages and/or versions for platforms
that use Yum, DNF, Apt, Zypper, or Chocolatey package managers.
Specifying multiple packages and/or versions allows a single transaction
to:

-   Download the specified packages and versions via a single HTTP
    transaction
-   Update or install multiple packages with a single resource during
    Chef Client run

For example, installing multiple packages:

``` ruby
package %w(package1 package2)
```

Installing multiple packages with versions:

``` ruby
package %w(package1 package2) do
  version [ '1.3.4-2', '4.3.6-1']
end
```

Upgrading multiple packages:

``` ruby
package %w(package1 package2)  do
  action :upgrade
end
```

Removing multiple packages:

``` ruby
package %w(package1 package2)  do
  action :remove
end
```

Purging multiple packages:

``` ruby
package %w(package1 package2)  do
  action :purge
end
```

Notifications, via an implicit name:

``` ruby
package %w(package1 package2)  do
  action :nothing
end

log 'call a notification' do
  notifies :install, 'package[package1, package2]', :immediately
end
```

{{< note >}}

Notifications and subscriptions do not need to be updated when packages
and versions are added or removed from the `package_name` or `version`
properties.

{{< /note >}}

## What's New in 12.0

The following items are new for chef-client 12.0 and/or are changes from
previous versions. The short version:

-   **Changing attributes** Attributes may be modified for named
    precedence levels, all precedence levels, and be fully assigned.
    These changes were [based on
    RFC-23](https://github.com/chef/chef-rfc/blob/master/rfc023-chef-12-attributes-changes.md).
-   **Ruby 2.0 (or higher) for Windows; and Ruby 2.1 (or higher) for
    Unix/Linux** Ruby versions 1.8.7, 1.9.1, 1.9.2, and 1.9.3 are no
    longer supported. See [this blog
    post](https://www.chef.io/blog/2014/11/25/ruby-1-9-3-eol-and-chef-12/)
    for more info.
-   **The number of changes between Ruby 1.9 and 2.0 is small** Please
    review the [Ruby 2.0 release
    notes](https://github.com/ruby/ruby/blob/v2_0_0_0/NEWS) or [Ruby 2.1
    release notes](https://github.com/ruby/ruby/blob/v2_1_0/NEWS) for
    the full list of changes.
-   **provides method for building custom resources** Use the `provides`
    method to associate a custom resource with a built-in chef-client
    resource and to specify platforms on which the custom resource may
    be used.
-   **Chef Client supports the AIX platform** Chef Client may now be
    used to configure nodes that are running on the AIX platform,
    versions 6.1 (TL6 or higher, recommended) and 7.1 (TL0 SP3 or
    higher, recommended). The **service** resource supports starting,
    stopping, and restarting services that are managed by System
    Resource Controller (SRC), as well as managing all service states
    with BSD-based init systems.
-   **New bff_package resource** Use the **bff_package** resource to
    install packages on the AIX platform.
-   **New homebrew_package resource** Use the **homebrew_package**
    resource to install packages on the macOS platform. The
    **homebrew_package** resource also replaces the
    **macports_package** resource as the default package installer on
    the macOS platform.
-   **New reboot resource** Use the **reboot** resource to reboot a node
    during or at the end of a chef-client run.
-   **New windows_service resource** Use the **windows_service**
    resource to manage services on the Microsoft Windows platform.
-   **New --bootstrap-template option** Use the `--bootstrap-template`
    option to install Chef Client with a bootstrap template. Specify the
    name of a template, such as `chef-full`, or specify the path to a
    custom bootstrap template. This option deprecates the `--distro` and
    `--template-file` options.
-   **New SSL options for bootstrap operations** The `knife bootstrap`
    subcommand has new options that support SSL with bootstrap
    operations. Use the `--[no-]node-verify-api-cert` option to perform
    SSL validation of the connection to the Chef server. Use the
    `--node-ssl-verify-mode` option to validate SSL certificates.
-   **New format options for knife status** Use the `--medium` and
    `--long` options to include attributes in the output and to format
    that output as JSON.
-   **New fsck_device property for mount resource** The **mount**
    resource supports fsck devices for the Solaris platform with the
    `fsck_device` property.
-   **New settings for metadata.rb** The metadata.rb file has two new
    settings: `issues_url` and `source_url`. These settings are used to
    capture the source location and issues tracking location for a
    cookbook. These settings are also used with Chef Supermarket. In
    addition, the `name` setting is now **required**.
-   **The http_request GET and HEAD requests drop the hard-coded query
    string** The `:get` and `:head` actions appended a hard-coded query
    string---`?message=resource_name`---that cannot be overridden. This
    hard-coded string is deprecated in Chef Client 12.0 release.
    Cookbooks that rely on this string need to be updated to manually
    add it to the URL as it is passed to the resource.
-   **New Recipe DSL methods** The Recipe DSL has three new methods:
    `shell_out`, `shell_out!`, and `shell_out_with_systems_locale`.
-   **File specificity updates** File specificity for the **template**
    and **cookbook_file** resources now supports using the `source`
    attribute to define an explicit lookup path as an array.
-   **Improved user password security for the user resource, macOS
    platform** The **user** resource now supports salted password hashes
    for macOS 10.7 (and higher). Use the `iterations` and `salt`
    attributes to calculate SALTED-SHA512 password shadow hashes for
    macOS version 10.7 and SALTED-SHA512-PBKDF2 password shadow hashes
    for version 10.8 (and higher).
-   **data_bag_item method in the Recipe DSL supports encrypted data
    bag items** Use `data_bag_item(bag_name, item, secret)` to specify
    the secret to use for an encrypted data bag item. If `secret` is not
    specified, Chef Client looks for a secret at the path specified by
    the `encrypted_data_bag_secret` setting in the client.rb file.
-   **value_for_platform method in the Recipe DSL supports version
    constraints** Version constraints---`>`, `<`, `>=`, `<=`, `~>`---may
    be used when specifying a version. An exception is raised if two
    version constraints match. An exact match will always take
    precedence over a match made from a version constraint.
-   **knife cookbook site share supports --dry-run** Use the `--dry-run`
    option with the `knife cookbook site` to take no action and only
    print out results.
-   **chef-client configuration setting updates** Chef Client now
    supports running an override run-list (via the `--override-runlist`
    option) without clearing the cookbook cache on the node. In
    addition, the `--chef-zero-port` option allows specifying a range of
    ports.
-   **Unforked interval runs are no longer allowed** The `--[no-]fork`
    option may no longer be used in the same command with the
    `--daemonize` and `--interval` options.
-   **Splay and interval values are applied before Chef Client run** The
    `--interval` and `--splay` values are applied before Chef Client run
    when using Chef Client and chef-solo executables.
-   **All files and templates in a cookbook are synchronized at the
    start of Chef Client run** The `no_lazy_load` configuration setting
    in the client.rb file now defaults to `true`. This avoids issues
    where time-sensitive URLs in a cookbook manifest timeout before the
    **cookbook_file** or **template** resources converged.
-   **File staging now defaults to the destination directory by
    default** Staging into a system's temporary directory---typically
    `/tmp` or `/var/tmp`---as opposed to the destination directory may
    cause issues with permissions, available space, or cross-device
    renames. Files are now staged to the destination directory by
    default.
-   **Partial search updates** Use `:filter_result` to build search
    results into a Hash. This replaces the previous functionality that
    was provided by the `partial_search` cookbook, albeit with a
    different API. Use the `--filter-result` option to return only
    attributes that match the specified filter. For example:
    `\"ServerName=name, Kernel=kernel.version\"`.
-   **Client-side key generation is enabled by default** When a new
    chef-client is created using the validation client account, the Chef
    server allows Chef Client to generate a key-pair locally, and then
    send the public key to the Chef server. This behavior is controlled
    by the `local_key_generation` attribute in the client.rb file and
    now defaults to `true`.
-   **New guard_interpreter property defaults** The `guard_interpreter`
    property now defaults to `:batch` for the **batch** resource and
    `:powershell_script` for the **powershell_script** resource.
-   **Events are sent to the Application event log on the Windows
    platform by default** Events are sent to the Microsoft Windows
    "Application" event log at the start and end of a chef-client run,
    and also if a chef-client run fails. Set the `disable_event_logger`
    configuration setting in the client.rb file to `true` to disable
    event logging.
-   **The installer_type property for the windows_package resource
    uses a symbol instead of a string** Previous versions of Chef Client
    (starting with version 11.8) used a string.
-   **The path property is deprecated for the execute resource** Use the
    `environment` property instead.
-   **SSL certificate validation improvements** The default settings for
    SSL certificate validation now default in favor of validation. In
    addition, using the `knife ssl fetch` subcommand is now an important
    part of setting up your workstation.
-   **New property for git resource** The **git** resource has a new
    property: `environment`, which takes a Hash of environment variables
    in the form of `{"ENV_VARIABLE" => "VALUE"}`.
-   **New encrypted a version 3** Format utilizes aes-256-gcm ciphers
    for enhanced security.

Please [view the notes](/upgrade_client_notes/) for more background
on the upgrade process from chef-client 11 to chef-client 12.

### Change Attributes

Starting with chef-client 12.0, attribute precedence levels may be

-   Removed for a specific, named attribute precedence level
-   Removed for all attribute precedence levels
-   Fully assigned attributes

#### Remove Precedence Level

A specific attribute precedence level for default, normal, and override
attributes may be removed by using one of the following syntax patterns.

For default attributes:

-   `node.rm_default('foo', 'bar')`

For normal attributes:

-   `node.rm_normal('foo', 'bar')`

For override attributes:

-   `node.rm_override('foo', 'bar')`

These patterns return the computed value of the key being deleted for
the specified precedence level.

**Examples**

The following examples show how to remove a specific, named attribute
precedence level.

**Delete a default value when only default values exist**

Given the following code structure under `'foo'`:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

And some role attributes:

``` ruby
# Please don't ever do this in real code :)
node.role_default['foo']['bar']['thing'] = 'otherstuff'
```

And a force attribute:

``` ruby
node.force_default['foo']['bar']['thing'] = 'allthestuff'
```

When the default attribute precedence `node['foo']['bar']` is removed:

``` ruby
node.rm_default('foo', 'bar') #=> {'baz' => 52, 'thing' => 'allthestuff'}
```

What is left under `'foo'` is only `'bat'`:

``` ruby
node.attributes.combined_default['foo'] #=> {'bat' => { 'things' => [5,6] } }
```

**Delete default without touching higher precedence attributes**

Given the following code structure:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

And some role attributes:

``` ruby
# Please don't ever do this in real code :)
node.role_default['foo']['bar']['thing'] = 'otherstuff'
```

And a force attribute:

``` ruby
node.force_default['foo']['bar']['thing'] = 'allthestuff'
```

And also some override attributes:

``` ruby
node.override['foo']['bar']['baz'] = 99
```

Same delete as before:

``` ruby
node.rm_default('foo', 'bar') #=> { 'baz' => 52, 'thing' => 'allthestuff' }
```

The other attribute precedence levels are unaffected:

``` ruby
node.attributes.combined_override['foo'] #=> { 'bar' => {'baz' => 99} }
node['foo'] #=> { 'bar' => {'baz' => 99}, 'bat' => { 'things' => [5,6] }
```

**Delete override without touching lower precedence attributes**

Given the following code structure, which has an override attribute:

``` ruby
node.override['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

with a single default value:

``` ruby
node.default['foo']['bar']['baz'] = 11
```

and a force at each attribute precedence:

``` ruby
node.force_default['foo']['bar']['baz'] = 55
node.force_override['foo']['bar']['baz'] = 99
```

Delete the override:

``` ruby
node.rm_override('foo', 'bar') #=> { 'baz' => 99, 'thing' => 'stuff' }
```

The other attribute precedence levels are unaffected:

``` ruby
node.attributes.combined_default['foo'] #=> { 'bar' => {'baz' => 55} }
```

**Non-existent key deletes return nil**

``` ruby
node.rm_default("no", "such", "thing") #=> nil
```

#### Remove All Levels

All attribute precedence levels may be removed by using the following
syntax pattern:

-   `node.rm('foo', 'bar')`

{{< note >}}

Using `node['foo'].delete('bar')` will throw an exception that points to
the new API.

{{< /note >}}

**Examples**

The following examples show how to remove all attribute precedence
levels.

**Delete all attribute precedence levels**

Given the following code structure:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

With override attributes:

``` ruby
node.override['foo']['bar']['baz'] = 999
```

Removing the `'bar'` key returns the computed value:

``` ruby
node.rm('foo', 'bar') #=> {'baz' => 999, 'thing' => 'stuff'}
```

Looking at `'foo'`, all that's left is the `'bat'` entry:

``` ruby
node['foo'] #=> {'bat' => { 'things' => [5,6] } }
```

**Non-existent key deletes return nil**

``` ruby
node.rm_default("no", "such", "thing") #=> nil
```

#### Full Assignment

Use `!` to clear out the key for the named attribute precedence level,
and then complete the write by using one of the following syntax
patterns:

-   `node.default!['foo']['bar'] = {...}`
-   `node.force_default!['foo']['bar'] = {...}`
-   `node.normal!['foo']['bar'] = {...}`
-   `node.override!['foo']['bar'] = {...}`
-   `node.force_override!['foo']['bar'] = {...}`

**Examples**

The following examples show how to remove all attribute precedence
levels.

**Just one component**

Given the following code structure:

``` ruby
node.default['foo']['bar'] = {'a' => 'b'}
node.default!['foo']['bar'] = {'c' => 'd'}
```

The `'!'` caused the entire 'bar' key to be overwritten:

``` ruby
node['foo'] #=> {'bar' => {'c' => 'd'}
```

**Multiple components; one "after"**

Given the following code structure:

``` ruby
node.default['foo']['bar'] = {'a' => 'b'}
# Please don't ever do this in real code :)
node.role_default['foo']['bar'] = {'c' => 'd'}
node.default!['foo']['bar'] = {'d' => 'e'}
```

The `'!'` write overwrote the "cookbook-default" value of `'bar'`, but
since role data is later in the resolution list, it was unaffected:

``` ruby
node['foo'] #=> {'bar' => {'c' => 'd', 'd' => 'e'}
```

**Multiple components; all "before"**

Given the following code structure:

``` ruby
node.default['foo']['bar'] = {'a' => 'b'}
# Please don't ever do this in real code :)
node.role_default['foo']['bar'] = {'c' => 'd'}
node.force_default!['foo']['bar'] = {'d' => 'e'}
```

With `force_default!` there is no other data under `'bar'`:

``` ruby
node['foo'] #=> {'bar' => {'d' => 'e'}
```

**Multiple precedence levels**

Given the following code structure:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
   'things' => [5, 6],
  },
}
```

And some attributes:

``` ruby
# Please don't ever do this in real code :)
node.role_default['foo']['bar']['baz'] = 55
node.force_default['foo']['bar']['baz'] = 66
```

And other precedence levels:

``` ruby
node.normal['foo']['bar']['baz'] = 88
node.override['foo']['bar']['baz'] = 99
```

With a full assignment:

``` ruby
node.default!['foo']['bar'] = {}
```

Role default and force default are left in default, plus other
precedence levels:

``` ruby
node.attributes.combined_default['foo'] #=> {'bar' => {'baz' => 66}, 'bat'=>{'things'=>[5, 6]}}
node.attributes.normal['foo'] #=> {'bar' => {'baz' => 88}}
node.attributes.combined_override['foo'] #=> {'bar' => {'baz' => 99}}
node['foo']['bar'] #=> {'baz' => 99}
```

If `force_default!` is written:

``` ruby
node.force_default!['foo']['bar'] = {}
```

the difference is:

``` ruby
node.attributes.combined_default['foo'] #=> {'bat'=>{'things'=>[5, 6]}, 'bar' => {}}
node.attributes.normal['foo'] #=> {'bar' => {'baz' => 88}}
node.attributes.combined_override['foo'] #=> {'bar' => {'baz' => 99}}
node['foo']['bar'] #=> {'baz' => 99}
```

### provides Method

Use the `provides` method to map a custom resource/provider to an
existing resource/provider, and then to also specify the platform(s) on
which the behavior of the custom resource/provider will be applied. This
method enables scenarios like:

-   Building a custom resource that is based on an existing resource
-   Defining platform mapping specific to a custom resource
-   Handling situations where a resource on a particular platform may
    have more than one provider, such as the behavior on the Ubuntu
    platform where both SysVInit and systemd are present
-   Allowing the custom resource to declare what platforms are
    supported, enabling the creator of the custom resource to use
    arbitrary criteria if desired
-   Not using the previous naming
    convention---`#{cookbook_name}_#{provider_filename}`

{{< warning >}}

The `provides` method must be defined in both the custom resource and
custom provider files and both files must have identical `provides`
statement(s).

{{< /warning >}}

The syntax for the `provides` method is as follows:

``` ruby
provides :resource_name, os: [ 'platform', 'platform', ...], platform_family: 'family'
```

where:

-   `:resource_name` is a chef-client resource: `:cookbook_file`,
    `:package`, `:rpm_package`, and so on
-   `'platform'` is a comma-separated list of platforms: `'windows'`,
    `'solaris2'`, `'linux'`, and so on
-   `platform_family` is optional and may specify the same parameters as
    the `platform_family?` method in the Recipe DSL; `platform` is
    optional and also supported (and is the same as the `platform?`
    method in the Recipe DSL)

A custom resource/provider may be mapped to more than one existing
resource/provider. Multiple platform associations may be made. For
example, to completely map a custom resource/provider to an existing
custom resource/provider, only specificy the resource name:

``` ruby
provides :cookbook_file
```

The same mapping, but only for the Linux platform:

``` ruby
provides :cookbook_file, os: 'linux'
```

A similar mapping, but also for packages on the Microsoft Windows
platform:

``` ruby
provides :cookbook_file
provides :package, os: 'windows'
```

Use multiple `provides` statements to define multiple conditions: Use an
array to match any of the platforms within the array:

``` ruby
provides :cookbook_file
provides :package, os: 'windows'
provides :rpm_package, os: [ 'linux', 'aix' ]
```

Use an array to match any of the platforms within the array:

``` ruby
provides :package, os: 'solaris2', platform_family: 'solaris2' do |node|
  node[:platform_version].to_f <= 5.10
end
```

### AIX Platform Support

Chef Client may now be used to configure nodes that are running on the
AIX platform, versions 7.1 (TL5 SP2 or higher, recommended) and 7.2. The
**service** resource supports starting, stopping, and restarting
services that are managed by System Resource Controller (SRC), as well
as managing all service states with BSD-based init systems.

**System Requirements**

Chef Client has the [same system
requirements](/chef_system_requirements/#chef-infra-client) on the
AIX platform as any other platform, with the following notes:

-   Expand the file system on the AIX platform using `chfs` or by
    passing the `-X` flag to `installp` to automatically expand the
    logical partition (LPAR)
-   The EN_US (UTF-8) character set should be installed on the logical
    partition prior to installing the chef-client

**Install Chef Client on the AIX platform**

Chef Client is distributed as a Backup File Format (BFF) binary and is
installed on the AIX platform using the following command run as a root
user:

``` text
# installp -aYgd chef-12.0.0-1.powerpc.bff all
```

**Increase system process limits**

The out-of-the-box system process limits for maximum process memory size
(RSS) and number of open files are typically too low to run Chef Client
on a logical partition (LPAR). When the system process limits are too
low, Chef Client will not be able to create threads. To increase the
system process limits:

1.  Validate that the system process limits have not already been
    increased.

2.  If they have not been increased, run the following commands as a
    root user:

    ``` bash
    chsec -f /etc/security/limits -s default -a "rss=-1"
    ```

    and then:

    ``` bash
    chsec -f /etc/security/limits -s default -a "data=-1"
    ```

    and then:

    ``` bash
    chsec -f /etc/security/limits -s default -a "nofiles=50000"
    ```

    {{< note spaces=4 >}}

    The previous commands may be run against the root user, instead of
    default. For example:

    ``` bash
    chsec -f /etc/security/limits -s root_user -a "rss=-1"
    ```

    {{< /note >}}

3.  Reboot the logical partition (LPAR) to apply the updated system
    process limits.

When the system process limits are too low, an error is returned similar
to:

``` none
Error Syncing Cookbooks:
==================================================================

Unexpected Error:
-----------------
ThreadError: can't create Thread: Resource temporarily unavailable
```

**Install the UTF-8 character set**

Chef Client uses the EN_US (UTF-8) character set. By default, the AIX
base operating system does not include the EN_US (UTF-8) character set
and it must be installed prior to installing the chef-client. The EN_US
(UTF-8) character set may be installed from the first disc in the AIX
media or may be copied from `/installp/ppc/*EN_US*` to a location on the
logical partition (LPAR). This topic assumes this location to be
`/tmp/rte`.

Use `smit` to install the EN_US (UTF-8) character set. This ensures
that any workload partitions (WPARs) also have UTF-8 applied.

Remember to point `INPUT device/directory` to `/tmp/rte` when not
installing from CD.

1.  From a root shell type:

    ``` text
    # smit lang
    ```

    A screen similar to the following is returned:

    ``` bash
    Manage Language Environment

    Move cursor to desired item and press Enter.

    Change/Show Primary Language Environment
    Add Additional Language Environments
    Remove Language Environments
    Change/Show Language Hierarchy
    Set User Languages
    Change/Show Applications for a Language
    Convert System Messages and Flat Files

    F1=Help             F2=Refresh          F3=Cancel           F8=Image
    F9=Shell            F10=Exit            Enter=Do
    ```

2.  Select `Add Additional Language Environments` and press `Enter`. A
    screen similar to the following is returned:

    ``` bash
    Add Additional Language Environments

    Type or select values in entry fields.
    Press Enter AFTER making all desired changes.

                                                            [Entry Fields]
      CULTURAL convention to install                                             +
      LANGUAGE translation to install                                            +
    * INPUT device/directory for software                [/dev/cd0]              +
      EXTEND file systems if space needed?                yes                    +

      WPAR Management
          Perform Operation in Global Environment         yes                    +
          Perform Operation on Detached WPARs             no                     +
              Detached WPAR Names                        [_all_wpars]            +
          Remount Installation Device in WPARs            yes                    +
          Alternate WPAR Installation Device             []

    F1=Help             F2=Refresh          F3=Cancel           F4=List
    F5=Reset            F6=Command          F7=Edit             F8=Image
    F9=Shell            F10=Exit            Enter=Do
    ```

3.  Cursor over the first two entries---`CULTURAL convention to install`
    and `LANGUAGE translation to install`---and use `F4` to navigate
    through the list until `UTF-8 English (United States) [EN_US]` is
    selected. (EN_US is in capital letters!)

4.  Press `Enter` to apply and install the language set.

**New providers**

The **service** resource has the following providers to support the AIX
platform:

<table>
<colgroup>
<col style="width: 27%" />
<col style="width: 14%" />
<col style="width: 58%" />
</colgroup>
<thead>
<tr class="header">
<th>Long name</th>
<th>Short name</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>Chef::Provider::Service::Aix</code></td>
<td><code>service</code></td>
<td>The provider that is used with the AIX platforms. Use the <code>service</code> short name to start, stop, and restart services with System Resource Controller (SRC).</td>
</tr>
<tr class="even">
<td><code>Chef::Provider::Service::AixInit</code></td>
<td><code>service</code></td>
<td>The provider that is used to manage BSD-based init services on AIX.</td>
</tr>
</tbody>
</table>

**Enable a service on AIX using the mkitab command**

The **service** resource does not support using the `:enable` and
`:disable` actions with resources that are managed using System Resource
Controller (SRC). This is because System Resource Controller (SRC) does
not have a standard mechanism for enabling and disabling services on
system boot.

One approach for enabling or disabling services that are managed by
System Resource Controller (SRC) is to use the **execute** resource to
invoke `mkitab`, and then use that command to enable or disable the
service.

The following example shows how to install a service:

``` ruby
execute "install #{node['chef_client']['svc_name']} in SRC" do
  command "mkssys -s #{node['chef_client']['svc_name']}
                  -p #{node['chef_client']['bin']}
                  -u root
                  -S
                  -n 15
                  -f 9
                  -o #{node['chef_client']['log_dir']}/client.log
                  -e #{node['chef_client']['log_dir']}/client.log -a '
                  -i #{node['chef_client']['interval']}
                  -s #{node['chef_client']['splay']}'"
  not_if "lssrc -s #{node['chef_client']['svc_name']}"
  action :run
end
```

and then enable it using the `mkitab` command:

``` ruby
execute "enable #{node['chef_client']['svc_name']}" do
  command "mkitab '#{node['chef_client']['svc_name']}:2:once:/usr/bin/startsrc
                  -s #{node['chef_client']['svc_name']} > /dev/console 2>&1'"
  not_if "lsitab #{node['chef_client']['svc_name']}"
end
```

### Recipe DSL, Encrypted Data Bags

The Recipe DSL provides access to data bags and data bag items
(including encrypted data bag items) with the following methods:

-   `data_bag(bag)`, where `bag` is the name of the data bag.
-   `data_bag_item('bag_name', 'item', 'secret')`, where `bag` is the
    name of the data bag and `item` is the name of the data bag item. If
    `'secret'` is not specified, Chef Client will look for a secret at
    the path specified by the `encrypted_data_bag_secret` setting in the
    client.rb file.

The `data_bag` method returns an array with a key for each of the data
bag items that are found in the data bag.

Some examples:

To load the secret from a file:

``` ruby
data_bag_item('bag', 'item', IO.read('secret_file'))
```

To load a single data bag item named `admins`:

``` ruby
data_bag('admins')
```

The contents of a data bag item named `justin`:

``` ruby
data_bag_item('admins', 'justin')
```

will return something similar to:

``` ruby
# => {'comment'=>'Justin Currie', 'gid'=>1005, 'id'=>'justin', 'uid'=>1005, 'shell'=>'/bin/zsh'}
```

If `item` is encrypted, `data_bag_item` will automatically decrypt it
using the key specified above, or (if none is specified) by the
`Chef::Config[:encrypted_data_bag_secret]` method, which defaults to
`/etc/chef/encrypted_data_bag_secret`.

### bff_package

Use the **bff_package** resource to manage packages for the AIX
platform using the installp utility. When a package is installed from a
local file, it must be added to the node using the **remote_file** or
**cookbook_file** resources.

{{< note >}}

A Backup File Format (BFF) package may not have a `.bff` file extension.
Chef Client will still identify the correct provider to use based on the
platform, regardless of the file extension.

{{< /note >}}

#### Syntax

A **bff_package** resource manages a package on a node, typically by
installing it. The simplest use of the **bff_package** resource is:

``` ruby
bff_package 'package_name'
```

which will install the named package using all of the default options
and the default action (`:install`).

The full syntax for all of the properties that are available to the
**bff_package** resource is:

``` ruby
bff_package 'name' do
  options                    String
  package_name               String, Array # defaults to 'name' if not specified
  source                     String
  timeout                    String, Integer
  version                    String, Array
  action                     Symbol # defaults to :install if not specified
end
```

where:

-   `bff_package` is the resource.
-   `name` is the name given to the resource block.
-   `action` identifies which steps Chef Client will take to bring the
    node into the desired state.
-   `options`, `package_name`, `source`, `timeout`, and `version` are
    properties of this resource, with the Ruby type shown. See
    "Properties" section below for more information about all of the
    properties that may be used with this resource.

#### Actions

The bff_package resource has the following actions:

`:install`

:   Default. Install a package. If a version is specified, install the
    specified version of the package.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:purge`

:   Purge a package. This action typically removes the configuration
    files as well as the package.

`:remove`

:   Remove a package.

#### Properties

This resource has the following properties:

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`options`

:   **Ruby Type:** String

    One (or more) additional options that are passed to the command.

`package_name`

:   **Ruby Type:** String, Array

    The name of the package. Default value: the `name` of the resource
    block. See "Syntax" section above for more information.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`source`

:   **Ruby Type:** String

    Required. The path to a package in the local file system. The AIX
    platform requires `source` to be a local file system path because
    `installp` does not retrieve packages using HTTP or FTP.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`timeout`

:   **Ruby Type:** String, Integer

    The amount of time (in seconds) to wait before timing out.

`version`

:   **Ruby Type:** String, Array

    The version of a package to be installed or upgraded.

#### Providers

This resource has the following providers:

`Chef::Provider::Package`, `package`

:   When this short name is used, Chef Client will attempt to determine
    the correct provider during Chef Client run.

`Chef::Provider::Package::Aix`, `bff_package`

:   The provider for the AIX platform. Can be used with the `options`
    attribute.

#### Example

**Install a package**

The **bff_package** resource is the default package provider on the AIX
platform. The base **package** resource may be used, and then when the
platform is AIX, Chef Client will identify the correct package provider.
The following examples show how to install part of the IBM XL C/C++
compiler.

Using the base **package** resource:

``` ruby
package 'xlccmp.13.1.0' do
  source '/var/tmp/IBM_XL_C_13.1.0/usr/sys/inst.images/xlccmp.13.1.0'
  action :install
end
```

Using the **bff_package** resource:

``` ruby
bff_package 'xlccmp.13.1.0' do
  source '/var/tmp/IBM_XL_C_13.1.0/usr/sys/inst.images/xlccmp.13.1.0'
  action :install
end
```

### homebrew_package

Use the **homebrew_package** resource to manage packages for the macOS
platform.

#### Syntax

A **homebrew_package** resource block manages a package on a node,
typically by installing it. The simplest use of the
**homebrew_package** resource is:

``` ruby
homebrew_package 'package_name'
```

which will install the named package using all of the default options
and the default action (`:install`).

The full syntax for all of the properties that are available to the
**homebrew_package** resource is:

``` ruby
homebrew_package 'name' do
  homebrew_user              String, Integer
  options                    String
  package_name               String, Array # defaults to 'name' if not specified
  source                     String
  timeout                    String, Integer
  version                    String, Array
  action                     Symbol # defaults to :install if not specified
end
```

where:

-   `homebrew_package` is the resource.
-   `name` is the name given to the resource block.
-   `action` identifies which steps Chef Client will take to bring the
    node into the desired state.
-   `homebrew_user`, `options`, `package_name`, `source`, `timeout`, and
    `version` are properties of this resource, with the Ruby type shown.
    See "Properties" section below for more information about all of the
    properties that may be used with this resource.

#### Actions

The homebrew_package resource has the following actions:

`:install`

:   Default. Install a package. If a version is specified, install the
    specified version of the package.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:purge`

:   Purge a package. This action typically removes the configuration
    files as well as the package.

`:remove`

:   Remove a package.

`:upgrade`

:   Install a package and/or ensure that a package is the latest
    version.

#### Properties

This resource has the following properties:

`homebrew_user`

:   **Ruby Type:** String, Integer

    The name of the Homebrew owner to be used by Chef Client when
    executing a command.

    The chef-client, by default, will attempt to execute a Homebrew
    command as the owner of `/usr/local/bin/brew`. If that executable
    does not exist, Chef Client will attempt to find the user by
    executing `which brew`. If that executable cannot be found, Chef
    Client will print an error message:
    `Could not find the "brew" executable in /usr/local/bin or anywhere on the path.`.
    Use the `homebrew_user` attribute to specify the Homebrew owner for
    situations where Chef Client cannot automatically detect the correct
    owner.

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`options`

:   **Ruby Type:** String

    One (or more) additional options that are passed to the command.

`package_name`

:   **Ruby Type:** String, Array

    The name of the package. Default value: the `name` of the resource
    block. See "Syntax" section above for more information.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`source`

:   **Ruby Type:** String

    Optional. The path to a package in the local file system.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`timeout`

:   **Ruby Type:** String, Integer

    The amount of time (in seconds) to wait before timing out.

`version`

:   **Ruby Type:** String, Array

    The version of a package to be installed or upgraded.

#### Providers

This resource has the following providers:

`Chef::Provider::Package`, `package`

:   When this short name is used, Chef Client will attempt to determine
    the correct provider during Chef Client run.

`Chef::Provider::Package::Homebrew`, `homebrew_package`

:   The provider for the macOS platform.

#### Example

**Install a package**

``` ruby
homebrew_package 'name of package' do
  action :install
end
```

**Specify the Homebrew user with a UUID**

``` ruby
homebrew_package 'emacs' do
  homebrew_user 1001
end
```

**Specify the Homebrew user with a string**

``` ruby
homebrew_package 'vim' do
  homebrew_user 'user1'
end
```

### reboot

Use the **reboot** resource to reboot a node, a necessary step with some
installations on certain platforms. This resource is supported for use
on the Microsoft Windows, macOS, and Linux platforms.

#### Syntax

A **reboot** resource block reboots a node:

``` ruby
reboot 'app_requires_reboot' do
  action :request_reboot
  reason 'Need to reboot when the run completes successfully.'
  delay_mins 5
end
```

The full syntax for all of the properties that are available to the
**reboot** resource is:

``` ruby
reboot 'name' do
  delay_mins                 Fixnum
  notifies                   # see description
  reason                     String
  subscribes                 # see description
  action                     Symbol
end
```

where

-   `reboot` is the resource
-   `name` is the name of the resource block
-   `action` identifies the steps Chef Client will take to bring the
    node into the desired state
-   `delay_mins` and `reason` are properties of this resource, with the
    Ruby type shown. See "Properties" section below for more information
    about all of the properties that may be used with this resource.

#### Actions

The reboot resource has the following actions:

`:cancel`

:   Cancel a reboot request.

`:nothing`

:   This resource block does not act unless notified by another resource
    to take action. Once notified, this resource block either runs
    immediately or is queued up to run at the end of the Chef Client
    run.

`:reboot_now`

:   Reboot a node so that Chef Client may continue the installation
    process.

`:request_reboot`

:   Reboot a node at the end of a chef-client run.

#### Properties

This resource has the following properties:

`delay_mins`

:   **Ruby Type:** Fixnum

    The amount of time (in minutes) to delay a reboot request.

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timer is available:

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

`reason`

:   **Ruby Type:** String

    A string that describes the reboot action.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timer is available:

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

#### Example

**Reboot a node immediately**

``` ruby
reboot 'now' do
  action :nothing
  reason 'Cannot continue Chef run without a reboot.'
  delay_mins 2
end

execute 'foo' do
  command '...'
  notifies :reboot_now, 'reboot[now]', :immediately
end
```

**Reboot a node at the end of a chef-client run**

``` ruby
reboot 'app_requires_reboot' do
  action :request_reboot
  reason 'Need to reboot when the run completes successfully.'
  delay_mins 5
end
```

**Cancel a reboot**

``` ruby
reboot 'cancel_reboot_request' do
  action :cancel
  reason 'Cancel a previous end-of-run reboot request.'
end
```

### windows_service

Use the **windows_service** resource to manage a service on the
Microsoft Windows platform.

#### Syntax

A **windows_service** resource block manages the state of a service on
a machine that is running Microsoft Windows. For example:

``` ruby
windows_service 'BITS' do
  action :configure_startup
  startup_type :manual
end
```

The full syntax for all of the properties that are available to the
**windows_service** resource is:

``` ruby
windows_service 'name' do
  init_command               String
  notifies                   # see description
  pattern                    String
  provider                   Chef::Provider::Service::Windows
  reload_command             String
  restart_command            String
  run_as_password            String
  run_as_user                String
  service_name               String # defaults to 'name' if not specified
  start_command              String
  startup_type               Symbol
  status_command             String
  stop_command               String
  subscribes                 # see description
  supports                   Hash
  timeout                    Integer
  action                     Symbol # defaults to :nothing if not specified
end
```

where

-   `windows_service` is the resource
-   `name` is the name of the resource block
-   `action` identifies the steps Chef Client will take to bring the
    node into the desired state
-   `init_command`, `pattern`, `reload_command`, `restart_command`,
    `run_as_password`, `run_as_user`, `service_name`, `start_command`,
    `startup_type`, `status_command`, `stop_command`, `supports`, and
    `timeout` are properties of this resource, with the Ruby type shown.
    See "Properties" section below for more information about all of the
    properties that may be used with this resource.

#### Actions

This resource has the following actions:

`:configure_startup`

:   Configure a service based on the value of the `startup_type`
    property.

`:disable`

:   Disable a service. This action is equivalent to a `Disabled` startup
    type on the Microsoft Windows platform.

`:enable`

:   Enable a service at boot. This action is equivalent to an
    `Automatic` startup type on the Microsoft Windows platform.

`:nothing`

:   Default. Do nothing with a service.

`:reload`

:   Reload the configuration for this service.

`:restart`

:   Restart a service.

`:start`

:   Start a service, and keep it running until stopped or disabled.

`:stop`

:   Stop a service.

#### Properties

This resource has the following properties:

`ignore_failure`

:   **Ruby Type:** true, false | **Default Value:** `false`

    Continue running a recipe if a resource fails for any reason.

`init_command`

:   **Ruby Type:** String

    The path to the init script that is associated with the service.
    This is typically `/etc/init.d/SERVICE_NAME`. The `init_command`
    property can be used to prevent the need to specify overrides for
    the `start_command`, `stop_command`, and `restart_command`
    attributes.

`notifies`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may notify another resource to take action when its state
    changes. Specify a `'resource[name]'`, the `:action` that resource
    should take, and then the `:timer` for that action. A resource may
    notify more than one resource; use a `notifies` statement for each
    resource to be notified.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `notifies` is:

    ``` ruby
    notifies :action, 'resource[name]', :timer
    ```

`pattern`

:   **Ruby Type:** String

    The pattern to look for in the process table. Default value:
    `service_name`.

`reload_command`

:   **Ruby Type:** String

    The command used to tell a service to reload its configuration.

`restart_command`

:   **Ruby Type:** String

    The command used to restart a service.

`retries`

:   **Ruby Type:** Integer | **Default Value:** `0`

    The number of attempts to catch exceptions and retry the resource.

`retry_delay`

:   **Ruby Type:** Integer | **Default Value:** `2`

    The retry delay (in seconds).

`run_as_password`

:   **Ruby Type:** String

    The password for the user specified by `run_as_user`.

`run_as_user`

:   **Ruby Type:** String

    The user under which a Microsoft Windows service runs.

`service_name`

:   **Ruby Type:** String

    The name of the service. Default value: the `name` of the resource
    block. See "Syntax" section above for more information.

`start_command`

:   **Ruby Type:** String

    The command used to start a service.

`startup_type`

:   **Ruby Type:** Symbol

    Use to specify the startup type for a Microsoft Windows service.
    Possible values: `:automatic`, `:disabled`, or `:manual`. Default
    value: `:automatic`.

`status_command`

:   **Ruby Type:** String

    The command used to check the run status for a service.

`stop_command`

:   **Ruby Type:** String

    The command used to stop a service.

`subscribes`

:   **Ruby Type:** Symbol, 'Chef::Resource\[String\]'

    A resource may listen to another resource, and then take action if
    the state of the resource being listened to changes. Specify a
    `'resource[name]'`, the `:action` to be taken, and then the `:timer`
    for that action.

    Note that `subscribes` does not apply the specified action to the
    resource that it listens to - for example:

    ``` ruby
    file '/etc/nginx/ssl/example.crt' do
      mode '0600'
      owner 'root'
    end

    service 'nginx' do
      subscribes :reload, 'file[/etc/nginx/ssl/example.crt]', :immediately
    end
    ```

    In this case the `subscribes` property reloads the `nginx` service
    whenever its certificate file, located under
    `/etc/nginx/ssl/example.crt`, is updated. `subscribes` does not make
    any changes to the certificate file itself, it merely listens for a
    change to the file, and executes the `:reload` action for its
    resource (in this example `nginx`) when a change is detected.

    A timer specifies the point during Chef Client run at which a
    notification is run. The following timers are available:

    `:delayed`

    :   Default. Specifies that a notification should be queued up, and
        then executed at the very end of Chef Client run.

    `:immediate`, `:immediately`

    :   Specifies that a notification should be run immediately, per
        resource notified.

    The syntax for `subscribes` is:

    ``` ruby
    subscribes :action, 'resource[name]', :timer
    ```

`supports`

:   **Ruby Type:** Hash

    A list of properties that controls how Chef Client is to attempt to
    manage a service: `:restart`, `:reload`, `:status`. For `:restart`,
    the init script or other service provider can use a restart command;
    if `:restart` is not specified, Chef Client attempts to stop and
    then start a service. For `:reload`, the init script or other
    service provider can use a reload command. For `:status`, the init
    script or other service provider can use a status command to
    determine if the service is running; if `:status` is not specified,
    Chef Client attempts to match the `service_name` against the process
    table as a regular expression, unless a pattern is specified as a
    parameter property. Default value:
    `{ :restart => false, :reload => false, :status => false }` for all
    platforms (except for the Red Hat platform family, which defaults to
    `{ :restart => false, :reload => false, :status => true }`.)

`timeout`

:   **Ruby Type:** Integer

    The amount of time (in seconds) to wait before timing out. Default
    value: `60`.

#### Example

**Start a service manually**

``` ruby
windows_service 'BITS' do
  action :configure_startup
  startup_type :manual
end
```

### knife bootstrap Settings

The following options are new:

`--[no-]node-verify-api-cert`

:   Verify the SSL certificate on the Chef server. When `true`, Chef
    Client always verifies the SSL certificate. When `false`, Chef
    Client uses the value of `ssl_verify_mode` to determine if the SSL
    certificate requires verification. If this option is not specified,
    the setting for `verify_api_cert` in the configuration file is
    applied.

`--node-ssl-verify-mode PEER_OR_NONE`

:   Set the verify mode for HTTPS requests.

    Use `none` to do no validation of SSL certificates.

    Use `peer` to do validation of all SSL certificates, including the
    Chef server connections, S3 connections, and any HTTPS
    **remote_file** resource URLs used in Chef Client run. This is the
    recommended setting.

`-t TEMPLATE`, `--bootstrap-template TEMPLATE`

:   The bootstrap template to use. This may be the name of a bootstrap
    template---`chef-full`, for example---or it may be the full path to
    an Embedded Ruby (ERB) template that defines a custom bootstrap.
    Default value: `chef-full`, which installs Chef Client using the
    omnibus installer on all supported platforms.

    {{< note spaces=4 >}}

    The `--distro` and `--template-file` options are deprecated.

    {{< /note >}}

### knife status Settings

The following options are new:

`-l`, `--long`

:   Display all attributes in the output and show the output as JSON.

`-m`, `--medium`

:   Display normal attributes in the output and to show the output as
    JSON.

### fsck_device Property

The following property is new for the **mount** resource:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>fsck_device</code></td>
<td>The fsck device on the Solaris platform. Default value: <code>-</code>.</td>
</tr>
</tbody>
</table>

### metadata.rb Settings

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
<td><p><code>issues_url</code></p></td>
<td><p>The URL for the location in which a cookbook's issue tracking is maintained. This setting is also used by Chef Supermarket. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>source_url <span class="st">&quot;https://github.com/chef-cookbooks/chef-client/issues&quot;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>source_url</code></p></td>
<td><p>The URL for the location in which a cookbook's source code is maintained. This setting is also used by Chef Supermarket. For example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>source_url <span class="st">&quot;https://github.com/chef-cookbooks/chef-client&quot;</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

{{< warning >}}

The `name` attribute is now a required setting in the metadata.rb file.

{{< /warning >}}

### http_request Actions

The `:get` and `:head` actions appended a hard-coded query
string---`?message=resource_name`---that cannot be overridden. This
hard-coded string is deprecated in Chef Client 12.0 release. Cookbooks
that rely on this string need to be updated to manually add it to the
URL as it is passed to the resource.

### Recipe DSL

The following methods have been added to the Recipe DSL: `shell_out`,
`shell_out!`, and `shell_out_with_systems_locale`.

#### shell_out

The `shell_out` method can be used to run a command against the node,
and then display the output to the console when the log level is set to
`debug`.

The syntax for the `shell_out` method is as follows:

``` ruby
shell_out(command_args)
```

where `command_args` is the command that is run against the node.

### shell_out!

The `shell_out!` method can be used to run a command against the node,
display the output to the console when the log level is set to `debug`,
and then raise an error when the method returns `false`.

The syntax for the `shell_out!` method is as follows:

``` ruby
shell_out!(command_args)
```

where `command_args` is the command that is run against the node. This
method will return `true` or `false`.

#### shell_out_with_systems_locale

The `shell_out_with_systems_locale` method can be used to run a command
against the node (via the `shell_out` method), but using the `LC_ALL`
environment variable.

The syntax for the `shell_out_with_systems_locale` method is as follows:

``` ruby
shell_out_with_systems_locale(command_args)
```

where `command_args` is the command that is run against the node.

#### value_for_platform

The `value_for_platform` helper may use version constraints, such as
`>=` and `~>` to help resolve situations where version numbers look like
`7.0.<buildnumber>`. For example:

``` ruby
value_for_platform(
  "redhat" => {
    "~> 7.0" => "version 7.x.y"
    ">= 8.0" => "version 8.0.0 and greater"
  }
}
```

{{< note >}}

When two version constraints match it is considered ambiguous and will
raise an exception. An exact match, however, will always take precedence
over a version constraint.

{{< /note >}}

### File Specificity

The pattern for file specificity depends on two things: the lookup path
and the source attribute. The first pattern that matches is used:

1.  /host-\$fqdn/\$source
2.  /\$platform-\$platform_version/\$source
3.  /\$platform/\$source
4.  /default/\$source
5.  /\$source

Use an array with the `source` attribute to define an explicit lookup
path. For example:

``` ruby
file '/conf.py' do
  source ["#{node.chef_environment}.py", 'conf.py']
end
```

or:

``` ruby
template '/test' do
  source ["#{node.chef_environment}.erb", 'default.erb']
end
```

### macOS, Passwords

The following properties are new for the **user** resource:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>iterations</code></td>
<td>The number of iterations for a password with a SALTED-SHA512-PBKDF2 shadow hash.</td>
</tr>
<tr class="even">
<td><code>salt</code></td>
<td>The salt value for a password shadow hash. macOS version 10.7 uses SALTED-SHA512 and version 10.8 (and higher) uses SALTED-SHA512-PBKDF2 to calculate password shadow hashes.</td>
</tr>
</tbody>
</table>

**Use SALTED-SHA512 passwords**

macOS 10.7 calculates the password shadow hash using SALTED-SHA512. The
length of the shadow hash value is 68 bytes, the salt value is the first
4 bytes, with the remaining 64 being the shadow hash itself. The
following code will calculate password shadow hashes for macOS 10.7:

``` ruby
password = 'my_awesome_password'
salt = OpenSSL::Random.random_bytes(4)
encoded_password = OpenSSL::Digest::SHA512.hexdigest(salt + password)
shadow_hash = salt.unpack('H*').first + encoded_password
```

Use the calculated password shadow hash with the **user** resource:

``` ruby
user 'my_awesome_user' do
  password 'c9b3bd....d843'  # Length: 136
end
```

**Use SALTED-SHA512-PBKDF2 passwords**

macOS 10.8 (and higher) calculates the password shadow hash using
SALTED-SHA512-PBKDF2. The length of the shadow hash value is 128 bytes,
the salt value is 32 bytes, and an integer specifies the number of
iterations. The following code will calculate password shadow hashes for
macOS 10.8 (and higher):

``` ruby
password = 'my_awesome_password'
salt = OpenSSL::Random.random_bytes(32)
iterations = 25000 # Any value above 20k should be fine.

shadow_hash = OpenSSL::PKCS5::pbkdf2_hmac(
  password,
  salt,
  iterations,
  128,
  OpenSSL::Digest::SHA512.new
).unpack('H*').first
salt_value = salt.unpack('H*').first
```

Use the calculated password shadow hash with the **user** resource:

``` ruby
user 'my_awesome_user' do
  password 'cbd1a....fc843'  # Length: 256
  salt 'bd1a....fc83'        # Length: 64
  iterations 25000
end
```

### chef-client Options

The following options are updated for Chef Client executable:

`--chef-zero-port PORT`

:   The port on which chef-zero listens. If a port is not
    specified---individually or as range of ports from within the
    command---Chef Client will scan for ports between 8889-9999 and will
    pick the first port that is available. This port or port range may
    also be specified using the `chef_zero.port` setting in the
    client.rb file.

`-o RUN_LIST_ITEM`, `--override-runlist RUN_LIST_ITEM`

:   Replace the current run-list with the specified items. This option
    will not clear the list of cookbooks (and related files) that is
    cached on the node.

The following configuration settings are updated for the client.rb file
and now default to `true`:

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
<td><code>disable_event_logger</code></td>
<td>Enable or disable sending events to the Microsoft Windows "Application" event log. When <code>false</code>, events are sent to the Microsoft Windows "Application" event log at the start and end of a chef-client run, and also if a chef-client run fails. Set to <code>true</code> to disable event logging. Default value: <code>true</code>.</td>
</tr>
<tr class="even">
<td><code>no_lazy_load</code></td>
<td>Download all cookbook files and templates at the beginning of Chef Client run. Default value: <code>true</code>.</td>
</tr>
<tr class="odd">
<td><code>file_staging_uses_destdir</code></td>
<td>How file staging (via temporary files) is done. When <code>true</code>, temporary files are created in the directory in which files will reside. When <code>false</code>, temporary files are created under <code>ENV['TMP']</code>. Default value: <code>true</code>.</td>
</tr>
<tr class="even">
<td><code>local_key_generation</code></td>
<td>Use to specify whether the Chef server or chef-client will generate the private/public key pair. When <code>true</code>, Chef Client will generate the key pair, and then send the public key to the Chef server. Default value: <code>true</code>.</td>
</tr>
</tbody>
</table>

### Filter Search Results

Use `:filter_result` as part of a search query to filter the search
output based on the pattern specified by a Hash. Only attributes in the
Hash will be returned.

{{< note >}}

Prior to chef-client 12.0, this functionality was available from the
`partial_search` cookbook and was referred to as "partial search".

{{< /note >}}

The syntax for the `search` method that uses `:filter_result` is as
follows:

``` ruby
search(:index, 'query',
  :filter_result => { 'foo' => [ 'abc' ],
                      'bar' => [ '123' ],
                      'baz' => [ 'sea', 'power' ]
                    }
      ).each do |result|
  puts result['foo']
  puts result['bar']
  puts result['baz']
end
```

where:

-   `:index` is of name of the index on the Chef server against which
    the search query will run: `:client`, `:data_bag_name`,
    `:environment`, `:node`, and `:role`
-   `'query'` is a valid search query against an object on the Chef
    server
-   `:filter_result` defines a Hash of values to be returned

For example:

``` ruby
search(:node, 'role:web',
  :filter_result => { 'name' => [ 'name' ],
                      'ip' => [ 'ipaddress' ],
                      'kernel_version' => [ 'kernel', 'version' ]
                    }
      ).each do |result|
  puts result['name']
  puts result['ip']
  puts result['kernel_version']
end
```

#### knife search

The `knife search` subcommand allows filtering search results with a new
option:

`-f FILTER`, `--filter-result FILTER`

:   Use to return only attributes that match the specified `FILTER`. For
    example: `\"ServerName=name, Kernel=kernel.version\"`.

### **execute** Resource, `path` Property

The `path` property has been deprecated and will throw an exception in
Chef Client 12 or later. We recommend you use the `environment` property
instead.

### **git** Property

The following property is new for the **git** resource:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>environment</code></p></td>
<td><p>A Hash of environment variables in the form of <code>({"ENV_VARIABLE" =&gt; "VALUE"})</code>. (These variables must exist for a command to be run successfully.)</p>
{{< note >}}
<p>The <strong>git</strong> provider automatically sets the <code>ENV['HOME']</code> and <code>ENV['GIT_SSH']</code> environment variables. To override this behavior and provide different values, add <code>ENV['HOME']</code> and/or <code>ENV['GIT_SSH']</code> to the <code>environment</code> Hash.</p>
{{< /note >}}</td>
</tr>
</tbody>
</table>

### Chef::Provider, Custom Resources

If a custom resource was created in the `/libraries` directory of a
cookbook that also uses a core resource from Chef Client within the
custom resource, the base class that is associated with that custom
resource must be updated. In previous versions of the chef-client, the
`Chef::Provider` class was all that was necessary because the Recipe DSL
was included in the `Chef::Provider` base class.

For example, the `lvm_logical_volume` custom resource from the [lvm
cookbook](https://github.com/chef-cookbooks/lvm/blob/master/libraries/provider_lvm_logical_volume.rb)
uses the **directory** and **mount** resources:

``` ruby
class Chef
  class Provider
    class LvmLogicalVolume < Chef::Provider
      include Chef::Mixin::ShellOut

      ...
      if new_resource.mount_point
        if new_resource.mount_point.is_a?(String)
          mount_spec = { :location => new_resource.mount_point }
        else
          mount_spec = new_resource.mount_point
        end

        dir_resource = directory mount_spec[:location] do
          mode '0755'
          owner 'root'
          group 'root'
          recursive true
          action :nothing
          not_if { Pathname.new(mount_spec[:location]).mountpoint? }
        end
        dir_resource.run_action(:create)
        updates << dir_resource.updated?

        mount_resource = mount mount_spec[:location] do
          options mount_spec[:options]
          dump mount_spec[:dump]
          pass mount_spec[:pass]
          device device_name
          fstype fs_type
          action :nothing
        end
        mount_resource.run_action(:mount)
        mount_resource.run_action(:enable)
        updates << mount_resource.updated?
      end
      new_resource.updated_by_last_action(updates.any?)
    end
```

Starting with chef-client 12, the Recipe DSL is removed from the
`Chef::Provider` base class and is only available by using `LWRPBase`.
Cookbooks that contain custom resources authored for Chef Client 11
version should be inspected and updated.

Cookbooks that contain custom resources in the `/libraries` directory of
a cookbook should:

-   Be inspected for instances of a) the `Chef::Provider` base class,
    and then b) for the presence of any core resources from the
    chef-client
-   Be updated to use the `LWRPBase` base class

For example:

``` ruby
class Chef
  class Provider
    class LvmLogicalVolume < Chef::Provider::LWRPBase
      include Chef::Mixin::ShellOut

      ...
      if new_resource.mount_point
        if new_resource.mount_point.is_a?(String)
          mount_spec = { :location => new_resource.mount_point }
        else
          mount_spec = new_resource.mount_point
        end

        dir_resource = directory mount_spec[:location] do
          mode '0755'
          owner 'root'
          group 'root'
          recursive true
          action :nothing
          not_if { Pathname.new(mount_spec[:location]).mountpoint? }
        end
        dir_resource.run_action(:create)
        updates << dir_resource.updated?

        mount_resource = mount mount_spec[:location] do
          options mount_spec[:options]
          dump mount_spec[:dump]
          pass mount_spec[:pass]
          device device_name
          fstype fs_type
          action :nothing
        end
        mount_resource.run_action(:mount)
        mount_resource.run_action(:enable)
        updates << mount_resource.updated?
      end
      new_resource.updated_by_last_action(updates.any?)
    end
```

### SSL Certificates

Chef server 12 enables SSL verification by default for all requests made
to the server, such as those made by knife and the chef-client. The
certificate that is generated during the installation of the Chef server
is self-signed, which means the certificate is not signed by a trusted
certificate authority (CA) that ships with the chef-client. The
certificate generated by the Chef server must be downloaded to any
machine from which knife and/or Chef Client will make requests to the
Chef server.

For example, without downloading the SSL certificate, the following
knife command:

``` bash
knife client list
```

responds with an error similar to:

``` bash
ERROR: SSL Validation failure connecting to host: chef-server.example.com ...
ERROR: OpenSSL::SSL::SSLError: SSL_connect returned=1 errno=0 state=SSLv3 ...
```

This is by design and will occur until a verifiable certificate is added
to the machine from which the request is sent.

See [SSL Certificates](/chef_client_security/#ssl-certificates) for
more information about how knife and Chef Client use SSL certificates
generated by the Chef server.

### Encrypted Databag Version 3

Chef 12.0 includes a new version 3.0 encrypted databag format using the
aes-256-gcm cipher for enhanced security. The default version remains
1.0 for compatibility with chef-client version 11.0. The new version can
be enabled in environments running Chef 12.0 by setting
`data_bag_encrypt_version 3` in the `client.rb` / `config.rb` files.

## Changelog

<https://github.com/chef/chef/blob/master/CHANGELOG.md>
