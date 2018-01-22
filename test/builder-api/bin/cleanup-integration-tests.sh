#!/bin/bash

# You might be asking yourself "Why does this file even exist?" The answer to that question lies
# in the amount of time it takes to run 'test.sh'. Since test.sh is designed to be run in CI,
# and requires a full compilation of the entire builder cluster, plus a spin-up of a temporary
# PG server, it's not speedy. When you're writing new integration tests, and want to iterate
# quickly, you don't want to be running test.sh every time.
#
# Instead, it's MUCH quicker to just spin up a builder cluster like so:
#
# env HAB_FUNC_TEST=1 BLDR_NO_MIGRATIONS=1 make bldr-run
#
# and leave it running. Then, write your tests and run them against your local cluster by executing
#
# npm run mocha
#
# from the test/builder-api directory. When they complete, just run this script, and it will clear
# out all of the test data from your local cluster, ensuring that the next time you run the tests,
# it will be like running them for the first time. This allows for a much more pleasant integration
# test writing experience.
#
# You may need to temporarily modify the "depot" variable to point to wherever the root of your local depot is.

set -eu

# base_dir is the root of the habitat project.
base_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
dir="$base_dir/target/debug"
depot=/root/habitat/tmp/depot
origins=( neurosis xmen )
users=( bobo mystique )

if [ ! -x "$dir/op" ]; then
  echo "This script requires the 'op' tool to determine shards. Please cd into components/op and run 'cargo build' then try this script again."
  exit 1
fi

# cleanup origins
for origin in "${origins[@]}"
do
  shard=$($dir/op shard --origin $origin)
  sql=$(cat <<EOF
SET SEARCH_PATH TO shard_$shard;
DELETE FROM origin_members WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origin_channel_packages WHERE channel_id IN (SELECT id FROM origin_channels WHERE origin_id=(SELECT id FROM origins WHERE name='$origin'));
DELETE FROM origin_channels WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origin_integrations WHERE origin='$origin';
DELETE FROM origin_project_integrations WHERE origin='$origin';
DELETE FROM origin_invitations WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origin_packages WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origin_projects WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origin_public_keys WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origin_secret_keys WHERE origin_id=(SELECT id FROM origins WHERE name='$origin');
DELETE FROM origins WHERE name='$origin';
EOF
)
  echo $sql | hab pkg exec core/postgresql psql -U hab builder_originsrv
done

# cleanup users
for user in "${users[@]}"
do
  shard=$($dir/op shard --origin $user)
  sql=$(cat <<EOF
SET SEARCH_PATH TO shard_$shard;
DELETE FROM account_invitations WHERE account_id=(SELECT id FROM accounts WHERE name='$user');
DELETE FROM account_origins WHERE account_id=(SELECT id FROM accounts WHERE name='$user');
DELETE FROM accounts WHERE name='$user';
EOF
)
  echo $sql | hab pkg exec core/postgresql psql -U hab builder_sessionsrv
done

# cleanup jobs
for origin in "${origins[@]}"
do
  sql=$(cat <<EOF
SET SEARCH_PATH TO shard_0;
DELETE FROM busy_workers WHERE job_id IN (SELECT id FROM jobs WHERE project_name LIKE '$origin%');
DELETE FROM graph_packages WHERE ident LIKE '$origin%';
DELETE FROM group_projects WHERE project_name LIKE '$origin%';
DELETE FROM groups WHERE project_name LIKE '$origin%';
DELETE FROM jobs WHERE project_name LIKE '$origin%';
EOF
)
  echo $sql | hab pkg exec core/postgresql psql -U hab builder_jobsrv
done

# cleanup files
if [ -d "$depot" ]; then
  pushd $depot
  find . -iname "*neurosis*.hart" -delete
  popd
fi

rm -f /tmp/neurosis*.hart
