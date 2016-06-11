---
title: Environment variables
---

# Environment Variables

This is a list of all environment variables that can be used to modify the operation of Habitat.

| Variable | Context | Default | Description |
|----------|---------|---------|-------------|
| `HAB_AUTH_TOKEN` | build system | no default | Authorization token used to perform privileged operations against the depot, e.g. uploadng packages or keys.
| `HAB_CACHE_KEY_PATH` | build system, supervisor | `/hab/cache/keys` if running as root; `$HOME/.hab/cache/keys` if running as non-root | Cache directory for origin signing keys |
| `HAB_DEPOT_URL` | build system, supervisor | `https://willem.habitat.sh/v1/depot` | The depot (or materialized view in the depot) used by the Habitat build system or supervisor |
| `HAB_ORG` | supervisor | no default | Organization to use when running with [service group encryption](/docs/run-packages-security/#service-group-encryption)
| `HAB_ORIGIN` | build system | no default | Origin used to build packages. The signing key for this origin is passed to the build system. |
| `HAB_ORIGIN_KEYS` | build system | no default | Comma-separated list of origin keys to automatically share with the build system |
| `HAB_RING` | supervisor | no default | The ring used by the supervisor when running with [wire encryption](/docs/run-packages-security/#wire-encryption) |
| `HAB_RING_KEY` | supervisor | no default | The name of the ring key when running with [wire encryption](/docs/run-packages-security/#wire-encryption) |
| `HAB_STUDIOS_HOME` | build system | `/hab/studios` if running as root; `$HOME/.hab/studios` if running as non-root | Directory in which to create build studios |
| `HAB_STUDIO_ROOT` | build system | no default | Root of the current studio under `$HAB_STUDIOS_HOME`. Infrequently overridden. |
| `HAB_USER` | supervisor | no default | User key to use when running with [service group encryption](/docs/run-packages-security/#service-group-encryption) |

