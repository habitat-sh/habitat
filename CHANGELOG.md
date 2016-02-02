# 0.4.0

## Enhancements
- [gh-156](https://github.com/chef/bldr/pull/156) WebUI sign in and sign out flow
- [gh-158](https://github.com/chef/bldr/pull/158) Runtime dependencies should come before build dependencies, fail fast if PGP signing key is not setup, add a function for running post-compilation tests
- [gh-163](https://github.com/chef/bldr/pull/163) Add a dashboard with a package list that can be clicked through
- [gh-164](https://github.com/chef/bldr/pull/164) Link to dependencies in the package view
- [gh-171](https://github.com/chef/bldr/pull/171) Implement encryption and decryption commands for `bldr` CLI
- [gh-175](https://github.com/chef/bldr/pull/175) Show other versions and packages on the package view
- [gh-178](https://github.com/chef/bldr/pull/178) Ensure the currently selected package doesn't look like a link
- [gh-180](https://github.com/chef/bldr/pull/180) Implement SWIM membership and gossip protocol
- [gh-183](https://github.com/chef/bldr/pull/183) Allow listing of all packages
- [gh-197](https://github.com/chef/bldr/pull/197) Use clap instead of docopt for CLI argument passing

## Bug Fixes
- [gh-157](https://github.com/chef/bldr/pull/157) Corrected and consistent licensing headers for all source files
- [gh-159](https://github.com/chef/bldr/pull/159) Don't crash if a pkg is missing a config directory
- [gh-161](https://github.com/chef/bldr/pull/161) Ensure plans' build environment is created before creating the package path
- [gh-162](https://github.com/chef/bldr/pull/162) Create the working directory for the build environment earlier to ensure it exists
- [gh-182](https://github.com/chef/bldr/pull/182) Set PATH directly after resolving dependencies
- [gh-187](https://github.com/chef/bldr/pull/187) Rumor peer reporting should be at debug log level
- [gh-189](https://github.com/chef/bldr/pull/189) Fix a missing Ack timeout
- [gh-192](https://github.com/chef/bldr/pull/192) Find a way to ressurect isolated members
- [gh-193](https://github.com/chef/bldr/pull/193) Workaround for a rust performance regression

# 0.3.0
## Enhancements
- [gh-99](https://github.com/chef/bldr/pull/99) Consistent and much improved output and error formatting
- [gh-101](https://github.com/chef/bldr/pull/101) Add ability to recursively install dependencies of a package
- [gh-104](https://github.com/chef/bldr/pull/104) Stream output of running hooks to stdout/stderr
- [gh-107](https://github.com/chef/bldr/pull/107) Use GGPme rust bindings instead of shelling out to `gpg` command
- [gh-109](https://github.com/chef/bldr/pull/109) Add `pkg_build_deps` variable to `bldr-build` for specifying build dependencies
- [gh-110](https://github.com/chef/bldr/pull/110) Add chroot-based build environments
- [gh-112](https://github.com/chef/bldr/pull/112) Add `attach()` function to `bldr-build` for pry like debugging of plans
- [gh-131](https://github.com/chef/bldr/pull/131) Use libarchive rust bindings instead of shelling out to `tar` command
- [gh-132](https://github.com/chef/bldr/pull/132) Greatly improved output and dependency checking in `bldr-build`
- [gh-136](https://github.com/chef/bldr/pull/136) Added new helper functions to `bldr-build` and default phase implementations
- [gh-143](https://github.com/chef/bldr/pull/143) Added two new metafiles `TDEPS` and `BUILD_TDEPS` to differentiate between direct and  transitive dependencies in a package

## Bug Fixes
- [gh-96](https://github.com/chef/bldr/pull/96) Fix distributed deadlock in leader initialization
- [gh-98](https://github.com/chef/bldr/pull/98) Check for services' generated run file/symlink before re-linking
- [gh-137](https://github.com/chef/bldr/pull/137) Don't cache failed source downloads in `bldr-build`

## Breaking Changes
- [gh-120](https://github.com/chef/bldr/pull/120) Prefixed all public callback functions in `bldr-build` with `do_`

# 0.2.0
## Enhancements
- [gh-5](https://github.com/chef/bldr/pull/5) Added automatic updating of packages. Pass the `-u` option to `install` to watch a repository for changes
- [gh-82](https://github.com/chef/bldr/pull/82) Add initializer topology
- [gh-82](https://github.com/chef/bldr/pull/82) Add init hook
- [gh-78](https://github.com/chef/bldr/pull/78) Require package derivation in package identifiers
- [gh-94](https://github.com/chef/bldr/pull/94) Honor derivation, release, and version of a package for install, upload, and start commands
- [gh-88](https://github.com/chef/bldr/pull/88) Add `--port` flag to allow configuring of listening port for `bldr repo` command
- [gh-89](https://github.com/chef/bldr/pull/89) Install package dependencies during `bldr-build` build process
- Move Bldr's plan from `$root/plans/bldr` to `bldr-plan`
- Move all bldr plans from `$root/plans` to [chef/bldr-plans](https://github.com/chef/bldr-plans) Github repository
- remove `-r`, `-d`, and `-v` flags from command line. Replaced by fully qualified package identifier

## Bug Fixes
- [gh-83](https://github.com/chef/bldr/pull/83) Fixed functional test suite due to the new required derivation rule
- [gh-86](https://github.com/chef/bldr/pull/86) Introduced $BLDR_ROOT for bldr-build
