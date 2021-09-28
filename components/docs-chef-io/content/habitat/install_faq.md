+++
title = "Download and Install FAQ"
description = "Download and Install FAQ"
draft = false
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Download and Install FAQ"
    identifier = "habitat/get_started/install-faq Install Frequently Asked Questions FAQ"
    parent = "habitat/get_started"
    weight = 30
+++

This section tracks some questions that are frequently encountered when downloading and installing the `hab` binary.

**Q: Can I just download a GitHub release of Chef Habitat?**

A: While we do cut releases in GitHub as part of our release process those archives are going to be a `.tar` point in time of our source code. As the `hab` cli is written in Rust, if you follow this approach you'll need to compile the source for your platform.

**Q: Compile for my platform? Does that mean there aren't any OS native packages of `hab`?**

A: We publish compiled packages for OSX, Linux, and Windows. `hab` has a requirement of either a Linux kernel >= 2.6.32, OSX >= 10.9, or 64-bit Windows 10 Pro, Enterprise, or Education editions (1511 November update, build 10586 or later) with Hyper-V enabled.

**Q: What if I need an old version of `hab`?**

A: We've got you covered! The script we provide for doing curl-bash installations will allow you to specify a `-v` flag to pull down a specific version of Chef Habitat, eg:

```
curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh \
    | sudo bash -s -- -v 0.56.0
```

**Q: Oh! A curl bash I (love||hate) those.**

A: Indeed they are divisive, we know, that's why we provide a few different ways for you to download. If you'd like to take a look at the script before running it, you can find it in [the core Chef Habitat repo](https://github.com/habitat-sh/habitat/blob/master/components/hab/install.sh).

If you're staunchly in the anti-curl-bash camp, you can get the latest packages from the [download links]({{< relref "install_habitat" >}}) listed previously.

**Q: How do I install `hab` across my server fleet?**

A: For the most part, we leave that up to you. You could just use the aforementioned curl-bash with your provisioner of choice. If your app was dockerized with Chef Habitat then you won't even need to ask this question, because you'll have everything you need inside your container. We are working on first class Mesosphere DC/OS, and Cloud Foundry integrations - which you can keep up to date on in our [Apache Mesos and DC/OS documentation]({{< relref "mesos_dcos" >}}) and [blog](https://blog.chef.io/).
