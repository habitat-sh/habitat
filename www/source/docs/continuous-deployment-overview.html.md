---
title: Continuous deployment with Habitat
---

# Continuous deployment with Habitat

Habitat provides a number of built-in features to support _continuous deployment_. Continuous deployment is the "last mile" of continuous delivery. Now that you have high-quality artifacts that have passed all tests, checked for compliance, and integrated with other components, there is still the problem of actually deploying them into production while maintaining service uptime and accounting for interdependencies over a network. For example, many applications may depend on a database cluster (with a write leader and many read followers) to be up and running. How do you upgrade the various components of that database cluster while maintaining availability to the clients?

## Philosophy

Many solutions take an orchestration-oriented approach to this problem. They coordinate deployments from a central point, and instruct the different entities on what to do in order. For example, we must quiesce and then terminate the write leader, wait for a failover to a follower, and then publish the fact that the old read follower is now the write leader to all the applications. We might need to signal the applications to reload themselves. Then we must upgrade the old write leader to the new version, and join it back to the cluster as a read follower. Then we must orchestrate the upgrades of all the other members, including the new write leader, and so on. The list of instructions becomes very complex, and accounting for all the possible error conditions becomes even more complex, if it is even possible.

More often than not, an unexpected error occurs during a complicated orchestration flow, leaving the entities in an unknown state, from which the operator must manually recover. What state have the entities been left in, and what action should be taken to converge them on the correct state? It's often impossible to know, which is why, when we refer to the scalability limits of modern distributed systems, we not only mean the _resource_ scalability, but the _cognitive_ scalability of the humans operating them. The latter is more likely to hit a limit sooner than technical scalability.

As described in our article, [Why Package the App and Automation Together?](/about/why-package-automation-with-app/), we bind automation directly to the application, thereby imbuing it with autonomy to converge on a desired state. In sum, continuous deployment in Habitat takes a promise-based approach rather than an obligation-based approach. We merely change the underlying condition -- the version of the application that should be running in a particular service group -- and the service group, plus all other service groups that depend on configuration values about that service group, automatically start responding to the change in configuration. They roll out those configuration changes according to the update strategies defined by the operator for that group.

## Implementation

Continuous deployment with Habitat uses two major features: materialized channels in the depot, and update strategies. We will illustrate the end-to-end flow by using the principles and nomenclature of [Chef Automate](https://www.chef.io/automate/).

We provide several components to help you implement continuous deployment with Chef Automate and Habitat:

* A [build cookbook](https://github.com/chef-cookbooks/habitat-build) that can build Habitat projects, including performing lint and syntax checks
* A [Ruby client library](https://rubygems.org/habitat-client) that you can use in your own Chef Automate projects to programatically interact with a depot API to achieve the tasks here.

The build cookbook accomplishes the tasks described in the following table. It performs Habitat-specific operations for promoting an artifact through acceptance, union, rehearsal and delivered environments, and performs no actions in situations where the Chef Automate user should define actions specific to their application.

### Pre-Artifact

| Lint | Uses [shellcheck](https://www.shellcheck.net/) to perform a check of the Habitat plan for conformance to shell best practices. |
| Syntax | Performs a basic syntax check using `bash -n` against the Habitat plan. |
| Unit | No default action. Intended to be overridden by the user if desired. |
| Security | No default action. Intended to be overridden by the user if desired. |
| Quality | No default action. Intended to be overriden by the user if desired. |
| Publish | Builds the package with Habitat and uploads it to the configured Habitat depot. |

At the end of the Publish phase, Chef Automate stores Habitat-specific information about the package (origin, package name, version, and release) for use in the Post-Artifact stage.

### Post-Artifact

Each of these phases runs per environment (acceptance, union, rehearsal, delivered).

| Provision | Updates the materialized channel in the depot for the application in the indicated environment with the metadata saved at the end of the pre-artifact stages. This will trigger supervisors in that environment to update the Habitat package in concordance with their configured update strategy, if any. |
| Deploy | No default action. Intended to be overridden by the user if desired. |
| Smoke | No default action. Intended to be overridden by the user if desired. |
| Functional | No default action. Intended to be overridden by the user if desired. |

### Supervisor Configuration

In order to enable automatic deployment in each environment during the provision stage, you should [configure the supervisor](/docs/run-packages-update-strategy/) to use a named channel as its depot URL, and set its update strategy. By convention, it's best to name the depot channel the same as the service group name.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/container-orchestration">Container orchestration</a></li>
</ul>
