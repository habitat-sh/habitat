#!/bin/bash

set -e

cd www
bundle install
bundle exec middleman build --clean --verbose

echo "habitat.sh built!"
