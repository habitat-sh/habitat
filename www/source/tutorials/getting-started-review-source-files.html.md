---
title: Review the source files
---

# Review the source files
The Node.js application in this tutorial is a simple web app that displays a message to anyone who connects to the application. You can configure both the listening port and the message that gets displayed. The application contains three files: `server.js`, `package.json`, and `config.json`. You are not expected to take any action in this step; however, it's helpful to understand the source files first before learning how to build, install, and configure them in your plan.

**server.js**

This is the main file in our web app. It uses the [nconf module](https://github.com/indexzero/nconf) to retrieve the `message` and `port` configuration values that are set in the `config.json` file.

~~~ javascript
var http = require('http'),
    nconf = require('nconf');


nconf.file({ file: '../config/config.json' });

var handleRequest = function(request, response) {
    response.writeHead(200, {"Content-Type": "text/plain"});
    response.end(nconf.get('message'));
}

var www = http.createServer(handleRequest);
var port = nconf.get('port');
www.listen(port, function() {
    console.log("Running on http://0.0.0.0:%d", port);
});
~~~

**config.json**

You can think of this file as the default configuration file that you would use if this was an existing application running on a VM or in a container. When you create and configure your package, you will create a templatized version of this file that will be used when the service starts up.

~~~ javascript
{
    "message": "Hello, World!",
    "port": "8080"
}
~~~

**package.json**

Because we are using npm to start up our Node.js web app, the npm binary looks for a `package.json` file to describe the Node.js app project and its dependencies.

~~~ javascript
{
    "name": "mytutorialapp",
    "version": "0.2.0",
    "description": "Node.js tutorial app for Habitat",
    "author": "First Last <first.last@example.com>",
    "license": "MIT",
    "main": "server.js",
    "scripts": {
        "start": "node server.js"
    },
    "dependencies": {
    	"nconf": "^0.8.4"
    }
}
~~~

Now that you know how the source files are defined, it's time to create your first plan.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started-create-plan" class="button cta">Next - Create your first plan</a></li>
  <li><a href="/tutorials/getting-started-setup-environment/">Back to previous step</a></li>
</ul>
