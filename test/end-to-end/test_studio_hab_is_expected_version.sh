#!/bin/bash

# This tests that the version of hab that we are releaseing is the same 
# version embedded in the studio package. Since the studio is built 
# with the previous version of `hab` this is useful to verify that the
# correct version was copied.
 
echo "--- Generating signing key for $HAB_ORIGIN"
hab origin key generate "$HAB_ORIGIN" 

echo "--- Checking hab version in the studio"
expected_version=$(hab --version)

# This needs to be escaped like this so that all of the evaluation
# happens on the inside of the studio and it remainds correctly quoted
hab studio run test \"\$\(hab --version\)\" == \""$expected_version"\"

