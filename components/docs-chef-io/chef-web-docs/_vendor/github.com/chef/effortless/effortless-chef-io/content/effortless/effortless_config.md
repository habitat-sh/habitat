+++
title = "Effortless Config"
draft = false

[menu]
  [menu.effortless]
    title = "Effortless Config"
    identifier = "effortless/effortless_config.md Effortless config"
    parent = "effortless"
    weight = 30
+++

# Effortless Config

Effortless Config is the pattern for managing your Chef Infra workloads. It uses [Chef Habitat](https://www.habitat.sh/docs/) and [Chef Policyfiles](https://docs.chef.io/policyfile/) to build an artifact that contains the cookbooks and their dependencies alongside the scripts necessary to run them on your systems.

## Effortless Environment Set-up

1. Install [Chef Workstation](https://downloads.chef.io/chef-workstation)
1. Install [Chef Habitat](https://www.habitat.sh/docs/install-habitat/)
1. Configure Chef Habitat on your workstation by running `hab setup`

## Patterns

### Chef Repo Cookbook Pattern

This pattern uses the [chef-repo](https://docs.chef.io/chef_repo/) to store and organize everything you need to define your infrastructure with Chef Infra, including:

- Cookbooks (including recipes, attributes, custom resources, libraries, and templates)
- Data bags
- Policyfiles

The Chef Effortless GitHub repository has an [example chef-repo](https://github.com/chef/effortless/tree/master/examples/effortless_config/chef_repo_pattern) for you to explore.

1. To use this pattern, navigate to the chef-repo directory that you want to use:

   ```bash
   cd chef-repo
   ```

1. Create a `habitat` directory from the command line with:

   ```bash
   mkdir habitat
   ```

1. Make a plan file

   Make  a `plan.ps1` for a cookbook that targets Microsoft Windows systems and a `plan.sh` for a cookbook that targets Linux systems. You can have both a `plan.ps1` and a `plan.sh` in the `habitat` directory for cookbooks that target both systems. Create a Linux plan:

   ```bash
   touch plan.sh
   ```

1. Add information about the cookbook to the plan

   Add this information to the `plan.sh` file:

   ```bash
   pkg_name=<my_policyfile>
   pkg_origin=<my_origin>
   pkg_version="0.1.0"
   pkg_maintainer="My Name and Email"
   pkg_license=("Apache-2.0")
   pkg_scaffolding="chef/scaffolding-chef-infra"
   pkg_svc_user=("root")
   scaffold_policy_name="<my_policyfile>"
   ```

1. If you do not have a `policyfiles` directory in your chef-repo, create one:

   ```bash
   mkdir policyfiles
   ```

1. Generate a Policyfile:

  ```bash
  chef generate policyfile policyfiles/my_policyfile
  ```

   Example of a `policyfile.rb`:

   ```ruby
   # Policyfile.rb - Describe how Chef Infra should build your system.
   #
   # For more information on the Policyfile feature, visit
   # https://docs.chef.io/policyfile.html

   name "base"

   # Where to find external cookbooks
   default_source :supermarket
   default_source :chef_repo, "../"

   # run_list: run these recipes in the order specified.
   run_list [
   "patching::default",
   "hardening::default"
   ]

   # attributes: set attributes from your cookbooks
   default['hardening'] = {}

   default['patching'] = {}

   ```

1. Build the package:

   Run the following command to build the package:

   ```bash
   hab pkg build .
   ```

1. Edit the `kitchen.yml` file to look similar to this:

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

   echo "Running chef for $pkg_origin/$pkg_name"
   cd $pkg_prefix
   hab pkg exec $pkg_origin/$pkg_name chef-client -z -c $pkg_prefix/config/bootstrap-config.rb
   ```

1. Run Test Kitchen to ensure the cookbook works

   Use this command to spin up a CentOS 7 virtual machine (VM) locally and run the cookbook using the latest Chef Infra Client:

   ```bash
   kitchen converge base-centos
   ```

   If you experience errors in this Chef run, you may need to supply attributes or make modifications to your Policyfile, so that your cookbook can run using the latest Chef Infra Client.

1. When ready, delete the VM instance by running:

   ```bash
   kitchen destroy
   ```

1. Upload the Policyfile pkg to Chef Habitat builder by running the following commands:

   ```bash
   source results/lastbuild.env
   hab pkg upload results/$pkg_artifact
   ```

1. To run the Policyfile on a system, install Chef Habitat services and run:

   ```bash
   hab svc load <my_origin>/<my_policyfile_name>
   ```

### Policyfile Cookbook Pattern

This pattern builds a Chef Habitat artifact for the Policyfile cookbook. You can find an [example Policyfile cookbook](https://github.com/chef/effortless/tree/master/examples/effortless_config/policyfile_cookbook_pattern) in the Chef Effortless GitHub repository.

1. To use this pattern, navigate to the cookbook you want to use:

   ```bash
   cd chef-repo/cookbooks/my_cookbook
   ```

1. Create a `habitat` directory from the command line with:

   ```bash
   mkdir habitat
   ```

1. Make a plan file

   Use a `plan.ps1` for a cookbook targeting Windows. Use a `plan.sh` for a cookbook targeting Linux. If the cookbook targets both Windows and Linux, you can have both a `plan.ps1` and a `plan.sh` in the `habitat` directory. Create a plan in Linux with the following command:

   ```bash
   touch plan.sh
   ```

1. Add some information about the cookbook to the plan

   plan.sh

   ```bash
   pkg_name=<Name of my_cookbook artifact>
   pkg_origin=<my_origin>
   pkg_version="<Cookbook version>"
   pkg_maintainer="<My Name>"
   pkg_license=("<License for my_cookbook example Apache-2.0>")
   pkg_scaffolding="chef/scaffolding-chef-infra"
   scaffold_policy_name="Policyfile"
   scaffold_policyfile_path="$PLAN_CONTEXT/../" # habitat/../Policyfile.rb
   ```

1. Make a Policyfile in the `cookbook` directory

   Example of a `policyfile.rb` file:

   ```ruby
   # Policyfile.rb - Describe how you want Chef to build your system.
   #
   # For more information on the Policyfile feature, visit
   # https://docs.chef.io/policyfile.html

   # A name that describes what the system you're building with Chef does.
   name '<my_cookbook_name>'

   # Where to find external cookbooks:
   default_source :supermarket

   # run_list: chef-client will run these recipes in the order specified.
   run_list '<my_cookbook_name>::default'

   # Specify a custom source for a single cookbook:
   cookbook '<my_cookbook_name>', path: '.'
   ```

1. Build the package:

   ```bash
   hab pkg build <my_cookbook>
   ```

1. Edit the `kitchen.yml` file to look similar to this:

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
         arguments: ["<my_origin>", "<my_cookbook>"]
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

   echo "Running chef for $pkg_origin/$pkg_name"
   cd $pkg_prefix
   hab pkg exec $pkg_origin/$pkg_name chef-client -z -c $pkg_prefix/config/bootstrap-config.rb
   ```

1. Run Test Kitchen to ensure the cookbook works on Linux

   Use this command to spin up a CentOS 7 virtual machine (VM) locally and run the cookbook using the latest Chef Infra Client:

   ```bash
   kitchen converge base-centos
   ```

   If you experience errors in this Chef run, you may need to supply attributes or make modifications to your Policyfile, so that your cookbook can run using the latest Chef Infra Client.

1. When ready, delete the VM instance by running:

   ```bash
   kitchen destroy
   ```

1. Upload the habitat pkg to Chef Habitat builder by running the following commands:

   ```bash
   source results/lastbuild.env
   hab pkg upload results/$pkg_artifact
   ```

1. To run the cookbook on a system, install Chef Habitat services and run:

   ```bash
   hab svc load my_origin/my_cookbook
   ```
