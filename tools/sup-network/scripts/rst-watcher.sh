#!/bin/bash

watch -n 0.5 -p -d "RUST_LOG=habitat_butterfly::rumor::dat_file=info rst-reader $(find /hab/sup/default/data -iname "*.rst")"
