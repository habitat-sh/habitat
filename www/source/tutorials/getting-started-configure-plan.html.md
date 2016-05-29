---
title: Add configuration to your plan
---

# Add configuration to your plan
When you create a plan, you may optionally define which configuration settings can be overridden. Those configuration settings are specific to the native application or service, but you may use a [handlebars](http://handlebarsjs.com/)-based version of the native application configuration file and then update the settings in a corresponding [TOML](https://github.com/toml-lang/toml) file.

In this tutorial, the archive for our Node.js app already has a configuration file called `config.json` that populates a message and specifies a listening port for the http server. We will use that file as a template for the settings that can be overridden at start up or while our service is running.

1. In your `plans/mytutorialapp` directory, create a new directory named `config` and add a new file to it named `config.json` to match the name of the configuration file that `server.js` references.

       [9][default:/src/plans/mytutorialapp/hooks:0]# cd /src/plans/mytutorialapp
       [10][default:/src/plans/mytutorialapp:0]# mkdir -p config
       [11][default:/src/plans/mytutorialapp:0]# touch config/config.json


2. Open `config/config.json` in your text editor.
3. Copy the contents from the original `config.json` (shown below) into `config/config.json`.

       {
           "message": "Hello, World!",
           "port": "8080"
       }

    Because we want to be able to configure both of the settings above, we are going to replace the existing values in the file with references to handlebar expressions. Those expressions will look to a TOML file to define an initial set of values if they are not overridden at start up.

4. Replace the values in `config.json` with the expressions **cfg.message** and **cfg.port**.

       {
           "message": "{{cfg.message}}",
           "port": "{{cfg.port}}"
       }

5. Save the file.

All user-defined expressions must have the **cfg** prefix. For general service settings, Habitat also defines several system expressions that you may use to configure your service at runtime. See the [Runtime configuration settings](/docs/plan-syntax#runtime-configuration-settings) section of the Plan syntax guide for more information.

As we said, a TOML file is associated with your configuration file and specifies the default values for your service at start up. If you have a templatized configuration file, then you must include a `default.toml` file in your plan folder.

1. If you are not in the `/src/plans/mytutorialapp` directory, change directories to it and create a file named `default.toml`.

       [12][default:/src/plans/mytutorialapp:0]# touch default.toml

2. Open `default.toml` in your text editor and add the default values.

       # Message of the Day
       message = "Hello, World!"

       # The port number that is listening for requests.
       port = 8080

    We use the same values as the ones specified in the original `config.json` file to keep the initial start up experience the same. Also, the port value specified is specific to the Node.js application and it will bind to the port of its host. In this case, the host is the Docker container you will create in the next step.

3. Save the file.

## Build the artifact
Now that you have defined how your source files should be installed and configured in your artifact, it's time to build it in the studio. Change directory to `/src` and enter the following command to create the artifact.

    [13][default:/src/plans/mytutorialapp:0]# cd /src
    [14][default:/src:0]# build plans/mytutorialapp

The last set of output messages from running `build plans/mytutorialapp` should look something like this:

~~~ bash
   mytutorialapp: Building package metadata
   mytutorialapp: Writing configuration
   mytutorialapp: Writing service management scripts
   mytutorialapp: Stripping unneeded symbols from binaries and libraries
   mytutorialapp: Creating manifest
   mytutorialapp: Generating package artifact
/hab/pkgs/core/tar/1.28/20160427205719/bin/tar: Removing leading `/' from member names
/hab/cache/artifacts/.myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.tar (1/1)
  100 %       120.9 KiB / 910.0 KiB = 0.133
» Signing /hab/cache/artifacts/.myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.tar.xz
☛ Signing /hab/cache/artifacts/.myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.tar.xz with myorigin-20160527200622 to create /hab/cache/artifacts/myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.hart
★ Signed artifact /hab/cache/artifacts/myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.hart.
'/hab/cache/artifacts/myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.hart' -> '/src/results/myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.hart'
   mytutorialapp: hab-plan-build cleanup
   mytutorialapp:
   mytutorialapp: Source Cache: /hab/cache/src/mytutorialapp-0.0.1
   mytutorialapp: Installed Path: /hab/pkgs/myorigin/mytutorialapp/0.0.1/20160527200628
   mytutorialapp: Artifact: /src/results/myorigin-mytutorialapp-0.0.1-20160527200628-x86_64-linux.hart
   mytutorialapp: Build Report: /src/results/last_build.env
   mytutorialapp:
   mytutorialapp: I love it when a plan.sh comes together.
   mytutorialapp:
   mytutorialapp: Build time: 0m12s
[15][default:/src:0]#
~~~

The next step will show you how to install your artifact and run your service for local testing.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started-process-build" class="button cta">Next - Process your build</a></li>
  <li><a href="/tutorials/getting-started-add-hooks/">Back to previous step</a></li>
</ul>
