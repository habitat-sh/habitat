#!/bin/bash
#
# Check to see what directories have been affected by a change. If directories
# have not been affected, exit 0
#
# Since we can only stop the build early by calling "exit" from within the
# .travis.yml in the `before_install`, we exit non-zero if we want the build to
# be skipped, so we can do `|| exit 0` in the YAML.

set -eu
src_root=$(cd "$(dirname "$0")/../../"; pwd)

if [ -z ${HAB_AUTH_TOKEN+x} ]; then
  echo "HAB_AUTH_TOKEN var is unset"
  exit 1
fi

for crate in `find components/builder-* | grep plan.sh | xargs dirname`; do
  hab pkg build -k core -s $src_root -R $crate
  source $src_root/results/last_build.env
  hab pkg upload $src_root/results/$pkg_artifact -z $HAB_AUTH_TOKEN
done
