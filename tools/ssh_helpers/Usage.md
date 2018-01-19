# Prerequisites

These scripts make it easy to connect to running Habitat instances. To use them, you'll need to set up a few things first:

* Make sure you've downloaded the `habitat-srv-admin` key and placed it at at `~/.ssh/`.
* If you haven't already, create an IAM user for yourself in Habitat AWS. You'll need membership in the `admins` group and programmatic access. Note your generated access key ID and secret.
* Install [the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-install-macos.html) and place it on your PATH.
* Configure the AWS CLI using your Habitat AWS IAM credentials.
* Install [jq](https://stedolan.github.io/jq/). (e.g., `brew install jq`)
* Install [tmux](https://github.com/tmux/tmux/wiki). (e.g., `brew install tmux`)
* Install [tmuxinator](https://github.com/tmuxinator/tmuxinator). (`gem install tmuxinator`)

# Generating and Updating Configuration

Once you're set up with the prerequisites listed above, you should be able to generate SSH and tmuxinator configurations using the following commands (executed from within this directory):

```
./update-habitat-ssh acceptance
./update-habitat-ssh live
```

And with that, connect to running environments:

```
./hab-env acceptance
./hab-env live
```

Any troubles, ask in #core-dev! Enjoy.
