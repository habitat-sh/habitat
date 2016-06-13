---
title: Add configuration to your plan
---

# Add configuration to your plan
When you create a plan, you may optionally define which configuration settings can be overridden through the use of a templatized [Handlebars.js](http://handlebarsjs.com) version of the native application configuration file. Default settings are then specified in and updated from a corresponding [TOML](https://github.com/toml-lang/toml) file.  

In this tutorial, the archive for our Node.js app already has a configuration file called `config.json` that populates a message and specifies a listening port for the http server. We will use that file as a template for the settings that can be overridden at start up or while our service is running.

1. In your `/src` directory, create a new directory named `config` and add a new file to it named `config.json` to match the name of the configuration file that `server.js` references.

       [8][default:/src/hooks:0]# cd /src
       [9][default:/src:0]# mkdir -p config
       [10][default:/src:0]# touch config/config.json

2. Open `config/config.json` in your text editor.
3. Copy the contents from the original `config.json` from our source files (shown below) into `config/config.json`.

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

All user-defined expressions must have the **cfg** prefix. For general service settings, Habitat also defines several system expressions that you may use to configure your service at runtime. See the [Runtime configuration settings](/docs/reference/plan-syntax#runtime-configuration-settings) section of the plan syntax guide for more information.

As we said, a TOML file is associated with your configuration file and specifies the default values for your service at start up. If you have a templatized configuration file, then you must include a `default.toml` file in your plan folder.

1. If you are not in the `/src` directory, change directories to it and create a file named `default.toml`.

       [11][default:/src:0]# touch default.toml

2. Open `default.toml` in your text editor and add the default values.

       # Message of the Day
       message = "Hello, World!"

       # The port number that is listening for requests.
       port = 8080

    We use the same values as the ones specified in the original `config.json` file to keep the initial start up experience the same. Also, the port value specified is specific to the Node.js application and it will bind to the port of its host. In this case, the host is the Docker container you will create in the next step.

3. Save the file.

The next step will show you how to install your package and run your service for local testing.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started-process-build" class="button cta">Next - Process your build</a></li>
  <li><a href="/tutorials/getting-started-add-hooks/">Back to previous step</a></li>
</ul>
