# WWW

Static site content for www.habitat.sh

## How-To: Deploy

1. Install Ruby 2.x
1. Install Bundler

    ```
    $ gem install bundler
    ```

1. Configure your environment

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

1. Ensure you have all the required gems installed, if this is your first time completing this process.

   ```
   $ bundle install
   ```

1. Run the deploy make task

    ```
    $ make deploy
    ```
