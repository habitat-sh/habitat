+++
title = "Chef Compliance Guide"
draft = false

[menu]
  [menu.compliance]
    title = "Chef Compliance Guide"
    identifier = "compliance/chef-benchmark.md Chef Compliance Guide"
    parent = "compliance"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/compliance/chef-benchmark.md)

Chef Compliance uses premium Chef content to audit and harden your fleet for [CIS](https://www.cisecurity.org/). Chef Compliance uses a combination of Effortless Audit and a customized Effortless Remediation package, and gives you the ability to turn controls on and off with a single YAML configuration file.

The Windows and Linux packages in this demonstration work with the Chef Compliance Audit and Remediation content. Find out more about our [Chef Compliance](https://www.chef.io/products/chef-compliance/) offerings.

## Chef Compliance Environment Set-up

1. Install [Chef Workstation](https://downloads.chef.io/chef-workstation)
1. Install [Chef Habitat](https://www.habitat.sh/docs/install-habitat/)
1. Configure Chef Habitat on your workstation by running `hab setup`
1. Create a directory for your Chef Compliance profiles--for example: `mkdir ~/dev`--and change into that directory with `cd dev`

## The Chef Compliance Pattern

The Chef Compliance pattern has two components. The first is the Chef Habitat `plan.sh` or `plan.ps1` file that you will use to invoke the Chef Compliance premium scan and remediation profiles. The second part of the pattern in the `config.yaml` file that you will use for customizing the Chef Compliance profile.

### Create a Chef Habitat Plan

1. Create a plan file for your target operating system (OS). Find out more about [writing plans](https://www.habitat.sh/docs/developing-packages/#write-plans).

   Use a `plan.ps1` for a package targeting Windows. Use a `plan.sh` for a package targeting Linux. If the package targets both Windows and Linux, you can have both a `plan.ps1` and a `plan.sh` in the `habitat` directory. Create a plan in Linux with the following command:

   ```bash
   touch plan.sh
   ```

1. In the next several steps, you will fill out your plan file. Copy this basic content into your plan file and make changes by following the next steps. The content values are the same for both Linux and Windows operating systems. Copy the correct file example for your operating system:

   A Linux plan file:

   ```bash
   pkg_name=<company-CIS-benchmark>
   pkg_origin=chef
   pkg_version="semver"
   pkg_maintainer="org"
   pkg_license=("Apache-2.0")
   pkg_scaffolding="chef/scaffolding-chef-benchmark"
   scaffold_scan_package="chef/<Chef_Compliance_scan_package>"
   scaffold_remediate_package="chef/<Chef_Compliance_remediation_package>"
   ```

   A Windows plan file (note the `$` before the plan variables):

   ```powershell
   $pkg_name=<company-CIS-benchmark>
   $pkg_origin=chef
   $pkg_version="2.2.0"
   $pkg_maintainer="org"
   $pkg_license=("Apache-2.0")
   $pkg_scaffolding="chef/scaffolding-chef-benchmark"
   $scaffold_scan_package="chef/<Chef_Compliance_scan_package>"
   $scaffold_remediate_package="chef/<Chef_Compliance_remediation_package>"
   ```

#### Define Plan Variables

1. Name the package

   Name the package after the CIS benchmark that it contains is the best practice, for example, `company-cis-linux-policy`. You can also name the package for the application stack that the exceptions and remediation changes apply, for example, `redis-company-cis-linux-policy`.

   For `plan.sh` for Linux:

   ```bash
   pkg_name=<company-CIS-benchmark>
   ```

   To your company name:

   ```bash
   pkg_name=4thcoffee-CIS-benchmark
   ```

1. Specify your Chef Habitat origin:

   ```bash
   pkg_origin="chef"
   ```

1. Add a version tag

   Use the same version number as the CIS profile. This best practice helps to avoid confusion as sometimes CIS changes the control IDs between versions.

   ```bash
   pkg_version="2.2.0"
   ```

1. Add a maintainer and license to the plan:

   ```bash
   pkg_maintainer="tls"
   pkg_license=("Apache-2.0")
   ```

1. Add the scaffolding to the plan

   The scaffolding is the most critical part. Scaffolding properly sets up the scan and remediation packages to run on the system. Find out more about [scaffolding](https://www.habitat.sh/docs/glossary/#sts=Scaffolding) in the Chef Habitat documentation.

   ```bash
   pkg_scaffolding="chef/scaffolding-chef-benchmark"
   ```

1. Specify the Effortless scan package or an array of Effortless scan packages that you are using to audit your system

   The Compliance packages in this guide are available to all for demonstration purposes.
   See our [Chef Compliance](https://www.chef.io/products/chef-compliance/) site for more information about our premium packages.

   As a single package:

   ```bash
   scaffold_scan_package="chef/chef-cis-sample-linux-inspec"
   ```

   As an array of packages:

   ```bash
   scaffold_scan_package=("chef/chef-cis-sample-linux-inspec" "chef/simple_profile")
   ```

## The Chef Compliance Remediation Pattern

If you are a Chef Compliance Remediation customer, add your remediation package next.

Remediation is hard! Remediation changes your system to according to the policy that you design. The Chef Compliance Remediation works in three steps:

- Scanning your system according to your Compliance profile
- Remediating your system according to the rules of the `scaffold_remediate_package` and your individual control customizations in the `config.yml`
- Re-scanning your system to confirm its compliance status

Chef Compliance Remediation is a **write** action and requires two components:

- A specified `scaffold_remediate_package` in your plan file
- A valid `config.yml`

In the absence of either of these components, the process stops after the first compliance scan.
The output for the first scan, the remediation, and the second scan are all written to the Chef Habitat Supervisor log.

Because compliance remediation changes your system, its defaults are different than compliance scanning:

- Scan is a `read` action, so it defaults to `true`
- Remediation is a `write` action, so it defaults to `false`

When a remediation action takes place, it works by executing a bash script found the remediation for each control. The script resets the failed setting by changing it to the CIS control specification. For example, if CIS requires the `AIDE` package, then the remediation step installs the package.

Chef InSpec scan:

```ruby
control "1.3.1_Ensure_AIDE_is_installed" do
  title "Ensure AIDE is installed"
  desc  "
    AIDE takes a snapshot of filesystem state including modification times, permissions, and file hashes which can then be used to compare against the current state of the filesystem to detect modifications to the system.
    Rationale: By monitoring the filesystem state compromised files can be detected to prevent or limit the exposure of accidental or malicious misconfigurations or modified binaries.
  "
  impact 1.0
  describe package("aide") do
    it { should be_installed }
  end
end

```

The remediation installs the "AIDE" package onto your system.

### Set Up Chef Compliance Remediation

Set the remediation package in your plan:

```bash
scaffold_remediate_package="chef/Chef_CIS_Sample_Linux_v_1_0_0_remediation"
```

You can set the `dry_run` variable in your `habitat` config, and the remediation will run a dry run for all of its controls and output a report on what actions it would have taken.

Once you have tested the changes and you are ready to remediate the entire profile for CIS, you can set the hab configuration to `all` and it will run all the remediation steps in the `scaffold_remediate_package`.

While this is an option, there is no unset on remediation. The only way to unset a remediation is to change it back using a remediate overlay command.
To protect against this, we run a scan before we run the remediation, so that you can reference the state to help you go back if needed.

### Review Finished Plan

Your plan file is complete.

A Linux plan file:

```bash
pkg_name=cis-linux-benchmark
pkg_origin=chef
pkg_version="2.2.0"
pkg_maintainer="tls"
pkg_license=("Apache-2.0")
pkg_scaffolding="chef/scaffolding-chef-benchmark"
scaffold_scan_package="chef/chef-cis-sample-linux-inspec"
scaffold_remediate_package="chef/Chef_CIS_Sample_Linux_v_1_0_0_remediation"
```

A Windows plan file (note the `$` before the plan variables):

```powershell
$pkg_name=cis-linux-benchmark
$pkg_origin=chef
$pkg_version="2.2.0"
$pkg_maintainer="tls"
$pkg_license=("Apache-2.0")
$pkg_scaffolding="chef/scaffolding-chef-benchmark"
$scaffold_scan_package="chef/chef-cis-sample-linux-inspec"
$scaffold_remediate_package="chef/Chef_CIS_Sample_Linux_v_1_0_0_remediation"
```

### Build and Run Your Package

1. Build the package

   Run the following command to build the package:

   ```bash
   hab pkg build .
   ```

1. You can now upload your profile pkg to Chef Habitat Builder by running the following commands:

   ```bash
   source results/lastbuild.env
   hab pkg upload results/$pkg_artifact
   ```

1. To run your profile on a system, install Chef Habitat as a service and run `hab svc load <my_origin>/<my_profile_name>`

## Customize Chef Compliance

Now that we have the package running, it is time to make customizations. When making a customization, consider if you want to customize the scan or the remediation? Here are a list of available options for scanning and remediation:

|         |Turn off run|Justification|End date|Start Date|Override|Dry-run|
|---------|------------|-------------|--------|----------|--------|-------|
|Scan     | &#9745;    | &#9745;     |        |          |        |       |
|Remediate| &#9745;    | &#9745;     |&#9745; |&#9745;   |&#9745; |&#9745;|

### Custom Compliance Configuration

In this next section, you will build out a custom configuration.

The `config.yml` is the interface to the remediation system. You will express all of your policy customizations in the `config.yml`. You can find the content of failed compliance scans in Chef Automate. Copy the controls for customization from the Chef Automate Compliance > Reporting results into the `config.yml`.

1. In the custom CIS package folder, create a new folder called `config` and a `config.yml` file within that folder:

   ```bash
   cd ~/dev/cis-linux-benchmark
   mkdir config
   touch config.yml
   ```

   Skip ahead and see the [completed configuration example](#completed-config-example) if desired.

1. In order for your remediation to work, you need to add the provider of the benchmark, the name of the benchmark, and the version of the benchmark as metadata as the first three lines of your `config.yml`. Add the metadata:

   ```yaml
   ---
   provider: Chef
   benchmark: CIS Sample Linux
   provider_version: v.1.0.0
   ```

1. Add a permanent exception for a control with scan and remediate turned off:

   ```yaml
   controls:
   - id: <control_id_number>
     scan:
       run: false
     remediate:
       run: false
     justification: "This is an example of a justification"
   ```

1. Turn on a remediation for a control:

   ```yaml
   - id: <control_id_number>
     remediate:
       run: true
   ```

1. Override a control's remediation and disable the scan:

   ```yaml
   - id: <control_id_number>
     scan:
       run: false
     remediate:
       run: true
       overlay_command:
         - local: echo "this is an overlay command"
     justification: "This control is disabled as the company standard is x"
   ```

1. Use one justification in a control:

   ```yaml
   controls:
   - id: <control_id_number>
     scan:
       run: false
       justification: "This is an example of a justification"
     remediate:
       run: false
      # For an overlay command:
      # An overlay command replaces the command in the remediation
      # In this example, the remediation does not install the 'aide' package
      # and instead outputs the overlay_command to the screen
      # run: true
      #  overlay_command:
      #    - local: echo "this is an overlay command"
     justification: "This is an example of another justification. But only use one justification."
   ```

1. Create an exception with an end date:

   ```yaml
   - id: <control_id_number>
     scan:
       run: false
     remediate:
       run: true
       waiver:
         start_date_utc: "--- 2019-10-17 08:25:57.571436000 Z\n"
         expiration_date_utc: "--- 2021-01-20 08:25:57.571522000 Z\n"
         identifier: ticket_12345
     justification: "This is a reason not to run this scan"
   ```

1. Re-build your custom CIS profile in the Chef Habitat studio:

   ```bash
   ~/dev/cis-linux-benchmark $ hab studio enter -D
   ...
   [2][default:/src:0]$ build
      cis-linux-benchmark:
      cis-linux-benchmark: I love it when a plan.sh comes together.
      cis-linux-benchmark:
      cis-linux-benchmark: Build time: 0m36s
   [2][default:/src:0]$
   ```

1. Upload your package to Chef Habitat Builder:

   ```bash
   source results/last_build.env
   hab pkg upload results/$pkg_artifact
   ```

1. It may take a little bit for the profile and the remediation to run, but once complete, you will see the newly skipped controls and the remediated controls.

### Completed Compliance config.yml Example

```yaml
---
provider: Chef
benchmark: CIS Sample Linux
provider_version: v.1.0.0
controls:
- id: 1.1.1.1_Ensure_mounting_of_cramfs_filesystems_is_disabled
  remediate:
    run: true
- id: 1.3.1_Ensure_AIDE_is_installed
  remediate:
    run: true
- id: 5.2.11_Ensure_SSH_PermitEmptyPasswords_is_disabled
  remediate:
    run: true
- id: 5.2.14_Ensure_SSH_access_is_limited
  scan:
    run: false
    justification: "I don't want to disable SSH access at this time: John Snow"
  remediate:
    run: false
- id: 5.2.4_Ensure_SSH_Protocol_is_set_to_2
  remediate:
    run: true
- id: 5.4.1.2_Ensure_minimum_days_between_password_changes_is_7_or_more
  remediate:
    run: true
```

## Summary

That's it! You can quickly change the running controls for exceptions and remediation, and deploy it to your system in minutes.
