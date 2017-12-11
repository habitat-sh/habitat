This is the *old* Docker export code, which is no longer being
developed. To make changes to Docker export code, please look in
[components/pkg-build-docker](../pkg-build-docker) instead.

Changes to this package *will not be built* by either TravisCI or
Habitat Builder.

The code still exists in the repository because `hab-pkg-cfize` has a
runtime dependency on `hab-pkg-dockerize`. The `hab-pkg-cfize`
exporter was written around the same time (but before)
`hab-pkg-export-docker` was rewritten. As a result, it uses the older
code.

Long term, `hab-pkg-cfize` should be rewritten in Rust, following the
basic pattern laid out by `hab-pkg-export-docker`. If changes need to
be made to `hab-pkg-cfize` in the meantime, and if those changes
should also require changes to `hab-pkg-dockerize`, try and make the
changes in `hab-pkg-cfize` instead of requiring new builds of old
packages. Newcomers to Habitat shouldn't need to be confused by having
"current" packages of `hab-pkg-dockerize` alongside
`hab-pkg-export-docker`.
