+++
title = "Runners"
draft = false
robots = "noindex"


aliases = ["/runners.html", "/job_dispatch.html"]

[menu]
  [menu.legacy]
    title = "Runners"
    identifier = "legacy/workflow/managing_workflow/runners.md Runners"
    parent = "legacy/workflow/managing_workflow"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/runners.md)



Chef Automate's workflow engine automatically creates phase jobs as
project code is promoted through the phases of a workflow pipeline.
These phase jobs are dispatched to special nodes, called runners, that
automatically execute each job as it is created.

{{< warning >}}

ChefDK 2.0 or later should only be installed on runners that are
associated with Chef Automate 1.5 or later. Using ChefDK 2.0 on runners
that are associated with an earlier version of Chef Automate will result
in an error during deployment. If you are running an older version of
Chef Automate, you should either downgrade your runners to use ChefDK
1.x or upgrade to Chef Automate 1.5 or later.

{{< /warning >}}

## Terms

`phase job`

:   A job corresponding to a phase (for example, your build cookbook's
    unit recipe).

`runner`

:   Any node that a job can be dispatched to. Usually refers to a node
    that will run a phase job.

## Managing Runners

### Adding a Runner

You can add a new runner via `automate-ctl` from your Chef Automate
server. Log in to your Chef Automate server and run the
[install-runner](/ctl_automate_server/#install-runner) command.

{{< note >}}

You can pin to a specific ChefDK version through the `--chefdk-version`
option on the `install-runner` command or by using a version of ChefDK
that you have installed locally on your Chef Automate server using the
`-I` option. As an example, this is useful if you have not upgraded your
cookbooks to be Chef Client 13 compliant and the latest version of
ChefDK installs Chef Client 13 on your runner.

{{< /note >}}

After the [install-runner](/ctl_automate_server/#install-runner)
command succeeds, the new runner should show up in the UI under
`Workflow -> Runners -> Manage Runners`. If you see it there, click the
`Test` button. That will test an ssh connection to your runner to verify
that jobs can be dispatched to it. If there are any issues, you should
get an error in the UI.

Supported runner platforms are listed [here](/platforms/#runners).

### Removing a Runner

You can delete a runner via `automate-ctl` from your Chef Automate
server. Log in to your Chef Automate server and run the `delete-runner`
command.

After the `delete-runner` command succeeds, the specified runner should
be deleted from the UI under `Workflow -> Runners -> Manage Runners`.

Runner can also be removed using the `delivery-cli-api` command.

To see a list of runners:

``` bash
delivery api get runners
```

To delete a runner:

``` bash
delivery api delete runners/<runner_hostname>
```

### Upgrading the version of ChefDK on a Runner

If you need to upgrade the version of ChefDK on your runner, you can do
so by logging into the runner, upgrading ChefDK, and manually appending
the Chef Automate server certificate to the cert file that ships in
ChefDK.

Typically, we recommend re-running the `install-runner` command rather
than manually updating as the installation process will take care of
this certification change for you when it bootstraps the node.

## Configuring Chef Automate Projects

Chef Automate 0.6 or later can use runners, and when setting up a
project using `delivery setup`, ChefDK v1.1.16 or later specifies the
use of runners in the `./delivery/config.json` file. If you are running
an older version of ChefDK, or your `config.json` was set up to use Push
Jobs-based build nodes, you must edit the file in the following manner:

At the bare minimum, you must set the version to v2:

``` javascript
{
   ...
   "job_dispatch": {
      "version": "v2"
   },
   ...
}
```

and remove the `build_nodes` setting from `config.json`.

``` none
"build_nodes": {
  "default"    : ["name:name_of_builder"]
},
```

You can also set which runners you want jobs to run on for your project.
You can set default, per phase, and matrix per phase filters to
customize exactly which runners are targeted at various points of your
pipeline. Refer to the [job_dispatch configuration
settings](/config_json_delivery/#job-dispatch-config-settings) for
more details and examples.

For more detail on `config.json`, see its
[config.json](/config_json_delivery/).

## Cancelling Jobs

You can cancel queued or running phase jobs in the new job dispatch
system. Simply click the trash can in the UI next to a phase run from
the change view for the job you wish to cancel.

## Managing and Inspecting Jobs

You can see the job queue, runnning jobs, what your runners are
currently doing, runner health, and so on. Navigate to <span
class="title-ref">Workflow -\> Runners</span> in the UI to see all the
possibilities.

## Job Dispatch and Push Jobs

Any project configured to use runners will not use Push Jobs as the
transport mechanism for managing the phase builds (unit, lint,
provision, etc.). Push Jobs is still required to execute the <span
class="title-ref">delivery_push_job</span> resource that the
delivery-sugar cookbook exposes. This means that if you use the default
[deploy.rb](https://github.com/chef-cookbooks/delivery-truck/blob/b9e386e720376f7f3173ca03311cba667eb7ef4b/recipes/deploy.rb)
recipe from delivery-truck, then Push Jobs is still used within the
deploy phase.

The SSH-based Job Dispatch system used with runners is not a replacement
for Push Jobs. Job Dispatch is a targeted solution for managing phase
builds and Push Jobs allows users to perform remote tasks on pools of
nodes. Job Dispatch uses SSH connections and allows additional features,
such as cancelling jobs.
