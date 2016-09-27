# Releasing Builder

This document contains step-by-step details for how to release Builder. All components are
continuously released from the master branch to the acceptance environment and made publicly
available when the project maintainers feel enough user value has been added since the last release.

## Workstation Preparation

1. Create a `code` directory

    ```
    $ mkdir -p ~/code
    ```

1. Clone the Habitat repository into a subdirectory of your code directory

    ```
    $ git clone git@github.com:habitat-sh/habitat.git ~/code/habitat
    ```

1. [Install Habitat for Mac](https://www.habitat.sh/docs/get-habitat/)
1. Install AWS CLI

    ```
    $ brew install awscli
    ```

1. Setup an AWS profile in `~/.aws/credentials` named `habitat` with your `aws_access_key_id` and `aws_secret_access_key`

    ```
    $ cat ~/.aws/credentials
    # ~~~~~~~~~~~~~~~~~~~
    # Your other profiles
    # ~~~~~~~~~~~~~~~~~~~

    [habitat]
    aws_access_key_id=$(KEY_ID)
    aws_secret_access_key=$(SECRET_KEY)
    region=us-west-2
    ```

## Prepare Master Branch for Release

The following steps need to be performed before cutting a new public release of Builder. These
steps are not required for releasing an incremental update to the acceptance environment.

1. Ensure you are on the master branch and have the latest of `~/code/habitat`

    ```
    $ cd ~/code/habitat
    $ git checkout master
    $ git pull origin master
    ```

1. Create a new release branch in the Habitat repo

    ```
    $ cd ~/code/habitat
    $ git checkout -b <branch>
    ```

1. Remove the `-dev` suffix from the version number found in the `VERSION` file

    ```
    $ vi ~/code/habitat/VERSION-BLDR
    ```

1. Issue a new PR await approval (in the form of a [dank gif](http://imgur.com/X0sNq)) from two maintainers
1. Pull master once again once the PR is merged into master
1. Create & push a Git tag

    ```
    $ make bldr-tag-release
    $ git push origin --tags
    ```

## Build Release

1. Fetch the tags from the upstream Habitat repository and checkout the release tag

    ```
    $ cd ~/code/habitat
    $ git fetch origin --tags
    $ git checkout <version>
    ```

1. Change to your code directory and enter a studio

    ```
    $ cd ~/code
    $ hab studio enter
    ```

1. Build each of the following components with the [instructions below](#how-to-build-a-linux-component)
  * hab-builder-api - `habitat/components/builder-api`
  * hab-builder-admin - `habitat/components/builder-admin`
  * hab-builder-depot - `habitat/components/builder-depot`
  * hab-builder-jobsrv - `habitat/components/builder-jobsrv`
  * hab-builder-router - `habitat/components/builder-router`
  * hab-builder-sessionsrv - `habitat/components/builder-sessionsrv`
  * hab-builder-vault - `habitat/components/builder-vault`
  * hab-builder-worker - `habitat/components/builder-worker`

## Publish Release

1. Create release in GitHub
1. Publish each Linux component to depot (`hab pkg upload results/*-x86_64-linux.hart`)
1. Drink beer

## Bump Version

1. Update the version number found in the `VERSION-BLDR` file to the next target release and append the `-dev` suffix to that number
1. Issue a PR and merge it yourself

> Example: If the release version was `0.9.0` then the contents of `VERSION-BLDR` might read `0.10.0-dev` if your next target is `0.10.0`.

# Release Notification

1. Create a new post in [Habitat Announcements](https://forums.habitat.sh/c/habitat-announcements)
1. Message Slack channels and include the link to the release post
  1. cft-announce (Chef Slack)
  1. eng-announce (Chef Slack)
  1. habitat (Chef Slack)
  1. general (Habitat Slack)
  1. announcements (Habitat Slack)

# How-To: Build a Linux Component

From within a studio, run the build command with the path to the component

    ```
    $ cd ~/code/habitat
    $ hab studio enter
    > build components/builder-api
    ```
