---
title: Base Plans Refresh is Coming to Habitat Core Plans!
date: 2019-01-14
author: Scott Macfarlane
tags: Core Plans
category: update
classes: body-article
---

Hello, friendly Habicats! 

We will be refreshing the Base Plans of Core Plans on Tuesday, January 29th. Let's go over what this means for us and, more important, what it means for you.

## Base Plans 101
Currently, there are about 600 Habitat plans under the Core Origin on Builder.

These include everything from services like Postgresql to binaries like Go to very low level system libraries like GCC, all of which you can easily pull into your own Habitat artifacts.

### What's a Base Plan

The Base Plans are a group of plans (mostly low level systems libraries) that:

* are used by nearly every other plan (or a dependency of that plan) on Builder
* are used to build Habitat itself
* need to be built in a certain order.

You can see the [full list of Base Plans](https://github.com/habitat-sh/Core-Plans/blob/master/CODEOWNERS) in the Core Plans CODEOWNERS file.

Any time one of these plans is updated (especially ones that are nearly universally used, like [glibc](https://en.wikipedia.org/wiki/GNU_C_Library) and [gcc](https://gcc.gnu.org/), all other Base Plans which depend on it need to be built in a certain order, and then all plans that depend (or depend on a plan that depends) on those Base Plans also need to be rebuilt. We call this the "rebuild the world" scenario.

### Why are you refreshing them?

In this release we are moving from gcc 7 to gcc 8. Any plan that depends on gcc 7 will be rebuilt with gcc 8. It is vital to update them - and consequently every plan that depends on them - to more recent stable versions.

### What else does it affect?

We are simultaneously rebuilding every Core Plan at the same time as we are refreshing the Base Plans. This is because the all Core Plans depend on at least one of the Base Plans in some fashion.

## Base Plans and You
So far we've discussed how and why we are refreshing the Base Plans and Core Plans, but what does that mean for the plans in your own origin?

### How does this affect me?

Chances are, any plan within your origin depends on at least one of the Core Base Plans somewhere in its chain of dependencies. When that plan is updated, your plan also must be rebuilt. This sounds simple, but if you have plans in your origin that depend on other plans - and those plans depend on a Core Plan (let's say glibc), all of them must be built with the same version of glibc. Let's look at this visually.

### Dependencies

Let's say I have a plan under my origin (nellshamrell) called widget_world - let's say it lives at nellshamrell/widget_world on Builder.

```
nellshamrell/widget_world
```

Now, let's say that plan depends on two more plans, nellshamrell/widget and nellshamrell/world (yes, it's contrived, just go with it for now).

```
nellshamrell/widget_world
-> nellshamrell/widget
-> nellshamrell/world
```

And let's say each of those plans depends on core/glibc for some reason:

```
nellshamrell/widget_world
-> nellshamrell/widget
--> core/glibc
-> nellshamrell/world
--> core/glibc
```

And let's say both are using the current core/glibc plan, which means they are both using version 2.22:

```
nellshamrell/widget_world
-> nellshamrell/widget
   --> core/glibc/2.22
-> nellshamrell/world
   --> core/glibc/2.22
```

My nellshamrell/widget_world plan will build fine, because both of its dependencies are built with the same version of glibc.

```
(studio) $ build widget_world
...
Success!
I love when a plan.sh comes together!
```

Then suppose the Habitat team upgrades core/glibc to 2.27. And let's say for some reason you only update one of the dependencies to use the updated glibc - so nellshamrell/widget will depend on core/glibc/2.27, while nellshamrell/world will still depend on core/glibc/2.22:

```
nellshamrell/widget_world
-> nellshamrell/widget
   --> core/glibc/2.27
-> nellshamrell/world
   --> core/glibc/2.22
```

This time, if I attempt to build nellshamrell/widget_world, it will return a very ugly error that looks similar to this:

```
(studio) $ build widget_world
(...)
	widget_world: WARN
	widget_world: WARN The following runtime dependencies have more than one version
	widget_world: WARN release in the full dependency chain:
	widget_world: WARN
	widget_world: WARN * core/glibc ( core/glibc/2.22/<timestamp> core/glibc/2.27/<timestamp> )
	widget_world: WARN
	widget_world: WARN The current situation usually arises when a plan has a direct
	widget_world: WARN dependency on one version of a package (acme/A/7.0/20160101200001)
	widget_world: WARN and has a direct dependency on another package which itself depends
	widget_world: WARN on another version of the same package (acme/A/2.0/20151201060001).
	widget_world: WARN If this package (acme/A) contains shared libraries which are
	widget_world: WARN loaded at runtime by the current plan, then both versions of
	widget_world: WARN acme/A could be loaded into the same process in a potentially
	widget_world: WARN surprising order. Worse, if both versions of acme/A are
	widget_world: WARN ABI-incompatible, runtime segmentation faults are more than likely.
	widget_world: WARN
	widget_world: WARN In order to preserve reliability at runtime the duplicate dependency
	widget_world: WARN entries will need to be resolved before this plan can be built.
	widget_world: WARN Below is an expanded graph of all $pkg_deps and their dependencies
	widget_world: WARN with the problematic lines noted.
	widget_world: WARN
	widget_world: WARN Computed dependency graph (Lines with '*' denote a problematic entry):

nellshamrell/widget_world/1.0.0/<timestamp>
		nellshamrell/widget/1.0.0/<timestamp> (*)
			core/glibc/2.27/<timestamp> (*)
		nellshamrell/world/1.0.0/<timestamp> (*)
			core/glibc/2.22/<timestamp> (*)
ERROR: Computed runtime dependency check failed, aborting
```

This error occurs because nellshamrell/widget_world has one dependency that was built with 2.27, and another that was built with 2.22. This makes them incompatible, and the nellshamrell/widget_world plan will not build.

### How do I resolve this?
In this case, you can resolve this error by rebuilding nellshamrell/world with core/glibc version 2.27. If we do that, and now both dependencies depend on the same version of glibc, then nellshamrell/widget_world will build fine:

```
nellshamrell/widget_world
-> nellshamrell/widget
    --> core/glibc/2.27
-> nellshamrell/world
    --> core/glibc/2.27
```

Success!

```
(studio) $ build widget_world
...
Success!
I love when a plan.sh comes together!
```

So...after the Core Plans refresh gets added to production and promoted to stable (which, again, we are doing on Tuesday, January 29th), and all of the Base Plans and dependent Base Plans are promoted to stable, you may see some of these failures with your own plans. It's also possible they may fail on more than one dependency being out of sync. You can fix these errors in 2 ways:

1. (This is the option we recommend!) Rebuilding a dependency with an older version of a Base Plan (e.g. nellshamrell/world with core/glibc/2.22) with a newer version of that Base Plan, or at least the same version as the rest of your dependencies (e.g. nellshamrell/world with core/glibc/2.27).
2. Pinning both dependencies to an older version of the Base Plan (e.g. we could pin both nellshamrell/widget and nellshamrell/world to core/glibc/2.22, and nellshamrell/widget_world would build just fine).

### Do I need to do this for all my plans?

If you have plans that depend on Base Plans (which I expect most if not all plans do), you will likely run into this dependency error on at least one of your plans. You can fix it as outlined above (note - the easiest way to rebuild a plan is to click the "Build Latest Version" button in Builder). If you need assistance, have questions, or just want some company through the updating process, please feel free to ask for help in the [Habitat Forums](https://forums.habitat.sh) - we are there during normal business hours and happy to help!
