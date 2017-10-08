---
title: Node Scaffolding - What's Available Today and What's Coming Soon!
date: 2017-10-09
author: Nell Shamrell-Harrington
tags: scaffolding
category: community
classes: body-article
---

Greetings fellow Habicats!

One of the most useful feature of Habitat is scaffoldings - these allow you to package your application with Habitat with very minimal code.  They enable you to very rapidly (within minutes) package an application and export it to whatever format you need.  One of the most popular Habitat scaffoldings is the NodeJS scaffolding.

There will be much more coming to this scaffolding in the coming months, but this post is meant to highlight what you can do with the NodeJS scaffolding today and what you can expect in the near future.

Ready to get started?

## Pre-requisites
* NodeJS installed on your workstation
* Habitat installed and set up on your workstation
* Text editor of your choice (in this post I use vim, but feel free to substitute your preferred editor)

## Creating your NodeJS application

Let's create an ultra-simple NodeJS application using [Express](https://expressjs.com/).

(This application is based on [this tutorial](https://developer.mozilla.org/en-US/docs/Learn/Server-side/Express_Nodejs/skeleton_website)).

```console
$ express --git misfit_toys
$ cd misfit_toys
$ npm install
$ vim routes/index.js
```

Your routes/index.js file should look like this:

**routes/index.js**

```
var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'Express' });
});

module.exports = router;
```

Change this line:

```
res.render('index', { title: 'Express' });
```

To this:

```
res.render('index', { title: 'Island of Misfit Toys' });
```

Now, your routes/index.js file should match the following:

```
var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'Island of Misfit Toys' });
});

module.exports = router;
```

If you'd like, start the app locally

```console
$ npm start
```

Then check out your running application at http://localhost:3000/

## Habiterizing your NodeJS application

Now let's package up this application with Habitat.

In the same directory as your application (the misfit_toys directory), run this command:

```
$ hab plan init -s node
```

This creates a habitat directory along with several starter files.  Open up habitat/plan.sh:

**habitat/plan.sh**

```
pkg_name=misfit_toys
pkg_origin=your_origin
pkg_version="0.1.0"
pkg_scaffolding="core/scaffolding-node"
```

Make sure that pkg_origin is set to your Habitat origin.  There are not other changes we need to make to package this with Habitat.  Save and close the file.

Now, enter the Habitat studio to build the application.

```console
$ hab studio enter
```

Once you are in the studio, build the application with the "build" command

```console
(studio) $ build
```

This will build your package!  You can find it in the results directory (both in and out of the studio).  We could take this package and run it on a Virtual Machine or Bare Metal Server...but let's try something new.  This particular application does not have any persistant data, which makes it ideal to run in a container.  With just one command, we can export this package to a ready-to-run Docker image.

```console
(studio) $ hab pkg export docker ./results/<your habitat package>.hart
```

Now exit out of the studio

```console
(studio) $ exit
```

And run the Docker image you just created:

```console
$ docker run -it -p 8000:8000 your_origin/misfit_toys
```

Then check it out in your browser by navigating to http://localhost:8000.

You can then upload this image to Amazon or Azure Container Services, Kubernetes, and more.

## What if I want to use a different version of NodeJS?

By default, the Node scaffolding uses the most recent release of the [core/node](https://bldr.habitat.sh/#/pkgs/core/node/latest) package available on Builder.

If you'd like to use a different version of Node, you can!  There are three ways to do this (the node scaffolding will still use the appropriate version of the core/node package).

* Specify it in your application's package.json, i.e.

```
"engines": {
  "node": ">=5.6.0",
},
```

You can use version numbers in these formats "5.0.0", "v5.0.0", "=5.0.0", ">=5.0.0", ">5.0.0","<=5.0.0","<5.0.0"

* Write an .nvmrc in your application's root directory
* Set the scaffolding_node_pkg variable in your Plan with a valid Habitat package identifier (i.e. your_origin/node/5.0.0)

What about if your application specifies different versions in different places?  A Plan variable will take priority ov package.json and package.sjon will take priority over an .nvmrc file.


## Coming Soon

If this seems like a very simple example, it is.  More functionality is on the way to allow you to use scaffolding with more complex NodeJS applications - including both React and Angular.

* Running pre-build and post-build hooks - this is critical to supporting any non-trivial React or Angular applications
* Allowing a connection to a database - also critical for any database backed application.

These are in active development now, look for them soon!
