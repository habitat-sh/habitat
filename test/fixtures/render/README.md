# About

These files are for testing `hab plan render` command.

see `hab plan render --help` for full usage instructions.

# Usage

`cd` to `habitat/components/hab`

Try:


```
cargo run -- plan render ../../test/fixtures/render/consul/config/consul_config.json \
  --default-toml ../../test/fixtures/render/consul/default.toml \
  --user-toml ../../test/fixtures/render/consul/user.toml \
  --mock-data ../../test/fixtures/render/consul/override.json \
  --render-dir ~/result/config \
  --print
```

```
cargo run -- plan render ../../test/fixtures/render/consul/config/consul_config.json \
  --default-toml ../../test/fixtures/render/consul/default.toml \
  --render-dir ~/result/config \
  --print
```

or

```
cargo run -- plan render ../../test/fixtures/render/consul/hooks/run \
  --default-toml ../../test/fixtures/render/consul/default.toml \
  --user-toml ../../test/fixtures/render/consul/user.toml \
  --mock-data ../../test/fixtures/render/consul/override.json \
  --render-dir ~/result/hooks \
  --print
```

# Example output

* `consul/config/basic_config.json` render:

```
{
  "datacenter": "IN_OVERRIDE_JSON",
  "data_dir": "IN_DEFAULT_TOML",
  "log_level": "IN_USER_TOML",
  "bind_addr": "9.9.9.9",
  "client_addr": "9.9.9.9",
  "server": true,
  "retry_join": [
  ],
  "ports": {
    "dns": 6666,
    "http": 6667,
    "https": 6668,
    "serf_lan": 8888,
    "serf_wan": 8302,
    "server": 9999
  }
}
```

* `consul/hook/run` render:

```
#!/bin/sh

exec 2>&1

SERVERMODE=true
export CONSUL_UI_LEGACY=false

CONSUL_OPTS="-dev"
if [ "$SERVERMODE" = true ]; then
  CONSUL_OPTS=" -ui -server -bootstrap-expect 3 -config-file=/basic_config.json"
fi

exec consul agent ${CONSUL_OPTS}
```
