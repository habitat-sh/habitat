
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

This happens automatically now as part of our CI pipeline.  When a PR merges
to master, the web site will automatically be deployed to production. Deploys
of the web site to acceptance is still a manual process, accomplished by
running `make deploy_acceptance`. If you need to deploy the web site to
production manually, you can run `make deploy_live`.
