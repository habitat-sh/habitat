+++
title = "Scaling Chef Habitat Builder on-prem"
description = "Tiered or HA deployment of Chef Habitat Builder on-prem services"

[menu]
  [menu.habitat]
    title = ""
    identifier = "habitat/builder-on-prem/scaling"
    parent = "habitat"

+++

With any tiered or HA deployment of the builder services you'll likely want to horizontally scale your front-end nodes. The most common deployment pattern for this case is a pool of front-end nodes fronted by a load-balancer.

## Deploying New Front-ends

The on-prem-builder install.sh script now supports scaling front-end nodes as a deployment pattern. It is require that new front-ends be deployed on a separate compute from your initial on-prem deployment. Similarly to Chef Automate's bootstrap pattern the on-prem builder install script can generate a bootstrap bundle which is used to simplify the deployment of new front-ends.

### Create and update bldr-frontend.env

The bldr.env file for your single on-prem builder node contains most of the information required to bootstrap a new front-end and will be used during the installation process. However, some configuration will  need to change.

First, you'll need to copy your `bldr.env` file to `bldr-frontend.env`.

In the case that your on-prem-builder cluster is backed by cloud services, you will only need to update the value of `OAUTH_REDIRECT_URL`. When running multiple front-end instances this value should be pointed to your load-balancer.

In the case that you are _not_ backing your cluster with cloud services you will need to update the values of `OAUTH_REDIRECT_URL`, `POSTGRES_HOST`, and `MINIO_ENDPOINT`.

1. Copy your `on-prem-builder/bldr.env` to `bldr-frontend.env`
1. Update the contents `bldr-frontend.env` to match your deployment pattern

### Generate & send bootstrap_bundle.tar

Once the bldr-frontend.env file's contents have been updated with appropriate information we should be ready to generate the bootstrap bundle. After creation, we'll send it to the target node we intend to run the front-end service on.

1. Generate a bootstrap bundle `./install.sh --generate-bootstrap`
1. Copy the generated `/hab/bootstrap_bundle.tar` to the same path on the new frontend node

### Install frontend

1. Run the front-end install script from the new front-end node `./install.sh --install-frontend`
