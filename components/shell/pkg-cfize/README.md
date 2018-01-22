# CF Exporter

Cloud Foundry is a Platform as a Service used to run 12 factor apps.

## Exporter
To export a habitat package to run on Cloud Foundry you can run:
```
$ hab pkg export cf <origin>/<package> <path/to/mapping.toml>
```

This will create 2 docker images. First it will run the docker exporter and then it will create an image based off of that one with additional layers to handle mapping from the Cloud Foundry environment to the Habitat native configuration file. The CF version of the docker image will have `cf-` as a prefix to the image tag.

```
$ docker images
starkandwayne/redmine                       cf-3.4.2-20170921100414     39d89fc95ca6        16 hours ago        553MB
starkandwayne/redmine                       3.4.2-20170921100414        9b9a155ece00        16 hours ago        549MB
```

## Mapping File
The mapping file is a toml file that can add Bash interpolated variables and scripts. The Bash code will have access to:
- all environment variables
- the jq binary
- a few helper methods

cf-mapping.toml
```
port = "${PORT}"
[db]
user = "$(service "redmine-pg" '.credentials.username')"
password = "$(service "redmine-pg" '.credentials.password')"
host = "$(service "redmine-pg" '.credentials.host')"
name = "$(service "redmine-pg" '.credentials.database')"
```

## Helpers

The helper methods are designed to extract information from the standard Cloud Foundry environment variables `VCAP_SERVICES` and `VCAP_APPLICATION`.

Helpers:
- `service <service-name> <jq-expression>` will extract the JSON associated with the given service-name from the `VCAP_SERVICES` environment variable and apply the jq-expression to it.
- `application <jq-expression>` will apply the jq-expression to the `VCAP_APPLICATION` environment variable
