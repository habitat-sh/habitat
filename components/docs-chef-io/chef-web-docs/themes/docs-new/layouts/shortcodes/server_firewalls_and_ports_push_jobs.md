TCP protocol ports 10000, 10002 and 10003. 10000 is the default heartbeat
port, 10002 is the default command port, 10003 is the default API port. These
may be configured in the Chef Push Jobs configuration file. The command port
allows Chef Push Jobs clients to communicate with the Chef Push Jobs server and
also allows chef server components to communicate with the push-jobs server. In
a configuration with both front and back ends, this port only needs to be open
on the back end servers. The Chef Push Jobs server waits for connections from
the Chef Push Jobs client, and never initiates a connection to a Chef Push Jobs
client. In situations where the chef server has a non-locally-assigned public
address (like a cloud deployment / or behind NAT ) the api port should be added
to the network security configuration for the chef server to connect to itself
on the public IP, if that is what the chef server hostname points to.
