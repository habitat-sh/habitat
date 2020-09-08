 +++
title = "Runners"

draft = false
[menu]
  [menu.automate]
    title = "Runners"
    parent = "automate/workflow"
    identifier = "automate/workflow/runners.md Runners"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/runners.md)

Workflow is a legacy feature for Chef Automate, which was designed for managing changes to both infrastructure and application code, giving your operations and development teams a common platform for developing, testing, and deploying cookbooks, applications, and more.

{{< warning >}}
Workflow is available in Chef Automate for existing users. If you are not already using Workflow, but are interested in the solution it offers, please contact your sales or success representative for support with continuous integration pipelines.
{{< /warning >}}

Chef Automate's workflow engine automatically creates phase jobs as project code is promoted through the phases of a workflow pipeline. These phase jobs are dispatched to special nodes, called runners, that automatically execute each job as it is created.

## Prerequisites

* Workflow runners for Chef Automate 2 require the latest version of [ChefDK](https://downloads.chef.io/chefdk/3.7.23), which includes Chef Infra Client 14.10. The runners will not function unless upgraded.
* Cookbooks used with Workflow for Chef Automate must be upgraded to work with Chef 14.10 or newer.
* Chef Automate requires either the latest version of [ChefDK](https://downloads.chef.io/chefdk/3.7.23), which includes the `delivery-cli` that supports Workflow.

## Terms

phase job
: A job corresponding to a phase (for example, your build cookbook's unit recipe).

runner
: Any node that a job can be dispatched to. Usually refers to a node that will run a phase job.

## Managing Runners

### Add a Runner

You can add a new runner via `workflow-ctl` from your Chef Automate server. Log in to your Chef Automate server and run the [install-runner](/ctl_automate_server.html#install-runner) command.

After the [install-runner](/ctl_automate_server.html#install-runner) command succeeds, the new runner will appear in the **Manage Runners** tab in the **Workflow** area on the _Client Runs_ page. Selecting  the `Test` button verifies that you can dispatch jobs to the runner by opening a ssh connection to it. If the test fails and the runner is unreachable, an error should appear in the UI.

Supported runner platforms are listed [here](https://docs.chef.io/platforms/#chef-automate-job-runners).

### Removing a Runner

You can delete a runner via `workflow-ctl` from your Chef Automate server. Log in to your Chef Automate server and run the `delete-runner` command.

After the `delete-runner` command succeeds, the runner should no longer appear in the **Manage Runners** tab of the **Workflow** area on the _Client Runs_ page

Runner can also be removed using the `delivery-cli-api` command.

To see a list of runners:

```bash
$ delivery api get runners
```

To delete a runner:

```bash
$ delivery api delete runners/<runner_hostname>
```

### Upgrading the version of ChefDK on a Runner

We recommend re-running the `install-runner` command rather than manually updating runners, as the installation process manages the Chef Automate server certification change for you when it bootstraps the node.

To upgrade the version of ChefDK on your runner manually, log into the runner, upgrade ChefDK, and manually append the Chef Automate server certificate to the cert file that ships in ChefDK.
