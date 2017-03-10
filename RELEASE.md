# Releasing Habitat

This document contains step-by-step details for how to release Habitat. All components are released
from the master branch on a bi-weekly schedule occurring every other Thursday.

## Prepare Master Branch for Release

1. Clone the Habitat repository if you do not already have it

    ```
    $ git clone git@github.com:habitat-sh/habitat.git ~/code/habitat
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
    $ vi ~/habitat/VERSION
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

## Publish Release

Create release in [GitHub](https://github.com/habitat-sh/habitat/releases)

On the Github releases page, there should already be a tag for the release (pushed up previously).
Draft a new Release, specify the tag, and title it with the same (eg, 0.18.0). Then hit Publish Release.

# Drink beer

## Bump Version

1. Update the version number found in the `VERSION` file to the next target release and append the `-dev` suffix to that number
1. Issue a PR and merge it yourself

> Example: If the release version was `0.9.0` then the contents of `VERSION` might read `0.10.0-dev` if your next target is `0.10.0`.

# Release Notification

1. Create a new post in [Habitat Announcements](https://forums.habitat.sh/c/habitat-announcements)
1. Message Slack channels and include the link to the release post
  1. cft-announce (Chef Slack)
  1. eng-announce (Chef Slack)
  1. habitat (Chef Slack)
  1. general (Habitat Slack)
  1. announcements (Habitat Slack)
