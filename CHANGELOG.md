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
