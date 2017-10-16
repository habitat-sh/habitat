---
title: Configuration Plans
date: 2017-10-15
author: Thom May
tags: [plans, configuration, glossary]
category: Packaging
classes: body-article
---

One of the hardest problems in computer science is naming things. But one of the
most important things for a healthy community is shared language. As the
Habitat community grows, we'll try to identify patterns and practices
that help us succeed, and give them names so that we can all talk about
them.

One pattern we use a lot is what we're calling a Configuration Plan. As
we share our packages, we've been seeing that our colleagues and
community members use those packages in ways that we could not have
anticipated. So what to do? We could try and add templates to
support all possible uses, but that will rapidly become untenable. What
good packages should do instead is provide well documented entry points, so
that consumers of our package can easily supply their own configuration
in a new plan -- a Configuration Plan.

## Building a Configuration Plan

Let's have a look at an example. There are actually a fair number of
live examples in the habitat repository - check out the [Builder API
Proxy](https://github.com/habitat-sh/habitat/tree/master/components/builder-api-proxy), which is a Configuration Plan for nginx. But for now, let's build our own.

I've created a tiny binary that simply reads a file and prints it out.
You can [see the
source](https://github.com/thommay/configuration_plan_demo), and you can
try running it by entering a studio and running the demo.

```bash
$ hab studio enter
(studio) $ hab svc start thom/configuration_plan_demo
(studio) $ sup-log
```

You'll see the demo print out the configuration file name, and then the
contents of the file.

```
configuration_plan_demo.default(O): Filename is: /hab/svc/configuration_plan/config/output.txt
configuration_plan_demo.default(O): The original plan

### Making a new plan
Now, let's create a Configuration Plan that provides a new file for the
demo. First, we'll create a new plan:

```bash
$ hab plan init -o <your_origin> configuration_plan
$ cd configuration_plan
```

We now have a `configuration_plan` directory, containing a `plan.sh`,
a `config` dir, and a `hooks` dir.

Let's create that configuration file!

```bash
$ echo 'a really great demo' > config/my_configuration_plan.txt
```

Open up your `plan.sh`, and replace it with the following, being sure to
set your origin correctly.

```bash
pkg_origin=your_origin
pkg_name=configuration_plan
pkg_version=0.1.0
pkg_deps=(thom/configuration_plan_demo)
pkg_svc_run="configuration_plan_demo ${pkg_svc_config_path}/my_configuration_plan.txt"

do_build() {
  return 0
}

do_install() {
  return 0
}

do_unpack() {
  return 0
}
```

Because we're shipping nothing but configuration, we tell Habitat that
we don't need to unpack, build or install. Instead, we simply depend on
`thom/configuration_plan_demo`, and then ensure that we use our new
configuration file.

### Running the plan
Now we can build and run our plan. Enter a studio as usual, run `build`
to build your plan, and then install it. On my machine, I ran:

```bash
$ hab studio enter
(studio) $ build
(studio) $ hab pkg install /src/results/thom-configuration_plan-0.1.0-20171016162630-x86_64-linux.hart
(studio) $ hab svc start thom/configuration_plan
```

By running `sup-log`, we should see our demo start and use the new
config file.

```
configuration_plan.default(O): Filename is: /hab/svc/configuration_plan/config/my_configuration_plan.txt
configuration_plan.default(O): a really great demo
```

## Conclusion

Rather than needing to build our own specific package, Configuration Plans
allow us to tailor widely used applications to our exact purposes. They
also free the maintainer of the application to provide up to date
packages, without trying to support every use case themselves.
