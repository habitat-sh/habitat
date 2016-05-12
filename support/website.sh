#!/bin/bash

set -e

cd www
bundle install --deployment
bundle exec middleman build

echo "habitat.sh built!"
