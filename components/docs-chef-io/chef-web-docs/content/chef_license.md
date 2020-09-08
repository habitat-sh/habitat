+++
title = "About Chef Licenses"
draft = false

aliases = ["/chef_license.html"]

[menu]
  [menu.overview]
    title = "About Licensing"
    identifier = "overview/packages_&_platforms/licensing/chef_license.md About Licensing"
    parent = "overview/packages_&_platforms/licensing"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_license.md)

All Chef products have a license that governs the entire product, while
some have separate licenses for the project source code and the
distribution that we build from it. Third-party software included in our
distributions may have individual licenses, which are listed in the
`/opt/<PRODUCT-NAME>/LICENSE` file. Individual copies of all referenced
licenses can be found in the `/opt/<PRODUCT-NAME>/LICENSES` directory.

In April 2019 many of our software distributions switched to being
governed under the Chef EULA, while the software projects remained
governed by the Apache 2.0 license. To understand which license applies
to those distributions, see the [versions page](/versions/). General
information about this change can be found in our
[announcement](https://blog.chef.io/2019/04/02/chef-software-announces-the-enterprise-automation-stack/).

## Chef EULA

The commercial distributions of our products---such as Chef Infra
Client, Chef Habitat, or Chef InSpec--- are goverened by either the
[Chef End User License Agreement (Chef
EULA)](https://www.chef.io/end-user-license-agreement/) or your
commercial agreement with Chef Software, Inc. as a customer. You are
required to accept these terms when using the distributions for the
first time. For additional information on how to accept the license, see
[Accepting the Chef License](/chef_license_accept/) documentation.

## Chef MLSA

Distributions of older proprietary Chef products---such as Chef Automate
1.x and the Chef Management Console---are governed by the [Chef Master
License and Services Agreement (Chef
MLSA)](https://www.chef.io/online-master-agreement/), which must be
accepted as part of any install or upgrade process.

## Apache 2.0

All source code of our open source Chef projects---such as Chef Infra Client, Chef
Automate, or Chef InSpec---are governed by the [Apache 2.0
license](https://www.apache.org/licenses/LICENSE-2.0).
