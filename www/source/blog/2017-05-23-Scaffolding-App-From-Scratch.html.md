---
title: Packaging an App from Scratch with Scaffolding
date: 2017-05-23
author: fnichol
tags: scaffolding
category: build
classes: body-article
published: true
---

Hey there, fellow developer, let's make ourselves a quick [Express](https://expressjs.com/) app and wrap it up in a Chef Habitat package. Ready? Me too, I like to learn.

We're going to want Chef Habitat installed on our system, but that's not too hard. Follow the instructions in [Install Chef Habitat](/docs/install-habitat/) and come on back here. Can you run `hab --version` now in a terminal? Excellent.

Okay, we're going to use the Express application generator, so make sure you have this npm packaged installed globally:

```shell
$ npm install express-generator -g
```

Oh right, you need Node.js installed on your workstation--but you already do don't you? Nice.

Next, we'll use the generator to make our amazing webapp and then write a quick Chef Habitat Plan.

```shell
$ express --git expresso
$ cd expresso
$ mkdir habitat
$ $EDITOR habitat/plan.sh
```

Here's what you want in your `plan.sh`:

```bash
pkg_origin=fnichol
pkg_name=expresso
pkg_version=0.1.0
pkg_scaffolding=core/scaffolding-node
```

Notice that I used my Chef Habitat origin of `fnichol` (that's the origin and key I generated when I ran `hab setup` earlier on). The last line is where the good stuff is--this tells the Chef Habitat build system to use the `core/scaffolding-node` Scaffolding package to build your app.

I'm sure you're as antsy as I am to commit this to version control--let's do that now:

```shell
git init
git add .
git commit -m 'Expresso: a typo or pun?'
```

Finally, we'll enter a Chef Habitat Studio.

```shell
$ hab studio enter
```

And build an isolated package for our app:

```studio
[1][default:/src:0]# build
```

After a lot of output flies by, we should have our first Chef Habitat package built and installed! Mine is called `fnichol/expresso` because my origin name is part of the [Package Identifier](/docs/glossary#glossary-artifacts).

Now let's start our app! When you enter a Chef Habitat Studio these days, a [Supervisor](/docs/glossary#glossary-supervisor) will be running in the background waiting to start services, so we can start ours with:

```shell
$ hab svc start fnichol/expresso
```

By the way, if you want to tail the Supervisor output, run `sup-log` and hit `Ctrl+c` when you're done.

To quickly check our webapp, we can use `wget` to fetch the app's index page. There isn't much software in a Studio by default to keep our builds isolated and lean, so we'll stick with good 'ol `wget` for now:

```shell
$ wget -q -O - http://localhost:8000
```

I'm hoping you're looking at your fabulously generated HTML, just as I am:

```
<!DOCTYPE html><html><head><title>Express</title><link rel="stylesheet" href="/stylesheets/style.css"></head><body><h1>Express</h1><p>Welcome to Express</p></body></html>
```

Finally, how about performing a next-level Chef Habitat trick like changing your app's listen port using a versioned runtime configuration injected into Chef Habitat's gossip ring? The Node Scaffolding has provided us with a configuration setting called `app.port` which we can change, so let's set it to `3000` instead:

```shell
$ echo 'app = { port = 3000 }' | hab config apply expresso.default $(date +'%s')
```

This command looks more complicated than it is. We're piping in some inline TOML on `stdin` for the `hab config apply` command and targeting all "expresso" apps running in the default group (in Chef Habitat we call this a Service Group). The `$(date +'%s')` part gives us the number of seconds since [the Unix epoch](https://www.epochconverter.com/clock) and acts as our version number.

If you want to see the Chef Habitat Supervisor reacting to this change, run `sup-log` and hit `Ctrl+c` when you're done. You can verify the port change was applied with `wget`:

```shell
$ wget -q -O - http://localhost:3000
```

You can exit your Studio session with `exit` (I guessed this too), and you will find a Chef Habitat package waiting for you in a `./results/` directory. The `.hart` file extension is for "Chef Habitat ARTifact". This is because we take puns seriously. We're punstoppable.


## The End

There's much more to Chef Habitat once you have a runnable package. Check out the [Chef Habitat website](https://www.habitat.sh/) for more documentation, tutorials, community events, and more.
