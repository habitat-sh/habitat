---
title: Packaging PHP in Habitat. A Tale Of Two Drupals.
date: 2017-05-16
author: Paul Adams
tags: [drupal, php]
category: Packaging
classes: body-article
---
(_This post was originally published at
[Baggerspion](http://baggerspion.net/2017/05/drupal-twice/)_)

Early in my [Habitat](https://habitat.sh) journey I packed a
super-simple Python application that I had developed. It served no
purpose other than to be packaged. Once I started to get my head
around the fundamentals of packaging with Habitat, my mind turned to
more useful things.

- [CrateDB](https://crate.io): done
- [Jenkins](https://jenkins.io): done

These are interesting in their own right and you'll be hearing more
about them after [ChefConf](https://chefconf.chef.io/2017/). For now,
I want to talk about Drupal.

Drupal is "just" some PHP. How hard can it be? [^1] Well, I saw enough
raised eyebrows from my friends at Chef, to understand this might not
be as easy as it seems.

## Packaging **Any** PHP

Let's forget Drupal for now. The first step is working out how to
package _any_ piece of PHP for Habitat. Well, we really only need
three ingredients:

- NGINX (or Apache HTTPD, if you so wish)
- PHP
- The sources of our PHP application

No rocket science here. We just need to glue it all together. Most of
the hard work is already done for us, since Habitat already has
`core/nginx` and `core/php` packages available.

All I really wanted to do was unpack my PHP in a location where NGINX
could access it and then point NGINX to my PHP-FPM socket/ip. This is
all easy enough when you know how.

I didn't know how. Thankfully, the [Habitat public
depot](https://bldr.habitat.sh) has a plan available for Wordpress [^2]
which gave me some insight. The "trick" here is to launch PHP-FPM and
then use an init hook to also launch NGINX with a config you give it.

OK, so now Drupal.

## Drupal: First Attempt

The first attempt a Drupal was easy enough:

- The sources of the plan can be found
[here](https://github.com/endocode/habitat-plans/tree/master/drupal);
- The final package is available
[here](https://bldr.habitat.sh/#/pkgs/endocode/drupal/8.3.2/20170515200337).

This plan was built exactly as I described above: I download the
source tarball, unpack it in a specific location and configure NGINX
to use that location as a site root. When it is run, the user is
presented with a shiny new Drupal install page:

![Drupal Install Page](http://res.cloudinary.com/baggerspion/image/upload/Images/drupal-start.png)

So, there you have it, a successful Drupal package for Habitat!

Or is it?

## Drupal: A Better Attempt?

The problem with my first attempt is that it is not very Habitat-y. Or
Drupal-y for that matter. The cool kids use the Drupal shell
([drush](http://www.drush.org)) to deploy/configure Drupal and here
comes my first problem: there's no Habitat package for `drush`.

Not to worry: `drush` could not be any simpler to install; it is used
as a dependency in composer. Thankfully, there _is_ a `core/composer`
package I can use for this purpose. I rolled out a `baggerspion/drush`
package with minimal effort. It can be found [here](https://bldr.habitat.sh/#/pkgs/baggerspion/drush/8/20170515140815).

Drush is important for one particular reason: having installed the
base Drupal ("core"?) system, drush can be used to build a
preconfigured site, including default user credentials and database
configuration. This means we do not need to force the user of the
Drupal package to go through the post-install install script. They'd
be presented with a ready-made site, all configured and ready to go.

Providing most of the details needed to configure a Drupal site using
drush is simple, we can simply dump the info in `default.toml` to be
pulled out anytime. Mildly trickier is the database config.

### Using Habitat Binds For The Drupal Database

Just as with the default account credentials, I could simply dump all
the default database config into `default.toml`. This would not be a
very Habitat-y thing to do.

Much better is to use service binding, to bind my Drupal service to an
existing database service. In Habitat a service bind is the mechanism
that allows me to say, "this service relies on this other service in
order to run properly". At runtime, I simply tell my service which
other service to bind to:

```shell
$ hab start baggerspion/drupal --bind database:mysql.default
```

I will spare the full details on how binds work. There is [plenty of
documentation
available](/docs/developing-packages#pkg-binds) on the
Habitat website. What's important here is that, because I have bound
my Drupal service to a Mysql service, I can now configure my service
to use that database automatically. What does the user see now?

![Drupal Login Page](http://res.cloudinary.com/baggerspion/image/upload/Images/drupal-better.png)

This is much better. I think.

I'm offering 10 points [^3] for the first comment telling me where the
default password (account_password) in the
[baggerspion/drupal](https://bldr.habitat.sh/#/pkgs/baggerspion/drupal/8.3.2/20170516112409)
plan comes from! No Googling! Answers below!

## Footnotes
[^1]: I actually said/thought this. I am not proud.
[^2]: The github repo for this is [here](https://github.com/starkandwayne/habitat-plans/tree/master/wordpress). A huge "thank you" to [Stark and Wayne](https://www.starkandwayne.com/) for basically teaching me everything I know about packaging PHP for Habitat.
[^3]: "Paul Adams Points"&trade; have no monetary value and are non-transferable.
