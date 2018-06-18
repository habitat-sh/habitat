#!/bin/bash

set -eu

output="/tmp/account-creation-report.csv"

if [ "$#" -ne 1  ]; then
  echo "Please pass the initial date as the argument to this script, e.g. 2017-09-01"
  exit 1
fi

# Note that although the query below specifies shard_0, the function itself queries and aggregates results across all shards
if command -v hab > /dev/null 2>&1; then
  hab pkg exec core/postgresql psql -U hab -q -c "COPY (SELECT * FROM shard_0.account_creation_report('$1')) TO '$output' WITH (FORMAT CSV, HEADER TRUE, FORCE_QUOTE *)" builder_sessionsrv

  if [ $? -eq 0 ]; then
    echo "The report ran successfully. The CSV file can be found at $output"
  else
    echo "There was an error running the report. Make sure you're passing a date as the sole argument to this script, e.g. 2017-09-01"
  fi
else
  echo "Habitat not installed. Aborting."
  exit 1
fi

