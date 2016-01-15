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
