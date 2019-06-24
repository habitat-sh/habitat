---
title: Scaffolding
date: 2017-05-23
author: fnichol
tags: scaffolding
category: build
classes: body-article
published: true
---

> "The best plan is to barely have one at all." - Ancient Chef Habitat proverb

Over the past several months we've been working on a feature of the build system that focuses on developers which helps them bundle up their applications to be run, managed, even updated with Chef Habitat. We call this Scaffolding.

> **Scaffolding** helps developers build and package their applications which follow the common patterns and practices for their application type. In other words, the more your application follows the conventions, the more Scaffolding helps you.

We're launching this feature with two Scaffolding packages: one for [**Ruby**](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-ruby)-based apps and one for [**Node.js**](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-node)-based apps. In the near future, we plan on adding support for Go, Python, and JVM based apps but we believe we have a good enough start to share this build pattern more widely.

## Scaffolding's Assumptions

First, this feature is obsessively focused on the application developer's experience. As a result what we are concerned with are developers building their own applications. Chef Habitat is developing a good track record for building **other** third-party software (for example: [PostgreSQL](https://app.habitat.sh/#/pkgs/core/postgresql), [Bash](https://app.habitat.sh/#/pkgs/core/bash), [Rust](https://app.habitat.sh/#/pkgs/core/rust), etc.), but as a developer, I want Chef Habitat to be great at building **my** software. We expect and assume that you want your Chef Habitat Plan to live alongside your codebase, in the same version control repository, etc.

Secondly, a Scaffolding tries to make the best possible build experience for your app type by looking for common application patterns and practices that exist today. If you've already deployed your app to a Platform-as-a-Service provider (PaaS) or packaged your app in a container, chances are your application follows some or most of the practices from Heroku's [Twelve-Factor App](https://12factor.net/) manifesto. Scaffolding exploits these same conventions so much like the Ruby on Rails framework, when you follow the conventions, you are rewarded with less configuration and setup.

## Detection

Scaffolding unlocks a powerful new behavior at build time: the ability to **detect and react** to the needs of an application codebase. Let's look at an example to see this in action. The output lines below are from building a Ruby on Rails web application called "habirails". The Plan which builds this app's package is very simple and contains one new build variable: `pkg_scaffolding`.

```bash
pkg_name=habirails
pkg_origin=fnichol
pkg_version=0.1.0
pkg_scaffolding=core/scaffolding-ruby
```

When we build this Plan, we'll see some of the following in the build output:

```
   habirails: Detected Rails 5 app type
```

The Ruby Scaffolding understands some specific Ruby web frameworks such as Ruby on Rails and Rack. In this case it has detected a Rails 5.x application and can use that knowledge later on.

```
   habirails: No Ruby version detected in Plan or Gemfile.lock, using default 'core/ruby'
```

There are canonical locations where Ruby developers select a specific version of Ruby, one of them uses the `ruby` keyword in a `Gemfile`. The Ruby Scaffolding loads the project's `Gemfile` and `Gemfile.lock` (using Ruby itself and calls the Bundler codebase as a library) so that it can correctly parse this information. In this case no version was specified in the `Gemfile` or in the `plan.sh`, so a default Chef Habitat package of `core/ruby` was chosen.

```
   habirails: Detected 'nokogiri' gem in Gemfile.lock, adding libxml2 & libxslt packages
```

Some RubyGems have native extensions or require other software to be present so this Scaffolding inspects the `Gemfile.lock` for some common gems that are used by the community, including `execjs` which requires Node.js, `sqlite3` which requires SQLite shared libraries, and `nokogiri` as shown above. In `nokogiri`'s case, we build this against system libraries in Chef Habitat packages so this gem typically takes a second or two to install.

```
   habirails: Detected 'pg' gem in Gemfile.lock, adding postgresql package
```

Similar to above, this detection will add the appropriate PostgreSQL Chef Habitat packages, but will create an [optional bind](/docs/developing-packages#pkg-binds) for the package which lets your app discover its database in a Chef Habitat ring. If you start the app service without a `--bind` option, the package will fall back to requiring database host and port configuration settings meaning that you can point your app at an existing database that lives outside a Chef Habitat ring.

```
   habirails: Installing dependencies using Bundler version 1.14.6
```

The Ruby Scaffolding knows how to use Bundler to install and vendor RubyGem dependencies for use in a production environment.      The exact version of Bundler which is used is also vendored into the app's package so there is one less runtime Chef Habitat package dependency to install and only one version of Ruby is pulled in for production.

```
   habirails: Detected and running Rake 'assets:precompile'
```

In this case, the `rake` RubyGem was detected in the `Gemfile.lock`, a `Rakefile` was present in the project's root directory, and an `assets:precompile` Rake task was found. This is default behavior for Ruby on Rails applications but also commonly used in other Rack-based applications and even static site generators. If the correct project markers are found, the Scaffolding code takes over.

```
   habirails: No user-defined init hook found, generating init hook
```

Based on the app detection above, the Ruby Scaffolding can generate a suitable [init hook](/docs/reference#reference-hooks) which checks to see that the Rails' [secret key base](http://guides.rubyonrails.org/security.html#session-storage) is set and will even test your database connection--all before the Supervisor even attempts to boot the app itself.

There are a lot more features and goodies that Scaffolding packages provide for your app and it is worth reading the reference docs for the [Ruby](https://github.com/habitat-sh/core-plans/blob/master/scaffolding-ruby/doc/reference.md) and [Node.js](https://github.com/habitat-sh/core-plans/blob/master/scaffolding-node/doc/reference.md) implementations.

## Next Up: More Languages

From the initial concept to early prototypes through to the first two releases supporting Ruby and Node.js, we've narrowed in on how Scaffolding should work and more importantly how it should feel: effortless. We're planning on developing and updating a few more Scaffolding implementations in the very near future which will help us find the common abstractions and behavior that can be shared.

## Try It!

Will a Scaffolding help you build and package your application? There's only one way to find out--jump in and see. If you want to see Scaffolding work on a small Node.js app, check out the [Packaging an App from Scratch with Scaffolding](/blog/2017/05/Scaffolding-App-From-Scratch) blog post. Any questions or feedback are most welcome on our [community Slack](http://slack.habitat.sh/). Happy packaging!
