## Native Packages

Native Packages are a new experimental Chef Habitat feature. They allow packages to be built outside of a studio environment.

For clarity we refer to packages that are built inside a studio environment as a Standard Package.

Native Packages can directly use dependencies that exist on the build system. They can also use Standard Packages as dependencies.
This enables users to build and run applications with the Chef Habitat Supervisor even on platforms that do not have any Standard Packages available. 

Users can create a Native Package for any unix platform that has support for Habitat. 
As of now the Habitat team only provides pre-built binaries for x86_64-linux and aarch64-linux (ARM) platforms.

Since Native Packages do not use a studio environment while building there is no guarantee that build succeeds on a new system. 
It is up to the user of the package to ensure that all build time dependencies are available on the system before building the package. 

While this seems limiting, it enables the use of any tool on the build system to build the package without having to first make 
a package for that build tool and each of it's dependencies.

Runtime portability of a Native Package, similar to a Standard Package, is the responsibility of the package author. The package
source has to be appropriately patched, configured and built to ensure the binaries can be executed in the final runtime
environments.

If a Native Package is intended to be used as a build time dependency of a Standard Package, the package author must ensure 
that the binaries will be able to run inside the studio environment during the build of the Standard Package.

### Pre requisites

Users must have the following tools available in their build environment in order for the Habitat plan builder to be able to
successfully build a native package.

- [rq](https://github.com/dflemstr/rq/blob/master/doc/installation.md) - Record query (Optional, required only if your plan uses `pkg_exposes`)
- wget
- stat
- tar
- xz
- gsha256sum or shasum256

### Building a Native Package

To build a native package use the following commands:

```bash
export HAB_FEAT_NATIVE_PACKAGE_SUPPORT=1
# -N indicates that this is a native package build
hab pkg build -N .
```

### Running a Native Package

Users can run a native package with the Habitat Supervisor by providing it with the HART file.

```bash
hab sup run your-native-package.hart
```

### Native Packages Builder Support

We also support native packages on the on-prem builder. Following are the key points to note: 
- A native package can be uploaded onto the builder and can be identified in the UI with the tag "native" next to the package.
- The type of package (i.e native or standard) cannot change in the subsequent releases. 
For instance, If a package is uploaded as native, all the future versions throughout its lifetime also have to be native.

#### Configuring native packages support on the builder
The builder-api service configuration has to be updated to support native packages.
- The features_enabled key must have "nativepackages".
- We only allow native packages from the origins mentioned in the allowed_native_package_origins array. 
Users can update their required origins using this template - `allowed_native_package_origins = ["origin1","origin2",...]`

The above configuration updates can be applied to the builder-api service using a user.toml file. Refer [this document](https://docs.chef.io/habitat/service_updates/#using-a-_usertoml_-file) to know how.

