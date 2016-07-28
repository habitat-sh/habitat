---
title: How to share packages
---

# Sharing packages

While you can build and run Habitat packages without sharing them on a [depot](/docs/concepts-depot/), uploading them to a depot enables you to collaborate with the Habitat community. In addition, a depot is necessary to do [continuous deployment](/docs/continuous-deployment-overview/) with Habitat.

You interact with a depot by performing the following operations:

* Creating an account in a depot
* Creating an origin, or being invited to join an origin that already exists.
* Setting up `hab` to authenticate to the depot.
* Uploading the keys for that origin.
* Uploading one or more packages into that origin.

We will use the public depot provided by the Habitat project for this section. However, it is possible to run your own depot by installing and running the `core/hab-depot` package.

## Creating an account in the depot

The depot software presently only supports GitHub authentication. For the public depot, visit https://app.habitat.sh/ in your browser and sign up for an account. Allow it to use GitHub for authorization.

## Creating an origin or joining an existing origin

You can create your own origin in the depot or be invited to join an existing one. If you already built some Habitat packages on your local computer prior to signing up for a depot account, you must rename your local packages' `pkg_origin` if the origin you want already exists.

## Setting up hab to authenticate to the depot

Because the depot uses GitHub to authenticate, you must generate a [GitHub access token](https://help.github.com/articles/creating-an-access-token-for-command-line-use/) for use with the `hab` command-line utility. The only rights this token needs at present is `user:email` since it is only used for authentication.

Once you have this token, you can set the `HAB_AUTH_TOKEN` [environment variable](/docs/reference/environment-vars/) to this value, so that any commands requiring authentication will use it.

## Uploading the keys for the origin

If you created a new origin and the depot does not already have the keys for that origin, you may upload them. If you do not upload at least the public key for the origin, the depot will reject the upload of your packages under that origin.

Providing the public key for the origin allows supervisors to verify the integrity of the Habitat packages (`.hart` files).

You can upload keys for the origin through the web interface for the depot, or by using the `hab origin key upload` command. You must be authenticated using the access token described in the previous step before you can upload keys.

## Uploading packages to the depot

Once the depot possesses at least the public key of the origin, you may upload one or more packages to that origin by using the `hab pkg upload` command. The depot will check the cryptographic integrity of the package before allowing you to upload it. Uploading packages is also a privileged operation for which you must have the access token.

## Running packages from the depot

You can instruct the supervisor to download and run packages from a depot by using the `hab start` command, for example:

       hab start core/postgresql

If the supervisor does not have the `core/postgresql` package in its local cache, it will contact the public depot, retrieve the latest version and the public key for the `core` origin, verify the cryptographic integrity of the package, and then start it.

You may also supply a `--url` argument to the `hab start` command to instruct the supervisor to use either a different depot, or a materialized channel in that depot for the purposes of continuous deployment:

       hab start core/postgresql --url http://mydepot.example.com/v1/depot

or

       hab start core/postgresql --url http://mydepot.example.com/v1/depot/channels/mychannel

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/continuous-deployment-overview">Continuous deployment</a></li>
</ul>
