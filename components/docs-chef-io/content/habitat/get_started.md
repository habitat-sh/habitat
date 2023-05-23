+++
title = "Get Started with Chef Habitat"
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Get Started"
    identifier = "habitat/get_started"
    parent = "habitat"
    weight = 5
+++

This getting started guide will show you how to use Chef Habitat to build and deploy a Node.js application.

## Prerequisites

Before getting started with this tutorial, you will need:

- a workstation running Linux or macOS
- a [GitHub account](https://github.com/join)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) installed locally (optional)
- the [Chef Habitat CLI]({{< relref "/habitat/install_habitat" >}}) installed locally
- an account on [Chef Habitat Builder]({{< relref "builder_account" >}})
- a [profile on your Builder account]({{< relref "builder_profile" >}})

### Create an origin and set up the Habitat CLI

1. If you don't already have one, [create an origin]({{< relref "habitat/builder_origins" >}}) in your Chef Habitat Builder account. This origin will store the Habitat package that you create in this tutorial.

1. If you haven't already done so, run `hab cli setup` and follow the instructions. This will configure the Hab CLI to work with your origin on the Habitat Builder SaaS.

   You will need to provide the origin you created in your Builder account and a personal access token, which you can generate in your [Profile page in Habitat Builder](https://bldr.habitat.sh/#/profile).

## Fork and Clone the Habitat sample-node-app repository

1. Create a fork of the [sample-node-app](https://github.com/habitat-sh/sample-node-app) GitHub repository in your own GitHub account.

1. Clone the fork onto your local workstation.

## Edit the Plan File

The [plan file]({{< relref "/habitat/plan_writing" >}}) instructs Chef Habitat in how to build, deploy, and manage your application.

Edit the plan file to use your origin:

1. Open the `sample-node-app/habitat/plan.sh` file with your editor of choice.
1. Update the value of `pkg_origin` with the name of the origin in your Habitat Builder account.

Note the `pkg_deps` setting. This lists dependencies that your package requires at runtime. This Node.js application requires the [`core/node` package](https://bldr.habitat.sh/#/pkgs/core/node/latest), which is maintained by the Habitat Core maintainers. You can see a full list of core packages in the [core-plans](https://github.com/habitat-sh/core-plans) repository, or by searching for **core** in Habitat Builder.

See the [plan settings]({{< relref "/habitat/plan_settings" >}}) documentation for more information about the settings in a plan file.

See the [plan writing]({{< relref "/habitat/plan_writing#define-your-dependencies" >}}) documentation for more information defining dependencies.

## Run the Service in the Habitat Studio

{{< readfile file="content/habitat/reusable/md/habitat_studio_overview.md" >}}

Use the Habitat Studio to generate a Habitat package and preview the running Node.js app:

1. In a terminal, switch to the root of the `sample-node-app` directory.

1. If you're using Docker, expose port 8000 so you can preview the application in a browser.

   To expose port 8000, run:

   ```bash
   export HAB_DOCKER_OPTS="-p 8000:8000"
   ```

1. Enter the Habitat Studio.

   On macOS run:

   ```bash
   hab studio enter
   ```

   On Linux run:

   ```bash
   hab studio enter -D
   ```

1. Now build the package.

   ```bash
   build
   ```

   The Studio will generate a package (a `.hart` file) and place it in the `results` directory.
   You will also find a `last_build.env` file in the `results` directory that has metadata about the last package build.

1. Start the service.

   ```bash
   hab svc load ORIGIN_NAME/sample-node-app
   ```

1. Verify that the service is running in the Habitat Studio.

   ```bash
   hab svc status
   ```

   This should return a status similar to:

   ```bash
   package                                           type        desired  state  elapsed (s)  pid   group
   ORIGIN_NAME/sample-node-app/1.1.0/20221018162840  standalone  up       up     58           3047  sample-node-app.default
   ```

   Now you can navigate to **localhost:8000** in your browser to see the running service.

### Shut Down the Service

1. Stop the service.

   ```bash
   hab svc stop ORIGIN_NAME/sample-node-app
   ```

1. Verify that the service has stopped running.

   ```bash
   hab svc status
   ```

1. And exit the Studio.

   ```bash
   exit
   ```

## Manage the Service with Habitat Builder

[Chef Habitat Builder]({{< relref "/habitat/builder_overview" >}}) acts as the core of Chef’s Application Delivery Enterprise hub. The Chef Habitat Builder SaaS stores plan files where they can be viewed and accessed by the Chef Habitat community and then deployed by the Chef Habitat Supervisor.

Add your package to your Builder origin and then connect that package to your clone of the sample-node-app repository. Any changes made to your clone of the sample-node-app will trigger the Chef Habitat Builder to rebuild the package.

1. If you haven't already done so, commit the changes you made earlier to the `plan.sh` file into the default branch of your clone of the sample-node-app repository, and push those changes up to your fork on GitHub.

   The `pkg_origin` setting in the `plan.sh` file should be set to your origin.

1. Log in to your account in [Chef Habitat Builder](https://bldr.habitat.sh) and select your origin.
1. Create a new package by selecting **Create Package**.

   The name of the package you create must match the package name defined by the `pkg_name` setting in the `plan.sh` file.

1. Connect the plan file to the new package by selecting **Connect a plan file** and then **for Linux (kernel version 3.2 or later)**.
1. Select your GitHub organization or user account under **Organization**, and then select **sample-node-app** under **Repository**.
1. Select **Save Connection** at the bottom of the page.

1. Once the plan is connected, select **Latest** towards the top of the page and build the package by selecting **Build latest version** on the right side of the page.

   {{< figure src="/images/habitat/builder-build-latest-version.png" alt="Build latest version button." width=250 >}}

   Select **View the output** and then the **View build job** button ({{< svg file="/static/images/habitat/builder-view-build-job-icon.svg" >}}) to watch the service building in Habitat Builder.

1. After Habitat Builder finishes building the service, select the **Latest** tab, and then select **Promote** to promote the service to the stable channel.

## Run the Service From Habitat Builder in the Habitat Studio

Now that the Node.js app is under management by the Chef Habitat Builder, you can update the service from Habitat Builder.

1. From a local terminal, enter the Habitat Studio.

   ```bash
   hab studio enter
   ```

1. Verify that a service is not running in the Studio.

   ```bash
   hab svc status
   ```

1. Load the service into the Studio using the at-once update strategy.

   The [at-once update strategy]({{< relref "/habitat/service_group_updates#at-once-strategy" >}}) will pull down the package from Habitat Builder whenever it detects that a new version has been released.

   ```bash
   hab svc load ORIGIN_NAME/sample-node-app --strategy at-once
   ```

1. Now monitor the Habitat Supervisor to watch it update when a new version of the package is created.

   ```bash
   sup-log
   ```

   This will show you a streaming log of the Habitat Supervisor output.

1. Back in the Habitat Builder web UI, rebuild the service by selecting **Build latest version** again.

1. When Habitat Builder finishes creating a new build of the service, select the **Versions** tab, then select the version of the sample-node-app (1.1.0), and then select **Promote to stable** next to the newest build of the service.

   In your local terminal, the Supervisor log will show your package updating in the Habitat Studio.

   See the [Supervisor Log Codes]({{< relref "/habitat/sup_log_keys" >}}) documentation for an explanation of the different Supervisor log codes.

## Rebuild the Service From Your GitHub Repository

From here you can try updating the service by making a change to the sample-node-app repository.

1. In your local clone of the sample-node-app repository, update the `pkg_version` to `1.2.0` in the `plan.sh` file.

   You might also try updating the message in `habitat/config/config.json` to something like: `Hello friend. This is version {{ pkg.version }} of the Habitat Node.js sample app.`

1. Commit the change into the default branch of the sample-node-app repository and push the commit up to your fork of sample-node-app.

1. If your terminal is still running the Supervisor log, continue to the next step. Otherwise, rerun the steps to load the service and view the Supervisor log.

   1. Expose port 8000.

      ```bash
      export HAB_DOCKER_OPTS="-p 8000:8000"
      ```

   1. Enter the Studio and load the service using the at-once update strategy.

      ```bash
      hab studio enter
      hab svc load ORIGIN_NAME/sample-node-app --strategy at-once
      ```

   1. View the Supervisor log.

      ```bash
      sup-log
      ```

1. In [Habitat Builder](https://bldr.habitat.sh/), select **Build latest version** of your package again, then **View the output**, and select the **View build job** button ({{< svg file="/static/images/habitat/builder-view-build-job-icon.svg" >}}).

   {{< note >}}

   If you get an error after selecting **Build latest version**, wait a few minutes and try again. It can take Builder a few minutes to update after changes are made in a GitHub repository.

   {{< /note >}}

1. After Habitat Builder has finished building the new version of your package, select the **Versions** tab, select the **1.2.0** row, and then select **Promote to stable** and **Yes, promote it**.

   The Supervisor should show a log in your terminal of the package updating to the latest version. After the Supervisor is done updating the service, navigate to **localhost:8000** in your browser and the webpage should show the updated version number and text.
