---
title: How to run packages
---

# Creating a Docker image from a package (Optional)
**Not verified. Need to go through new workflow and update steps**

To create a Docker container for any package in the public depot, perform the following steps:

1. Open the dev shell container and enter the studio with the `hab studio enter` command.
2. Change directory to the plans directory.
3. Build your version of the mytutorialapp package, or any package you want to create a Docker image from.
4. Run `dockerize` with the origin and name of your package. These values are referenced in the pkg_origin and pkg_name settings of your plan, respectively.

        [2][hab:/src/plans:0]$hab install core/hab-pkg-dockerize
        [3][hab:/src/plans:0]$hab pkg exec core/hab-pkg-dockerize hab-pkg-dockerize chef/mytutorialapp

Habitat will proceed to unpack and install the docker package, the Habitat binary package (currently named hab), your mytutorialapp package, and all of its dependencies. Then it will create an image using the Docker scratch image as the base image and build up the rest of the image from there.

Once that process has completed, you can run your Docker image inside a container from to any terminal window that has access to the Docker CLI, such as the dev shell container or your host machine.

    docker run -it chef/mytutorial-app

Again, your package must have already been uploaded into the public depot for this to work; otherwise, you will get an error like the following:

    ➜  hab git:(master) ✗ docker run -it chef/mytutorial-app
    hab(MN): Starting chef/mytutorial-app
    hab(CS): Checking remote for newer versions...
    hab(RC)[src/depot/client.rs:79:19]: Cannot find a release of package in any sources: chef/mytutorial-app
