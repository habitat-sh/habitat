+++
title = "About Chef Desktop"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "About Chef Desktop"
    identifier = "desktop/index.md About Chef Desktop"
    parent = "desktop"
    weight = 10
+++
[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/_index.md)

Chef Desktop is a deployment pattern that automates desktop and laptop management with code. Use Chef curated Desktop content with YAML support to define profiles for your devices, audit the profiles with Chef curated Desktop CIS profiles before deploying them with Chef Infra, and gain continuous visibility into the configuration status of your fleet in Chef Automate.

The desktop services target the following basic functions:

- Hard drive encryption
- Screensaver with a password
- Password policy to set complexity and other elements
- Creating a rescue account or additional user accounts
- Application management to deploy and manage apps that you care about
- Update management to control where, when, and how OS and related patches are installed
- Desktop control to limit access to features or services

## Target Audience

The target audience for Chef Desktop are administrators with limited command-line or tools experience. Our goal is to create a work pattern that leads you to explore the Chef Infra Client from the command line on your own. The instructions for the Chef Desktop pattern should guide you from starting from scratch to managing your fleet in less than a day.

## Configuration Overview

A work triangle is in play in Chef Desktop. The first leg is the Chef Automate with Chef Infra Server that holds and applies configurations to your nodes. The second leg is your administrator's development environment, which runs Chef Workstation, where you will create and define the policies and settings that the Chef Infra Server metes out. The third leg of the triangle is the list of devices, or "nodes", to which you apply the polices and settings.

## The Chef Software Stack

Chef Infra
: Chef Infra is a powerful automation platform that transforms infrastructure into code. Chef Infra automates how infrastructure is configured, deployed, and managed across your network, no matter its size.

Chef Workstation
: Chef Workstation gives you everything you need to get started with Chef. Start scanning and configuring your environments today with Chef InSpec and the `chef-run` tool. Chef Workstation runs on the computer you use everyday, whether it's Linux, macOS, or Windows.
Chef Workstation ships with Chef Infra Client, Chef InSpec, Chef CLI, Test Kitchen, Cookstyle, and other useful Chef tools. With this collection of programs and tools, you can make sure your Chef Infra code does what you intended before you deploy it to environments used by others.

Chef InSpec
: Chef InSpec is a testing framework with a human- and machine-readable language for specifying compliance, security and policy requirements. When compliance is expressed as code, you can integrate it into your deployment pipeline and automatically test for adherence to security policies.

- Next: [Chef Desktop Requirements]({{< relref  "desktop_requirements.md" >}})
