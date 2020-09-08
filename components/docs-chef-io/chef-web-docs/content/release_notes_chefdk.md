+++
title = "Release Notes: ChefDK 0.19 - 4.10"
draft = false

aliases = ["/release_notes_chefdk.html"]
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/release_notes_chefdk.md)

This page documents the ChefDK major changes for each release. For
a detailed list of changes, see the [ChefDK Changelog on
GitHub](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md)

## What's New In 4.10

### Updates Components

#### Chef Infra Client

Chef Infra Client has been updated from 15.12.2 to 15.3.8. This new release includes a new deprecation warning when resources specify `resource_name` without also specifying `provides` which results in failures on Chef Infra Client 16.2 and later. This release also improves the warning message that occurs when a cookbook includes a resource that is now bundled directly in Chef Infra Client.

#### Chef InSpec

Chef InSpec has been updated from 4.21.3 to 4.22.1:

- The `=` character is now allowed for command line inputs
- `apt-cdrom` repositories are now skipped when parsing out the list of apt repositories
- Faulty profiles are now reported instead of causing a crash
- Improved macOS support in the `service` resource

#### cookbook-omnifetch

cookbook-omnifetch has been updated from 0.9.1 to 0.10.0. This release adds support for Chef Server API v2 so we can support segmentless cookbooks with the Chef CLI and Policyfiles.

#### knife-cloud

knife-cloud has been updated from 4.0.0 to 4.0.2. This release properly supports jump hosts when the bootstrap flags are used.

#### knife-google

knife-google has been updated from 5.0.0 to 5.0.5. This release adds support for all the aliases for new OSes and distributions and removes several EOL distributions.

#### knife-opc

kitchen-opc has been updated from 0.4.4 to 0.4.6. This release fixes several errors that could occur when running `knife opc user edit USERNAME`.

#### kitchen-dokken

kitchen-dokken has been updated from 2.9.0 to 2.10.0. This release adds a `memory_limit` config to set memory usage limits on the container. It also fixes the `multiple_converge` and `enforce_idempotency` configurations so they work correctly.

#### kitchen-ec2

kitchen-ec2 has been updated from 3.7.0 to 3.7.1. This release fixes the default search for CentOS machines to use the official CentOS images and adds support for subnet filtering with spot instances.

### Bug Fixes

- Support for legacy DSA host keys has been restored in `knife ssh` and `knife bootstrap` commands.

## What's New In 4.9

### Updated Components

#### Chef Infra Client

The Chef Infra Client has been updated from 15.11 to 15.12. This release includes a large number of backported improvements to resources from our Chef Infra Client 16 releases. See the [release notes](https://docs.chef.io/release_notes/#whats-new-in-1512) for a complete list of what's new.

#### InSpec

InSpec was updated from 4.19 to 4.21. This new release includes the following improvements:

* Certain substrings within a .toml file no longer cause unexpected crashes.
* Accurate InSpec CLI input parsing for numeric values and structured data, which were previously treated as strings. Numeric values are cast to an integer or float and YAML or JSON structures are converted to a hash or an array.
* Suppress deprecation warnings on `inspec exec` with the `--silence-deprecations` option.

#### knife bootstrap

The `knife bootstrap` command has been updated with several fixes and improvements

- knife bootstrap will now warn when bootstrapping a system using a validation key. Users should instead use validatorless bootstrapping with `knife bootstrap` which generates node and client keys using the client key of the user bootstrapping the node. This method is far more secure as an organization-wide validation key does not need to be distributed or rotated. Users can switch to validatorless bootstrapping by removing any `validation_key` entries in their config.rb (knife.rb) file.
- Resolved an error bootstrapping Linux nodes from Windows hosts
- Improved information messages during the bootstrap process
- Bootstrapping will now be done using a single SSH connection improving bootstrap times on high latency network connection.

#### Knife Tidy

Knife Tidy has been updated from 2.0.12 to 2.1.0 which adds support for a `--keep-versions` command line flag. Specifying this keeps a minimum number of versions of each cookbook and defaults to `0`.

#### net-ssh

The `net-ssh` gem which powers `knife ssh` and `knife bootstrap` commands has been updated from 5.2.0 to 6.1.0 which includes the following updates:

- Support empty lines and comments in known_hosts.
- Add sha2-{256,512}-etm@openssh.com MAC algorithms.
- curve25519-sha256 support.

#### kitchen-ec2

The Test Kitchen driver kitchen-ec2 has been updated from 3.6.0 to 3.7.0. This new release improves how instances and volumes are tagged to ensure that these are tagged at create time. This resolves failures on AWS accounts that enforced tagging rules on all objects.

#### kitchen-inspec

The Test Kitchen verifier kitchen-inspec has been updated from 1.3.2 to 2.0.0. This new release adds a config option `load_plugins` which can be used to load all InSpec plugins during the Test Kitchen verify phase.

Sample kitchen.yml config:

```yaml
    verifier:
      name: inspec
      load_plugins: true
```

## What's New In 4.8

### New Platforms

ChefDK packages are now created for Ubuntu 20.04 and Debian 10! Additionally, we have increased package validation for our Windows 10 packages to ensure compatibility. See the [ChefDK Downloads Page](https://downloads.chef.io/chefdk) for a complete list of platforms.

### macOS Binary Signing

Each binary in the macOS ChefDK installation is now signed to improve the integrity of the installation and ensure compatibility with macOS Catalina security requirements.

### Updated Components

#### Chef Infra Client 15.11

Chef Infra Client has updated from 15.7 to 15.11, which includes improvements to resources, additional cookbook helpers, and critical bug fixes for bootstrapping nodes using `knife bootstrap` and SSHing to nodes with `ed25519` keys from Windows hosts. For a complete list of changes, see the [Chef Infra Client 15.11 release notes](https://docs.chef.io/release_notes/#whats-new-in-1511).

#### Chef InSpec 4.19

Chef InSpec has updated from 4.18.51 to 4.19.0. This update includes a large number of fixes to resources and these significant new features:

- You can now develop your own Chef InSpec Reporter plugin and determine how Chef InSpec will report result data. Learn more about Chef InSpec [plugins and implementation](/inspec/plugins/) in our documentation
- The `inspec archive` command packs your profile into a tar.gz file that includes the profile in JSON form as the `inspec.json` file. Use this JSON file to programmatically examine the profile without needing to load it into Chef InSpec
- Chef InSpec accepts a variety of date formats in the `waivers.yaml` configuration file, rather than only the `YYYY-MM-DD` format
- Use the new `inspec` command options to control the size of reports:
    - `--reporter-message-truncation` sets a length limit for the `message` field in test failure report data
    - `--reporter-backtrace-inclusion` determines if Ruby backtraces should be included in test failure report data
- Implemented VMware and Hyper-V detection on Linux systems
- Implemented VMware, Hyper-V, Virtualbox, KVM, and Xen detection on Windows systems
- Added helpers `virtual_system?` and `physical_system?`

#### Cookstyle 5.23

Cookstyle has upgraded from 5.20 to 5.23, which includes 8 new cops, and significant improvements to the detection and autocorrect capabilities in existing cops.

**New Cops**

- ChefModernize/NodeInitPackage
- ChefDeprecations/WindowsFeatureServermanagercmd
- ChefModernize/WindowsRegistryUAC
- ChefModernize/UseRequireRelative
- ChefStyle/UnnecessaryOSCheck
- ChefModernize/SimplifyAptPpaSetup
- ChefRedundantCode/StringPropertyWithNilDefault
- ChefRedundantCode/PropertySplatRegex

**Note**: Chef Workstation ships with Cookstyle 6.x, which includes a significantly improved RuboCop engine, and 24 additional cops for resolving deprecations and preparing cookbooks for Chef Infra Client 16. Cookstyle 5.x does not include Chef Infra Client 16 preparation cops.

#### Test Kitchen

Test Kitchen itself has updated from 2.3.4 to 2.5.0 with several significant improvements to the provisioners and verifiers:

- The CHEF_LICENSE env var is now automatically exported from the workstation to the instance running in Test Kitchen. Thanks [@Xorima](https://github.com/xorima)
- All local Workstation env vars are now passed to the instance running in Test Kitchen with the TKENV_ prefix. Thanks [@Xorima](https://github.com/xorima)
- Test Kitchen now includes support for Ohai plugins stored in the `ohai` directory of cookbooks. Thanks [@SAPDanJoe](https://github.com/SAPDanJoe)
- Failures using the PowerShell provisioner have been resolved. Thanks[@alanghartJC](https://github.com/alanghartJC)
- You can now download content from your test instance to you workstation using `downloads` config option in `verify`. Thanks [@smurawski](https://github.com/smurawski)

**Kitchen AzureRM**

The Kitchen AzureRM driver has updated from 0.15.1 to 1.0. This release fixes several failures from running the Kitchen Azurerm driver. It also includes support for Azure Marketplace plans and Managed Service Identity (MSI). Thanks [@jasonwbarnett](https://github.com/jasonwbarnett), [@zanecodes](https://github.com/zanecodes), [@albertvaka](https://github.com/albertvaka), and [@KSerrania](https://github.com/KSerrania) for these improvements.

**Kitchen Hyper-V**

The Kitchen Hyper-V driver has updated from 0.5.3 to 0.5.4, which resolves failures from getting the default VM Switch if there were spaces in the name. Thanks [@kdoores](http://github.com/kdoores) for this improvement.

**Kitchen DigitalOcean**

The Kitchen DigitalOcean driver has updated from 0.10.5 to 0.11.0. This release adds slugs for Ubuntu 20.04 / RHEL 8 / Fedora 31 support, increases the the default instance memory size to 1GB, and adds support for VPCs. Thanks [@zmaupin](https://github.com/zmaupin), [@tolland](https://github.com/tolland), and [@gregf](https://github.com/gregf) for these improvements.

**Kitchen EC2**

The Kitchen EC2 driver has updated from 3.3 to 3.6. This release lets the driver cleanly exit if the test instance was destroyed outside of the Test Kitchen run, either by automation or in the console. Test Kitchen will also now select the subnet with the most available IPs to better distribute systems across multiple Availability Zones. Thanks [@bdwyertech](http://github.com/bdwyertech) and [@kamaradclimber](http://github.com/kamaradclimber) for these improvements.

**Kitchen InSpec**

The Kitchen InSpec verifier has updated to allow setting Chef InSpec plugins for use during the verification. This new functionality can be enabled by adding `load_plugins: true` to your InSpec verifier config. Thanks [@tecracer-theinen](https://github.com/tecracer-theinen) for this improvement.

**Kitchen Dokken**

The Kitchen Dokken driver has updated from 2.8.1 to 2.9.0. This release adds a new provisioning configuration, `clean_dokken_sandbox`, that does not require cleaning the Chef Infra and Test Kitchen files between converges. This configuration will speed up repeatedly converging systems. This defaults to `true` which maintains the existing behavior. Thanks [@chrisUsick](https://github.com/chrisUsick).

#### Knife Plugins

**Knife Tidy**

Knife Tidy has updated from 2.0.9 to 2.0.12, which provides compatibility with Chef Infra Client 15 and improves error handling in JSON parsing.

**Knife Azure**

Knife Azure has updated from 2.0.6 to 3.0.0, which includes significant performance enhancements.

**Knife EC2**

Knife EC2 has updated from 1.0.28 to 2.0. This update resolves several errors bootstrapping nodes and avoids attempting to bootstrap nodes using private DNS which may not be accessible from the node running the bootstrap command.

**Knife Spork**

Knife Spork has updated from 1.7.2 to 1.7.3. This release adds a new `--fail-if-frozen` flag to `knife spork check` to only fail when local version matches a frozen version and allows the git plugin to push to the current branch. Thanks to [@shoekstra](https://github.com/shoekstra) and [@zmaupin](https://github.com/zmaupin) for these improvements.

**Knife Windows**

Knife Windows has updated from 3.0.6 to 4.0.2. This update includes significant performance improvements, fixes for errors when using the concurrency flag, and better indication that the legacy bootstrap commands have been replaced.

#### Security Updates

**Git**

Git has updated from 2.24.1 to 2.26.2 to resolve the following CVEs:
  - [CVE-2020-5260](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-5260/): Heap exposure vulnerability in the socket library
  - [CVE-2020-11008](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-11008/): Heap exposure vulnerability in the socket library

**Ruby**

Ruby has updated from 2.6.5 to 2.6.6 to resolve the following CVEs:

  - [CVE-2020-16255](https://www.ruby-lang.org/en/news/2020/03/19/json-dos-cve-2020-10663/): Unsafe Object Creation Vulnerability in JSON (Additional fix)
  - [CVE-2020-10933](https://www.ruby-lang.org/en/news/2020/03/31/heap-exposure-in-socket-cve-2020-10933/): Heap exposure vulnerability in the socket library

**libarchive**

libarchive has updated from 3.4.0 to 3.4.2 to resolve multiple security vulnerabilities including the following CVEs:

  - [CVE-2019-19221](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-19221): archive_wstring_append_from_mbs in archive_string.c has an out-of-bounds read because of an incorrect mbrtowc or mbtowc call
  - [CVE-2020-9308](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-9308): archive_read_support_format_rar5.c in libarchive before 3.4.2 attempts to unpack a RAR5 file with an invalid or corrupted header

**OpenSSL**

openSSL has updated from 1.0.2u to 1.0.2v, which does not address any particular CVEs, but includes multiple security hardening updates.

**Rake**

Rake has updated to 13.0.1 to resolve [CVE-2020-8130](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-8130).

## What's New In 4.7

### Updated Components

#### Berkshelf

Berkshelf has been updated from 7.0.8 to 7.0.9, which resolves errors when running `berks verify` and when using the ``--skip-syntax-check`` flag.

#### Chef Infra Client

Chef Infra Client has been updated from 15.5 to 15.7 which includes improvements to the `apt_repository`, `archive_file`, `cron`, `cron_d`, `file`, `launchd`, `sudo`, `user`, `windows_task`, `x509_certificate`, and `yum_repository` resources as well as the usual collection of bug fixes and bootstrap improvements.

#### Chef InSpec

Chef InSpec has been updated from 4.18.39 to 4.18.51 with the following improvements:

  - Example groups can now use InSpec resources
  - The user resource can now check the last login date on Windows
  - Improved the fetchers to fail consistently

#### Cookstyle

Cookstyle has been updated from 5.13 to 5.20 with 30 new cops, improvements to existing cops, a new TargetChefVersion config option, and expanded cop departments.

**TargetChefVersion Config**

Cookstyle now includes a new top-level configuration option `TargetChefVersion`. This new configuration option works similarly to RuboCop's `TargetRubyVersion` config option and allows you to specify a Chef Infra version that you want to target in your Cookstyle analysis. This prevents Cookstyle from autocorrecting cookbook code in a way that would make your cookbook incompatible with your desired Chef Infra Client version. It also makes it easier to perform staged upgrades of the Chef Infra Client by allowing you to step the `TargetChefVersion` one major version at a time.

Example .rubocop.yml config specifying a TargetChefVersion of 14.0:

```yaml
AllCops:
  TargetChefVersion: 14.0
```

**New ChefSharing and ChefRedundantCode Departments**

Cookstyle now includes two new Chef cop departments with a large number of existing cops moved into these more appropriate departments. Our goal is to have clearly defined cop departments that can be enabled or disabled to detect particular conditions in your cookbooks. Cops in the new ChefSharing department are focused around sharing cookbooks internally or on the public Supermarket. This includes things like ensuring proper license strings and complete metadata. Cops in the ChefRedundantCode category detect and correct unnecessary cookbook code. Anything detected by ChefRedundantCode cops can be removed regardless of the Chef Infra Client release you run in your infrastructure, so these are always safe to run.

With the addition of these new departments, we've moved many cops out of the ChefCorrectness department. Going forward only cops that detect code that may fail a Chef Infra Client run or cause it to behave incorrectly will be included in this category. We hope that ChefCorrectness along with ChefDeprecations are used in most cookbook CI pipelines.

#### kitchen-azurerm

kitchen-azurerm has been updated from 0.14.9 to 0.15.1 with the following improvements:

- Enable the WinRM HTTP listener by default. Thanks [@sean-nixon](https//github.com/sean-nixon)
- Allow overriding of the `subscription_id` by setting the `AZURE_SUBSCRIPTION_ID` ENV variable.
- Add a new `nic_name` config. Thanks [@libertymutual](https//github.com/libertymutual)
- Support for creating VM with Azure KeyVault certificate. Thanks [@javgallegos](https//github.com/javgallegos)

#### kitchen-dokken

kitchen-dokken has been updated to 2.8.1 which fixes a bug that prevented ENV vars from being passed into containers.

#### kitchen-google and knife-google

kitchen-google and knife-google plugins have been updated to allow the updated google-api-client SDK v0.35.

#### knife-ec2

knife-ec2 has been updated from 1.0.17 to 1.0.28 with the following fixes:

- Resolved a missing credential error when using aws-profile.
- Mask AWS access keys data in any error or debug logs.
- Resolved ssh_gateway uninitialised error.
- Fixed invalid format of auto generated keypair file name.
- Raises an error if password length is less than 8 characters on Windows and will stop warning on passwords over 14 characters.

#### knife-tidy

knife-tidy has been updated from 2.0.1 to 2.0.6 to resolve issues if an org was named `cookbooks` and to improve error messages.

#### mixlib-install

mixlib-install has been updated from 3.11.21 to 3.11.24 and will now properly identify Windows 2019 hosts.

#### chef-vault

The chef-vault gem has been updated to 4.0.1. This release includes bug fixes from [@MarkGibbons](https://github.com/MarkGibbons) and [@jeremy-clerc](https://github.com/jeremy-clerc) as well as a new way to update existing keys to sparse-mode by running `knife vault update --keys_mode sparse` thanks to [@jeunito](https://github.com/jeunito).

#### kitchen-ec2

kitchen-ec2 has been updated to 3.3.0. This new version improves how we search for security groups by tags, improves the logic that detects usage of the chef Test Kitchen provisioner, and improves security group and spot instance logic. Thanks [@slapvanilla](https://github.com/slapvanilla) and [@bdwyertech](https://github.com/bdwyertech) for these enhancements.

### Smaller Size

We continue to optimize the size of the ChefDK package with this release taking up 12% less space on disk and containing 7,000 fewer files.

### Platform Support

ChefDK packages are no longer produced for Windows 2008 R2 as this release reached its end of life on January 14th, 2020.

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2u to resolve [CVE-2019-1551](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1551)

#### Git

The embedded git client has been updated to 2.24.1 to resolve the following CVEs:

- [CVE-2019-1348](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1348)
- [CVE-2019-1349](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1349)
- [CVE-2019-1350](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1350)
- [CVE-2019-1351](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1351)
- [CVE-2019-1352](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1352)
- [CVE-2019-1353](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1353)
- [CVE-2019-1354](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1354)
- [CVE-2019-1387](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1387)
- [CVE-2019-19604](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-19604)

## What's New in 4.7

### Updated Components

#### Berkshelf

Berkshelf has been updated from 7.0.8 to 7.0.9 which resolves errors when running `berks verify` and when using the `--skip-syntax-check` flag.

#### Chef Infra Client

Chef Infra Client has been updated from 15.5 to 15.7 which includes improvements to the `apt_repository`, `archive_file`, `cron`, `cron_d`, `file`, `launchd`, `sudo`, `user`, `windows_task`, `x509_certificate`, and `yum_repository` resources as well as the usual collection of bug fixes and bootstrap improvements.

#### Chef InSpec

Chef InSpec has been updated from 4.18.39 to 4.18.51 with the following improvements:

* Example groups can now use InSpec resources
* The user resource can now check the last login date on Windows
* Improved the fetchers to fail consistently

#### Cookstyle

Cookstyle has been updated from 5.13 to 5.20 with 30 new cops, improvements to existing cops, a new TargetChefVersion config option, and expanded cop departments.

**TargetChefVersion Config**

Cookstyle now includes a new top-level configuration option `TargetChefVersion`. This new configuration option works similarly to RuboCop's `TargetRubyVersion` config option and allows you to specify a Chef Infra version that you want to target in your Cookstyle analysis. This prevents Cookstyle from autocorrecting cookbook code in a way that would make your cookbook incompatible with your desired Chef Infra Client version. It also makes it easier to perform staged upgrades of the Chef Infra Client by allowing you to step the `TargetChefVersion` one major version at a time.

Example .rubocop.yml config specifying a TargetChefVersion of 14.0:

```
AllCops:
  TargetChefVersion: 14.0
```

**New ChefSharing and ChefRedundantCode Departments**

Cookstyle now includes two new Chef cop departments with a large number of existing cops moved into these more appropriate departments. Our goal is to have clearly defined cop departments that can be enabled or disabled to detect particular conditions in your cookbooks. Cops in the new ChefSharing department are focused around sharing cookbooks internally or on the public Supermarket. This includes things like ensuring proper license strings and complete metadata. Cops in the ChefRedundantCode category detect and correct unnecessary cookbook code. Anything detected by ChefRedundantCode cops can be removed regardless of the Chef Infra Client release you run in your infrastructure, so these are always safe to run.

With the addition of these new departments, we've moved many cops out of the ChefCorrectness department. Going forward only cops that detect code that may fail a Chef Infra Client run or cause it to behave incorrectly will be included in this category. We hope that ChefCorrectness along with ChefDeprecations are used in most cookbook CI pipelines.

#### kitchen-azurerm

kitchen-azurerm has been updated from 0.14.9 to 0.15.1 with the following improvements:

* Enable the WinRM HTTP listener by default. Thanks @sean-nixon
* Allow overriding of the `subscription_id` by setting the `AZURE_SUBSCRIPTION_ID` ENV variable.
* Add a new `nic_name` config. Thanks @libertymutual
* Support for creating VM with Azure KeyVault certificate. Thanks @javgallegos

#### kitchen-dokken

kitchen-dokken has been updated to 2.8.1 which fixes a bug that prevented ENV vars from being passed into containers.

#### kitchen-google and knife-google

kitchen-google and knife-google plugins have been updated to allow the updated google-api-client SDK v0.35.

#### knife-ec2

knife-ec2 has been updated from 1.0.17 to 1.0.28 with the following fixes:

* Resolved a missing credential error when using aws-profile.
* Mask AWS access keys data in any error or debug logs.
* Resolved ssh_gateway uninitialised error.
* Fixed invalid format of auto generated keypair file name.
* Raises an error if password length is less than 8 characters on Windows and will stop warning on passwords over 14 characters.

#### knife-tidy

knife-tidy has been updated from 2.0.1 to 2.0.6 to resolve issues if an org was named `cookbooks` and to improve error messages.

#### mixlib-install

mixlib-install has been updated from 3.11.21 to 3.11.24 and will now properly identify Windows 2019 hosts.

#### chef-vault

The chef-vault gem has been updated to 4.0.1. This release includes bug fixes from [@MarkGibbons](https://github.com/MarkGibbons) and [@jeremy-clerc](https://github.com/jeremy-clerc) as well as a new way to update existing keys to sparse-mode by running `knife vault update --keys_mode sparse` thanks to [@jeunito](https://github.com/jeunito).

#### kitchen-ec2

kitchen-ec2 has been updated to 3.3.0. This new version improves how we search for security groups by tags, improves the logic that detects usage of the chef Test Kitchen provisioner, and improves security group and spot instance logic. Thanks [@slapvanilla](https://github.com/slapvanilla) and [@bdwyertech](https://github.com/bdwyertech) for these enhancements.

### Smaller Size

We continue to optimize the size of the ChefDK package with this release taking up 12% less space on disk and containing 7,000 fewer files.

### Platform Support

ChefDK packages are no longer produced for Windows 2008 R2 as this release reached its end of life on January 14th, 2020.

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2u to resolve [CVE-2019-1551 ](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1551)

#### Git

The embedded git client has been updated to 2.24.1 to resolve the following CVEs:

* [CVE-2019-1348](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1348)
* [CVE-2019-1349](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1349)
* [CVE-2019-1350](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1350)
* [CVE-2019-1351](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1351)
* [CVE-2019-1352](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1352)
* [CVE-2019-1353](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1353)
* [CVE-2019-1354](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1354)
* [CVE-2019-1387](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1387)
* [CVE-2019-19604](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-19604)

## What's New in 4.6

### Updated Components

#### Chef Infra Client

The Chef Infra Client has been updated from 15.4.45 to 15.5.17 with
updated helpers, Chefignore improvements, and a new chef_sleep
resource:

**New Cookbook Helpers**

Chef Infra Client now includes a new `chef-utils` gem which ships with a
large number of helpers to make writing cookbooks easier. Many of these
helpers existed previously in the `chef-sugar` gem. We have renamed many
of the named helpers for consistency while providing backwards
compatibility with existing `chef-sugar` names. Existing cookbooks
written with `chef-sugar` should work unmodified with any of these new
helpers. Expect a Cookstyle rule in the near future to help you update
existing `chef-sugar` code to use the newer built-in helpers.

For more information on all of the new helpers available, see the
[chef-utils
readme](https://github.com/chef/chef/blob/master/chef-utils/README.md).

**Chefignore Improvements**

We've reworked how chefignore files are handled in `knife` which has
allowed us to close out a large number of long outstanding bugs. `knife`
will now traverse all the way up the directory structure looking for a
chefignore file. This means you can place a chefignore file in each
cookbook or any parent directory in your repository structure.
Additionally, we have made fixes that ensure that commands like
`knife diff` and `knife cookbook upload` always honor your chefignore
files.

**chef_sleep Resource**

The new `chef_sleep` resource can be used to sleep for a specified
number of seconds during a Chef Infra Client run. This may be helpful to
use with other commands that return a completed status before they are
actually ready. In general, do not use this resource unless you truly
need it.

Using with a Windows service that starts, but is not immediately ready:

> ``` ruby
> service 'Service that is slow to start and reports as started' do
>   service_name 'my_database'
>   action :start
>   notifies :sleep, chef_sleep['wait for service start']
> end
>
> chef_sleep 'wait for service start' do
>   seconds 30
>   action :nothing
> end
> ```

#### Cookstyle

The Cookstyle cookbook linter has been updated from 5.9 to 5.13 and
includes 28 new Chef cops for detecting deprecated and outdated cookbook
code. This release also updates the underlying RuboCop engine used by
Cookstyle which includes a large number of bug fixes that better detect
violations and prevent false positives. See the [Cookstyle Release
Notes](https://github.com/chef/cookstyle/blob/master/RELEASE_NOTES.md#cookstyle-513)
for a complete list of changes between 5.9 and 5.13.

This new release also allows you to use `cookstyle` specific comments in
your cookbook code to enable or disable cops instead of the standard
`rubocop` comments. We think that it will be easier to understand the
cops that you intend to control if you use `cookstyle` comments. You can
continue to use the existing `rubocop` comments, if you prefer them,
since both types of comments will be honored by Cookstyle.

Rubocop comment to disable a cop:

> ``` ruby
> node.normal[:foo] # rubocop: disable ChefCorrectness/Bar
> ```

Cookstyle comment to disable a cop:

> ``` ruby
> node.normal[:foo] # cookstyle: disable ChefCorrectness/Bar
> ```

#### Foodcritic

Foodcritic has been updated from 16.1.1 to 16.2.0. This release includes
a fix for detecting incorrect notification actions and ships with
updated Chef Infra Client Metadata. Keep in mind that Foodcritic is no
longer being actively developed and users should migrate to Cookstyle
instead.

#### Chef InSpec

Chef InSpec has been updated from 4.17.17 to 4.18.38. This release
includes a large number of bug fixes in addition to some great resource
enhancements:

-   Inputs can now be used within a `describe.one` block
-   The `service` resource now includes a `startname` property for
    Windows and systemd services
-   The `interface` resource now includes a `name` property
-   The `user` resource now better supports Windows with the addition of
    `passwordage`, `maxbadpasswords`, and `badpasswordattempts`
    properties
-   The nginx resource now includes parsing support for wildcard, dot
    prefix, and regex
-   The `iis_app_pool` resource now handles empty app pools
-   The `filesystem` resource now supports devices with very long names
-   The `apt` resource better handles URIs and supports repos with an
    arch
-   The `oracledb_session` resource has received multiple fixes to make
    it work better
-   The `npm` resource now works under sudo on Unix and on Windows with
    a custom PATH

#### Test Kitchen

We updated Test Kitchen has to 2.3.4, which includes more robust code
for finding the Chef binary on Windows and also improves some logging
messages.

#### knife-ec2

The <span class="title-ref">knife-ec2</span> plugin has been updated
from 1.0.16 to 1.0.17 which includes a fix for an error when launching
non-T2 type instances.

#### kitchen-digitalocean

kitchen-digitalocean has been updated to 0.10.5 which adds new image
aliases for `Debian-10` and `FreeBSD-12`.

#### kitchen-dokken

kitchen-dokken has been updated to 2.8.0. This will make the `CI` and
`TEST_KITCHEN` environmental variables match the behavior of
`kitchen-vagrant`.

#### kitchen-inspec

We updated the kitchen-inspec plugin to 1.3.1, which allows relative
paths in the git fetcher and resolves failures when using inputs.

### Performance Improvements

This release of ChefDK ships with several optimizations to our Ruby
installation that improve the performance of the included commands,
especially on Windows systems. Expect to see more here in future
releases.

### Security Updates

libxlst was updated from 1.1.30 to 1.1.34 to resolve these
vulnerabilities:

> -   [CVE-2019-11068](https://www.cvedetails.com/cve/CVE-2019-11068/)
> -   [CVE-2019-13117](https://www.cvedetails.com/cve/CVE-2019-13117/)
> -   [CVE-2019-13118](https://www.cvedetails.com/cve/CVE-2019-13118/)

## What's New in 4.5

### Habitat Packages

We are now publishing Habitat packages for ChefDK 4. See
[chef/chef-dk](https://bldr.habitat.sh/#/pkgs/chef/chef-dk) on Habitat
Depot for a complete list of available versions.

### Updated Components

#### Chef Infra Client

Chef Infra Client has been updated from 15.3 to 15.4 with updated
resources and several significant fixes to `knife bootstrap`. See the
[Chef Infra Client 15.4 Release
Notes](https://discourse.chef.io/t/chef-infra-client-15-4-45-released/16081)
for a complete list of the new and improved functionality.

#### Chef InSpec

Chef InSpec has been updated from 4.16 to 4.18 with the following
changes:

**New Features**

-   We have released our beta Chef InSpec plug-in for HashiCorp Vault.
    Check it out in our [inspec-vault GitHub
    repo](https://github.com/inspec/inspec-vault) and let us know what
    you think -- or better yet, start jumping in and contributing with
    us on it.
-   Waivers, our new beta feature, was added to InSpec! Waivers allows
    you to better manage compliance failures. We would love to hear your
    feedback on this! See the [InSpec Waivers
    documentation](/inspec/waivers/) for
    more details.

**Improvements**

-   The `interface` resource now has a name property.
-   Expanded `user` resource to include the passwordage,
    maxbadpasswords, and badpasswordattempts properties with Windows.
-   The `sys_info` resource now supports ip_address, fqdn, domain, and
    short options when giving a version of the hostname.
-   Sped up initial load/response time for all commands by removing
    pre-leading of resources on invocation of inspec.
-   If an error occurs when using the `json` resource with a command
    source, you will now get the error message from STDERR returned in
    the report.
-   We improved the formatting of the usage help, so what you see when
    you type `inspec exec --help` should look better!

#### Cookstyle

Cookstyle has been updated from 5.6.2 to 5.9.3, which includes 13 new
Chef cops, improved detection in existing cops, and improved
autocorrection. See the [Cookstyle 5.7, 5.8, and 5.9 release
notes](https://github.com/chef/cookstyle/blob/master/RELEASE_NOTES.md)
for additional information on the new cops.

#### knife-google

knife-google was updated from 4.1.0 to 4.2.0 with support for adding
multiple local SSD interfaces to a new instance.

#### knife-vsphere

knife-vsphere was updated from 4.0.1 to 4.0.3, which resolves a bug in
determining the state of instances.

### Security Updates

#### Ruby

Ruby has been updated from 2.6.4 to 2.6.5 in order to resolve the
following CVEs:

-   [CVE-2019-16255](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16255):
    A code injection vulnerability of Shell\#\[\] and Shell\#test
-   [CVE-2019-16254](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16254):
    HTTP response splitting in WEBrick (Additional fix)
-   [CVE-2019-15845](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-15845):
    A NUL injection vulnerability of File.fnmatch and File.fnmatch?
-   [CVE-2019-16201](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16201):
    Regular Expression Denial of Service vulnerability of WEBrick's
    Digest access authentication

## What's New in 4.4

### Updated Components

#### Chef Infra Client

Chef Infra Client has been updated from 15.2 to 15.3 with updated
resources, a new way to write streamlined custom resources, and updated
platform support! See the [Chef Infra Client 15.3 Release
Notes](https://discourse.chef.io/t/chef-infra-client-15-3-14-released/15909)
for a complete list of the new and improved functionality.

#### Chef InSpec

Chef InSpec has been updated from 4.10.4 to 4.16.0 with the following
changes:

-   A new `postfix_conf` has been added for inspecting Postfix
    configuration files.
-   A new `plugins` section has been added to the InSpec configuration
    file which can be used to pass secrets or other configurations
    into Chef InSpec plugins.
-   The `service` resource now includes a new `startname` property for
    determining which user is starting the Windows services.
-   The `groups` resource now properly gathers membership information
    on macOS hosts.

See the [Chef InSpec 4.16.0 Release
Notes](https://discourse.chef.io/t/chef-inspec-4-16-0-released/15818)
for more information.

#### Cookstyle

Cookstyle has been updated from 5.1.19 to 5.6.2. This update brings the
total number of Chef cops to 94 and divides the cops into four separate
departments. The new departments make it easier to search for specific
cops, and to enable and disable groups of cops. Instead of just "Chef",
we now have the following departments:

-   `ChefDeprecations`: Cops that detect, and in many cases correct,
    deprecations that will prevent cookbooks from running on modern
    versions of Chef Infra Client.
-   `ChefStyle`: Cops that will help you improve the format and
    readability of your cookbooks.
-   `ChefModernize`: Cops that will help you modernize your cookbooks
    by including features introduced in new releases of Chef Infra
    Client.
-   `ChefEffortless`: Cops that will help you migrate your cookbooks
    to the Effortless pattern. These are disabled by default.

You can run cookstyle with just a single department:

``` bash
cookstyle --only ChefDeprecations
```

You can also exclude a specific department from the command line:

``` bash
cookstyle --except ChefStyle
```

You can also disable a specific department by adding the following to
your `.rubocop.yml` config:

``` yaml
ChefStyle:
  Enabled: false
```

See the [Cookstyle cops
documentation](https://github.com/chef/cookstyle/blob/master/docs/cops.md)
for a complete list of cops included in Cookstyle 5.6.

Going forward, Cookstyle will be our sole Ruby and Chef Infra cookbook
linting tool. With the release of Cookstyle 5.6, we're officially
deprecating Foodcritic and will not be shipping Foodcritic in the next
major release of Chef Workstation (April 2020). See our [Goodbye,
Foodcritic blog post](https://blog.chef.io/goodbye-foodcritic/) for more
information on why Cookstyle is replacing Foodcritic.

#### kitchen-ec2

`kitchen-ec2` has been updated from 3.1.0 to 3.2.0. This adds support
for Windows Server 2019 and adds the ability to look up security group
by `subnet_filter` in addition to `subnet_id`.

#### kitchen-inspec

`kitchen-inspec` has been updated from 1.1.0 to 1.2.0. This renames the
`attrs` key to `input_files`, and the `attributes` key to `inputs` to
match InSpec 4. The old names are still supported, but issue a warning.

#### knife-ec2

`knife-ec2` has been updated from 1.0.12 to 1.0.16. This resolves the
following issues:

-   Fix argument error for --platform option
    [\#609](https://github.com/chef/knife-ec2/pull/609)
    ([dheerajd-msys](https://github.com/dheerajd-msys))
-   Fix for Generate temporary keypair when none is supplied
    [\#608](https://github.com/chef/knife-ec2/pull/608)
    ([kapilchouhan99](https://github.com/kapilchouhan99))
-   Color code fixes in json format output of knife ec2 server list
    [\#606](https://github.com/chef/knife-ec2/pull/606)
    ([dheerajd-msys](https://github.com/dheerajd-msys))
-   Allow instances to be provisioned with source/dest checks disabled
    [\#605](https://github.com/chef/knife-ec2/pull/605)
    ([kapilchouhan99](https://github.com/kapilchouhan99))

#### Test Kitchen

Test Kitchen has been updated from 2.2.5 to 2.3.2 with the following
changes:

-   Add `keepalive_maxcount` setting for better control of ssh
    connection timeouts.
-   Add `lifecycle_hooks` information to `kitchen diagnose` output.

#### knife-google

The knife-google plugin has been updated to 4.1.0 with support for
bootstrapping Chef Infra Client 15, and also includes a new
`knife google image list command`, which lists project and public
images.

For example `knife google image list --gce_project "chef-msys"`:

``` bash
NAME                             PROJECT        FAMILY         DISK SIZE  STATUS
kpl-w-image                      chef-msys      windows        60 GB      READY
centos-6-v20190916               centos-cloud   centos-6       10 GB      READY
centos-7-v20190916               centos-cloud   centos-7       10 GB      READY
coreos-alpha-2261-0-0-v20190911  coreos-cloud   coreos-alpha   9 GB       READY
coreos-beta-2247-2-0-v20190911   coreos-cloud   coreos-beta    9 GB       READY
....
....
....
```

### Security Updates

#### Git

Git has been updated from 2.20.0 to 2.23.0 on Windows and from 2.14.1 to
2.23.0 on non-Windows systems. This brings the latest git workflows to
our users who do not have it installed another way and fixes two CVEs:

-   non-Windows systems:
    [CVE-2017-14867](https://www.cvedetails.com/cve/CVE-2017-14867/)
-   Windows systems:
    [CVE-2019-1211](https://portal.msrc.microsoft.com/en-US/security-guidance/advisory/CVE-2019-1211)

#### Nokogiri

Nokogiri has been updated from 1.10.2 to 1.10.4 in order to resolve
[CVE-2019-5477](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-5477)

#### OpenSSL

OpenSSL has been updated from 1.0.2s to 1.0.2t in order to resolve
[CVE-2019-1563](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1563)
and
[CVE-2019-1547](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1547).

#### Ruby

Ruby has been updated from 2.6.3 to 2.6.4 in order to resolve
[CVE-2012-6708](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2012-6708)
and
[CVE-2015-9251](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-9251).

### Platform Support Updates

ChefDK is now validated against macOS 10.15 (Catalina). Additionally,
ChefDK will no longer be validated against macOS 10.12.

## What's New in 4.3

### Updated Components

#### Chef Infra Client

Chef Infra Client has been updated from 15.1 to 15.2 with new and
improved resources and support for RHEL 8. See the [Chef Infra Client
15.2 Release
Notes](/release_notes/#chef-infra-client-15-2)
for a complete list of new and improved functionality.

#### Chef InSpec

Chef InSpec has been updated from 4.7.3 to 4.10.4 with the following
changes:

-   Fixed handling multiple triggers in the `windows_task` resource
-   Fixed exceptions when resources are used with incompatible
    transports
-   Un-deprecated the `be_running` matcher on the `service` resource
-   Added `sys_info.manufacturer` and `sys_info.model` resources
-   Added `ip6tables` resource

#### Cookstyle

Cookstyle has been updated from 5.0 to 5.1.19 with twenty-four new Chef
specific cops to detect, and in many cases, to auto-correct errors in
your cookbook code. With the release of Cookstyle 5.1, we have started
the process of replacing Foodcritic with Cookstyle. Cookstyle offers a
modern configuration system, auto-correction, and a faster and more
reliable engine thanks to RuboCop. We will continue to port useful rules
from Foodcritic to Cookstyle, as well as add rules that were not
possible in the legacy Foodcritic engine. See the [Cookstyle 5.1 Release
Notes](https://github.com/chef/cookstyle/blob/master/RELEASE_NOTES.md#cookstyle-51)
for a complete list of new rules.

#### kitchen-azurerm

kitchen-azurerm has been updated from 0.14.8 to 0.14.9, which adds a new
`use_ephemeral_osdisk` configuration option. See Microsoft's [Ephemeral
OS Disk
Announcement](https://azure.microsoft.com/en-us/updates/azure-ephemeral-os-disk-now-generally-available/)
for more information on this new feature.

#### kitchen-ec2

kitchen-ec2 has been updated from 3.0.1 to 3.1.0 with several new
features:

-   Added support for SSH through Session Manager. Thanks
    [@awiddersheim](https://github.com/awiddersheim)
-   Adds support for searching for multiple security groups, as well as
    searching by group name. Thanks
    [@bdwyertech](https://github.com/bdwyertech)
-   Allows asking for multiple instance types and subnets for spot
    pricing. Thanks
    [@vmiszczak-teads](https://github.com/vmiszczak-teads)

#### kitchen-vagrant

kitchen-vagrant has been updated from 1.5.2. to 1.6.0. This new version
properly truncates the instance name to avoid hitting the 100 character
limit in Hyper-V, and also updates the hostname length limit on Windows
from 12 characters to 15 characters. Thanks
[@Xorima](https://github.com/Xorima) and
[@PowerSchill](https://github.com/PowerSchill).

#### knife-ec2

knife-ec2 has been updated from 1.0.8 to 1.0.12. This new version
includes multiple fixes for network configuration setup, a new
`--cpu-credits` option for launching T2/T3 instances as unlimited, and
fixes for issues with attaching ephemeral disks.

### Platform Support Updates

#### RHEL 8 Support Added

ChefDK 4.3 now includes native packages for RHEL 8 with all builds now
validated on RHEL 8 hosts.

#### SLES 11 EOL

Packages will no longer be built for SUSE Linux Enterprise Server (SLES)
11 as SLES 11 exited the 'General Support' phase on March 31, 2019. See
[Chef's Platform End-of-Life
Policy](/platforms/#platform-end-of-life-policy)
for more information on when Chef ends support for an OS release.

## What's New in 4.2

### Bug Fixes

-   Rubygems has been rolled back to 3.0.3 to resolve duplicate bundler
    gems that shipped in ChefDK 4.1.7. This resulted in warning messages
    when running commands as well as performance degradations.
-   Fixed 'chef install foo.lock.json' errors when loading cookbooks
    from Artifactory.

### Updated Components

#### knife-ec2 1.0.8

Knife-ec2 has been updated to 1.0.8. This release removes previously
deprecated bootstrap command-line options that were removed from Chef
Infra Client 15.

#### knife-vsphere 3.0.1

Knife-vsphere has been updated to 3.0.1 to resolve Ruby warnings that
occurred when running some commands.

#### Fauxhai 7.4.0

Fauxhai has been updated to 7.4.0, which adds additional platforms for
use with ChefSpec testing.

-   Updated <span class="title-ref">suse</span> 15 from 15.0 to 15.1
-   Added a new <span class="title-ref">redhat</span> 8 definition to
    replace the 8.0 definition, which is now deprecated
-   Updated all <span class="title-ref">amazon</span> and <span
    class="title-ref">ubuntu</span> releases to Chef 15.1
-   Added <span class="title-ref">debian</span> 10 and 9.9

#### Chef InSpec 4.7.3

Chef InSpec has been updated to 4.7.3, which adds a new `ip6tables`
resource and includes new `aws-sdk` gems that are necessary for the Chef
InSpec AWS Resource Pack.

## What's New in 4.1

### Updated Components

#### Chef Infra Client 15.1

Chef Infra Client has been updated to 15.1 with new and improved
resources, improvements to target mode, bootstrap bug fixes, new Ohai
detection on VirtualBox hosts, and more. See the [Chef Infra Client 15.1
Release
Notes](https://github.com/chef/chef/blob/master/RELEASE_NOTES.md#chef-infra-client-151)
for a complete list of new and improved functionality.

#### Chef InSpec 4.6.9

Chef InSpec has been updated from 4.3.2 to 4.6.9 with the following
changes:

-   InSpec `Attributes` have now been renamed to `Inputs` to avoid
    confusion with Chef Infra attributes.
-   A new InSpec plugin type of `Input` has been added for defining new
    input types. See the [InSpec Plugins
    documentation](https://github.com/inspec/inspec/blob/master/docs/dev/plugins.md#implementing-input-plugins)
    for more information on writing these plugins.
-   InSpec no longer prints errors to the stdout when passing
    `--format json`.
-   When fetching profiles from GitHub, the URL can now include periods.
-   The performance of InSpec startup has been improved.

#### Cookstyle 5.0.0

Cookstyle has been updated to 5.0.0 with a large number of bugfixes and
major improvements that lay the groundwork for future autocorrecting of
cookbook style and deprecation warnings.

The RuboCop engine that powers Cookstyle has been updated from 0.62 to
0.72, which includes several hundred bugfixes to the codebase. Due to
some of these bugfixes, existing cookbooks may fail when using Cookstyle
5.0. Additionally, some cops have had their names changed and the
Rubocop Performance cops have been removed. If you disabled individual
cops in your .rubocop.yml file, this may require you to update your
confg.

This new release also merges in code from the `rubocop-chef` project,
providing new alerting and autocorrecting capabilities specific to Chef
Infra Cookbooks. Thank you [@coderanger](http://github.com/coderanger)
for your work in the rubocop-chef project and
[@chrishenry](http://github.com/chrishenry) for helping with new cops.

Foodcritic 16.1.1

Foodcritic has been updated from 16.0.0 to 16.1.1 with new rules and
support for the latest Chef:

-   Updated Chef Infra Client metadata for 15.1 to include the new
    `chocolatey_feature` resources, as well as new properties in the
    `launchd` and `chocolatey_source` resources
-   Added new rule to detect large files shipped in a cookbook:
    `FC123: Content of a cookbook file is larger than 1MB`. Thanks
    [@mattray](http://github.com/mattray)
-   Allowed configuring the size of the AST cache with a new
    `--ast-cache-size` command line option. Thanks
    [@Babar](http://github.com/Babar)

#### ChefSpec 7.4.0

ChefSpec has been updated to 7.4 with better support stubbing commands,
and a new `policyfile_path` configuration option for specifying the path
to the PolicyFile.

#### kitchen-dokken 2.7.0

kitchen-dokken has been updated to 2.7.0 with new options for
controlling how containers are setup and pulled. You can now disable
user namespace mode when running privileged containers with a new
`userns_host` config option. There is also a new option
`pull_chef_image` (true/false) to control force-pulling the chef image
on each run to check for newer images. This option now defaults to
`true` so that testing on latest and current always actually mean latest
and current. See the [kitchen-dokken
readme](https://github.com/someara/kitchen-dokken/blob/master/README.md)for
`kitchen.yml` config examples.

#### kitchen-digitalocean 0.10.4

kitchen-digitalocean has been updated to 0.10.4 with support for new
distros and additional configuration options for instance setup. You can
now control the default DigitalOcean region systems that are spun up by
using a new `DIGITALOCEAN_REGION` env var. You can still modify the
region in the driver section of your `kitchen.yml` file if you'd like,
and the default region of `nyc1` has not changed. This release also adds
slug support for `fedora-29`, `fedora-30`, and `ubuntu-19`. Finally, if
you'd like to monitor your test instances, the new `monitoring`
configuration option in the `kitchen.yml` driver section allows enabling
DigitalOcean's instance monitoring. See the [kitchen-digitalocean
readme](https://github.com/test-kitchen/kitchen-digitalocean/blob/master/README.md)
for `kitchen.yml` config examples.

#### knife-vsphere 3.0.0

knife-vsphere has been updated to 3.0. This new version adds support for
specifying the `bootstrap_template` when creating new VMs. This release
also improves how the plugin finds VM hosts, in order to support hosts
in nested directories.

#### knife-ec2 1.0.7

knife-ec2 has received a near-complete rewrite with this release of
ChefDK. The new knife-ec2 release switches the underlying library used
to communicate with AWS from `fog-aws` to Amazon's own `aws-sdk`. The
official AWS SDK has greatly improved support for the many AWS
authentication methods available to users. It also has support for all
of the latest AWS regions and instance types. As part of this switch to
the new SDK we did have to remove the `knife ec2 flavor list` command as
this used hard coded values from fog-aws and not AWS API calls. The good
news is, we were able to add several new commands to the plugin. This
makes provisioning systems in AWS even easier.

**knife ec2 vpc list**

This command lists all VPCs in your environment including the ID, which
you need when provisioning new systems into a specific VPC.

``` none
knife ec2 vpc list
ID            State      CIDR Block     Instance Tenancy  DHCP Options ID  Default VPC?
vpc-b1bc8d9d  available  10.0.0.0/16    default           dopt-1d78412a    No
vpc-daafd931  available  172.0.0.0/16   default           dopt-1d78412a    Yes
```

**knife ec2 eni list**

This command lists all ENIs in your environment including the ID, which
you need when adding the ENI to a newly provisioned instance.

``` none
knife ec2 eni list
ID                     Status  AZ          Public IP       Private IPs    IPv6 IPs  Subnet ID        VPC ID
eni-0123f25ae7805b651  in-use  us-west-2a  63.192.209.236  10.0.0.204               subnet-4ef3b123  vpc-b1bc8d9d
eni-2451c913           in-use  us-west-2a  137.150.209.123 10.0.0.245               subnet-4ef3b123  vpc-b1bc8d9d
```

**knife ec2 securitygroup list**

This command lists all security groups in your environment including the
ID, which you need when assigning a newly provisioned instance to a
group.

``` none
$knife ec2 securitygroup list
ID                    Name                                     VPC ID
sg-12332d875a4a123d6  not-today-hackers                        vpc-dbbf59a2
sg-123708ab12388cac5  open-to-the-world                        vpc-dbbf59a2
```

**knife ec2 subnet list**

This command lists all subnets in your environment including the ID,
which you need when placing a newly provisioned instance in a subnet.

``` none
knife ec2 subnet list
ID               State      CIDR Block      AZ          Available IPs  AZ Default?  Maps Public IP?  VPC ID
subnet-bd2333a9  available  172.31.0.0/20   us-west-2b  4091           Yes          Yes              vpc-b1bc8d9d
subnet-ba1135c9  available  172.31.16.0/20  us-west-2a  4091           Yes          Yes              vpc-b1bc8d9d
```

### Platform Support Updates

Ubuntu 14.04 entered the end-of-life phase April 30, 2019. Since this
version of Ubuntu is now end-of-life, we have stopped building packages
for Ubuntu 14.04. If you rely on Ubuntu 14.04 in your environment, we
highly recommend upgrading your host to Ubuntu 16.04 or 18.04.

### Security Updates

#### curl 7.65.1

-   CVE-2019-5435: Integer overflows in curl_url_set
-   CVE-2019-5436: tftp: use the current blksize for recvfrom()
-   CVE-2018-16890: NTLM type-2 out-of-bounds buffer read
-   CVE-2019-3822: NTLMv2 type-3 header stack buffer overflow
-   CVE-2019-3823: SMTP end-of-response out-of-bounds read
-   CVE-2019-5443: Windows OpenSSL engine code injection

#### cacerts 5-11-2019

Our <span class="title-ref">cacert</span> bundle has been updated to the
5-11-2019 bundle, which adds four additional CAs.

## What's New in 4.0

### Breaking Changes

#### Chef EULA

Usage of ChefDK 4.0, Chef Infra Client 15, and Chef InSpec 4 requires
accepting the [Chef
EULA](/chef_license/#chef-eula). See the
[frequently asked questions](https://www.chef.io/bmc-faq/) for
information about the license update and the associated business model
change.

#### Chef Provisioning

Chef Provisioning is no longer included with ChefDK, and will be
officially end of life on August 31, 2019. The source code of Chef
Provisioning and the drivers have been moved into the chef-boneyard
GitHub organization and will not be further maintained. Current users of
Chef Provisioning should contact your Chef Customer Success Manager or
Account Representative to review your options.

#### knife bootstrap against cloud providers

`knife bootstrap` was
[rewritten](https://github.com/chef/chef/blob/cfbb01cb5648297835941679bc9638d3a823ad5e/RELEASE_NOTES.md#knife-bootstrap)
in Chef Infra Client 15. The `knife-*` cloud providers need to be
updated to use this new API. As of ChefDK 4.0, `knife bootstrap`
functionality against the cloud providers will be broken. We will fix
this ASAP in a ChefDK 4.1 release. The only gem *not* affected is the
`knife-windows` gem. It has already been re-written to leverage the new
bootstrap library.

Affected gems:

-   `knife-ec2`
-   `knife-google`
-   `knife-vsphere`

If you leverage this functionality, please wait to update ChefDK until
4.1 is released with fixes for these gems.

### Improved Chef Generate command

The `chef generate` command has been updated to produce cookbooks and
repositories that match Chef's best practices.

-   `chef generate repo` now generates a Chef repository with
    Policyfiles by default. You can revert to the previous roles /
    environment behavior with the `--roles` flag.
-   `chef generate cookbook` now generates a cookbook with a
    Policyfile and no Berksfile by default. You can revert to the
    previous behavior with the `--berks` flag.
-   `chef generate cookbook` now includes ChefSpecs that utilize the
    ChefSpec 7.3+ format. This is a much simpler syntax that requires
    less updating of specs as older platforms are deprecated.
-   `chef generate cookbook` no longer creates cookbook files with the
    unnecessary `frozen_string_literal: true` comments.
-   `chef generate cookbook` no longer generates a full Workflow
    (Delivery) build cookbook by default. A new `--workflow` flag has
    been added to allow generating the build cookbook. This flag
    replaces the previously unused `--delivery` flag.
-   `chef generate cookbook` now generates cookbooks with metadata
    requiring Chef 14 or later.
-   `chef generate cookbook --kitchen dokken` now generates a fully
    working kitchen-dokken config.
-   `chef generate cookbook` now generates Test Kitchen configs with
    the `product_name`/`product_version` method of specifying Chef
    Infra Client releases as `require_chef_omnibus` will be removed in
    the next major Test Kitchen release.
-   `chef generate cookbook_file` no longer places the specified file
    in a "default" folder as these aren't needed in Chef Infra Client
    12 and later.
-   `chef generate repo` no longer outputs the full Chef Infra Client
    run information while generating the repository. Similar to the
    <spancommand class="title-ref">cookbook</spancommand you can view this
    verbose output with the `--verbose` flag.

### Updated Components

#### Chef InSpec 4

Chef InSpec has been updated to 4.3.2 which includes the new InSpec
AWS resource pack with **59** new AWS resources, multi-region support,
and named credentials support. This release also includes support for
auditing systems that use `ed25519` SSH keys.

#### Chef Infra Client 15

Chef Infra Client has been updated to Chef 15 with **8** new resources,
target mode prototype functionality, `ed25519` SSH key support, and
more. See the [Chef Infra Client 15 Release
Notes](/release_notes/#chef-infra-client-15-0-293)
for more details.

#### Fauxhai 7.3

Fauxhai has been updated from 6.11 to 7.3. This removes all platforms
that were previously marked as deprecated. So if you've noticed
deprecation warnings during your ChefSpec tests, you will need to update
those specs for the latest [supported Fauxhai
platforms](https://github.com/chefspec/fauxhai/blob/master/PLATFORMS.md).
This release also adds the following new platform releases for testing
in ChefSpec:

-   RHEL 6.10 and 8.0
-   openSUSE 15.0
-   CentOS 6.10
-   Debian 9.8 / 9.9
-   Oracle Linux 6.10, 7.5, and 7.6

#### Test Kitchen 2.2

Test Kitchen has been updated from 1.24 to 2.2.5. This update adds
support for accepting the Chef Infra Client and Chef InSpec EULAs during
testing, as well as support for newer `ed25519` format SSH keys on
guests. The newer release does remove support for the legacy Librarian
depsolver and testing of Chef Infra Client 10/11 releases in some
scenarios. See the [Test Kitchen Release
Notes](https://github.com/test-kitchen/test-kitchen/blob/master/RELEASE_NOTES.md#test-kitchen-22-release-notes)
for additional details on this release.

#### Kitchen-ec2 3.0

Kitchen-ec2 has been updated to 3.0, which uses the newer `aws-sdk-v3`
and includes a large number of improvements to the driver including
improved hostname detection, backoff retries, additional security group
configuration options, and more. See the [kitchen-ec2
Changelog](https://github.com/test-kitchen/kitchen-ec2/blob/master/CHANGELOG.md#v300-2019-05-01)
for additional details.

#### kitchen-dokken 2.6.9

Kitchen-dokken has been updated to 2.6.9 with a new config option
`pull_platform_image`, which allows you to disable pulling the platform
Docker image on every Test Kitchen converge / test. This is particularly
useful for local platform image testing.

kitchen.yml example:

``` none
driver:
  name: dokken
  pull_platform_image: false
```

## What's New in 3.13

### Updated Components

#### chef-vault

The chef-vault gem has been updated to 4.0.1. This release includes bug
fixes from [@MarkGibbons](https://github.com/MarkGibbons) and
[@jeremy-clerc](https://github.com/jeremy-clerc) as well as a new way to
update existing keys to sparse-mode by running
`knife vault update --keys_mode sparse` thanks to
[@jeunito](https://github.com/jeunito).

#### kitchen-azurerm

kitchen-azurerm has been updated from 0.14.9 to 0.15.1 with the
following improvements:

-   Enable the WinRM HTTP listener by default. Thanks
    [@sean-nixon](https//github.com/sean-nixon)
-   Allow overriding of the `subscription_id` by setting the
    `AZURE_SUBSCRIPTION_ID` ENV variable.
-   Add a new `nic_name` config. Thanks
    [@libertymutual](https//github.com/libertymutual)
-   Support for creating VM with Azure KeyVault certificate. Thanks
    [@javgallegos](https//github.com/javgallegos)

#### kitchen-dokken

kitchen-dokken has been updated to 2.8.1 which fixes a bug that
prevented <span class="title-ref">ENV</span> vars from being passed into
containers.

#### knife-tidy

knife-tidy has been updated from 2.0.1 to 2.0.6 to resolve issues if an
org was named `cookbooks` and to improve error messages.

#### mixlib-install

mixlib-install has been updated from 3.11.21 to 3.11.24 and will now
properly identify Windows 2019 hosts.

### Performance Improvements

This release of ChefDK ships with several optimizations to our Ruby
installation to improve the performance of loading the various commands
bundled with ChefDK. These improvements are particularly noticeable on
non-SSD hosts and on Windows.

### Smaller Size

We continue to optimize the size of the ChefDK package with this release
taking up 11% less space on disk and containing nearly 5,000 fewer
files.

### Platform Support

ChefDK packages are no longer produced for Windows 2008 R2 as this
release reached its end of life on Jan 14th, 2020.

### Security Updates

#### OpenSSL

OpenSSL has been updated to 1.0.2u to resolve
[CVE-2019-1551](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1551)

#### Git

The embedded git client has been updated to 2.24.1 to resolve the
following CVEs:

-   [CVE-2019-1348](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1348)
-   [CVE-2019-1349](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1349)
-   [CVE-2019-1350](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1350)
-   [CVE-2019-1351](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1351)
-   [CVE-2019-1352](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1352)
-   [CVE-2019-1353](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1353)
-   [CVE-2019-1354](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1354)
-   [CVE-2019-1387](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1387)
-   [CVE-2019-19604](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-19604)

## What's New in 3.12.10

### Updated Components

#### Chef Infra Client 14.14.29

Chef Infra Client has been updated to 14.14.29 with the following bug
fixes:

-   Fixed an error with the `service` and `systemd_unit` resources which
    would try to re-enable services with an indirect status.
-   The `systemd_unit` resource now logs at the info level.
-   Fixed knife config when it returned a
    `TypeError: no implicit conversion of nil into String` error.

#### kitchen-digitalocean 0.10.4

kitchen-digitalocean has been updated to 0.10.5 which adds new image
aliases for <span class="title-ref">Debian-10</span> and <span
class="title-ref">FreeBSD-12</span>.

#### kitchen-dokken 2.8.0

kitchen-dokken has been updated to 2.8.0. This will make the `CI` and
`TEST_KITCHEN` environmental variables match the behavior of
`kitchen-vagrant`.

### Security Updates

#### libxslt

libxslt has been updated to 1.1.34 to resolve
[CVE-2019-13118](https://nvd.nist.gov/vuln/detail/CVE-2019-13118).

#### Ruby

Ruby has been updated from 2.5.6 to 2.5.7 in order to resolve the
following CVEs:

-   [CVE-2019-16255](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16255):
    A code injection vulnerability of Shell\#\[\] and Shell\#test
-   [CVE-2019-16254](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16254):
    HTTP response splitting in WEBrick (Additional fix)
-   [CVE-2019-15845](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-15845):
    A NUL injection vulnerability of File.fnmatch and File.fnmatch?
-   [CVE-2019-16201](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16201):
    Regular Expression Denial of Service vulnerability of WEBrick's
    Digest access authentication

## What's New in 3.12

### Chef Generate Updates

Many of the non-breaking updates to the `chef generate` command that
shipped in ChefDK 4 have been backported to ChefDK 3.

-   `chef generate cookbook` now includes ChefSpecs that utilize the
    ChefSpec 7.3+ format. This is a much simpler syntax that requires
    less updating of specs as older platforms are deprecated.
-   `chef generate cookbook` now generates Test Kitchen configs with
    Ubuntu 18.04
-   `chef generate cookbook` now generates non-hidden Test Kitchen
    configs (kitchen.yml instead of .kitchen.yml)
-   `chef generate cookbook --kitchen dokken` now generates a fully
    working kitchen-dokken config.
-   `chef generate cookbook` no longer creates cookbook files with the
    unnecessary `frozen_string_literal: true` comments.
-   `chef generate cookbook` now generates Test Kitchen configs with the
    `product_name`/`product_version` method of specifying Chef Infra
    Client releases as `require_chef_omnibus` will be removed in the
    next major Test Kitchen release.
-   `chef generate cookbook_file` no longer places the specified file in
    a `default` folder as these aren't needed in Chef Infra Client 12
    and later.
-   `chef generate cookbook` now generates cookbooks with updated
    `.gitignore` and `chefignore` files

### Updated Components

#### Chef Infra Client 14.14.25

Chef Infra Client has been updated from 14.13 to 14.14.25. This release
includes support for the new `unified_mode` in custom resources, a large
number of improvements to resources, improved platform detection
support, as well as bug fix. See the [Chef Infra Client 14.14.25 Release
Notes](https://github.com/chef/chef/blob/chef-14/RELEASE_NOTES.md#chef-client-release-notes-141425)
for a detailed list of changes.

#### ChefSpec 7.4.0

ChefSpec has been updated to 7.4 with better support stubbing commands,
and a new `policyfile_path` configuration option for specifying the path
to the PolicyFile.

#### kitchen-azurerm

kitchen-azurerm has been updated from 0.14.8 to 0.14.9, which adds a new
`use_ephemeral_osdisk` configuration option. See Microsoft's [Empheral
OS Disk
Announcement](https://azure.microsoft.com/en-us/updates/azure-ephemeral-os-disk-now-generally-available/)
for more information on this new feature.

#### kitchen-digitalocean 0.10.4

kitchen-digitalocean has been updated to 0.10.4 with support for new
distros and additional configuration options for instance setup. You can
now control the default DigitalOcean region systems that are spun up by
using a new `DIGITALOCEAN_REGION` environmental variable. You can still
modify the region in the driver section of your `kitchen.yml` file if
you'd like, and the default region of `nyc1` has not changed. This
release also adds slug support for `fedora-29`, `fedora-30`, and
`ubuntu-19`. Finally, if you'd like to monitor your test instances, the
new `monitoring` configuration option in the `kitchen.yml` driver
section allows enabling DigitalOcean's instance monitoring. See the
[kitchen-digitalocean
readme](https://github.com/test-kitchen/kitchen-digitalocean/blob/master/README.md)
for `kitchen.yml` config examples.

#### kitchen-vagrant

kitchen-vagrant has been updated from 1.5.2. to 1.6.0. This new version
properly truncates the instance name to avoid hitting the 100 character
limit in Hyper-V, and also updates the hostname length limit on Windows
from 12 characters to 15 characters. Thanks
[@Xorima](https://github.com/Xorima) and
[@PowerSchill](https://github.com/PowerSchill).

#### knife-vsphere 3.0.1

Knife-vsphere has been updated to 3.0.1. This new version adds support
for specifying the `bootstrap_template` when creating new VMs. This
release also improves how the plugin finds VM hosts, in order to support
hosts in nested directories.

### Platform Support Updates

#### macOS 10.15 Support

ChefDK is now validated against macOS 10.15 (Catalina) with packages
available at [downloads.chef.io](https://downloads.chef.io/chefdk/).
Additionally, ChefDK will no longer be validated against macOS 10.12.

#### RHEL 8 Support

ChefDK is now validated against RHEL 8 with packages available at
[downloads.chef.io](https://downloads.chef.io/chefdk/).

#### Windows 2019 Support

ChefDK is now validated against Windows 2019 with packages available at
[downloads.chef.io](https://downloads.chef.io/chefdk/).

#### SLES 11 EOL

Packages will no longer be built for SUSE Linux Enterprise Server (SLES)
11 as SLES 11 exited the 'General Support' phase on March 31, 2019. See
[Chef's Platform End-of-Life
Policy](/platforms/#platform-end-of-life-policy)
for more information on when Chef ends support for an OS release.

#### Ubuntu 14.04 EOL

Packages will no longer be built for Ubuntu 14.04 as Ubuntu 14.04
entered "End of life" status April 2019. See [Chef's Platform
End-of-Life
Policy](/platforms/#platform-end-of-life-policy)
for more information on when Chef ends support for an OS release.

### Security Updates

#### Ruby

Ruby has been updated from 2.5.5 to 2.5.6 in order to resolve the
following CVEs:

-   [CVE-2019-16255](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16255):
    A code injection vulnerability of Shell\#\[\] and Shell\#test
-   [CVE-2019-16254](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16254):
    HTTP response splitting in WEBrick (Additional fix)
-   [CVE-2019-15845](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-15845):
    A NUL injection vulnerability of File.fnmatch and File.fnmatch?
-   [CVE-2019-16201](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16201):
    Regular Expression Denial of Service vulnerability of WEBrick's
    Digest access authentication

#### openssl

OpenSSL has been updated from 1.0.2r to 1.0.2t to resolve the following
CVEs:

-   [CVE-2019-1563](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1563)
-   [CVE-2019-1547](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1547)
-   [CVE-2019-1552](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1552)

#### Nokogiri

Nokogiri has been updated from 1.10.3 to 1.10.4 in order to resolve
[CVE-2019-5477](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-5477).

## What's New in 3.11

### Updated Components

#### Chef Infra Client 14.13.11

Chef Infra Client has been updated to 14.13.11 with resource
improvements and bug fixes. See the [Release
Notes](https://github.com/chef/chef/blob/chef-14/RELEASE_NOTES.md#chef-client-release-notes-1413)
for a detailed list of changes.

#### Test Kitchen 1.25

Test Kitchen has been updated to 1.25 with backports of many
non-breaking Test Kitchen 2.0 features:

-   Support for accepting the Chef 15 license in Test Kitchen runs. See
    [Accepting the Chef
    License](/chef_license_accept/) for usage
    details.
-   A new `--fail-fast` command line flag for use with the <span
    class="title-ref">concurrency</span> flag. With this flag set, Test
    Kitchen will immediately fail when any converge fails instead of
    continuing to test additional instances.
-   The `policyfile_path` config option now accepts relative paths.
-   A new `berksfile_path` config option allows specifying Berkshelf
    files in non-standard locations.
-   Retries are now honored when using SSH proxies

#### kitchen-dokken 2.7.0

-   The Chef Docker image is now pulled by default so that locally
    cached <span class="title-ref">latest</span> or <span
    class="title-ref">current</span> container versions will be compared
    to those available on DockerHub. See the
    [readme](https://github.com/someara/kitchen-dokken#disable-pulling-chef-docker-images)
    for instructions on reverting to the previous behavior.
-   User namespace mode can be disabled when running privileged
    containers with a new `userns_host` config option. See the
    [readme](https://github.com/someara/kitchen-dokken#running-with-user-namespaces-enabled)
    for details.
-   You can now disable pulling the platform Docker images for local
    platform image testing or air gapped testing. See the
    [readme](https://github.com/someara/kitchen-dokken#disable-pulling-platform-docker-images)
    for details.

### Security Updates

#### curl 7.65.0

-   CVE-2019-5435: Integer overflows in curl_url_set
-   CVE-2019-5436: tftp: use the current blksize for recvfrom()
-   CVE-2018-16890: NTLM type-2 out-of-bounds buffer read
-   CVE-2019-3822: NTLMv2 type-3 header stack buffer overflow
-   CVE-2019-3823: SMTP end-of-response out-of-bounds read

## What's New in 3.10

### New Policy File Functionality

`include_policy` now supports `:remote` policy files. This new
functionality allows you to include policy files over http. Remote
policy files require remote cookbooks and `install` will fail otherwise
if the included policy file includes cookbooks with paths. Thanks
[mattray](https://github.com/mattray)!

### Updated Components

-   `kitchen-vagrant`: 1.5.1 -\> 1.5.2
-   `mixlib-install`: 3.11.12 -\> 3.11.18
-   `ohai`: 14.8.11 -\> 14.8.12

## What's New in 3.9

### Updated Components

#### Chef 14.12.3

ChefDK now ships with Chef 14.12.3. See [Chef 14.12 release
notes](/release_notes/#whats-new-in-14-12) for
more information on what's new.

#### InSpec 3.9.0

ChefDK now ships with InSpec 3.9.0. See [InSpec 3.9.0 release
details](https://github.com/inspec/inspec/releases/tag/v3.9.0) for more
information on what's new.

#### kitchen-hyperv

kitchen-hyperv has been updated to 0.5.3, which now automatically
disables snapshots on the VMs and properly waits for the IP to be set.

#### kitchen-vagrant

kitchen-vagrant has been updated to 1.5.1, which adds support for using
the new `bento/amazonlinux-2` box when setting the platform to
`amazonlinux-2`.

#### kitchen-ec2

kitchen-ec2 has been updated to 2.5.0 with support for Amazon Linux 2.0
image searching using the platform `amazon2`. This release also adds
supports Windows Server 1709 and 1803 image searching.

#### knife-vsphere

knife-vsphere has been updated to 2.1.3, which adds support for knife's
`bootstrap_template` flag and removes the legacy `distro` and
`template_file` flags.

#### Push Jobs Client

Push Jobs Client has been updated to 2.5.6, which includes significant
optimizations and minor bug fixes.

### Security Updates

#### Rubygems 2.7.9

Rubygems has been updated from 2.7.8 to 2.7.9 to resolves the following
CVEs:

-   CVE-2019-8320: Delete directory using symlink when decompressing tar
-   CVE-2019-8321: Escape sequence injection vulnerability in verbose
-   CVE-2019-8322: Escape sequence injection vulnerability in gem owner
-   CVE-2019-8323: Escape sequence injection vulnerability in API
    response handling
-   CVE-2019-8324: Installing a malicious gem may lead to arbitrary code
    execution
-   CVE-2019-8325: Escape sequence injection vulnerability in errors

## What's New in 3.8

### Updated Components

#### InSpec 3.6.6

ChefDK now ships with Inspec 3.6.6. See
<https://github.com/inspec/inspec/releases/tag/v3.6.6> for more
information on what's new.

#### Fauxhai 6.11.0

-   Added Windows 2019 Server, Red Hat Linux 7.6, Debian 9.6, and CentOS
    7.6.1804.
-   Updated Windows7, 8.1, and 10, 2008 R2, 2012, 2012 R2, and 2016 to
    Chef 14.10.
-   Updated Oracle Linux 6.8/7.2/7.3/7.4 to Ohai 14.8 in EC2.
-   Updated the fetcher logic to be compatible with ChefSpec 7.3+.
    Thanks [oscar123mendoza](https://github.com/oscar123mendoza)!
-   Removed duplicate json data in gentoo 4.9.6.

#### Other Component Updates

-   \`kitchen-digitalocean\`: 0.10.1 -\> 0.10.2
-   \`mixlib-install\`: 3.11.5 -\> 3.11.11

## What's New in 3.7

### Updated Components

#### Chef 14.10.9

ChefDK now ships with Chef 14.10.9. See [Chef 14.10 release
notes](/release_notes/#whats-new-in-14-10) for more information on
what's new.

#### InSpec 3.4.1

-   New aws_billing_report / aws_billing_reports resources
-   Many under the hood improvements

#### kitchen-inspec 1.0.1

-   Support for bastion configuration in transport options.

#### kitchen-vagrant 1.4.0

-   This fixes audio for VirtualBox users by disabling audio in
    VirtualBox by default to prevent interrupting host Bluetooth audio.

#### kitchen-azurerm 0.14.8

-   Support Azure Managed Identities and apply vm_tags to all resources
    in resource group.

#### Other Updated Components

> -   \`chef-apply\`: 0.2.4 -\> 0.2.7
> -   \`knife-tidy\`: 1.2.0 -\> 2.0.0

### Deprecations

Chef Provisioning has been in maintenance mode since 2015 and due to the
age of its dependencies it cannot be included in ChefDK 4 which is
scheduled for an April 2019 release.

## What's New in 3.6

### Chef CLI Improvements

The Chef CLI now includes a new option: <span class="title-ref">chef
generate cookbook --kitchen (dokken|vagrant)</span> Generate cookbooks
with a specific kitchen configuration (defaults to vagrant).

### Updated Components

#### Chef 14.8.12

ChefDK now ships with Chef 14.8.12. See [Chef 14.8 release
notes](/release_notes/#whats-new-in-14-8) for more information on
what's new.

#### InSpec 3.2.6

-   Added new <span class="title-ref">aws_sqs_queue</span> resource.
    Thanks [amitsaha](https://github.com/amitsaha)
-   Exposed additional WinRM options for transport, basic auth, and
    SSPI. Thanks [frezbo](https://github.com/frezbo)
-   Improved UI experience throughout including new CLI flags
    --color/--no-color and --interactive/--no-interactive

#### Berkshelf 7.0.7

-   Added <span class="title-ref">berks outdated --all</span> command to
    get a list of outdated dependencies, including those that wouldn't
    satisfy the version constraints set in Berksfile. Thanks
    [jeroenj](https://github.com/jeroenj)

#### Fauxhai 6.10.0

-   Added Fedora 29 Ohai data for use in ChefSpec

#### chef-sugar 5.0

-   Added a new parallels? helper. Thanks
    [ehanlon](https://github.com/ehanlon)
-   Added support for the Raspberry Pi 1 and Zero to armhf? helper
-   Added a centos_final? helper. Thanks
    [kareiva](https://github.com/kareiva)

#### Foodcritic 15.1

-   Updated the Chef metadata to Chef versions 13.12 / 14.8 and removed
    all other Chef metadata

#### kitchen-azurerm 0.14.7

-   Resolved failures in the plugin by updating the azure API gems

#### kitchen-ec2 2.4.0

-   Added support for arm64 architecture instances
-   Support Windows Server 1709 and 1803 image searching. Thanks
    [xtimon](https://github.com/xtimon)
-   Support Amazon Linux 2.0 image searching. Use the platform
    'amazon2'. Thanks [pschaumburg](https://github.com/pschaumburg)

#### knife-ec2 0.19.16

-   Allow passing the <span
    class="title-ref">--bootstrap-template</span> option during node
    bootstrapping

#### knife-google 3.3.7

-   Allow running knife google zone list, region list, region quotas,
    project quotas to run without specifying the <span
    class="title-ref">gce_zone</span> option

#### stove 7.0.1

-   The yank command has been removed as this command causes large
    downstream impact to other users and should not be part of the
    tooling
-   The metadata.rb file will now be included in uploads to match the
    behavior of berkshelf 7+

#### test-kitchen 1.24

-   Added support for the Chef 13+ root aliases. With this chance you
    can now test a cookbook with a simple recipe.rb and attributes.rb
    file.
-   Improve WinRM support with retries and graceful connection cleanup.
    Thanks [bdwyertech](https://github.com/bdwyertech) and
    [dwoz](https://github.com/dwoz)

### Security Updates

#### OpenSSL updated to 1.0.2q

-   Microarchitecture timing vulnerability in ECC scalar multiplication
    [CVE-2018-5407](https://nvd.nist.gov/vuln/detail/CVE-2018-5407)
-   Timing vulnerability in DSA signature generation
    [CVE-2018-0734](https://nvd.nist.gov/vuln/detail/CVE-2018-0734)
-   **New Chef Command Functionality**

## What's New in 3.5

### Docker Image Updates

The [chef/chefdk](https://hub.docker.com/r/chef/chefdk) Docker image now
includes graphviz (to support `berks viz`) and rsync (to support
`kitchen-dokken`) which makes it a little bigger, but also a little more
useful in development and test pipelines.

### Updated Components

#### Chef 14.7.17

ChefDK now ships with Chef 14.7.17. See [Chef 14.7 release
notes](/release_notes/#whats-new-in-14-7) for more information on
what's new.

## What's New in 3.4

### Updated Components

#### Chef 14.6.47

ChefDK now ships with Chef 14.6.47. See [Chef 14.6 release
notes](/release_notes/#whats-new-in-14-6) for more information on
what's new.

#### Fauxhai 6.9.1

-   Updated mock Ohai run data for use with ChefSpec for multiple
    platforms
-   Added Linux Mint 19, macOS 10.14, Solaris 5.11 (11.4 release), and
    SLES 15.
-   Deprecated the following platforms for removal April 2018: Linux
    Mint 18.2, Gentoo 4.9.6, All versions of ios_xr, All versions of
    omnios, All versions of nexus, macOS 10.10, and Solaris 5.10.
-   See [Fauxhai Supported
    Platforms](https://github.com/chefspec/fauxhai/tree/master/lib/fauxhai/platforms)
    for a complete list of supported platform data for use with
    ChefSpec.

#### Foodcritic 14.3

-   Updated the metadata that ships with Foodcritic to provide the
    latest Chef 13.11 and 14.5 metadata
-   Removed metadata from older Chef releases. This update also
-   Removed the FC121 rule, which was causing confusion with community
    cookbook authors. This rule will be added back when Chef 13 goes EOL
    in April 2019.

#### InSpec 3.0.12

-   Added a new plugin system for inspec and the train transport system
-   Added a new global attributes system
-   Enhanced skip messages
-   Many more enhancements

#### Kitchen AzureRM

-   Added support for the Shared Image Gallery.

#### Kitchen DigitalOcean

-   Added support for FreeBSD 10.4 and 11.2

#### Kitchen EC2

-   Improved Windows system support. The auto-generated security group
    will now include support for RDP and the log directory will alway be
    created.

#### Kitchen Google

-   Added support for adding labels to instances with a new <span
    class="title-ref">labels</span> config that accepts labels as a
    hash.

#### Knife Windows

-   Improved Windows detection support to identify Windows 2012r2, 2016,
    and 10.
-   Added support for using the client.d directories when bootstrapping
    nodes.

### Smaller Package Size

ChefDK RPM and Debian packages are now compressed. Additionally many
gems were updated to remove extraneous files that do not need to be
included. The download size of packages has decreased accordingly (all
measurements in megabytes):

-   .deb: 108 -\> 84 (22%)
-   .rpm: 112 -\> 86 (24%)

### Platform Support Updates

macOS 10.14 (Mojave) is now fully tested and packages are available on
downloads.chef.io.

### Security Updates

Ruby has been updated to 2.5.3 to resolve the following vulnerabilities:

-   \`CVE-2018-16396\`: Tainted flags are not propagated in Array\#pack
    and String\#unpack with some directives
-   \`CVE-2018-16395\`: OpenSSL::X509::Name equality check does not work
    correctly

## What's New in 3.3

### Updated Components

#### Chef 14.5.33

ChefDK now ships with Chef 14.5.33. See [Chef 14.5 release
notes](/release_notes/#whats-new-in-14-5) for more information on
what's new.

#### ChefSpec 7.3

A new simplified ChefSpec syntax now allows testing of custom resources.
See the [ChefSpec
README](https://github.com/chefspec/chefspec/blob/v7.3.2/README.md) and
especially the section on [testing custom
resources](https://github.com/chefspec/chefspec/blob/v7.3.2/README.md#testing-a-custom-resource)
for examples of the new syntax.

#### Other Updated Components

-   `chef-provisioning-aws`: 3.0.4 -\> 3.0.6
-   `chef-vault`: 3.3.0 -\> 3.4.2
-   `foodcritic`: 14.0.0 -\> 14.1.0
-   `inspec`: 2.2.70 -\> 2.2.112
-   `kitchen-inspec`: 0.23.1 -\> 0.24.0
-   `kitchen-vagrant`: 1.3.3 -\> 1.3.4

### New Chef CLI Functionality

The Chef CLI now includes a new option: <span class="title-ref">chef
update --exclude-deps</span> for policyfiles which will only update the
cookbook(s) given on the command line.

### Deprecations

-   `chef generate app` - Application repos were a pattern that didn't
    take off.
-   `chef generate lwrp` - Use <span class="title-ref">chef generate
    resource</span>. Every supported release of Chef supports custom
    resources. Custom resources are awesome. No one should be writing
    new LWRPs any more. LWRPS are not awesome.

## What's New in 3.2

-   **Chef 14.4.56**

    ChefDK now ships with Chef 14.4.56. See [Chef 14.4 release
    notes](/release_notes/#whats-new-in-14-4) for more information
    on what's new.

-   **New Functionality**

    -   New <span class="title-ref">chef describe-cookbook</span>
        command to display the cookbook checksum.
    -   Change policyfile generator to use `policyfiles` directory
        instead of `policies` directory

-   **New Tooling**

    **Kitchen AzureRM**

    :   ChefDK now includes a driver for [Azure Resource
        Manager](https://github.com/test-kitchen/kitchen-azurerm). This
        allows Microsoft Azure resources to be provisioned prior to
        testing. This driver uses the new Microsoft Azure Resource
        Management REST API via the azure-sdk-for-ruby.

-   **Updated Tooling**

    **Test Kitchen**

    Test Kitchen 1.23 now includes support for [lifecycle
    hooks](https://github.com/test-kitchen/test-kitchen/blob/master/RELEASE_NOTES.md#life-cycle-hooks).

-   **Updated Components**

    -   `berkshelf`: 7.0.4 -\> 7.0.6
    -   `chef-provisioning`: 2.7.1 -\> 2.7.2
    -   `chef-provisioning-aws`: 3.0.2 -\> 3.0.4
    -   `chef-sugar`: 4.0.0 -\> 4.1.0
    -   `fauxhai`: 6.4.0 -\> 6.6.0
    -   `inspec`: 2.1.72 -\>2.2.70
    -   `kitchen-google`: 1.4.0 -\> 1.5.0

-   **Security Updates**

    **OpenSSL**

    OpenSSL updated to 1.0.2p to resolve:

    -   Client DoS due to large DH parameter
        [CVE-2018-0732](https://nvd.nist.gov/vuln/detail/CVE-2018-0732)
    -   Cache timing vulnerability in RSA Key Generation
        [CVE-2018-0737](https://nvd.nist.gov/vuln/detail/CVE-2018-0737)

## What's New in 3.1

-   **Chef 14.2.0**

    ChefDK now ships with Chef 14.2.0. See [Chef 14.2 release
    notes](/release_notes/#whats-new-in-14-2-0) for more information
    on what's new.

-   **Habitat Packages**

    ChefDK is now released as a habitat package under the identifier
    `chef/chef-dk`. All successful builds are available in the unstable
    channel and all promoted builds are available in the stable channel.

-   **Updated Homebrew Cask Tap**

    You can install ChefDK on macOS using
    `brew cask install chef/chef/chefdk`. The tap name is new, but not
    the behavior.

-   **Updated Tooling**

    **Fauxhai 6.4**

    -   Added for 3 new platforms - CentOS 7.5, Debian 8.11, and FreeBSD
        11.2.
    -   Updated platform data for Amazon Linux, Red Hat, SLES, and
        Ubuntu to match Chef 14.2 output.
    -   Deprecated the FreeBSD 10.3 platform data.

    **Foodcritic 14.0**

    -   Added support for Chef 14.2 metadata
    -   Removes older Chef 13 metadata.
    -   Updated rules for clarity and removes an unnecessary rule.
    -   Added a new rule saying when cookbooks have unnecessary
        dependencies now that resources moved into core Chef.

    **knife-acl**

    -   `knife-acl` is now included with ChefDK. This knife plugin
        allows admin users to modify Chef Server ACLs from their command
        line.

    **knife-tidy**

    -   `knife-tidy` is now included with ChefDK. This knife plugin
        generates reports about stale nodes and helps clean them up.

    **Test Kitchen 1.22**

    -   Added a new `ssh_gateway_port` config.
    -   Fixed a bug on Unix systems where scripts are not created as
        executable.

-   **Other Updated Components and Tools**

    -   `kitchen-digitalocean: 0.9.8 -> 0.10.0`
    -   `knife-opc: 0.3.2 -> 0.4.0`

-   **Security Updates**

    -   **ffi**

        CVE-2018-1000201: DLL loading issue which can be hijacked on
        Windows OS

## What's New in 3.0

-   **Chef 14.1.1**

    ChefDK now ships with Chef 14.1.1. See the [Chef 14.1 release
    notes](/release_notes/#what-s-new-in-14-1-1) for more
    information on what's new.

-   **Updated Operating System support**

    ChefDK now ships packages for Ubuntu 18.04 and Debian 9. In
    accordance with Chef's platform End Of Life policy, ChefDK is no
    longer shipped on macOS 10.10.

-   **Enhanced cookbook archive handling**

    ChefDK now uses an embedded copy of `libarchive` to support
    Policyfile and Berkshelf. This improves overall performance and
    provides a well tested interface to different types of archives. It
    also resolves the long standing "not an octal string" problem users
    face when depending on certain cookbooks in the supermarket.

-   **Policyfiles: updated include_policy support**

    Policyfiles now support git targets for included policies.

    ``` ruby
    include_policy 'base_policy',
                  git: 'https://github.com/happychef/chef-repo.git',
                  branch: master,
                  path: 'policies/base/Policyfile.lock.json'
    ```

-   **Updated Tooling**

    -   *Test Kitchen*

        Test Kitchen has been updated from 1.20.0 to 1.21.2. This
        release allows you to use a `kitchen.yml` config file instead of
        `.kitchen.yml` so the kitchen config will no longer be hidden in
        your cookbook directories. It also introduces new config options
        for SSH proxy servers and allows you to specify multiple paths
        for data bags. See the
        [CHANGELOG](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md)
        for a complete list of changes.

    -   **InSpec**

        InSpec has been updated from 1.51.21 to 2.1.68. InSpec 2.0
        brings compliance automation to the cloud, with new resource
        types specifically built for AWS and Azure clouds. Along with
        these changes are major speed improvements and quality of life
        updates. Please visit [Inspec](/inspec/) for more
        information.

    -   **ChefSpec**

        ChefSpec has been updated to 7.2.1 with Fauxhai 6.2.0. This
        release removes all platforms that were previously marked as
        deprecated in Fauxhai. If you saw Fauxhai deprecation warnings
        during your ChefSpec runs you will now see failures. This update
        also adds 9 new platforms and updates existing data for Chef 14.
        To see a complete list of platforms that can be mocked in
        ChefSpec see
        <https://github.com/chefspec/fauxhai/blob/master/PLATFORMS.md>.

    -   **Foodcritic**

        Foodcritic has been updated to from 12.3.0 to 13.1.1. This
        updates Foodcritic for Chef 13 or later by removing Chef 12
        metadata and removing several legacy rules that suggested
        writing resources in a Chef 12 manner. The update also adds 9
        new rules for writing custom resources and updating cookbooks to
        Chef 13 and 14, resolves several long standing file detection
        bugs, and improves performance.

    -   **Cookstyle**

        Cookstyle has been updated to 3.0, which updates the underlying
        RuboCop engine to 0.55 with a long list of bug fixes and
        improvements. This release of Cookstyle also enables 19 new
        rules available in RuboCop. See the
        [CHANGELOG](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md)
        for a complete list of newly enabled rules.

    -   **Berkshelf**

        Berkshelf has been updated to 7.0.2. Berkshelf 7 moves to using
        the same libraries as the Chef Client, ensuring consistent
        behavior - for instance, ensuring that `chefignore` files work
        the same - and enabling a quicker turnaround on bug fixes. The
        "Actor crashed" failures of celluloid will no longer be produced
        by Berkshelf.

    -   **VMware vSphere support**

        The `knife-vsphere` plugin for managing VMware vSphere is now
        bundled with ChefDK.

    -   **Cookbook generator creates a CHANGELOG.md**

        `chef cookbook generate [cookbook_name]` now creates a
        CHANGELOG.md file.

-   **Updated Components and Tools**

    -   `chef-provisioning 2.7.0 -> 2.7.1`
    -   `knife-ec2 0.17.0 -> 0.18.0`
    -   `opscode-pushy-client 2.3.0 -> 2.4.11`

-   **Security Updates**

    -   **Ruby**

        Ruby has been updated to 2.5.1 to resolve the following
        vulnerabilities:

        -   [CVE-2017-17742](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-17742)
        -   [CVE-2018-6914](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-6914)
        -   [CVE-2018-8777](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8777)
        -   [CVE-2018-8778](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8778)
        -   [CVE-2018-8779](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8779)
        -   [CVE-2018-8780](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-69148780)
        -   Multiple vulnerabilities in RubyGems

    -   **OpenSSL**

        OpenSSL has been updated to 1.0.2o to resolve CVE-2018-0739.

## What's New in 2.5.3

-   **Rename smoke tests to integration tests**

    The cookbook, recipe, and app generators now name the test directory
    `integration` instead of `smoke`. This will not impact existing
    cookbooks generated with older releases of ChefDK, but it does
    simplify the `.kitchen.yml` configuration for all new cookbooks.

-   **Chef 13.8.5**

    ChefDK now ships with Chef 13.8.5. See the [Chef 13.8 release
    notes](/release_notes/#what-s-new-in-13-8-5) for more
    information.

-   **Updated chef_version in cookbook generator**

    When running `chef generate cookbook` the generated cookbook will
    now specify a minimum Chef release of 12.14 not 12.1.

-   **Security Updates**

    -   Ruby has been updated to 2.4.3 to resolve
        [CVE-2017-17405](https://nvd.nist.gov/vuln/detail/CVE-2017-17405)
    -   OpenSSL has been updated to 1.0.2n to resolve
        [CVE-2017-3738](https://nvd.nist.gov/vuln/detail/CVE-2017-3738),
        [CVE-2017-3737](https://nvd.nist.gov/vuln/detail/CVE-2017-3737),
        [CVE-2017-3736](https://nvd.nist.gov/vuln/detail/CVE-2017-3736),
        and
        [CVE-2017-3735](https://nvd.nist.gov/vuln/detail/CVE-2017-3735)
    -   LibXML2 has been updated to 2.9.7 to fix
        [CVE-2017-15412](https://access.redhat.com/security/cve/cve-2017-15412)
    -   minitar has been updated to 0.6.1 to resolve
        [CVE-2016-10173](https://nvd.nist.gov/vuln/detail/CVE-2016-10173)

-   **Updated Components**

    -   chefspec 7.1.1 -\> 7.1.2
    -   chef-api 0.7.1 -\> 0.8.0
    -   chef-provisioning 2.6.0 -\> 2.7.0
    -   chef-provisioning-aws 3.0.0 -\> 3.0.2
    -   chef-sugar 3.6.0 -\> 4.0.0
    -   foodcritic 12.2.1 -\> 12.3.0
    -   inspec 1.45.13 -\> 1.51.21
    -   kitchen-dokken 2.6.5 -\> 2.6.7
    -   kitchen-ec2 1.3.2 -\> 2.2.1
    -   kitchen-inspec 0.20.0 -\> 0.23.1
    -   kitchen-vagrant 1.2.1 -\> 1.3.1
    -   knife-ec2 0.16.0 -\> 0.17.0
    -   knife-windows 1.9.0 -\> 1.9.1
    -   test-kitchen 1.19.2 -\> 1.20.0
    -   chef-provisioning-azure has been removed as it used deprecated
        Azure APIs

## What's New in 2.4.17

-   **Improved performance downloading cookbooks from a Chef server**

    Policyfile users who use a Chef server as a cookbook source will
    experience faster cookbook downloads when running `chef install`.
    Chef server's API requires each file in a cookbook to be downloaded
    separately; ChefDK will now download the files in parallel.
    Additionally, HTTP keepalives are enabled to reduce connection
    overhead.

-   **Cookbook artifact source for policyfiles**

    Policyfile users may now source cookbooks from the Chef server's
    cookbook artifact store. This is mainly intended to support the
    upcoming `include_policy` feature, but could be useful in some
    situations.

    Given a cookbook that has been uploaded to the Chef server via
    `chef push`, it can be used in another policy by adding code like
    the following to the ruby policyfile:

    ``` ruby
    cookbook "runit",
      chef_server_artifact: "https://chef.example/organizations/myorg",
      identifier: "09d43fad354b3efcc5b5836fef5137131f60f974"
    ```

-   **Added include_policy directive**

    Policyfile can use the `include_policy` directive as described in
    [RFC097](https://github.com/chef/chef-rfc/blob/master/rfc097-policyfile-includes.md).
    This directive's purpose is to allow the inclusion policyfile locks
    to the current policyfile. In this iteration, we support sourcing
    lock files from a local path or a Chef server. Below is a simple
    example of how the `include_policy` directive can be used:

    Given a policyfile `base.rb`:

    ``` ruby
    name 'base'

    default_source :supermarket

    run_list 'motd'

    cookbook 'motd', '~> 0.6.0'
    ```

    Run:

    ``` none
    chef install ./base.rb

    Building policy base
    Expanded run list: recipe[motd]
    Caching Cookbooks...
    Using      motd         0.6.4
    Using      chef_handler 3.0.2

    Lockfile written to /home/jaym/workspace/chef-dk/base.lock.json
    Policy revision id: 1238e7a353ec07a4df6636cdffd8805220a00789bace96d6d70268a4b0064023
    ```

    This will produce the `base.lock.json` file that will be included in
    our next policy, `users.rb`:

    ``` ruby
    name 'users'

    default_source :supermarket

    run_list 'user'

    cookbook 'user', '~> 0.7.0'

    include_policy 'base', path: './base.lock.json'
    ```

    Run:

    ``` none
    chef install ./users.rb

    Building policy users
    Expanded run list: recipe[motd::default], recipe[user]
    Caching Cookbooks...
    Using      motd         0.6.4
    Installing user         0.7.0
    Using      chef_handler 3.0.2

    Lockfile written to /home/jaym/workspace/chef-dk/users.lock.json
    Policy revision id: 20fac68f987152f62a2761e1cfc7f1dc29b598303bfb2d84a115557e2a4a8f27
    ```

    This will produce a `users.lock.json` file that has the `base`
    policyfile lock merged in.

    More information can be found in
    [RFC097](https://github.com/chef/chef-rfc/blob/master/rfc097-policyfile-includes.md)
    and the [Policyfile documentation](/policyfile/).

-   **New tools bundled**

    We are now shipping these tools as part of ChefDK:

    -   [kitchen-digitalocean](https://github.com/test-kitchen/kitchen-digitalocean)
    -   [kitchen-google](https://github.com/test-kitchen/kitchen-google)
    -   [knife-ec2](https://github.com/chef/knife-ec2)
    -   [knife-google](https://github.com/chef/knife-google)

See the detailed [change
log](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md#v2417-2017-11-29)
for additional information.

## What's New in 2.3.4

ChefDK 2.3.4 pins the net-ssh gem to version 4.1 to prevent errors in
test-kitchen and kitchen-inspec that would prevent systems from properly
converging or verifying. This release is recommended for all users of
ChefDK 2.3.

## What's New in 2.3.3

This release restores macOS support in ChefDK 2.3. See the [change
log](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md#v233-2017-09-21)
for more information.

## What's New in 2.3.1

This release includes Ruby 2.4.2 to fix the following CVEs:

-   [CVE-2017-0898](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-0898)
-   [CVE-2017-10784](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-10784)
-   CVE-2017-14033
-   [CVE-2017-14064](https://nvd.nist.gov/vuln/detail/CVE-2017-14064)

ChefDK 2.3 includes:

-   Chef 13.4.19
-   InSpec 1.36.1
-   Berkshelf 6.3.1
-   Chef Vault 3.3.0
-   Foodcritic 11.4.0
-   Test Kitchen 1.17.0
-   Stove 6.0

Additionally, the cookbook generator now adds a `LICENSE` file when
creating a new cookbook.

See the detailed [change
log](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md#v231-2017-09-14)
for a complete list of changes.

{{< note >}}

Due to issues beyond our control, this release is only built for Linux
(x86_64) and Windows. We'll release a new build with macOS support as
soon as possible.

{{< /note >}}

## What's New in 2.2.1

This release includes RubyGems 2.6.13 to address the following CVEs:

-   [CVE-2017-0899](https://nvd.nist.gov/vuln/detail/CVE-2017-0899)
-   [CVE-2017-0900](https://nvd.nist.gov/vuln/detail/CVE-2017-0900)
-   [CVE-2017-0901](https://nvd.nist.gov/vuln/detail/CVE-2017-0901)
-   [CVE-2017-0902](https://nvd.nist.gov/vuln/detail/CVE-2017-0902)

ChefDK 2.2.1 includes:

-   Chef 13.3.42
-   InSpec 1.35.1
-   Berkshelf 6.3.1
-   Chef Vault 3.3.0
-   Foodcritic 11.3.1
-   Test Kitchen 1.17.0

## What's New in 2.1.11

This release updates the version of git shipped in ChefDK to 2.14.1 to
address
[CVE-2017-1000117](https://bugzilla.redhat.com/show_bug.cgi?id=CVE-2017-1000117).

### Notable Updated Gems

-   berkshelf 6.2.0 -\> 6.3.0
-   chef-provisioning 2.4.0 -\> 2.5.0
-   chef-zero 13.0.0 -\> 13.1.0
-   fauxhai 5.2.0 -\> 5.3.0
-   fog 1.40 -\> 1.41
-   inspec 1.31.1 -\> 1.33.1
-   kitchen-dokken 2.5.1 -\> 2.6.1
-   kitchen-vagrant 1.1.0 -\> 1.2.0
-   knife-push 1.0.2 -\> 1.0.3
-   ohai 13.2.0 -\> 13.3.0
-   serverspec 2.39.1 -\> 2.40.0
-   test-kitchen 1.16 -\> 1.17

See the detailed [change
log](https://github.com/chef/chef-dk/blob/master/CHANGELOG.md#v2111-2017-08-11)
for a full list of changes.

## What's New in 2.0.28

Chef 2.0.28 fixes an
[issue](https://github.com/chef/chef-dk/issues/1322) in ChefDK 2.0 where
`chef push` would upload incomplete cookbooks.

## What's New in 2.0

### Chef Client 13.2

Chef Client 13 is the most delightful version of Chef Client available.
We've taken what we've learned from many bug reports, forum posts, and
conversations with our users, and we've made it safer and easier than
ever to write great cookbooks. We've also included a number of new
resources that better support our most popular operating systems, and
we've made it easier to write patterns that result in reusable,
efficient code.

Chef Client 13.2 solves a number of issues that were reported in our
initial releases of Chef Client 13, and we regard it as suitable for
general use.

### PolicyFiles

It's now possible to update a single cookbook using
`chef update <cookbook>`. Artifactory is now supported as a cookbook
source.

### Cookbook Generator

Adds `chef generate helpers <HELPERS_NAME>` to generate a helpers file
in libraries.

### Berkshelf 6.2.0

Berkshelf adds support for two new sources:

-   Artifactory: source artifactory:
    '<https://myserver/api/chef/chef-virtual>'
-   Chef Repo: source chef_repo: '.'

### Chef Vault 3.1

Chef Vault 3.1 includes a number of optimizations for large numbers of
nodes. In most situations, we've seen at least 50% faster creation,
update, and refresh operations, and much more efficient memory usage.
We've also added a new `sparse` mode, which dramatically reduces the
amount of network traffic that occurs as nodes decrypt vaults. A lot of
the scalability work has been built and tested by our friends at Criteo.

Chef Vault 3.1 also makes it much easier to use provisioning nodes to
manage vaults by using the `public_key_read_access` group, which is
available in Chef server 12.5 and above.

### Foodcritic 11

Foodcritic 11 covers many of the patterns that were removed in Chef
Client 13, so you'll get up-front notification that your cookbooks will
no longer work with this release. In general, the patterns that were
removed enabled dangerous ways of writing cookbooks. Ensuring that
you're compliant with Foodcritic 11 means your cookbooks are safer with
every version of Chef.

The release of Foodcritic 11 also marks the creation of the Foodcritic
org on [GitHub](https://github.com/foodcritic), which makes it easier to
get involved in writing rules and contributing code. We are excited to
start building more of a community around Foodcritic, and can't wait to
see what the community cooks up.

### InSpec 1.30

Since the last release of ChefDK, InSpec has been independently released
multiple times with a number of great enhancements, including some new
resources (rabbitmq_config, docker, docker_image, docker_container,
oracledb_session), some enhancements to the Habitat package creator for
InSpec profiles, and a whole slew of bug fixes and documentation
updates.

### ChefSpec 7.1.0

It's no longer necessary to create custom matchers; ChefSpec will
automatically create matchers for any resources in the cookbooks under
test.

### Cookstyle 2.0

Cookstyle 2.0 is based on Rubocop 0.49.1, which changed a large number
of rule names.

## What's New in 1.6.11

This release contains only dependency updates, including several
security fixes:

-   Ruby has been upgraded to 2.3.5 to address the following CVEs:
    -   [CVE-2017-0898](https://www.ruby-lang.org/en/news/2017/09/14/sprintf-buffer-underrun-cve-2017-0898/)
    -   [CVE-2017-10784](https://www.ruby-lang.org/en/news/2017/09/14/webrick-basic-auth-escape-sequence-injection-cve-2017-10784/)
    -   [CVE-2017-14033](https://www.ruby-lang.org/en/news/2017/09/14/openssl-asn1-buffer-underrun-cve-2017-14033/)
    -   [CVE-2017-14064](https://www.ruby-lang.org/en/news/2017/09/14/json-heap-exposure-cve-2017-14064/)
-   Chef Client has been upgraded to 12.21.26
-   Push Jobs Client has been upgraded to 2.4.5

## What's New in 1.5

### Chef Client 12.21

Chef has been updated to the 12.21 release, fixing a number of bugs:

-   Debian-based systems will now correctly prefer Systemd to Upstart
-   Better handling of the `supports` pseudo-property
-   Fixes crashes that occurred when downgrading from Chef 13 to Chef 12
-   Provides better system information when Chef crashes

See the full [release
notes](https://github.com/chef/chef/blob/chef-12/RELEASE_NOTES.md#chef-client-release-notes-1221)
for more details.

Chef Client 12.21 also contains a new version of zlib, fixing 4 CVEs:

-   [CVE-2016-98402](https://www.cvedetails.com/cve/CVE-2016-9840/)
-   [CVE-2016-9841](https://www.cvedetails.com/cve/CVE-2016-9841/)
-   [CVE-2016-9842](https://www.cvedetails.com/cve/CVE-2016-9842/)
-   [CVE-2016-9843](https://www.cvedetails.com/cve/CVE-2016-9843/)

### Notable Updated Gems

-   cookstyle 1.3.1 -\> 1.4.0

## What's New in 1.4

### InSpec 1.25.1

-   Consistent hashing for InSpec profiles
-   Add platform info to json formatter
-   Allow mysql_session to test databases on different hosts
-   Add an oracledb_session resource
-   Support new Chef Automate compliance backend
-   Add command-line completions for fish shell

### Cookstyle 1.3.1

-   Disabled Style/DoubleNegation rule, which can be necessary in
    not_if / only_if blocks

## What's New in 1.3

### Chef Client 12.19

ChefDK now ships with Chef 12.19. Check out [Release
Notes](/release_notes/) for all the details of
this new release.

### Workflow Build Cookbooks

Build cookbooks generated via `chef generate build-cookbook` will no
longer depend on the delivery_build or delivery-base cookbook. Instead,
the Test Kitchen instance will use ChefDK as the standard workflow
runner setup.

The build cookbook generator will not overwrite your `config.json` or
`project.toml` if they exist already on your project.

### ChefSpec 6.0

ChefDK includes the new ChefSpec 6.0 release with improvements to the
ServerRunner behavior. Rather than creating a Chef Zero instance for
each ServerRunner test context, a single Chef Zero instance is created
that all ServerRunner test contexts will leverage. The Chef Zero
instance is reset between each test case, emulating the existing
behavior without needing a monotonically increasing number of Chef Zero
instances.

Additionally, if you are using ChefSpec to test a pre-defined set of
Cookbooks, there is now an option to upload those cookbooks only once,
rather than before every test case. To take advantage of this
performance enhancer, simply set the `server_runner_clear_cookbooks`
RSpec configuration value to `false` in your `spec_helper.rb`.

``` ruby
RSpec.configure do |config|
  config.server_runner_clear_cookbooks = false
end
```

Setting `server_runner_clear_cookbooks` value to `false` has been shown
to increase the ServerRunner performance by 75%, improve stability on
Windows, and make the ServerRunner as fast as SoloRunner.

This new release also includes three new matchers: `dnf_package`,
`msu_package`, and `cab_package` and utilizes the new Fauxhai 4.0
release. This release adds several new platforms and removes many older
end-of-life platforms. See
[PLATFORMS.md](https://github.com/customink/fauxhai/blob/master/PLATFORMS.md)
for a list of all supported platforms for use in ChefSpec.

### InSpec

InSpec has been updated to 1.19.1 with the following new functionality:

-   Better filter support for the [processes
    resource](/inspec/resources/processes/).
-   New `packages`, `crontab`, `x509_certificate`, and
    `x509_private_key` resources
-   New `inspec habitat profile create` command to create a Habitat
    artifact for a given InSpec profile.
-   Functional JUnit reporting
-   A new command for generating profiles has been added

### Foodcritic

Foodcritic has been updated to 10.2.2. This release includes the
following new functionality

-   FC003, which required gating certain code when running on Chef Solo
    has been removed
-   FC023, which preferred conditional (only_if / not_if) code within
    resources has been removed as many disagreed with this coding style
-   False positives in FC007 and FC016 have been resolved
-   New rules have been added requiring the license (FC068), supports
    (FC067), and chef_version (FC066) metadata properties in cookbooks

### Kitchen EC2 Driver

Kitchen-ec2 has been updated to 1.3.2 with support for Windows 2016
instances

### Cookbook generator improvements

`chef generate cookbook` has been updated to better generate cookbooks
for sharing with the Chef community. Generated cookbooks now require
Chef client 12.1+, include the chef_version metadata, and use SPDX
standard license strings.

### Notable Updated Gems

-   berkshelf 5.6.0 -\> 5.6.4
-   chef-provisioning 2.1.0 -\> 2.2.1
-   chef-provisioning-aws 2.1.0 -\> 2.2.0
-   chef-zero 5.2.0 -\> 5.3.1
-   chef 12.18.31 -\> 12.19.36
-   cheffish 4.1.0 -\> 5.0.1
-   chefspec 5.3.0 -\> 6.2.0
-   cookstyle 1.2.0 -\> 1.3.0
-   fauxhai 3.10.0 -\> 4.1.0
-   foodcritic 9.0.0 -\> 10.2.2
-   inspec 1.11.0 -\> 1.19.1
-   kitchen-dokken 1.1.0 -\> 2.1.2
-   kitchen-ec2 1.2.0 -\> 1.3.2
-   kitchen-vagrant 1.0.0 -\> 1.0.2
-   mixlib-install 2.1.11 -\> 2.1.12
-   opscode-pushy-client 2.1.2 -\> 2.2.0
-   specinfra 2.66.7 -\> 2.67.7
-   test-kitchen 1.15.0 -\> 1.16.0
-   train 0.22.1 -\> 0.23.0

## What's New in 1.2

### Delivery CLI

-   The `project.toml` file, which can be used to execute [local
    phases](/delivery_cli/#delivery-local), now supports:
    -   An optional `functional` phase.
    -   New `remote_file` option to specify a remote `project.toml`.
    -   The ability to run stages (collection of phases).
-   Fixed bug where the generated `project.toml` file did not include
    the prefix <span class="title-ref">chef exec</span> for some phases.
-   Project git remotes will now update automatically, if applicable,
    based on the values in the `cli.toml` or options provided through
    the command-line.
-   Project names specified in project config (`cli.toml`) or options
    provided through the command-line will now be honored.

### Policyfiles

-   Added a `chef_server` default source option to
    [Policyfiles](/config_rb_policyfile/#settings).

### Automate Workflow Adopts SSH for Cookbook Generation

The `chef generate cookbook` command now uses the SSH based job dispatch
system as its default behavior. For more details on this new system and
how to use it, see [Job Dispatch
Docs](/runners/)

### FIPS (Windows and RHEL only)

-   ChefDK now comes bundled with the Stunnel tool and the FIPS OpenSSL
    module for users who need to enforce FIPS compliance.
-   Support for FIPS options in <span class="title-ref">delivery</span>
    CLI's `cli.toml` was added to handle communication with the Automate
    Server when FIPS mode is enabled.

### Notable Updated Gems

-   berkshelf 5.2.0 -\> 5.5.0
-   chef 12.17.44 -\> 12.18.31
-   chef-provisioning 2.0.2 -\> 2.1.0
-   chef-vault 2.9.0 -\> 2.9.1
-   chef-zero 5.1.0 -\> 5.2.0
-   cheffish 4.0.0 -\> 4.1.0
-   cookstyle 1.1.0 -\> 1.2.0
-   foodcritic 8.1.0 -\> 8.2.0
-   inspec 1.7.2 -\> 1.10.0
-   kitchen-dokken 1.0.9 -\> 1.1.0
-   kitchen-vagrant 0.21.1 -\> 1.0.0
-   knife-windows 1.7.1 -\> 1.8.0
-   mixlib-install 2.1.9 -\> 2.1.10
-   ohai 8.22.1 -\> 8.23.0
-   test-kitchen 1.14.2 -\> 1.15.0
-   train 0.22.0 -\> 0.22.1
-   winrm 2.1.0 -\> 2.1.2

## What's New in 1.1

### New InSpec Test Location

To address bugs and confusion with the previous `test/recipes` location,
all newly generated cookbooks and recipes will place their InSpec tests
in `test/smoke/default`. This placement creates the association of the
<span class="title-ref">smoke</span> phase in Chef Automate and the
<span class="title-ref">default</span> Test Kitchen suite where the
tests are run.

### Default Docker image in kitchen-dokken is now official Chef image

[chef/chef](https://hub.docker.com/r/chef/chef) is now the default
Docker image used in
[kitchen-dokken](https://github.com/someara/kitchen-dokken).

### New Test Kitchen driver caching mechanisms

Test Kitchen will automatically cache downloaded chef-client packages
for use between provisions. For people who use the `kitchen-vagrant`
driver to run Chef, it will automatically consume the new caching
mechanism to share the client packages to the guest VM, meaning that you
no longer have to wait for the client to download on every guest
provision.

In addition, if Chef Infra Client packages are already cached, then it
is now possible to use Test Kitchen completely off-line.

### Cookstyle 1.1.0 with new code linting Cops

Cookstyle has been updated from `0.0.1` to `1.1.0`, which upgrades the
RuboCop engine from `0.39` to `0.46`, and enables several new cops. This
will most likely result in Cookstyle warnings on cookbooks that
previously passed.

**Newly Disabled Cops:**

-   Metrics/CyclomaticComplexity
-   Style/NumericLiterals
-   Style/RegexpLiteral in 'tests' directory
-   Style/AsciiComments
-   Style/TernaryParentheses
-   Metrics/ClassLength
-   All rails/\* cops

**Newly Enabled Cops:**

-   Bundler/DuplicatedGem
-   Style/SpaceInsideArrayPercentLiteral
-   Style/NumericPredicate
-   Style/EmptyCaseCondition
-   Style/EachForSimpleLoop
-   Style/PreferredHashMethods
-   Lint/UnifiedInteger
-   Lint/PercentSymbolArray
-   Lint/PercentStringArray
-   Lint/EmptyWhen
-   Lint/EmptyExpression
-   Lint/DuplicateCaseCondition
-   Style/TrailingCommaInLiteral
-   Lint/ShadowedException

### New DCO tool included

We have included a new DCO command-line tool that makes it easier to
contribute to projects like Chef that use the Developer Certificate of
Origin. The tool allows you to enable/disable DCO sign-offs for each
repository and also allows you to retroactively sign off all commits on
a branch. See <https://github.com/coderanger/dco> for details.

### Notable Upgraded Gems

-   chef `12.16.42` -\> `12.17.44`
-   ohai `8.21.0` -\> `8.22.0`
-   inspec `1.4.1` -\> `1.7.2`
-   train `0.21.1` -\> `0.22.0`
-   test-kitchen `1.13.2` -\> `1.14.2`
-   kitchen-vagrant `0.20.0` -\> `0.21.1`
-   winrm-elevated `1.0.1` -\> `1.1.0`
-   winrm-fs `1.0.0` -\> `1.0.1`
-   cookstyle `0.0.1` -\> `1.1.0`

## What's New in 1.0

### Version 1.0!

We're recognizing ChefDK's continued stability with the honor of a 1.0
tag. There is nothing in this release that breaks backwards
compatibility with previous installations of ChefDK: it is simply a
formal recognition of the stability of the product.

### Foodcritic

-   Foodcritic constraint updated to require v8.0 or greater.
-   Supermarket Foodcritic rules are now disabled by default when you
    run `chef generate cookbook`.

### InSpec

The `inspec` command is now included in the PATH managed by ChefDK. Just
run `chef shell-init` to update your PATH.

### knife-opc

[Knife OPC](https://github.com/chef/knife-opc) is now bundled with
ChefDK adding chef server organization and user commands to knife

### Notable Upgraded Gems

-   chef `12.15.19` -\> `12.16.42`
-   inspec `1.2.0` -\> `1.4.1`
-   train `0.20.1` -\> `0.21.1`
-   kitchen-dokken `1.0.3` -\> `1.0.4`
-   kitchen-inspec `0.15.2` -\> `0.16.1`
-   berkshelf `5.1.0` -\> `5.2.0`
-   fauxhai `3.9.0` -\> `3.10.0`
-   foodcritic `7.1.0` -\> `8.1.0`

## What's New in 0.19

### InSpec 1.2.0

InSpec Updated to v1.2.0. See the [InSpec
CHANGELOG](https://github.com/chef/inspec/blob/v1.2.0/CHANGELOG.md) for
details.

### Mixlib::Install

New `mixlib-install` command allows you to quickly download Chef
binaries. Run `mixlib-install help` for command usage.

### Delivery CLI

-   Deprecation of GitHub V1 backed project initialization.
-   Initialization of GitHub V2 backed projects
    (`delivery init --github`). Requires Chef Automate server version
    `0.5.432` or above.
-   Project name verification with repository name for projects with
    Source Control Management (SCM) integration.
-   Increased clarity of the command structure by introducing the
    `--pipeline` alias for the `--for` option.
-   Honor custom config on project initialization
    (`delivery init -c /my/config.json`).
-   Build cookbook is now generated using the more appropriate
    `chef generate build-cookbook` on project initialization.
-   Support providing your password non-interactively to
    `delivery token` via the `AUTOMATE_PASSWORD` environment variable
    (`AUTOMATE_PASSWORD=password delivery token`).

### Notable Upgraded Gems

-   chef `12.14.89` -\> `12.15.19`
-   inspec `1.0.0` -\> `1.2.0`
-   kitchen-dokken `1.0.0` -\> `1.0.3`
-   knife-windows `1.6.0` -\> `1.7.0`
-   mixlib-install `2.0.1` -\> `2.1.1`
-   winrm `2.0.3` -\> `2.1.0`

## Changelog

<https://github.com/chef/chef-dk/blob/master/CHANGELOG.md>
