+++
title = "Chef Habitat Builder on-prem"
description = "Get Started with Chef Habitat Builder on-prem"

[menu]
  [menu.habitat]
    title = "Chef Habitat Builder on-prem"
    identifier = "habitat/builder-on-prem/getting-started"
    parent = "habitat"
    weight = 10

+++
{{< note >}}

Chef Automate has the `./chef-automate deploy --product builder --product automate` command, which offers the fastest and easiest Chef Habitat Builder on-prem installation and authentication path. See the [Install Chef Habitat on-prem with Chef Automate Guide](https://docs.chef.io/automate/on_prem_builder/) for more information.

{{< /note >}}

Chef Habitat Builder overview and pre-requisites

This documentation contains scripts to install Chef Habitat Builder on-prem services. These services (referred to as the Chef Habitat Builder on-prem) allow privately hosting Chef Habitat packages (and associated artifacts such as keys) on-premise. Chef Habitat clients (such as the `hab` cli, Supervisors and Studios) can be pointed to the Chef Habitat Builder on-prem and allow for development, execution and management without depending on the public Chef Habitat services.

## Audience

This Chef Habitat Builder on-prem [GitHub repository](https://github.com/habitat-sh/on-prem-builder/) helps customers with a substantial number of Chef Habitat packages host them in their own infrastructure. Users should be prepared to actively update their installations to benefit from continued improvements and updates.

Chef Habitat Builder (SaaS) stores application source code alongside the build package, which means that it is visible to everyone with access to the package. Chef Habitat Builder on-prem provides a private alternative to the cloud-based service. You can download plan files from the Chef Habitat Builder SaaS to an on-prem installation.

For more information on how the SaaS and On-Prem versions of Chef Habitat Builder work together read the blog - [Builder On-Prem Enhancements that Extend Support to Airgap Environments and Simplify Set-Up](https://blog.chef.io/chef-habitat-product-announcement-builder-on-prem-enhancements-that-extend-support-to-airgap-environments-and-simplify-set-up/)

### Compare Chef Habitat Builder and Builder on-prem

| Feature | Builder on-prem | Builder SaaS |
|---------|-----------------|------------- |
|Application Manifest| Yes | Yes |
|Deployment Channel Management| Yes | Yes |
|Origin Management| Yes | Yes |
|Content Library| Yes | Yes |
|Custom Data and Reporting APIs| Yes | Yes |
|DevOps Integration APIs| Yes | Yes |
|Role Based User Access| Yes | Yes |
|Container Registry Support| No | Yes |

## Requirements

The following are minimum requirements for installation/deployment of the Chef Habitat Builder on-prem:

* Services should be deployed on a Chef Habitat supported [operating system](/install-habitat)
* OS should support `systemd` process manager
* Deployment to bare-metal, VM or container image
* CPU / RAM should be appropriate for the deployment purpose:
  * 2 CPU/4 GB RAM for trial deployments
  * 16 CPU/32 GB RAM for production deployments
* Significant free disk space
  * 2GB for the baseline Chef Habitat Builder on-prem services
  * 15GB+ for the latest Chef Habitat Builder core packages
  * 30GB+ for downloading and expanding the core package bootstrap in the volume containing the `/tmp` directory
* We recommend:
  * 50 GB disk space for trial deployments
  * 100 GB disk space for production deployments
* Deploy services single-node - scale out is not yet supported
* Outbound network (HTTPS) connectivity to WAN is required for the _initial_ install
* Inbound network connectivity from LAN (HTTP/HTTPS) is required for internal clients to access the Chef Habitat Builder on-prem
* OAuth2 authentication provider (Chef Automate v2, Azure AD, GitHub, GitHub Enterprise, GitLab, Okta and Bitbucket (cloud) have been verified - additional providers may be added on request)

### Chef Habitat Builder SaaS Account

To leverage the SaaS or on-prem version of Chef Habitat Builder you will need an account on the SaaS version of Builder of Chef Habitat, which you will use to bootstrap the core Chef Habitat packages and--if necessary--to synchronize your on-prem installation with the contents of your Chef Habitat Builder SaaS account.

## Functionality

Once installed, the following functionality will be available to users:

* Logging into the Chef Habitat Builder on-prem web site
* Creation of origins, keys, access tokens, etc
* Invitation of users to origins
* Upload and download of Chef Habitat packages
* Promotion and demotion of Chef Habitat packages to channels
* Normal interactions of the `hab` client with the Chef Habitat Builder API
* Package builds using the `hab` client and Chef Habitat Studio
* Ability to import core packages from the upstream Chef Habitat Builder

The following Chef Habitat Builder on-prem functionalities are *NOT* currently available:

* Automated package builds using Chef Habitat Builder on-prem
* Automated package exports using Chef Habitat Builder on-prem

### Memory Filesystem Storage

Preparing your filesystem (Optional)
Since substantial storage may be required for holding packages, please ensure you have an appropriate amount of free space on your filesystem.
The package artifacts will be stored in your Minio instance by default, typically at the following location: `/hab/svc/builder-minio/data`
If you need to add additional storage, it is recommended that you create a mount at `/hab` and point it to your external storage. This is not required if you already have sufficient free space.
*Note*: If you would prefer to Artifactory instead of Minio for the object storage, please see the [Artifactory](#using-artifactory-as-the-object-store-(alpha)) section below.

## Next Steps

* [Install Builder on-prem authenticating with Chef Automate](./builder-automate.md)
* [Install Builder on-prem authenticating with another OAuth service](./builder-oauth.md)
