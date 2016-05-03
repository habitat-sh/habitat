---
title: Making a permanent change
---

# Make a permanent change
Now that you have seen how to push configuration updates to a Habitat package, let's make a permanent change by updating a default TOML file in the package source folder. This will require you to get the source code for the project, make the change, and then rebuild the nodejs-tutorial-app package.

## Update the configuration file
In your bash prompt on your local machine, do the following steps:

1. Change directory to the src/plans/nodejs-tutorial-app folder and open the default.toml file with your favorite text editor.
2. Change the MOTD value from `"Hello, World!"` to the temporary message from the previous step, `"Habitat rocks!"`
3. Save your changes.

  > Note: If pkg_version is part of the pkg_source variable in plan.sh, you should not update the pkg_version number when making a change to a package file, as is the case with nodejs-tutorial-app.

## Rebuild the package
Now that you have saved your changes to the default.toml file, you need to rebuild the nodejs-tutorial-app package. To do that, you will have to rebuild it on your local machine, which will require you to build the development environment used for building Habitat itself. We will not go too in-depth on the workflow here as the process will change in the future. The goal is to get you to be able to build your package and see the results.

1. From the root of your local repo, run `make shell`. This will build the Docker container that will then be used to build the Studio environment. The studio is where you actually will build your version of the nodejs-tutorial-app package.

    Once `make shell` completes, you will be at a bash prompt inside the habitat/devshell container. You should see a prompt like the following:

    `root@622a476a8d66:/src#`
3. Enter into the studio environment by typing `studio enter`. The studio is a self-contained, minimal shell environment that you use to make Habitat packages. If successful, you should see another bash prompt like this:

    `[1][hab:/src:0]$`
3. Type `cd plans`. This is where all of the current Habitat source packages are contained. This is also where you create new plans for other packages.
4. Run `make gpg` to import a public/private key pair into the Studio environment. The public key will be used to sign the tarball created from your package.

    > Note: This command also imports a secret key used to decrypt the package. Both public and private keys are stored in a local cache.

5. Building packages is a simple process of typing `build packagename`, so while you are in the `/src/plans` directory, type `build nodejs-tutorial-app` to build the package. The end of the build process should look like this:


       nodejs-tutorial-app: Installing
       nodejs-tutorial-app: Building package metadata
       nodejs-tutorial-app: Writing configuration
       nodejs-tutorial-app: Writing service management scripts
       nodejs-tutorial-app: Stripping binaries
       nodejs-tutorial-app: Creating manifest
       nodejs-tutorial-app: Generating package
      /hab/pkgs/chef/tar/1.28/20160310205614/bin/tar: Removing leading `/' from member names
       nodejs-tutorial-app: hab cleanup
       nodejs-tutorial-app: Cache: /hab/cache/src/nodejs-tutorial-app-0.1.0
       nodejs-tutorial-app: Installed: /hab/pkgs/chef/nodejs-tutorial-app/0.1.0/20160327065554
       nodejs-tutorial-app: Package: /hab/cache/pkgs/chef-nodejs-tutorial-app-0.1.0-20160327065554.hab
       nodejs-tutorial-app:
       nodejs-tutorial-app: I love it when a plan.sh comes together.
       nodejs-tutorial-app:
       nodejs-tutorial-app: Build time: 1m8s
       [5][hab:/src/plans:0]$


    The process of how packages are built, what they are comprised of, and how to configure them are documented in more detail in both the /plans/hab-build script file and in the [Creating packages](/docs/create-packages-overview) and [Building packages](/docs/build-packages-overview) sections of the product documentation.

6. After your package finishes building, type `exit` to leave the studio, but keep this dev shell container running in a terminal window on your host machine.

    You are now ready to move onto the [last step](/tutorials/getting-started-process-build) where you will upload your package to a depot and run it.
