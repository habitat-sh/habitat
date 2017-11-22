#!/bin/bash

# This script exists to run our integration tests.

set -e

# Check to see if a command exists
exists() {
  if command -v $1 >/dev/null 2>&1
  then
    return 0
  else
    return 1
  fi
}

# base_dir is the root of the habitat project.
base_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
tmp_dir=/tmp
pg_tmp_version=2.3
export BLDR_FULL_TEST_RUN=1
unset npm_config_prefix
pg_svc_dir="/hab/svc/postgresql"

# If this script is being run by Travis, then we need to point cargo to the libs we installed ourselves.
# Failure to do this means that the bldr-* components won't be able to find the shared libraries they need.
# We also need to specify that we want the "travis" user to run things instead of "hab"
if [ -n "$TRAVIS" ]; then
  rp=$(find "$HOME/pkgs" -name "lib" -type d | paste -sd ":" -)
  export LD_RUN_PATH="$LD_RUN_PATH:$rp"
  user="travis"
else
  user="hab"
fi

if ! exists curl; then
  echo "curl is required to run the integration tests. Please ensure it's installed and try again."
  exit 1
fi

if ! exists hab; then
  echo "Installing hab"
  curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
fi

if exists md5sum; then
  md5=md5sum
elif exists md5; then
  md5=md5
else
  echo "A program to calculate the md5 hash of a string is required to run the integration tests. Please sort this out and try again."
  exit 1
fi

# Install nvm if we don't have it already
if ! nvm &> /dev/null; then
  echo "Installing nvm"
  curl -o- "https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh" | bash
fi

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh"  ] && . "$NVM_DIR/nvm.sh"

# First make sure that we have services already compiled to test.
cd $base_dir

make build-srv || exit $?
cd $tmp_dir

if [[ $(uname -a) == *"Darwin"* ]]; then
  platform="mac"
else
  platform="linux"
fi

name=$(date | $md5 | awk '{ print $1 }')
dir="$tmp_dir/$name"
key_dir="$dir/key-dir"

echo "Created $dir for this test run"

mkdir -p $dir $key_dir
chmod -R 777 $dir $key_dir

env HAB_CACHE_KEY_PATH=$key_dir hab user key generate bldr

if [ -f "$tmp_dir/builder-github-app.pem" ]; then
  cp "$tmp_dir/builder-github-app.pem" $key_dir
else
  cp /root/key-dir/builder-github-app.pem $key_dir
fi

# Install pg_tmp if it's not there already
if ! exists pg_tmp; then
  echo "These tests require the use of pg_tmp. Installing version $pg_tmp_version now."
  cd $dir
  curl -O "http://ephemeralpg.org/code/ephemeralpg-$pg_tmp_version.tar.gz"
  tar zxvf ephemeralpg-$pg_tmp_version.tar.gz
  cd eradman-ephemeralpg-038b5747af8d
  sudo make install
  hash -r
fi

# Ensure normal pg commands are available for pg_tmp
if ! exists pg_ctl; then
  sudo hab pkg install core/postgresql
  sudo hab pkg binlink core/postgresql
fi

# If we're running this on travis, drop a user.toml that configures it differently
if [ -n "$TRAVIS" ]; then
  if [ ! -d "$pg_svc_dir" ]; then
    echo "hab svc directory for postgres doesn't exist. Creating now."
    sudo mkdir -p $pg_svc_dir
  fi

cat << EOF > $dir/user.toml
max_locks_per_transaction = 128

[superuser]
name = 'travis'
password = 'travis'
EOF

  if [ -d "$pg_svc_dir" ]; then
    echo "$pg_svc_dir exists. Moving user.toml into place."
    sudo mv $dir/user.toml $pg_svc_dir
  else
    echo "$pg_svc_dir still doesn't exist. WTF. Expect more failures."
  fi
fi

# This will produce a URI that looks like
# postgresql://hab@127.0.0.1:39605/test
pg_url=$(sudo su $user -c "pg_tmp -t -w 240 -o \"-c max_locks_per_transaction=128\"")
port=$(echo "$pg_url" | awk -F ":" '{ print $3 }' | awk -F "/" '{ print $1 }')

# Write out some config files
cat << EOF > $dir/config_api.toml
[depot]
builds_enabled = true
non_core_builds_enabled = true
key_dir = "$key_dir"

[github]
app_private_key = "$key_dir/builder-github-app.pem"

[segment]
write_key = "hahano"
EOF

cat << EOF > $dir/config_jobsrv.toml
key_dir = "$key_dir"

[archive]
backend = "local"
local_dir = "/tmp"

[datastore]
host = "127.0.0.1"
port = $port
user = "$user"
database = "test"
connection_retry_ms = 300
connection_timeout_sec = 3600
connection_test = false
pool_size = 8
EOF

cat << EOF > $dir/config_sessionsrv.toml
[permissions]
admin_team = 1995301
build_worker_teams = [2555389]
early_access_teams = [1995301]

[github]
app_private_key = "$key_dir/builder-github-app.pem"

[datastore]
host = "127.0.0.1"
port = $port
user = "$user"
database = "test"
connection_retry_ms = 300
connection_timeout_sec = 3600
connection_test = false
pool_size = 8
EOF

cat << EOF > $dir/config_originsrv.toml
[datastore]
host = "127.0.0.1"
port = $port
user = "$user"
database = "test"
connection_retry_ms = 300
connection_timeout_sec = 3600
connection_test = false
pool_size = 8
EOF

cat << EOF > $dir/Procfile
api: $base_dir/target/debug/bldr-api start --path $dir/depot --config $dir/config_api.toml
router: $base_dir/target/debug/bldr-router start
jobsrv: $base_dir/support/run-server jobsrv $dir/config_jobsrv.toml
sessionsrv: $base_dir/support/run-server sessionsrv $dir/config_sessionsrv.toml
originsrv: $base_dir/support/run-server originsrv $dir/config_originsrv.toml
EOF

cat << EOF > $dir/bldr.env
RUST_LOG=debug,postgres=error,habitat_builder_db=error,hyper=error,habitat_builder_router=error,zmq=error,habitat_net=error
RUST_BACKTRACE=1
HAB_DOCKER_STUDIO_IMAGE="habitat-docker-registry.bintray.io/studio"
EOF

# Travis can't run the worker, due to the inability to add the krangschnak user. If we're
# not running on Travis, add worker config in.
if [ -z "$TRAVIS" ]; then
cat << EOF > $dir/config_worker.toml
auth_token = "bobo"
bldr_url = "http://localhost:9636"
auto_publish = true
data_path = "/tmp"

[github]
app_private_key = "$key_dir/builder-github-app.pem"
EOF

  echo "worker: $base_dir/target/debug/bldr-worker start --config $dir/config_worker.toml" >> $dir/Procfile
fi

# Start all the services up
if [ "$platform" = "mac" ]; then
  echo "Running these tests on a mac"
  env HAB_FUNC_TEST=1 "$base_dir/support/mac/bin/forego" start -f "$dir/Procfile" -e "$dir/bldr.env" > "$dir/services.log" 2>&1 &
else
  echo "Running these tests on linux"
  # env HAB_FUNC_TEST=1 "$base_dir/support/linux/bin/forego" start -f "$dir/Procfile" -e "$dir/bldr.env" > "$dir/services.log" 2>&1 &
  env HAB_FUNC_TEST=1 "$base_dir/support/linux/bin/forego" start -f "$dir/Procfile" -e "$dir/bldr.env"
fi

forego_pid=$!

echo "**** Spinning up the services ****"
# total=0
# originsrv=0
# sessionsrv=0
# router=0
# api=0
# jobsrv=0
# worker=0

# while [ $total -ne 6 ]; do
#   if [ ! -f "$dir/services.log" ]; then
#     continue
#   fi

#   for svc in originsrv sessionsrv router api jobsrv worker; do
#     if grep -q "builder-$svc is ready to go" "$dir/services.log"; then
#       declare "$svc=1"
#     else
#       echo "Waiting on $svc"
#     fi
#   done

#   total=$((originsrv + sessionsrv + router + api + jobsrv + worker))

#   echo ""
#   sleep 1
# done
# echo "**** All services ready ****"

# Run the tests
cd "$base_dir/test/builder-api"
nvm install

npm install
npm run mocha
mocha_exit_code=$?
echo "**** Stopping services ****"
kill $forego_pid

cd $tmp_dir
rm -fr $dir
rm neurosis*.hart
exit $mocha_exit_code
