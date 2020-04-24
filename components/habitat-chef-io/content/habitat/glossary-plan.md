+++
title = "Plans"
description = "Chef Habitat Plans for Linux and Windows Systems"

[menu]
  [menu.habitat]
    title = "Plans"
    identifier = "habitat/overview/concepts/concepts/plan"
    parent = "habitat/overview/concepts"

+++

A plan is a set of files that describe how to build a Chef Habitat package. At the heart of the plan is a configurable script named `plan.sh` for Linux and `plan.ps1` for Windows, containing instructions on how to download, compile, and install its software.

Chef Habitat's build phase defaults can be overridden using [callbacks](/docs/reference/#reference-callbacks). [Application lifecycle hooks](/docs/reference/#reference-hooks) for services can be defined so the Supervisor running your services takes specific actions in response to specific lifecycle events. Optionally included are a set of TOML variables and their defaults that can be used to generate configuration files via [Handlebar.js templates](/docs/reference/#handlebars-helpers).
