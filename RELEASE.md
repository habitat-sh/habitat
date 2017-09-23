# Releasing Habitat

This document contains step-by-step details for how to release Habitat. All components are released
from the master branch on a bi-weekly schedule occurring every other Thursday.

## Prepare Master Branch for Release

1. Clone the Habitat repository if you do not already have it

    ```
    $ git clone git@github.com:habitat-sh/habitat.git ~/habitat
    ```

1. Ensure you are on the master branch and have the latest of `~/habitat`

    ```
    $ cd ~/habitat
    $ git checkout master
    $ git pull origin master
    ```

1. Create a new release branch in the Habitat repo

    ```
    $ cd ~/habitat
    $ git checkout -b <branch>
    ```

1. Remove the `-dev` suffix from the version number found in the `VERSION` file

    ```
    $ sed -i '' -e 's/-dev//' VERSION
    ```

1. Generate a new `CHANGELOG.md`

    ```
    $ export GITHUB_TOKEN=<your-token>
    $ make changelog
    ```

1. Commit the `CHANGELOG.md` and `VERSION` changes and push your branch
1. Issue a new PR await approval (in the form of a [dank gif](http://imgur.com/X0sNq)) from two maintainers
1. Pull master once again once the PR is merged into master
1. Create & push a Git tag

    ```
    $ make tag-release
    $ git push origin --tags
    ```

## Update Builder Bootstrap Bundle

Next we generate a release bundle of all Habitat and Builder components which are uploaded to an S3 bucket which we read from when we bootstrap new nodes. This bundle is useful if you are bootstrapping in an environment which doesn't have access to Builder or there simply isn't a Builder instance in existence (ah, those were the days).

1. Configure your AWS credentials in your environment
1. Run the bundle build make task

    ```
    $ make bundle
    ```

## Publish Release

Create release in [GitHub](https://github.com/habitat-sh/habitat/releases)

On the Github releases page, there should already be a tag for the release (pushed up previously).
Draft a new Release, specify the tag, and title it with the same (eg, 0.18.0). Then hit Publish Release.

# Drink. It. In.

## Bump Version

1. Update the version number found in the `VERSION` file to the next target release and append the `-dev` suffix to that number
1. Issue a PR and merge it yourself

> Example: If the release version was `0.9.0` then the contents of `VERSION` might read `0.10.0-dev` if your next target is `0.10.0`.

# If your Release is going to cause downtime

1. Put a scheduled maintanence window into PagerDuty so the on-call doesn't go off.
1. Pre-announce outage on Twitter & Slack #general channel. There is no hard rule for a length of time you need to do this ahead of a release outage.
1. When your release begins, post announcement in our statuspage.io with outage information.
1. Per [ON_CALL.md](#ON_CALL.md) you are responsible for updating status changes in statuspage.io as the downtime proceeds. You are not responsible for regular minutes or responding in any other venue.
1. When the downtime is over, announce the end of the outage via statuspage.io. It will automatically post an announcement to #general and twitter.

# Release Notification

1. Create a new post in [Habitat Announcements](https://forums.habitat.sh/c/habitat-announcements)
1. Link forum post to the github release
1. Link github release to forum post
1. Message Slack channels and include the link to the release post
   1. cft-announce (Chef Slack)
   1. eng-announce (Chef Slack)
   1. product-marketing (Chef Slack)
   1. general (Habitat Slack)
   1. announcements (Habitat Slack)
1. Tweet a link to the announcement @habitatsh
