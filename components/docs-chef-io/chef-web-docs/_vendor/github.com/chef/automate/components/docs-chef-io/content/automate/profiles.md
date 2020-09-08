+++
title = "Profiles"

date = 2018-03-26T16:02:53-07:00
draft = false
[menu]
  [menu.automate]
    title = "Profiles"
    parent = "automate/compliance"
    identifier = "automate/compliance/profiles.md Profiles"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/profiles.md)

## Overview

Compliance profiles help you secure your infrastructure continuously.
Chef Automate compliance profiles translate CIS Benchmarks and other security standards into easily readable policy.
You can install and download one of our 300+ ready-to-use compliance profiles from Profiles, or upload your custom profiles.

![Chef Automate Profiles](/images/automate/asset-store-installed.png)

## Using Profiles

Navigate to compliance profiles by selecting the **Compliance** tab and then selecting the _Profiles_ page, or by heading to `https://{{< example_fqdn "automate" >}}/compliance/compliance-profiles`.

Profiles has a search bar and two views: the _Profiles_ page, which displays the profiles you've installed
in your unique namespace within Chef Automate,
and the _Available_ page, which displays all of the ready-to-use compliance profiles in Chef Automate.

### Installing Profiles

![Available Profiles](/images/automate/asset-store-profiles.png)

Locate profiles by browsing the list, or by using the search bar.
To install a compliance profile into your namespace, simply select **Get** on the right side of the profile name.

Download compliance profiles to your workstation for use with [_Scan Jobs_]({{< relref "scan_jobs.md" >}}) and the [_Audit Cookbook_](https://github.com/chef-cookbooks/audit/blob/master/README.md), or as a basis for your own customizations. Start by selecting the arrow on the far right side of the profile, which redirects you to the _Profile Details_ view. Select the download button on the upper right corner of the profile description and download your selected profile as a tarball compressed with gzip ('.tar.gz').

### Uploading profiles

Upload Any InSpec2 compatible profile--including inherited profiles--to Chef Automate with the **upload** button on the _Profiles_ page.
Uploads use either the `.tar.gz` or zip archive file formats.

All profiles are stored in PostgreSQL, and are covered by backup and restore functionality.

{{< note >}}
All profiles should have a valid version in their `inspec.yml`. Older builds of Chef Automate require at least three elements of the x.y.z form -- for example, "1.2.0" -- and optionally can have one additional element, such as "1.2.0-20". Newer Chef Automate builds allow both two-element and three-element profile versions.
{{< /note >}}

### Updating profiles

New releases of profiles are shipped with the product when available.
Chef ships only the latest versions of profiles.
When Chef publishes a newer profile version that a user has installed to their namespace,
a small notification appears, prompting the user to download the newest version.
Installing the newest version adds the updated profile to the user namespace,
but it does not overwrite an existing profile.
You can keep multiple versions of a profile and it is up to you to curate your installed profiles.

## Understanding Profiles

The _Profile Details_ page appearance varies, depending on if a profile is installed or not.
You can **Get** or **Download** uninstalled profiles from the details page of an uninstalled profile.
You can **Download** or **Delete** a profile from the page of an installed profile.
Deleting a profile removes it from the profiles collection in your namespace, but it remains available in Profiles.
Installed or uninstalled, a profile's header and body contains important information.

### Profile Header

The profile header displays the profile title, a brief description and **Get** and **Delete** buttons.
If you have installed the profile, and **Get** or **Download** button if the profile is available but not installed in your unique namespace.
In either case, the profile header displays a status box detailing the:

Status
: Installed or uninstalled

Version
: The version number of the profile, which changes with updates

Author
: The organization responsible for composing and updating the profile

License
: Restrictions on the profile's re-use.

Installed profiles display a cURL command for an ad-hoc profile run.

![Installed Profile Details](/images/automate/asset-store-details-installed.png)

### Profile Body

A profile is made up of a series of controls, which are listed in the Profile Detail's body.
Each control has one or more InSpec tests.
The control table shows the number and names of controls in the profile, as well as the:

Total tests
: the number of tests in the control

Severity
: The impact of a control, from 0 to 1.
  See the Chef InSpec documentation for more information about the [severity measure](https://docs.chef.io/inspec/dsl_inspec#syntax)

Selecting the shaded area next to the control name or the `+` on the right side expands the control to show a more detailed description.
Selecting **View Code** displays the control's InSpec code.

![Profiles Body](/images/automate/profile-detail-body.png)

### About the Profile Identifier

The profile identifier is composed of the user's username and the profile name, found in the installed profiles list at `https://{{< example_fqdn "automate" >}}/profiles`.
Use this identifier when specifying profiles for the audit cookbook as well as specifying profiles through the InSpec CLI.

{{< note >}}
The identifier is mapped to user's username.
This is only guaranteed to be unique for a user group, either local or saml.
Users in a Chef Automate instance with the same username in both saml and local groups have access to each other's profiles.
{{< /note >}}

## Interacting with Chef Automate Profiles

You can interact with Chef Automate Profiles from the command line, as well as from the user interface.
For more information, see the [InSpec CLI](https://docs.chef.io/inspec/cli/) subcommand.

### API Calls

We've provided you with some essential cURL commands for interacting with Chef Automate Profiles.
In these examples, the owner is same value as first part of identifier, as discussed in [About the Profile Identifier]({{< relref "#about-the-profile-identifier" >}}).

#### Get All Installed Profiles

```bash
curl --insecure -H "X-Data-Collector-Token: token-value" https://automate.example.com/api/v0/compliance/profiles/search -d '{"owner": "test"}'
```

#### Get All Available Profiles

```bash
curl --insecure -H "X-Data-Collector-Token: token-value" https://automate.example.com/api/v0/compliance/profiles/search -d '{}'
```

#### Download .tar

```bash
curl --insecure -H "x-data-collector-token: token-val" https://automate.example.com/api/v0/compliance/profiles/tar -d '{"name":"cis-aix-5.3-6.1-level1","owner":"admin","version":"1.1.0-3"}'
```

#### Upload tar

```bash
curl --insecure -F file=@cis-ubuntu12_04lts-level1-1.1.0-2.tar.gz -H "x-data-collector-token: token-val"  https://automate.example.com/api/v0/compliance/profiles?owner=admin
```
