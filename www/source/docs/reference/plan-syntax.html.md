---
title: Habitat plan syntax reference
---

# Plan syntax guide

When defining a plan, there are several different settings, variables, and functions that you can use to set up specific installation and configuration details. Because plans are simply script files, you have a lot of flexibility in how you build and define the configuration of your application.

This syntax guide is divided into six parts:

- [Basic settings](/docs/reference/basic-settings): The top-level settings that define your package identifier, runtime dependencies, etc.
- [Callbacks](/docs/reference/callbacks): Functions that can be overridden to customize how your application is built.
- [Build variables](/docs/reference/build-variables): Useful variables that you can use in callbacks to help package your application or service.
- [Hooks](/docs/reference/hooks): Lifecycle event handlers that are called during the Habitat service's runtime.
- [Runtime settings](/docs/reference/runtime-settings): Settings that can be used in hooks to get information about the currently-running service including templatized configuration settings in `default.toml`.
- [Utility functions](/docs/reference/utility-functions): Functions that are useful in debugging buildtime errors, building packages, and so on.
