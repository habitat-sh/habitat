# Bintray Artifact Publishing

There are two types of artifacts currently published to the Habitat Bintray
account: a simple platform-native archive containing a `hab` CLI binary and a
Docker image containing a pre-created Studio instance. At present, only 64-bit
Linux and 64-bit Mac binaries are being produced and published and more target
platforms may be added in the future. The Docker image containing a pre-created
Studio is primarily used by Mac installations to simulate a Linux build
environment when the `hab studio` and `hab pkg build` subcommands are invoked.

## Required credentials

In order to publish one or all of these artifact types, there are several
required credentials relating to the Bintray platform:

* `BINTRAY_USER` - Bintray account username, required for both `publish-hab`
  and `publish-studio`
* `BINTRAY_KEY` - Bintray user API key, required for both `publish-hab` and
  `publish-studio`
* `BINTRAY_PASSPHRASE` - Passphrase for Bintray GPG signing key, required only
  for `publish-hab`

## Publishing `hab` binaries

The software to publish binaries is shipped and executed as a Habitat package
(naturally) which is hosted on the public Depot as the
`core/hab-bintray-publish` package. Currently this software is only supported
in a Linux environment, so an operator using a Mac workstation may opt to run
the following from a Docker container, a virtual machine, a cloud instance, a
CI worker, etc.

First, install the latest package from the Depot:

```sh
hab install core/hab-bintray-publish
```

Next, ensure that the 3 required credentials are exported as environment
variables. The program will fail if any of the required variables are not
present. Contact a Habitat core maintainer if you require access to Bintray.

```sh
export BINTRAY_USER=jdoe BINTRAY_KEY=mykey BINTRAY_PASSPHRASE=gpgkeypassphrase
```

Finally, run the publish program using `hab pkg exec` in order to have the
program's `PATH` correctly set.

```sh
# specifying a published version with a package identifier
hab pkg exec core/hab-bintray-publish publish-hab core/hab

# using a local package artifact
hab pkg exec core/hab-bintray-publish publish-hab \
  ./results/core-hab-0.7.0-20160614231131-x86_64-darwin.hart
```

## Publishing Studio Docker images

A similar workflow is used to produce and push a Docker image containing a
pre-created Studio instance.

First, install the latest package from the Depot if it's not already installed:

```sh
hab install core/hab-bintray-publish
```

Next, ensure that the 2 required credentials are exported as environment
variables. The program will fail if any of the required variables are not
present. Contact a Habitat core maintainer if you require access to Bintray.

```sh
export BINTRAY_USER=jdoe BINTRAY_KEY=mykey
```

Finally, run the publish program using `hab pkg exec` in order to have the
program's `PATH` correctly set. By default, the program will fetch `core/hab`
and `core/hab-studio` (i.e. the latest version from the Depot) but you can
specify one or more package identifiers and/or package artifacts as arguments.
Here are few examples:

```sh
# running the default behavior
hab pkg exec core/hab-bintray-publish publish-studio

# specifying more precise versions of hab and Studio
hab pkg exec core/hab-bintray-publish publish-studio core/hab core/hab-studio

# using a mix of package identifiers and local package artifacts
hab pkg exec core/hab-bintray-publish publish-studio \
  core/hab core-hab-studio-0.7.0-20160614232531-x86_64-linux.hart
```

## Building publishing package

The `core/hab-bintray-publish` Plan is located under `support/bintray-publish`:

```sh
# build the package
hab pkg build ./support/bintray-publish

# upload a result to the Depot
hab pkg upload \
  ./results/core-hab-bintray-publish-0.7.0-20160614234255-x86_64-linux.hart

# install a result
hab pkg install \
  ./results/core-hab-bintray-publish-0.7.0-20160614234255-x86_64-linux.hart
```
