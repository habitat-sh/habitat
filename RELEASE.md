# Releasing Habitat

This document contains step-by-step details for how to release Habitat. All components are released
from the master branch on a bi-weekly schedule occurring every other Thursday.

## Workstation Preparation

1. Create a `code` directory

    ```
    $ mkdir -p ~/code
    ```

1. Clone the Habitat repository into a subdirectory of your code directory

    ```
    $ git clone git@github.com:habitat-sh/habitat.git ~/code/habitat
    ```

1. Clone the core-plans repository into a subdirectory of your code directory

    ```
    $ git clone git@github.com:habitat-sh/core-plans.git ~/code/core-plans
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

1. Download OSX 10.9 Vagrant Box

    ```
    $ aws s3 cp --profile habitat s3://habitat-initial-hab/macosx-10.9-2.1.20160526021718.git.6fdd2e497a8fc2999c4195bab9f3f5127dd70d6a.vmware.box .
    ```

1. Install [Vagrant](https://www.vagrantup.com/downloads.html)
1. Install [VMWare Fusion](http://www.vmware.com/products/fusion/fusion-evaluation.html)
1. [Purchase VMWare Fusion License](https://www.vagrantup.com/vmware/#buy-now)
1. Install [Vagrant VMWare Fusion Provider & License](https://www.vagrantup.com/docs/vmware/installation.html)
1. Install OSX 10.9 Vagrant Box

    ```
    $ vagrant box add bento/macosx-10.9 macosx-10.9-2.1.20160526021718.git.6fdd2e497a8fc2999c4195bab9f3f5127dd70d6a.vmware.box
    ```

## Prepare Master Branch for Release

1. Ensure you are on the master branch and have the latest of `~/code/habitat` and `~/code/core-plans`

    ```
    $ cd ~/code/habitat
    $ git checkout master
    $ git pull origin master
    $ cd ~/code/core-plans
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
    $ vi ~/code/habitat/VERSION
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
    $ git checkout <version>
    ```

## Build Linux Components

Before we begin a build of a plan we first need to determine which other plans from the Habitat
and core-plans repository depend on it and re-build each of those packages after we build our
target component.

> note: Dependency re-build will later be handled automatically by the Builder service

### Build `hab`

1. From outside of a studio, change into your code directory and run the build-dependent-order tool with the component's package identifier as the first argument. This will output a list of dependent packages. In the following example we will assume we are rebuilding `core/hab`

    ```
    $ cd ~/code
    $ rm build.manifest
    $ find core-plans habitat -name plan.sh | ./core-plans/bin/build-dependent-order.rb core/hab > build.manifest
    ```

1. From within a studio, run the build command *first* for the package we specified to build-dependent-order (in this case `core/hab`)

    ```
    $ hab studio enter
    > build habitat/components/hab
    ```

1. Now run the build command for each package path output by build-dependent-order

    ```
    $ cat build.manifest | while read entry; do echo "Building $(echo $entry | cut -d ' ' -f 1)"; build $(echo $entry | cut -d ' ' -f 2) || break; done
    ```

Next we will be running the same commands for hab-sup, hab-sup-static and hab-director.

### Build `hab-sub`

1. From outside of a studio:

    ```
    $ cd ~/code
    $ rm build.manifest
    $ find core-plans habitat -name plan.sh | ./core-plans/bin/build-dependent-order.rb core/hab-sup > build.manifest
    ```

1. From within a studio:

    ```
    $ hab studio enter
    > build habitat/components/sup
    > cat build.manifest | while read entry; do echo "Building $(echo $entry | cut -d ' ' -f 1)"; build $(echo $entry | cut -d ' ' -f 2) || break; done
    ```

### Build `hab-sub-static`

1. From outside of a studio:

    ```
    $ cd ~/code
    $ rm build.manifest
    $ find core-plans habitat -name plan.sh | ./core-plans/bin/build-dependent-order.rb core/hab-sup-static > build.manifest
    ```

1. From within a studio:

    ```
    $ hab studio enter
    > build habitat/components/sup/static
    > cat build.manifest | while read entry; do echo "Building $(echo $entry | cut -d ' ' -f 1)"; build $(echo $entry | cut -d ' ' -f 2) || break; done
    ```

### Build `hab-director`

1. From outside of a studio:

    ```
    $ cd ~/code
    $ rm build.manifest
    $ find core-plans habitat -name plan.sh | ./core-plans/bin/build-dependent-order.rb core/hab-director > build.manifest
    ```

1. From within a studio:

    ```
    $ hab studio enter
    > build habitat/components/director
    > cat build.manifest | while read entry; do echo "Building $(echo $entry | cut -d ' ' -f 1)"; build $(echo $entry | cut -d ' ' -f 2) || break; done
    ```

## Build Mac Components

1. Ensure no pre-existing old virtual machine, then turn on and enter the system

    ```
    $ cd ~/code/habitat/components/hab/mac
    $ vagrant destroy
    $ vagrant up
    $ vagrant ssh
    ```

1. Have the secret core origin key ready for pasting into the terminal. The `mac-build.sh` script will interactively prompt for pasting the key contents if no core origin key is installed on the VM.

1. Build Hab for Mac

    ```
    $ cd /src/components/hab/mac
    $ sudo ./mac-build.sh
    ```

## Build Windows Components

1. Ensure no pre-existing old virtual machine, then provision a new Windows machine

    ```
    $ cd ~/code/habitat/support/win
    $ vagrant destroy
    $ export ORIGIN_KEY=$(hab origin key export core --type secret)
    $ vagrant up
    ```

The script provisioner in the Vagrantfile will extract the core origin key from your environment and feed it to a powershell script on the Windows machine for import. It will also pull down all dependencies and build the windows hab binary.

## Publish Release

Create release in [GitHub](https://github.com/habitat-sh/habitat/releases)

## Publish linux components to depot

Publish each Linux component to the Depot

```
$ hab pkg upload -z <github auth token> results/*-x86_64-linux.hart
```

## Release to Bintray

1. On your workstation, change your code directory and enter a studio

    ```
    $ cd ~/code
    $ hab studio enter
    ```

1. Install the Bintray publishing code and export your credentials

    ```
    $ hab install core/hab-bintray-publish
    $ export BINTRAY_USER=yourusername BINTRAY_KEY=yourkey BINTRAY_PASSPHRASE=commongpgkeypassphrase
    ```

1. Publish the new Docker Studio image
    ```
    $ hab pkg exec core/hab-bintray-publish publish-studio
    ```

1. Publish the Linux, Mac and Windows artifacts by selecting the appropriate `.hart` file

    ```
    $ hab pkg exec core/hab-bintray-publish publish-hab \
      ./results/core-hab-0.10.2-20160930230245-x86_64-linux.hart
    $ hab pkg exec core/hab-bintray-publish publish-hab \
      ./habitat/components/hab/mac/results/core-hab-0.10.2-20160930230245-x86_64-darwin.hart
    $ hab pkg exec core/hab-bintray-publish publish-hab \
      ./habitat/components/hab/win/results/core-hab-0.10.2-20160930230245-x86_64-windows.hart
    ```

More documentation for the Bintray releasing software can be found in the component's [Readme](components/bintray-publish/README.md).

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
