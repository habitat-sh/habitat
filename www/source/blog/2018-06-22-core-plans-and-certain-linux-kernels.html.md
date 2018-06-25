---
title: Core Plans and Certain Linux Kernels
date: 2018-06-22
author: Nell Shamrell-Harrington
tags: Core Plans
category: update
classes: body-article
---

Greetings, fellow Habicats!

This past Tuesday we completed a refresh [of the Habitat Core Plans](https://www.habitat.sh/blog/2018/06/base-plans-refresh/).

While we expected some dependency resolution errors to happen as a result of the refresh, another issue came up which we did not anticipate.

## What Happened?

Part of the major base plans refresh was upgrading [core/glibc](https://github.com/habitat-sh/core-plans/tree/master/glibc) from 2.22 to 2.27.

[GNU glibc](https://www.gnu.org/software/libc/) is the interface to the Linux kernel. Most if not all Linux core plans (and Habitat core components for Linux) use glibc. We did not realize before the refresh that glibc 2.26 and up require the kernel headers from Linux kernel 3.2 or later. This was partly due to the length of time since the last base plans refresh (over a year) - so many things had to be upgraded that we missed the change in required kernel.

This meant that after we completed the base plans refresh anyone who attempted to use any Habitat components or core plans on a system running a Linux kernel lower than 3.2 starting receiving failures.

Our [Habitat Docs](https://www.habitat.sh/docs/install-habitat/) state that we support Linux kernel 2.6.32 or greater, so we are now out of sync with our stated supported versions.

## Who Is Affected?

This affects anyone running Habitat core plans on systems with Linux kernels < 3.2. This includes (but is not limited to):

* Red Hat Enterprise Linux (RHEL) 6
* SUSE Linux Enterprise Server (SLES) 6
* CentOS 6

Anyone running Habitat core plans on systems with Linux kernels greater than or equal to 3.2 is not affected.

## What Are You Doing About It?

Our first inclination was to roll back the base plans refresh to a lower version of glibc. However, this would have re-broken anyone who already had consumed the refreshed base plans. Additionally, the base plans very badly needed to be refreshed for security and other reasons. If we rolled back the base plans refresh, we would find ourselves needing to redo the refresh soon and experience the pain of a giant refresh again. We decided it is better to continue forward.

We are starting to lay out the work for supporting multiple Linux architectures - Linux using glibc 2.25 (the latest version of glibc that works with Linux kernel 2.6.x) and then Linux using glibc 2.26 and up (which requires Linux kernel 3.2.x). This will set us up well for supporting additional Linux architectures in the future (i.e. ARM Linux).

We will also be adding testing for core plans and Habitat components to ensure that they work with both Linux architectures or that, if one does not work with one or the other, there is an alternative and it is well communicated.

## Questions?

If you have further questions about this, please [feel free to reach out in the Habitat forums!](https://forums.habitat.sh/)
