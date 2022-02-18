#!/bin/bash

HAB_PLAN_BUILD_DIR='../plan-build/bin'
# Change this if you want to test release binaries
HAB_BUILD_DIR='../../target/debug'

copy_binaries_into_build_context() {
    # Create the files directory and copy all debug binaries over
    mkdir -p files
    for hab_binary in hab hab-launch hab-sup hab-pkg-export-tar hab-pkg-export-container; do
      local src="${HAB_BUILD_DIR}/${hab_binary}"
      if [ -x "$src" ]; then
        cp -v $src files/${hab_binary}
      fi
    done
    # Delete and re-copy the habitat plan build shell scripts
    rm -rvf files/hab-plan-build
    cp -rv ${HAB_PLAN_BUILD_DIR} files/hab-plan-build

    # Install the record query rust binary using cargo and copy it over
    cargo install record-query
    cp -rv "$(which rq)" files
}

# Copy all the relevant binaries into the docker build context
cargo build
copy_binaries_into_build_context
docker build -t bootstrap .
