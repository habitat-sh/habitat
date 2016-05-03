#!/bin/bash

set -e

cd website
bundle install
bundle exec middleman build --clean --verbose

echo "Website Built!"
