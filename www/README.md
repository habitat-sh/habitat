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

1. [Setup your workstation](#setup)
2. Configure your environment

  * AWS_BUCKET - for production deploys, this should be `habitat-www`, for acceptance deploys `habitat-www-acceptance`
  * AWS_DEFAULT_REGION - this should be `us-west-2`
  * AWS_ACCESS_KEY_ID - your personal AWS account identifier
  * AWS_SECRET_ACCESS_KEY - your AWS account's personal access key
  * FASTLY_API_KEY - your Fastly account's personal access key
  * FASTLY_SERVICE_KEY - service identifier for `www.habitat.sh`

  ```
  $ export AWS_ACCESS_KEY_ID=AKIBJTPWS4EK8L2RXUBZ
  ```

  > Note: values for each of these can be found by logging into your AWS and Fastly control panels.
  > Note: make sure that the AWS credentials you use are for the Habitat AWS account and not the regular Chef AWS account.

### A slight wrinkle
If you are a Chef employee, you might be (should be) using [okta_aws](https://github.com/chef/okta_aws).

In this case, you will need to run the following:

```sh
okta_aws habitat
export AWS_DEFAULT_PROFILE=habitat
```

This eliminates the need to set `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`
manually. The other variables still need to be set.

1. Change to the `www` directory

   ```
   $ cd www
   ```

1. Run the deploy make task

    ```
    $ make deploy
    ```

    > Note: If the above task fails with a 403 Forbidden error, and you're
    > a Chef employee using okta_aws, you can deploy the web site an alternate
    > way.

    ```
    make build
    cd build
    aws s3 sync . s3://$AWS_BUCKET
    ```
