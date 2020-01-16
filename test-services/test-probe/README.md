# Habitat package: test-probe

## Description

A small [actix-web](https://github.com/actix/actix-web)-powered API
service used to introspect the operation of the
[Habitat](https://habitat.sh) Supervisor.

## Usage

Deploy to your Habitat Supervisor:

```
hab svc load habitat-testing/test-probe
```
Introspect the service's view of things by querying what would be its
[template data](https://www.habitat.sh/docs/reference/#template-data)
over its HTTP API. Simply provide the path through the data you would
like to inspect, and it will be returned to you as JSON. This approach
allows you to actively verifiy Habitat _behaviors_, as opposed to
relying on implementation details, such as knowing where a Supervisor
may write data files out to disk.

```
$ curl http://localhost:8000/context/svc/me
{
  "alive": true,
  "application": null,
  "cfg": {},
  "confirmed": false,
  "departed": false,
  "election_is_finished": false,
  "election_is_no_quorum": false,
  "election_is_running": false,
  "environment": null,
  "follower": false,
  "group": "default",
  "leader": false,
  "member_id": "af3906a38dd8450e80c313a421967669",
  "org": null,
  "persistent": true,
  "pkg": {
    "name": "test-probe",
    "origin": "habitat-testing",
    "release": "20180416163256",
    "version": "0.1.0"
  },
  "service": "test-probe",
  "suspect": false,
  "sys": {
    "ctl_gateway_ip": "127.0.0.1",
    "ctl_gateway_port": 9632,
    "gossip_ip": "127.0.0.1",
    "gossip_port": 20000,
    "hostname": "yokai",
    "http_gateway_ip": "0.0.0.0",
    "http_gateway_port": 20001,
    "ip": "192.168.67.162"
  },
  "update_election_is_finished": false,
  "update_election_is_no_quorum": false,
  "update_election_is_running": false,
  "update_follower": false,
  "update_leader": false
}

$ curl http://localhost:8000/context/svc/me/sys/ip
"192.168.67.162"
```

## Local Development

```
cargo build
```

Build a standard Habitat package

```
hab pkg build <REPO>/test-services/test-probe
```
