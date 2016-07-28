#!/bin/bash

ACTION=$1
ALL_ACTIONS="start stop build restart"
ALL_SERVICES="api router jobsrv sessionsrv vault"

# Ensure a place for pidfiles exists
mkdir -p /src/tmp/run-api

# Install and start redis if it's not running
if ! pgrep -f redis-server > /dev/null; then
  if [[ ! -f /usr/bin/redis-server ]]; then
    echo "Redis not found. Installing..."
    apt-get update
    apt-get install redis-server -y
  fi
  service redis-server start
fi

build() {
  local root=/src/components
  local binroot=/src/target
  for service in $ALL_SERVICES; do
    local dir="builder-$service"
    local bin="bldr-$service"
    # sessionsrv has a different naming convention
    if [[ "$service" = "sessionsrv" ]]; then
      local bin=bldr-session-srv
    fi
    if [[ "$service" = "jobsrv" ]]; then
      local bin=bldr-job-srv
    fi
    # If the compiled binary does not exist, compile it
    if [[ ! -f "$binroot/debug/$bin" ]]; then
      echo "Compiling builder-$service..."
      (cd "$root/$dir" && cargo build)
    fi
  done
}

start() {
  export RUST_LOG=debug
  local root=/src/components
  local binroot=/src/target
  for service in $ALL_SERVICES; do
    local dir="builder-$service"
    local bin="bldr-$service"
    # sessionsrv has a different naming convention
    if [[ "$service" = "sessionsrv" ]]; then
      local bin=bldr-session-srv
    fi
    if [[ "$service" = "jobsrv" ]]; then
      local bin=bldr-job-srv
    fi
    # Start the service and record it's pid if it's not already running
    if ! pgrep -f "$bin" > /dev/null; then
      echo "Starting builder-$service..."
      "$binroot/debug/$bin" start & \
        echo $! > "/src/tmp/run-api/$service.pid"
    fi
  done
}

stop() {
  for service in $ALL_SERVICES; do
    local pidfile="/src/tmp/run-api/$service.pid"
    if [ -f "$pidfile" ]; then
      echo "Stopping builder-$service..."
      kill "$(cat "$pidfile")"
      rm -f "$pidfile"
    fi
  done
}

restart() {
  stop
  start
}

if [[ "$ACTION" != "" ]] && [[ "$ALL_ACTIONS" =~ $ACTION ]]; then
  "$ACTION"
else
  echo "Usage: $(basename "$0") {${ALL_ACTIONS// /|}}"
  exit 1
fi
