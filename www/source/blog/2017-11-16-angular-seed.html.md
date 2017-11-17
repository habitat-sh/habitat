---
title: NodeJS Scaffolding with Angular Seed!
date: 2017-11-16
author: Nell Shamrell-Harrington
tags: Scaffolding, Node, Angular
category: community
classes: body-article
---

Hello my fellow Habicats!

Do you have legacy Angular projects that use [angular-seed](https://github.com/mgechev/angular-seed)? Now you can package them with Habitat using the Node Scaffolding! Let's give it a go!

## Pre-requisites
* A Github Account
* NodeJS installed on your workstation
* Angular installed on your workstation
* Habitat installed on your workstation
* Docker installed and running on your workstation
* Text editor of your choice (in this post I use vim, but feel free to substitute your preferred editor)

## Setting up your application

Go ahead and clone the source code for [angular-seed](https://github.com/mgechev/angular-seed).

```console
$ git clone https://github.com/mgechev/angular-seed
$ cd angular-seed
```

Now install the application's dependencies - in this example we use npm, but you can use yarn if you like.

```console
$ npm install
```

Angular-seed comes with both a package-lock.json and yarn.lock file. We can only use one package manager with the Node Scaffolding. If you are using npm, delete yarn.lock. If you are using yarn, delete package-lock.json.

```console
$ rm yarn.lock
```

If you'd like, go ahead and run the application locally:

```console
$ npm start
```

And you should be able to access the app at http://localhost:5555.

## Code changes

There are two code changes we need to make to angular-seed in order to use it with Habitat. Open up the package.json file and downgrade the following dependencies to the shown versions:

```console
"typescript": "2.4.2",
"@compodoc/compodoc": "1.0.3",
```

## Habitizing your application

Now, run this command to create your Habitat scaffolding:

```console
$ hab plan init -s node
```

Now let's open up the plan file that was created when we ran "hab init", it should look like this:

**habitat/plan.sh**

```
pkg_name=angular-seed
pkg_origin=your_origin
pkg_version="0.1.0"
pkg_scaffolding="core/scaffolding-node"
```

And there is one line we need to add - currently, we need to manually say that we will run Habitat as the root user. Add this line to the plan.sh:

```
pkg_svc_user=root
```

Now, enter the Habitat studio with this command:

```console
$ hab studio enter
```

And, once you're in the studio, build the application:

```console
(studio) $ build
```

When the build is complete, export your new package as a Docker container image

```console
(studio) $ hab pkg export docker ./results/<your_new_package>.hart
```

Once that is complete, exit out of the studio

```console
(studio) $ exit
```

## Starting your application

Start up a docker container running your application with this command:

```console
$ docker run -it -p 5555:5555 core/angular-seed
```

Navigate to http://localhost:5555 and check out the app!
