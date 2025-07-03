+++
title = "About Chef Habitat Builder"
description = "Chef Habitat Builder is Chef's Application Delivery Enterprise hub"
gh_repo = "habitat"
aliases = ["/habitat/using-builder/"]

[menu]
  [menu.habitat]
    title = "Chef Habitat Builder"
    identifier = "habitat/builder/builder"
    parent = "habitat/builder"
    weight = 10
+++

Chef Habitat Builder is the core of Chef's Application Delivery Enterprise hub.
It was first launched as a cloud service and as the repository of all available plan templates built by Chef and the supporting community.
Since the application source code is stored alongside the build package, many users preferred storing packages and running [Chef Habitat On-prem Builder](/habitat/on_prem_builder/).
As a result, Chef Habitat Builder can be used as either a cloud-based or on-premises solution.
Plan files are stored in the Chef Habitat Builder SaaS, where they can be viewed and accessed by the Chef Habitat community and shared with Habitat On-prem Builder for local copying and maintenance.

## Chef Habitat Builder components

- **Application Manifest**: The application manifest provides a single application directory, which includes, at a minimum, the compiled app artifact, dynamic links to all direct and transitive runtime dependencies, and instructions to install and run the app.
- **Deployment Channel Management**: Pre-canned deployment channels that you can use as-is or customize. Apps deployed through Chef Habitat can subscribe to a channel and be auto-upgraded whenever the app is promoted.
- **Content Library**: Hundreds of pre-built [application delivery packages](https://bldr.habitat.sh/#/pkgs/core) and core dependencies make it easy to get started with Chef Habitat.
- **Custom Data and Reporting APIs**: Rich APIs enable exporting data to CSV or JSON.
- **DevOps Integration APIs**: APIs allow clients to find and download the necessary packages to run their applications. Additional APIs enable integration with popular DevOps tools, including Jenkins, Terraform, Artifactory, Hashi Vault, and others.
- **Role-Based User Access**: Improves your organization's operational safety by letting you assign specific levels of access to each origin member.

For more information about how the SaaS and on-premises versions of Chef Habitat Builder work together, read the blog: [Chef Habitat Builder On-Prem Enhancements that Extend Support to Airgap Environments and Simplify Set-Up](https://www.chef.io/blog/chef-habitat-product-announcement-builder-on-prem-enhancements-that-extend-support-to-airgap-environments-and-simplify-set-up).
