# Docker compose example of habitized ELK stack

## Usage
From the `ELK` directory:
* `direnv allow` or `source ../.envrc`
* `hab studio enter`
* `hab pkg export docker core/elasticsearch`
* `hab pkg export docker core/kibana`
* `build examples/logstash`
* `hab pkg export docker $HAB_ORIGIN/logstash`
* `exit`
* `cd docker`
* `docker-compose up`
