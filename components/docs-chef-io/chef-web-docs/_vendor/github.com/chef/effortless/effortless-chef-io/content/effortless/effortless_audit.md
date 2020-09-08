+++
title = "Effortless Audit"
draft = false

[menu]
  [menu.effortless]
    title = "Effortless Audit"
    identifier = "effortless/effortless_audit.md Effortless Audit"
    parent = "effortless"
    weight = 20
+++


# Effortless Audit

Effortless Audit is the pattern for managing your Chef InSpec profiles. It uses [Chef Habitat](https://www.habitat.sh/docs/) and [Chef InSpec](https://www.inspec.io/docs/) to build an artifact that contains your profiles and its dependencies alongside the scripts necessary to run them on your systems.

Learn more about [Chef InSpec profiles](https://www.inspec.io/docs/reference/profiles/).

## Effortless Environment Set-up

1. Install [Chef Workstation](https://downloads.chef.io/chef-workstation)
1. Install [Chef Habitat](https://www.habitat.sh/docs/install-habitat/)
1. Configure Chef Habitat on your workstation by running `hab setup`

## Patterns

### Wrapper Profile Pattern

In Chef InSpec, a common pattern is to write a wrapper profile that depends on another profile. This pattern pulls profiles from a main profile source like the [Chef Automate Profile Store](https://automate.chef.io/docs/profiles/). See an [example of this pattern](https://github.com/chef/effortless/tree/master/examples/effortless_audit).

1. To use this pattern, navigate to your profile:

   ```bash
   cd my_profile
   ```

1. Make a `habitat` directory:

   ```bash
   mkdir habitat
   ```

1. Make a plan file

   Use a `plan.ps1` for a profile targeting Windows. Use a `plan.sh` for a profile targeting Linux. If the profile targets both Windows and Linux, you can have both a `plan.ps1` and a `plan.sh` in the `habitat` directory. Create a plan in Linux with the following command:

   ```bash
   touch plan.sh
   ```

1. Add some information about your profile to the plan file

   Add this profile information to the Linux `plan.sh` file:

   ```bash
   pkg_name=<my_profile>
   pkg_origin=<my_origin>
   pkg_version=<my_profile_version>
   pkg_maintainer="Your Name and Email"
   pkg_license=("Apache-2.0")
   pkg_scaffolding="chef/scaffolding-chef-inspec"
   ```

   Add this profile information to the Microsoft Windows `plan.ps1` file:

   ```powershell
   $pkg_name="<my_profile>"
   $pkg_origin="<my_origin>"
   $pkg_version="<my_profile_version>"
   $pkg_maintainer="My Name and Email"
   $pkg_license=("Apache-2.0")
   $pkg_scaffolding="chef/scaffolding-chef-inspec"
   ```

1. Build the package

   Run the following command to build the package:

   ```bash
   hab pkg build .
   ```

1. Add a `kitchen.yml` file to your profile with the following content:

   ```yml
   ---
   driver:
     name: vagrant
     synced_folders:
       - ["./results", "/tmp/results"]

   provisioner:
     name: shell

   verifier:
     name: inspec

   platforms:
     - name: centos-7.6

   suites:
     - name: base
       provisioner:
         arguments: ["<my_origin>", "<my_package_name>"]
       verifier:
         inspec_tests:
           test/integration/base
   ```

1. Create a `bootstrap.sh` script and include:

   ```bash
   #!/bin/bash
   export HAB_LICENSE="accept-no-persist"
   export CHEF_LICENSE="accept-no-persist"

   if [ ! -e "/bin/hab" ]; then
   curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
   fi

   if grep "^hab:" /etc/passwd > /dev/null; then
   echo "Hab user exists"
   else
   useradd hab && true
   fi

   if grep "^hab:" /etc/group > /dev/null; then
   echo "Hab group exists"
   else
   groupadd hab && true
   fi

   pkg_origin=$1
   pkg_name=$2

   echo "Starting $pkg_origin/$pkg_name"

   latest_hart_file=$(ls -la /tmp/results/$pkg_origin-$pkg_name* | tail -n 1 | cut -d " " -f 9)
   echo "Latest hart file is $latest_hart_file"

   echo "Installing $latest_hart_file"
   hab pkg install $latest_hart_file

   echo "Determining pkg_prefix for $latest_hart_file"
   pkg_prefix=$(find /hab/pkgs/$pkg_origin/$pkg_name -maxdepth 2 -mindepth 2 | sort | tail -n 1)

   echo "Found $pkg_prefix"

   echo "Running inspec for $pkg_origin/$pkg_name"
   cd $pkg_prefix
   hab pkg exec $pkg_origin/$pkg_name inspec exec $pkg_prefix/*.tar.gz
   ```

1. Run Test Kitchen to ensure your profile executes.

   Use this command to spin up a CentOS 7 virtual machine (VM) locally and run your profile using the latest Chef InSpec:

   ```bash
   kitchen converge base-centos
   ```

   If you experience failures when running the profile, know that most basic virtual machines are not fully hardened to your security policies. If you want to fix the failures, look at [Chef Infra and the Effortless Config Pattern](effortless-config.md).

1. When ready, delete the VM instance by running:

   ```bash
   kitchen destroy
   ```

1. You can now upload your profile pkg to Chef Habitat Builder by running the following commands:

   ```bash
   source results/lastbuild.env
   hab pkg upload results/$pkg_artifact
   ```

1. To run your profile on a system, install Chef Habitat as a service and run:

   ```bash
   hab svc load <your_origin>/<your_profile_name>
   ```

## Features

### Waivers

With the release of `scaffolding-chef-inspec` version 0.16.0 (Linux) and version 0.18.0 (Windows), we added the Chef InSpec Waivers feature. This feature allows you to specify a control ID in your Chef Habitat config that you would like to skip, or waive.

1. Build an Effortless Audit profile and load it on your systems.
1. Create a `my_config.toml` file similar to:

   ```toml
   [waivers]
   [waivers.control_id]
   run = false
   expiration_date: 2021-11-31
   justification = I don't want this control to run cause it breaks my app
   ```

1. Apply the new change to your Chef Habitat config:

   ```bash
   hab config apply <my_profile_service>.<my_profile_service_group> $(date) <my_config.toml>
   ```

1. Habitat will see a configuration change, automatically re-run your profile, and skip the control you specified in the `my_config.toml` file.
