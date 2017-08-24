---
title: Build variables
---

# Build variables
The following variables can be used in your plans to place binaries, libraries, and files into their correct locations during package compilation or when running as a service.

**$pkg_prefix**
: The absolute path for your package.

**$pkg_dirname**
: If a .tar file extracts to a directory that's different from the filename, then you would need to override this value to match the directory name created during extraction. Default value: `${pkg_name}-${pkg_version}`. 

**$pkg\_svc_path**
: Where the running service is located. Default value: `$HAB_ROOT_PATH/svc/$pkg_name`.

**$pkg\_svc_data_path**
: Where the running service data is located. Default value: `$pkg_svc_path/data`.

**$pkg\_svc_files_path**
: Where the gossiped configuration files are located. Default value: `$pkg_svc_path/files`.

**$pkg\_svc_var_path**
: Where the running service variable data is located. Default value: `$pkg_svc_path/var`.

**$pkg\_svc_config_path**
: Where the running service configuration is located. Default value: `$pkg_svc_path/config`.

**$pkg\_svc_static_path**
: Where the running service static data is located. Default value: `$pkg_svc_path/static`.

**$HAB\_CACHE_SRC_PATH**
: The default path where source archives are downloaded, extracted, & compiled.

**$HAB\_CACHE\_ARTIFACT_PATH**
: The default download root path for packages.

**$HAB\_PKG_PATH**
: The root path containing all locally installed packages.

**$PLAN_CONTEXT**
: The location on your local dev machine for the files in your plan directory.

**$CFLAGS**
: C compiler options.

**$LDFLAGS**
: C linker options.

**$PREFIX**
: Where to install the software; same as `$pkg_prefix`.

**$LD\_RUN_PATH**
: Where to find the binaries at run time.

