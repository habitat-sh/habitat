# WWW

Static site content for www.habitat.sh

## Setup

1. Install Ruby 2.3.3 or greater
1. Install Bundler

    ```
    $ gem install bundler
    ```

## How-To: Serve Docs Locally

1. Execute the `build` task to build the docs your local machine

    `make run`

2. View the docs locally with

    `bundle exec middleman server`

3. The task will contain server output indicating what URL you should load in your browser to preview it

    `== View your site at "http://mylaptop.example.com:4567", "http://192.168.1.101:4567"`

4. You can continue to make changes to the documentation files and Middleman will reload them live
5. Press `Ctrl-C` to terminate the server when you are finished

### Troubleshooting

In some cases, you may need to install `gawk` in order to obtain the `ffi` gem. Use [Homebrew](https://brew.sh/):

  `brew install gawk`

## How-To: Deploy

This happens automatically now as part of our CI pipeline.  When a PR
merges to master, the web site will automatically be deployed to
production.

If you would like to deploy your changes to the acceptance
environment, you can manually invoke the website-acceptance
pipeline from (Buildkite > Pipelines > website-acceptance).
Hit the "New Build" button and specify your PR branch.
This pipeline does _not_ run automatically, and is provided as
a way to encapsulate all that is necessary to deploy a build to
acceptance. Alternatively, you may run `make deploy_acceptance`
locally, provided you know the appropriate Fastly service ID and have
an appropriate build environment set up. The pipeline is the preferred
way, however, as all that is taken care of for you.

Note that there is currently _no_ isolation provided for this
acceptance pipeline, so you will need to coordinate with your
teammates if more than one of you have website changes you'd like to
see at the same time. We're all adults here, though, so make it
happen.

Once your PR merges, it would be nice for you to re-invoke the
[website-acceptance pipeline][] again, pointed to the `master` branch,
in order to "reset" the acceptance website to its expected state.

[website-acceptance pipeline]: https://buildkite.com/chef/habitat-sh-habitat-master-website-acceptance
