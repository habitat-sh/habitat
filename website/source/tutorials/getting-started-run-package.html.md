---
title: Finding and running the sample package
---

# Run and update the package

> **Note** This content is outdated and will be refactored into the current tutorial at a later time.

In this tutorial, we have a simple Node.js application that is already packaged up using Habitat. It simply prints out a "Hello World" message whenever you access its URL endpoint. This step will show you how to run the application in a Docker container, make a small change, verify the change, and proceed onto updating the plan for the package itself.

## Run the container
Because you pulled down a Docker image in the previous step, simply run the container using the same Docker CLI commands as you would for any other Docker container. Also, this Node.js app requires port 8080 to be open, so you need to include that in your run command. As that executes, Habitat performs some additional actions that are not part of a typical Docker container deployment. Let's walk through them.

~~~ bash
$ docker run -p 8080:8080 -it quay.io/hab/nodejs-tutorial-app
hab(MN): Starting chef/nodejs-tutorial-app
hab(CS): Checking remote for newer versions...
hab(CS): Already running latest.
hab(GS): Supervisor 172.17.0.5: 74c1df5f-cd6a-4cf2-80de-173b6330a6c3
hab(GS): Census nodejs-tutorial-app.default: e308aadf-c93d-4f3e-82cb-e82080544fe7
hab(GS): Starting inbound gossip listener
hab(GS): Starting outbound gossip distributor
hab(GS): Starting gossip failure detector
hab(CN): Starting census health adjuster
hab(SC): Updated config.json
nodejs-tutorial-app(TS): Starting
nodejs-tutorial-app(SO):
nodejs-tutorial-app(O): > nodejs_tutorial_app@0.1.0 start /hab/srvc/nodejs-tutorial-app
nodejs-tutorial-app(O): > node server.js
nodejs-tutorial-app(O):
nodejs-tutorial-app(O): Running on http://0.0.0.0:8080
~~~

The first thing that you should notice is that Habitat checks to see if there are any more recent versions of the nodejs-tutorial-app package from the depot. Then, unlike a traditional Docker container, we use a supervisor to start up and monitor the application inside the package payload. This combination of a supervisor and application is known as a Habitat service.

The second thing you should notice is that the Supervisor in the Habitat service begins listening for other Habitat services that might be in its same service group. These service groups are used to share rumors about configuration changes, leader elections, health information of other services in the group, and so on.

Finally, a configuration file name `config.json` was added or updated in the Habitat service. You'll learn more about configuration in the next section.

Now that the Node.js app is running, connect to the Node server itself in your web browser to see the default message of the day. To do that, you must connect to the IP address of the VM that is running the Docker container. Type in the following command in your terminal window to bring up the site:  

      open "http://$(docker-machine ip default):8080/"

Here's an example of what you should see in your browser:

![Screen shot of node.js tutorial output](/images/nodejs-tutorial-output.png)

We will learn how to make permanent updates to our application in the next step, but for now, let's re-run our docker container and update the message value when our Habitat service starts up. To do this, we must pass in a Docker environment variable with the following format: HAB_PACKAGENAME='keyname=newvalue'.

  > Note: Your packagename must be upppercase.

Here is how you change the message for nodejs-tutorial-app:

~~~ bash
$ docker run -e HAB_NODEJS_TUTORIAL_APP='message="Habitat rocks!"' -p 8080:8080 -it chef/nodejs-tutorial-app
hab(MN): Starting chef/nodejs-tutorial-app
hab(CS): Checking remote for newer versions...
hab(CS): Already running latest.
hab(GS): Supervisor 172.17.0.3: 4b029dca-d365-486e-87ac-20cebc517127
hab(GS): Census nodejs-tutorial-app.default: b5c66ceb-55dc-426d-b96a-d2998b6dfec3
hab(GS): Starting inbound gossip listener
hab(GS): Starting outbound gossip distributor
hab(GS): Starting gossip failure detector
hab(CN): Starting census health adjuster
hab(SC): Updated config.json
nodejs-tutorial-app(TS): Starting
nodejs-tutorial-app(SO):
nodejs-tutorial-app(O): > nodejs_tutorial_app@0.1.0 start /hab/srvc/nodejs-tutorial-app
nodejs-tutorial-app(O): > node server.js
nodejs-tutorial-app(O):
nodejs-tutorial-app(O): Running on http://0.0.0.0:8080
~~~

Now refresh, or connect again to the local URL through your web browser.

![Screen shot of node.js output with new message](/images/nodejs-tutorial-update-output.png)

Making these configuration changes at startup can be useful when using the same Habitat package in multiple environments without changing the default settings defined in the default.toml file.

Learn how to make the change permanent by [updating the package itself](/tutorials/getting-started-make-change).
