#!/bin/bash

if [ "$I_MEAN_IT" != "1" ]; then 
  echo "**** THESE TESTS WILL DESTROY YOUR DATABASES ****"
  echo "**** AND I MEAN THE REALLY REAL ONES ****"
  echo
  echo "Set I_MEAN_IT=1 to continue"
  exit 500
fi

sudo rm -rf /hab/svc/postgresql
pushd ../../
make build-srv || exit $?
env HAB_FUNC_TEST=1 ./support/linux/bin/forego start -f support/Procfile -e support/bldr.env 2>&1 > ./test/builder-api/services.log &
forego_pid=$!
popd

# This is so brittle. Need to figure out a better way. It works on my machine!
# :)
echo "**** Spinning up the services; waiting 45 seconds ****"
sleep 45
npm run mocha
mocha_exit_code=$?
echo "**** Stopping services ****"
kill -INT $forego_pid
exit $mocha_exit_code
