+++
title = "Exporting Packages"
description = "Export Chef Habitat packages to Docker, Kubernetes, Helm, Mesos, DC/OS, Cloud Foundry, or as a tarball "
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Exporting Packages"
    identifier = "habitat/packages/pkg-exports Export Chef Habitat Packages"
    parent = "habitat/packages"
    weight = 50
+++

Chef Habitat Artifacts--`.hart` files--can be exported in a number of different formats depending on what you need and where you need it. This is powerful because you can use the same immutable Chef Habitat artifact by exporting it into a format that you need for a specific job.

You can export packages into several different external, immutable runtime formats. Currently there are exports for: docker, mesos, tar, and cloudfoundry.

The command to export a package is `hab pkg export <FORMAT> <PKG_IDENT>`. See the [Chef Habitat CLI Reference Guide]({{< relref "habitat_cli#hab-pkg-export" >}}) for more CLI information.

> **Note** If you specify an `origin/package` identifier, such as `core/postgresql`, the Chef Habitat CLI will check Builder for the latest stable version of the package and export that.

> If you wish to export a package that is not on Builder, create a Chef Habitat artifact by running the `build` command, then point `hab pkg` to the `.hart` file within the `/results` directory:

```bash
hab pkg export tar ./results/example-app.hart
```

Read on for more detailed instructions.

### Exporting to Docker

You can create a Docker container image for any package by performing the following steps:

1. Ensure you have a Docker daemon running on your host system. On Linux, the exporter shares the Docker socket (`unix:///var/run/docker.sock`) into the studio.

1. Create an interactive studio with the `hab studio enter` command.

1. [Build]({{< relref "pkg_build" >}}) the Chef Habitat package from which you want to create a Docker container image and then run the Docker exporter on the package.

    ```bash
    hab pkg export docker ./results/<hart-filename>.hart
    ```

    > **Note** The command above is for local testing only. If you have uploaded your package to Builder, you can export it by calling `hab pkg export docker origin/package`. The default is to use the latest stable release; however, you can override that by specifying a different channel in an optional flag.

    > **Note** On Linux, exporting your Chef Habitat artifact to a Docker image requires the Docker Engine supplied by Docker. Packages from distribution-specific or otherwise alternative providers are currently not supported.

    > **Note** In a Windows container studio, the `export` command will not be able to access the host docker engine. To export a Windows package or hart file built inside of a Windows container studio, first exit the studio and then export the `.hart` file in your local `results` directory.

1. You may now exit the studio. The new Docker image exists on your computer and can be examined with `docker images` or run with `docker run`.

1. Please note that when you run this docker container, you will need to pass the `HAB_LICENSE` environment variable into the container in order to accept the Habitat license. If you don't, your container will abort at a license acceptance prompt. One way to do this would be `docker run --env HAB_LICENSE=accept-no-persist IMAGE`. Alternatively, if you use a scheduler to run these docker containers, you should add that environment variable to your scheduler configuration.

### Exporting to a Tarball

1. Enter the Chef Habitat studio by using `hab studio enter`.

2. Install or [build]({{< relref "pkg_build" >}}) the Chef Habitat package from which you want to create a tarball, for example:

    ```bash
    hab pkg install <ORIGIN>/<NAME>
    ```

3. Run the tar exporter on the package.

    ```bash
    hab pkg export tar <ORIGIN>/<NAME>
    ```

    If you receive an error, try running

    ```bash
    hab pkg export tar /results/<your_package>.hart
    ```

4. Your package is now in a tar file that exists locally on your computer in the format `<ORIGIN>-<NAME>-<VERSION>-<TIMESTAMP>.tar.gz` and can be deployed and run on a target machine.

5. If you wish to run this tar file on a remote machine (i.e. a virtual machine in a cloud environment), scp (or whatever transfer protocol you prefer) the file to whatever you wish to run it.

6. SSH into the virtual machine

7. Run these commands to set up the required user and group:

    ```bash
    sudo adduser --group hab
    sudo useradd -g hab hab
    ```

8. Next, unpack the tar file:

    ```bash
    sudo tar xf your-origin-package-version-timestamp.tar.gz
    sudo cp -R hab /hab
    ```

9. Now, start the Supervisor and load your service package using the `hab` binary, which is included in the tar archive:

    ```bash
    sudo /hab/bin/hab sup run
    sudo /hab/bin/hab svc load <ORIGIN>/<NAME>
    ```

### Exporting to Apache Mesos and DC/OS

1. Create an interactive studio in any directory with the `hab studio enter` command.

2. Install or [build]({{< relref "pkg_build" >}}) the Chef Habitat package from which you want to create a Marathon application, for example:

    ```bash
    hab pkg install <ORIGIN>/<NAME>
    ```

3. Run the Mesos exporter on the package.

    ```bash
    hab pkg export mesos <ORIGIN>/<NAME>
    ```

4. This will create a Mesos container-format tarball in the results directory, and also print the JSON needed to load the application into Marathon. Note that the tarball needs to be uploaded to a download location and the "uris" in the JSON need to be updated manually. This is an example of the output:

    ```json
    { "id": "yourorigin/yourpackage", "cmd": "/bin/id -u hab &>/dev/null || /sbin/useradd hab; /bin/chown -R hab:hab *;
    mount -t proc proc proc/; mount -t sysfs sys sys/;mount -o bind /dev dev/; /usr/sbin/chroot . ./init.sh start
    yourorigin/yourpackage", "cpus": 0.5, "disk": 0, "mem": 256, "instances": 1, "uris":
    ["https://storage.googleapis.com/mesos-habitat/yourorigin/yourpackage-0.0.1-20160611121519.tgz" ] }
    ```

5. Note that the default resource allocation for the application is very small: 0.5 units of CPU, no disk, one instance, and 256MB of memory. To change these resource allocations, pass different values to the Mesos exporter as command line options (defaults are documented with `--help`).

6. See the [Apaches Mesos and DC/OS documentation]({{< relref "mesos_dcos" >}}) for more information on getting your application running on Mesos.

### Exporting to Cloud Foundry

Packages can be exported to run in a [Cloud Foundry platform](https://www.cloudfoundry.org/certified-platforms/) through the use of a Docker image that contains additional layers meant to handle mapping from the Cloud Foundry environment to a Chef Habitat default.toml file.

#### Setting up Docker Support in Cloud Foundry

If you have not done so already, you must enable Docker support for Cloud Foundry before you can upload your Cloud Foundry-specific Docker image.

To do so, make sure you have done the following:

1. Log in as an Admin user.
2. Enable Docker support on your Cloud Foundry deployment by enabling the `diego_docker` feature flag.

```bash
cf enable-feature-flag diego_docker
```

#### Creating a Mapping File

The mapping file is a TOML file that can add Bash-interpolated variables and scripts. The Bash code will have access to:

* all environment variables
* the jq binary
* the helper methods listed below

Here's an example of a mapping TOML file named `cf-mapping.toml`:

```toml cf-mapping.toml
secret_key_base = "$SECRET_KEY_BASE"
rails_env = "$RAILS_ENV"
port = ${PORT}

[db]
user = "$(service "elephantsql" '.credentials.username')"
password = "$(service "elephantsql" '.credentials.password')"
host = "$(service "elephantsql" '.credentials.host')"
name = "$(service "elephantsql" '.credentials.database')"
```

#### Helpers

The helper methods are designed to extract information from the standard Cloud Foundry environment variables [VCAP_SERVICES](https://docs.cloudfoundry.org/devguide/deploy-apps/environment-variable.html#VCAP-SERVICES) and [VCAP_APPLICATION](https://docs.cloudfoundry.org/devguide/deploy-apps/environment-variable.html#VCAP-APPLICATION).

* `service <service-name> <jq-expression>` will extract the JSON associated with the given service-name from the `VCAP_SERVICES` environment variable and apply the jq-expression to it.
* `application <jq-expression>` will apply the jq-expression to the `VCAP_APPLICATION` environment variable

### Exporting and Pushing to a Cloud Foundry Endpoint

1. Create a mapping.toml file using the format specified above and place that file in your local project repo.

2. Enter the Studio through `hab studio enter`.

3. Install or [build]({{< relref "pkg_build" >}}) the package that you want to export.

    ```bash
    hab pkg install <ORIGIN>/<NAME>
    ```

4. Run the Cloud Foundry exporter on the package.

    ```bash
    hab pkg export cf <ORIGIN>/<NAME> /path/to/mapping.toml
    ```

   > **Note** To generate this image, a base Docker image is also created. The Cloud Foundry version of the docker image will have `cf-` as a prefix in the image tag.

5. (Optional) If you are creating a web app that binds to another Cloud Foundry service, such as ElephantSQL, you must have this service enabled in your deployment before running your app.

6. [Upload your Docker image to a supported registry](https://docs.cloudfoundry.org/devguide/deploy-apps/push-docker.html). Your Docker repository should be match the `origin/package` identifier of your package.

    ```bash
    docker push origin/package:cf-version-release
    ```

7. After your Cloud Foundry Docker image is built, you can deploy it to a Cloud Foundry platform.

    ```bash
    cf push APP-NAME --docker-image docker_org/repository
    ```

   Your application will start after it has been successfully uploaded and deployed.

