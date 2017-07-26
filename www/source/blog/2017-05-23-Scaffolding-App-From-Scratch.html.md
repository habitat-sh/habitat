---
title: Packaging an App from Scratch with Scaffolding
date: 2017-05-23
author: fnichol
tags: scaffolding
category: Build
classes: body-article
published: true
---

Hey there, fellow developer, let's make ourselves a quick [Express](https://expressjs.com/) app and wrap it up in a Habitat package. Ready? Me too, I like to learn.

We're going to want Habitat installed on our system, but that's not too hard. Follow the instructions in [Download and Install](/tutorials/download) and come on back here. Can you run `hab --version` now in a terminal? Excellent.

Okay, we're going to use the Express application generator, so make sure you have this npm packaged installed globally:

~~~sh
npm install express-generator -g
~~~

Oh right, you need Node.js installed on your workstation--but you already do don't you? Nice.

Next, we'll use the generator to make our amazing webapp and then write a quick Habitat Plan.

~~~sh
express --git expresso
cd expresso
mkdir habitat
$EDITOR habitat/plan.sh
~~~

Here's what you want in your `plan.sh`:

~~~sh
pkg_origin=fnichol
pkg_name=expresso
pkg_version=0.1.0
pkg_scaffolding=core/scaffolding-node
~~~

Notice that I used my Habitat origin of `fnichol` (that's the origin and key I generated when I ran `hab setup` earlier on). The last line is where the good stuff is--this tells the Habitat build system to use the `core/scaffolding-node` Scaffolding package to build your app.

I'm sure you're as antsy as I am to commit this to version control--let's do that now:

~~~sh
git init
git add .
git commit -m 'Expresso: a typo or pun?'
~~~

Finally, we'll enter a Habitat Studio where we can build an isolated package for our app:

~~~sh
hab studio enter
build .
~~~

After a lot of output flies by, we should have our first Habitat package built and installed! Mine is called `fnichol/expresso` because my origin name is part of the [Package Identifier](https://www.habitat.sh/docs/concepts-packages/).

Now let's start our app! When you enter a Habitat Studio these days, a [Supervisor](https://www.habitat.sh/docs/concepts-supervisor/) will be running in the background waiting to start services, so we can start ours with:

~~~sh
hab svc start fnichol/expresso
~~~

By the way, if you want to tail the Supervisor output, run `sup-log` and hit `Ctrl+c` when you're done.

To quickly check our webapp, we can use `wget` to fetch the app's index page. There isn't much software in a Studio by default to keep our builds isolated and lean, so we'll stick with good 'ol `wget` for now:

~~~sh
wget -q -O - http://localhost:8000
~~~

I'm hoping you're looking at your fabulously generated HTML, just as I am:

~~~
<!DOCTYPE html><html><head><title>Express</title><link rel="stylesheet" href="/stylesheets/style.css"></head><body><h1>Express</h1><p>Welcome to Express</p></body></html>
~~~

Finally, how about performing a next-level Habitat trick like changing your app's listen port using a versioned runtime configuration injected into Habitat's gossip ring? The Node Scaffolding has provided us with a configuration setting called `app.port` which we can change, so let's set it to `3000` instead:

~~~sh
echo 'app = { port = 3000 }' | hab config apply expresso.default $(date +'%s')
~~~

This command looks more complicated than it is. We're piping in some inline TOML on `stdin` for the `hab config apply` command and targeting all "expresso" apps running in the default group (in Habitat we call this a Service Group). The `$(date +'%s')` part gives us the number of seconds since [the Unix epoch](https://www.epochconverter.com/clock) and acts as our version number.

If you want to see the Habitat Supervisor reacting to this change, run `sup-log` and hit `Ctrl+c` when you're done. You can verify the port change was applied with `wget`:

~~~sh
wget -q -O - http://localhost:3000
~~~

You can exit your Studio session with `exit` (I guessed this too), and you will find a Habitat package waiting for you in a `./results/` directory. The `.hart` file extension is for "Habitat ARTifact". This is because we take puns seriously. We're punstoppable.


## The End

There's much more to Habitat once you have a runnable package. Check out the [Habitat website](https://www.habitat.sh/) for more documentation, tutorials, community events, and more.
