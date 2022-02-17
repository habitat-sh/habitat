#!/bin/bash

HAB_PLAN_BUILD_DIR='../plan-build/bin'
HAB_BUILD_DIR='../../target/debug'

copy_build() {
    for hab_binary in hab hab-launch hab-sup hab-pkg-export-tar hab-pkg-export-container; do
      local src="${HAB_BUILD_DIR}/${hab_binary}"
      if [ -x "$src" ]; then
        cp -v $src src/${hab_binary}
      fi
    done
    rm -rvf src/hab-plan-build
    cp -rv ${HAB_PLAN_BUILD_DIR} src/hab-plan-build
}

copy_build
docker build -t stage1 .