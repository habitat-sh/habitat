+++
title = "Chef Habitat and Configuration Management"
description = "Chef Habitat and Configuration Management"
draft = true

[menu]
  [menu.habitat]
    title = "Chef Habitat and Configuration Management"
    identifier = "habitat/containers/habitat-and-configuration-management"
    parent = "habitat/containers"
    weight = 20

+++

**Examples: [Ansible](https://www.ansible.com/), [Chef](https://www.chef.io/chef/), [Puppet](https://puppet.com/), and [Salt](https://saltstack.com/)**

Configuration management tools allow you write configuration files, using a declarative language to manage a server. These tools focus on building working servers by installing and configuring system settings, system libraries, and application libraries before an application is installed on the server. Chef Habitat focuses on the application first instead of the server. Chef Habitat builds and packages your application's entire binary toolchain, including the system libraries, application libraries, and runtime dependencies necessary for your application to function. As a result, Chef Habitat can replace many use-cases that configuration management tools perform related to installing system binaries, application dependent libraries, or templating configuration files.

Configuration management tools perform tasks at run time by converging resources. The value from configuration management tools comes from this converging process -- checking the existing state of a server, and fixing it if it does not match the intended state. Because converging modifies resources at runtime, it can result in surprising and complex runtime errors. In addition, since environments are often mutable and unique, maintaining server automation occurs out-of-band with application development, creating conflict between application developers and software reliability engineers. Chef Habitat avoids these classes of errors entirely by shifting these processes to build time, and by creating an atomic package of an application's binaries, application lifecycle hooks, and configuration files. Chef Habitat's approach to packaging automation with the application package allows application developers and software reliability engineers to work closer together.

Chef Habitat is not a full replacement for configuration management tools on mutable infrastructure. Instead, it allows configuration management tools to focus better on system-level tasks for virtual machines and bare metal, such as kernel tuning, system hardening tasks, and compliance remediation tasks. Chef Habitat can then take over application automation roles, which results in a significant reduction in automation complexity for both infrastructure-focused automation and application-focused automation.

Chef Habitat can make it easier to run your existing configuration management tool. You can create a Chef Habitat package of your configuration management tool's agent and/or dependencies, and run it on your existing mutable infrastructure. The Chef Habitat Supervisor's responsibility is to update your configuration management tool's agent, while your configuration management tool can still perform its normal tasks.

Chef Habitat can provide an easier transition from virtual machine or bare metal workloads to containers, without needing to rewrite a monolithic application into microservices all at once. In this scenario, you can run the [Chef Habitat Supervisor](https://www.habitat.sh/docs/using-habitat/#overview) on your existing virtual machine or bare metal infrastructure as you migrate away from your configuration management tool. Then, when you're ready, you export your application to the container format of your choice using the [Chef Habitat Studio](https://www.habitat.sh/docs/plan-overview/#plan-builds). While you migrate your applications and services, the [Chef Habitat Supervisor](https://www.habitat.sh/docs/using-habitat/#overview) runs on your existing mutable infrastructure, and runs your existing configuration management tool. New packages that do not require configuration management can also run under the [Chef Habitat Supervisor](https://www.habitat.sh/docs/using-habitat/#overview) on your existing mutable infrastructure. As a result, you can continue to verify the working state of your application as you incrementally migrate your services. This approach provides an alternative to the "all-or-nothing" migration many teams are faced with when moving workloads to containers.
