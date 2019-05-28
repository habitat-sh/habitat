#!/bin/bash 

set -euo pipefail

exit_code=0

check_dot_studio_for() {
  if ! grep -q "$1" /.studio; then
    echo "Expected '$1' not found in .studio"
    exit_code=1
  fi
}

if [ ! -f /.studio ]; then 
  echo "Unable to find .studio metadata"
  exit_code=1
fi


check_dot_studio_for 'studio_type="default"' 
check_dot_studio_for 'studio_path="/hab/bin"'
check_dot_studio_for 'studio_build_environment=""'
check_dot_studio_for 'studio_enter_command="/hab/bin/hab pkg exec core/hab-backline bash --login +h"'
check_dot_studio_for 'studio_build_command="_record_build /hab/bin/build"'
check_dot_studio_for 'studio_run_environment=""'


if [[ $exit_code -eq 0 ]]; then 
  echo "[PASS] - Studio metadata present"
else 
  echo "[FAIL] - Studio metadata incorrect"
fi

exit "$exit_code"
