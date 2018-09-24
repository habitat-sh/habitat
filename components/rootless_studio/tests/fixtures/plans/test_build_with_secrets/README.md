# Habitat package: test_build_with_secrets

## Description

Simple package used to verify that secrets can be passed into a build
using the `HAB_STUDIO_SECRET_*` method. It produces nothing of
interest, but will fail to build if a `FOO` environment variable has
not been set.

## Usage

```bash
HAB_STUDIO_SECRET_FOO=something hab pkg build test_build_with_secrets
```
