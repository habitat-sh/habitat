# WWW

Static site content for www.habitat.sh

## Setup

1. Install Ruby 2.3.3 or greater
1. Install Bundler

    ```
    $ gem install bundler
    ```

## How-To: Serve Docs Locally

1. Execute the `run` task to build and start the docs server on your local machine

    `make run`

1. The task will contain server output indicating what URL you should load in your browser to preview it

    `== View your site at "http://mylaptop.example.com:4567", "http://192.168.1.101:4567"`

1. You can continue to make changes to the documentation files and Middleman will reload them live
1. Press `Ctrl-C` to terminate the server when you are finished

## How-To: Deploy

1. [Setup your workstation](#setup)
1. Configure your environment

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

1. Change to the `www` directory

   ```
   $ cd www
   ```

1. Run the deploy make task

    ```
    $ make deploy
    ```
