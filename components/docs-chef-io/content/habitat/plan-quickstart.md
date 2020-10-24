+++
title = "Plan Quickstart"
draft = false

[menu]
  [menu.habitat]
    title = "Plan Quickstart"
    identifier = "habitat/plans/plan-quickstart Plan Quickstart"
    parent = "habitat/plans"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/plan-quickstart.md)

All plans must have a `plan.sh` or `plan.ps1` at the root of the plan context. They may even include both if a package is targeting both Windows and Linux platforms. This file will be used by the `hab-plan-build` command to build your package. To create a plan, do the following:

1. If you haven't done so already, [download the `hab` CLI](/install-habitat/) and install it per the instructions on the download page.

2. Run `hab cli setup` and follow the instructions in the setup script.

3. The easiest way to create a plan is to use the `hab plan init` subcommand. This subcommand will create a directory, known as the plan context, that contains your plan file and any runtime hooks and/or templated configuration data.

    To use `hab plan init` as part of your project repo, navigate to the root of your project repo and run `hab plan init`. It will create a new `habitat` sub-directory with a plan.sh (or plan.ps1 on Windows) based on the name of the parent directory, and include a `default.toml` file as well as `config` and `hooks` directories for you to populate as needed. For example:

    ```bash
    cd /path/to/<reponame>
    hab plan init
    ```

    will result in a new `habitat` directory located at `/path/to/<reponame>/habitat`. A plan file will be created and the `pkg_name` variable will be set to _\<reponame\>_. Also, any environment variables that you have previously set (such as `HAB_ORIGIN`) will be used to populate the respective `pkg_*` variables.

    If you want to auto-populate more of the `pkg_*` variables, you also have the option of setting them when calling `hab plan init`, as shown in the following example:

    ```bash
    env pkg_svc_user=someuser pkg_deps="(core/make core/coreutils)" \
       pkg_license="('MIT' 'Apache-2.0')" pkg_bin_dirs="(bin sbin)" \
       pkg_version=1.0.0 pkg_description="foo" pkg_maintainer="you" \
       hab plan init yourplan
    ```

     See [hab plan init](/habitat-cli#hab-plan-init) for more information on how to use this subcommand.

4. Now that you have stubbed out your plan file in your plan context, open it and begin modifying it to suit your needs.

When writing a plan, it's important to understand that you are defining both how the package is built and the actions Chef Habitat will take when the Supervisor starts and manages the child processes in the package. The following sections explain what you need to do for each phase.

