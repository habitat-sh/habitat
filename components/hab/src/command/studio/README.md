## Native Packages

Native Packages are a new experimental Chef Habitat feature. They allow packages to be built outside of a studio environment 
and are therefore built against the libraries that exist directly on one's Linux distribution. 
This enables the user to run and manage applications with the Chef Habitat Supervisor on platforms which do not have package support.

Users can create a package for any unix platform that has support for the Habitat Supervisor. 
As of now only Linux x86_64 and ARM is supported.

Native Packages have the following limitations:
- No depedency on other standard or native packages
- No runtime environment gaurantee. You must ensure that all required libraries and executables are available in the environment 
where the supervisor runs the package.
- The pkg_deps and pkg_build_deps plan variables are not allowed.

### Pre-requistes

Users must have the following tools available in their build environment in order for the Habitat plan builder to be able to
successfully build a native package.

- [rq](https://github.com/dflemstr/rq/blob/master/doc/installation.md) - Record query
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

