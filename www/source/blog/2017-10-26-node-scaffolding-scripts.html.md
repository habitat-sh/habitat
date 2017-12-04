---
title: "Node Scaffolding now supports pre and post build scripts!"
date: 2017-10-26
author: Nell Shamrell-Harrington
tags: Node, scaffolding
category: community
classes: body-article
---

We have just released a new version of the Node scaffolding that supports scripts defined in package.json!

It is common for Node applications to include scripts in their package.json files like so:

```json package.json
"scripts": {
	"start": "react-scripts start",
	"build": "react-scripts build",
	"test": "react-scripts test --env=jsdom",
	"eject": "react-scripts eject"
}
```

The [Habitat Node Scaffolding](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-node) now supports these scripts out of the box.

Let's take it for a test run:

One of my favorite sample React apps is the [Pokedex](https://alik0211.ru/pokedex/) by Ali Gasymov.

Let's Habitize this application and deploy it in a Docker container!

First, clone the Github repo:

```shell
$ git clone git@github.com:alik0211/pokedex.git
$ cd pokedex
```

Now install the dependencies:

```shell pokedex
$ npm install
```

Check out the package.json - notice the scripts section? We will be able to run these to start, build, etc. the application through the Node Scaffolding.

```json package.json
{
	"name": "pokedex",
		"version": "1.1.5",
		"private": true,
		"dependencies": {
			"react": "^16.0.0",
			"react-dom": "^16.0.0",
			"react-redux": "^5.0.6",
			"redux": "^3.7.2",
			"redux-thunk": "^2.2.0"
		},
		"devDependencies": {
			"react-scripts": "1.0.10",
			"redux-logger": "^3.0.6"
		},
		"scripts": {
			"start": "react-scripts start",
			"build": "react-scripts build",
			"test": "react-scripts test --env=jsdom",
			"eject": "react-scripts eject"
		}
}
```


Let's Habitize this app:

```shell
$ hab plan init -s node
$ hab studio enter
[1][default:/src:0]# build
```

Currently, this only builds your application as a HART package - scripts within the package.json file will run when installing this package to wherever you want to run it.

Once the build completes, export your new HART package as a Docker image:

```studio
[1][default:/src:0]# $ hab pkg export docker ./results/your_new_hart_file.hart
[1][default:/src:0]# $ exit
```

Now, start up a new container with your container image

```shell
$ docker run -it -p 8000:8000 your_origin/pokedex
```

Then navigate to http://localhost:8000 in your browser and you should see a running pokedex!
