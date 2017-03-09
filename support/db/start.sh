#/bin/bash
#
# Oh habitat, how you bring me back to my most hack-worthy roots. I love you for it.
#
# What this does - we trap our own exit, and at exit, we send a SIGINT to all the
# children in our process group - this brings habitat down. When we run tests, we
# start this script, and it will take care of setting up the test database on your
# behalf, no matter what.
#
# The gpid stuff below is because we need to track the parent process ID of the
# sudo command that executes us.

function stop_pg {
  sudo /usr/bin/killall hab-sup
  exit 0
}

trap stop_pg SIGHUP SIGINT SIGTERM

pwd
sudo mkdir -p /hab/svc/postgresql
sudo cp support/db/pg_hba.conf /hab/svc/postgresql
sudo cp support/db/user.toml /hab/svc/postgresql
sudo -s hab start core/postgresql &

echo "Waiting for the database to start before creating databases"

sleep 10

echo "Creating builder_jobsrv database if neccessary"
sudo -u hab -E TERM=vt100 hab pkg exec core/postgresql createdb -O hab -h 127.0.0.1 builder_jobsrv
echo "Creating builder_originsrv database if neccessary"
sudo -u hab -E TERM=vt100 hab pkg exec core/postgresql createdb -O hab -h 127.0.0.1 builder_originsrv

while true; do
  sleep 1
done

