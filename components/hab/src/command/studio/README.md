## Native Packages

Native Packages are a new Habitat feature. They enable the user to run and manage applications with
the Habitat Supervisor on platforms which do not have package support.

Native Packages enable the following capabilities:
- User can create a package for any unix platform that has support for the Habitat Supervisor. Eg: M1 Macs, ARM, x86_64
- User can define hooks that can be run by a native interpreter other than bash. Eg: You could have node js or python hooks that get executed by a native node js or python interpreter in your runtime environment.

Native Packages have the following limitations:
- No depedency on other standard or native packages
- No runtime environment gaurantee. You must ensure that all required libraries and executables are available in the environment the supervisor runs the package.
- You currently cannot upload or download native packages from the Habitat Builder. You must copy the built hart file to the production runtime environment through your own deployment mechanisms.

### Pre-requistes

User must have the following tools available in their build environment in order for the Habitat plan builder to be able to
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

User can run a native package with the Habitat Supervisor by providing it with the HART file.

```bash
hab sup run your-native-package.hart
```