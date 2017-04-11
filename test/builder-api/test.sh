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

echo "**** Spinning up the services ****"
count=0;
while [ $count -ne 5 ]; do
  count=0;
  for svc in builder-originsrv builder-sessionsrv builder-router builder-api builder-jobsrv; do
    if grep -q "$svc is ready to go" ./services.log; then
      ((count++))
    fi
  done
  sleep 1
done
echo "**** Services ready ****"
npm run mocha
mocha_exit_code=$?
echo "**** Stopping services ****"
kill -INT $forego_pid
exit $mocha_exit_code
