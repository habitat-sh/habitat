---
title: Happy Birthday, Habitat!
date: 2017-06-28
author: Tasha Drew
tags: community, supervisor, builder, scaffolding
category: community
classes: body-article
---

![habitat-birthday-image](media/2017-06-28-Happy-Birthday-Habitat/habitat_birthday_dark.svg)

Happy Birthday, Habitat!

Last night marked Habitat’s first birthday party, celebrated with the project’s core team, which flew from their various distributed locations, local community members, and friends of the project. A public livestream allowed team members who couldn’t make it in person to also join in on a celebration of the past year’s successes and project growth, [which you can see on our YouTube channel.](https://www.youtube.com/channel/UC0wJZeP2dfPZaDUPgvpVpSg)

Only one year in, Habitat has added major features to its fully open sourced ecosystem to allow users to build, manage, and run both cloud-native and legacy applications in a service oriented architecture, using the correct format for the workload, and compatible with your scheduler or provisioning tool of choice.

The past year has seen a broad array of functionality added to the Supervisor. The Supervisor is Habitat’s process manager and has two fundamental responsibilities: starting and monitoring the child application service defined in a package, and receiving and acting upon configuration changes from other Supervisors to which it is connected. [Read all about Supervisors here](https://www.Habitat.sh/docs/concepts-Supervisor/)

  * **Multi-service Supervisors**: The Habitat Supervisor was enhanced to be able to supervise more than one service concurrently so if you are running Habitat on bare metal or a virtual machine there is only need for one Supervisor. This is also useful in a container environment if you require a secondary sidecar service running alongside your primary service. [Read more here](https://www.Habitat.sh/docs/run-packages-multiple-services/)
  * **Additional application lifecycle hooks**: as new users and contributors emerge with their own unique perspectives on what they need from lifecycle hooks (enthusiastic shout out to [Stark & Wayne’s](https://www.starkandwayne.com/) [Justin Carter](https://github.com/bodymindarts), who took the initiative to build highly available postgreSQL on Habitat ([see the core plan here](https://github.com/Habitat-sh/core-plans/tree/master/postgresql)), and became a core maintainer in his own right, the community has requested and built a series of lifecycle hooks to increase the scope of what Habitat’s runtime can achieve. [Check out Supervisor hooks here](https://www.Habitat.sh/docs/reference/plan-syntax/#hooks)
  * **Rolling application update strategy**: an on-site with [GE Digital’s Predix team](https://www.ge.com/digital/predix) in December proved the need to add a [rolling application update strategy](https://www.Habitat.sh/docs/run-packages-update-strategy/), which was quickly added as a way to update your service groups one at a time when a new package is available. The near future will see this being grown to include canary style and blue-green deploys, in conjunction with Builder’s channel capabilities.
  * **Major stability enhancements**: since the Habitat core team uses Habitat’s Supervisor to run our own services, over the past year we have added a bunch of stability enhancements, including in the run up to launching the public builder service, which saw three point releases in a week as new and interesting edge cases under load emerged.

The [Studio](https://www.habitat.sh/docs/create-packages-build/) is a clean room for Habitat package development that prevents any dependencies from being introduced other than what you explicitly use to build your package, and then only what you explicitly use to run your package is exported into your final habitat package. As such, it is a hub for local development for Habitat users, and has seen a steady evolution of features and shortcuts over the past year, the most recent set of which are discussed [here](https://github.com/habitat-sh/habitat/pull/2418).

ChefConf in May 2017 saw several major launches from the Habitat team, including:

  * **Builder**: a public build service that automatically rebuilds all `core` maintained packages (i.e. packages maintained by the core development team) as code and upstream dependencies are updated. Habitat users can then take advantage of these packages to build their own applications - and yes, Builder Neighborhoods, which extends Builder to allow you to build private application code on your own Builder cluster, is coming soon. [Read the release announcement, which has many more details, on our blog.](https://www.Habitat.sh/blog/2017/05/Builder/)
  * **Scaffoldings**: are a high level way for a developer to declare their application type in their plan.sh and then have Habitat’s automation build out your application from there. They are currently available for Ruby and Node.js applications, with more in the works as communities request and collaborate with the core team on them. [Read all about Scaffolding](https://www.Habitat.sh/blog/2017/05/Scaffolding/), and
[get step by step instructions on using the Node.js Scaffolding.](https://www.Habitat.sh/blog/2017/05/Scaffolding-App-From-Scratch/)
  * **Enterprise ready core plan push**: Habitat’s partners released a series of big data and enterprise grade core plans to extend and enhance the ecosystem for our users. [Get more details.](https://blog.chef.io/2017/05/23/enterprise-ready-Habitat-plans-now-available/)
  * **Architecture infographics released**: a picture is worth 10,000 words, and our amazing designer [Liz’s](https://twitter.com/lizchen_uw) architectural diagrams of the systems we have built help quickly communicate how the different components interact with one another. [Check them out!](https://www.habitat.sh/docs/reference/habitat-infographics/)

Tomorrow’s regularly scheduled Thursday release will continue the cadence of capability leaps, with both a new Go Scaffolding for our Go development enthusiasts, and the release of the launcher capability.

Launcher changes the process model for the Supervisor and how the Supervisor spawns and Supervisors running processes by adding a new binary, `hab-launch` which is the smallest possible Rust program whose sole responsibility is launching the latest Supervisor and spawning child processes. This allows Supervisors to be updated and upgraded without shutting down the other services that are being run, which is a major operational enhancement for our users who want the benefit of the latest Habitat Supervisor with zero downtime.  [Read the pull request here.](https://github.com/Habitat-sh/Habitat/pull/2605)

As Habitat continues to grow, extend, and mature, this team is incredibly excited to see it jump to the next level of user awareness and adoption.

Ways to get involved:

  * Check out our website: [https://www.habitat.sh/](https://www.habitat.sh)
  * Read our code (everything mentioned here is 100% open source): [https://github.com/habitat-sh/](https://github.com/habitat-sh)
  * Talk to us in Slack: [http://slack.habitat.sh/](http://slack.habitat.sh)
  * Tweet at us: [https://twitter.com/habitatsh](https://twitter.com/habitatsh)
  * Send us pizza at Seattle HQ (no red onions please)
