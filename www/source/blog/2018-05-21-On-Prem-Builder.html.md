---
title: Introducing Habitat On-Premises Builder Depot
date: 2018-05-21
author: Salim Alam
tags: habitat, builder, depot, on-premises
category: builder
classes: body-article
---

### Introduction

One of the frequent requests that we have had since we introduced public Habitat Builder last year has been to allow Habitat users to have the same workflow behind the firewall in their private cloud- and on-premises infrastructure. There are several reasons why this can be desirable - for example, having an enterprise policy that prohibits hosting of internal software in the public cloud, or having a completely "air-gapped" network isolation requirement. 

Running Builder services on-premises represents an interesting challenge. On the one hand, it is a fairly sophisticated set of scaled out services with a broad range of operational parameters. On the other hand, it is also entirely self-hosted with Habitat itself, and therefore can take advantage of all the Habitat capabilities for deploying, configuring and managing services. So the undertaking was not as daunting as it might have seemed otherwise.

Earlier in the year we rolled out an alpha version of the on-premise Builder Depot to some of Chef's early-adopter partners. We are now happy to make the functionality available more broadly, so that more members of the Habitat community can make use of these capabilities.

### What is the Habitat Builder Depot?

The Habitat Builder Depot is the set of services that provide the capability to host Habitat packages on-premises. It comes with a Web front end to allow users to log in with a number of different authentication providers, and also hosts a REST API that Habitat clients (i.e., the `hab` binary running from any workstation) can be pointed to.

The set of capabilities provided by On-Premises Habitat Builder Depot are:

* Logging into the on-premises Builder Depot web site
* Creation of origins, keys, access tokens, etc
* Invitation of users to origins
* Upload and download of Habitat packages
* Promotion and demotion of Habitat packages to channels
* Normal interactions of the `hab` client with the Builder API
* Package builds using the `hab` client and Habitat Studio
* Ability to import core packages from the upstream Habitat Builder

Additionally, the on-premises experience integrates with existing CI/CD tool chains and workflows. For example, Habitat builds via Jenkins or GitLab build clusters can be uploaded to the on-premises Builder Depot. The on-premise Builder Depot does not provide the automated build functionality that the public Builder does.

Here is an architectural diagram that illustrates how the Builder Depot can function integrated into a CI/CD pipeline on-premises:

![On-Premise Builder Depot Architecture](https://www.habitat.sh/images/infographics/habitat-on-premises-builder-depot-flow-011f12f5.png)

Finally, there are some features that are present in the on-premises Builder Depot that are not available in public Builder. The authentication options available are broader - the operator has the ability to pick from a set of OAuth and OpenId Connect providers for the on-premise install. Also, the on-premise installation can be configured (if desired) to automatically download stable versions of core packages from the upstream SaaS Builder. This allows the on-premises installations to more easily keep their core packages up to date. 

### Where and how to install?

The installation instructions and scripts for the on-premises Builder Depot can be found in the GitHub repository [here](https://github.com/habitat-sh/on-prem-builder). The [README](https://github.com/habitat-sh/on-prem-builder/blob/master/README.md) has detailed information on pre-requisites, as well as step-by-step guidance on installation. 

For questions or feedback, please drop a message in the [Habitat Forum](https://forums.habitat.sh/latest), or on our [Slack channel](https://habitat-sh.slack.com), or even open an issue in the GitHub repository.

We hope you enjoy this new capability!

The Habitat Team
