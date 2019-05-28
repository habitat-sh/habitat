## sup-network

This is a tool designed to simulate a supervisor network using docker-compose.
It takes much inspiration from
[Terrarium](https://github.com/christophermaier/terrarium).

### Rationale
I wanted a tool that I could use to get a network of Habitat supervisors up and
running quickly. More than that though, I specifically wanted to be able to
test supervisors that were under active development. I wanted to run `cargo
build` on the `sup` crate and then immediately be able to test the changes
I had just made with a minimum of hassle and fuss. That's why I made this.

### Usage
To start the network, run `make up`. This will build `hab`, `hab-launch`, and
`hab-sup`, write out a `CTL_SECRET` file, and start the network. By default it
will start 1 bastion node and 3 peer nodes. The number of peer nodes that are
started can be controlled via the `HAB_SUP_TEST_NETWORK_SIZE` environment
variable.

To stop the network, run `make down`. This not only stops the network and all
the containers, but also removes them, as well as removes the `CTL_SECRET`
file. If you want to just stop containers, without removing them, you can run
`make stop`. `make start` will start them back up from a stopped state.

Nodes are started up in detached mode. Presumably, you'd like to look at the
logs for a service at some point. You can do that via `make logs`. This shows
the logs for the `rando` service by default but you can display the logs of the
bastion service by setting the `HAB_SUP_TEST_NETWORK_LOG_SERVICE` environment
variable to `bastion`.

If you would like to get a shell prompt into one of the containers, you can run
`make console`. This will get you into the `rando_1` container by default. You
can specify a different container using the `HAB_SUP_TEST_NETWORK_PEER_NAME`
environment variable.

If you wish to simulate a departure, you can kill a container using `make
kill`. Which container is killed works the same as `make console`.

Sometimes you may want the IP address of one of the containers, perhaps to do
a service load on a remote supervisor, or perhaps to satisfy your own
curiosity. `make ip` will get you that. Specifying the container works the same
as `make console`.
