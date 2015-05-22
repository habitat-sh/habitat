pwd = $(shell pwd)
container_prefix = bldr
NO_CACHE = false

.PHONY: container test run shell clean

all: volumes container packages redis

baseimage:
	docker-compose run package bash -c 'cd /src/bldr-build; make baseimage_root'

packages:
	docker-compose run package bash -c 'cd /src/bldr-build; make world'

volumes: pkg-cache-volume key-cache-volume cargo-volume installed-cache-volume src-cache-volume

installed-cache-volume:
	docker create -v /opt/bldr/pkgs --name bldr-installed-cache tianon/true /bin/true

src-cache-volume:
	docker create -v /opt/bldr/cache/src --name bldr-src-cache tianon/true /bin/true

src-cache-clean:
	docker rm bldr-src-cache

pkg-cache-volume:
	docker create -v /opt/bldr/cache/pkgs --name bldr-pkg-cache tianon/true /bin/true

pkg-cache-clean:
	docker rm bldr-pkg-cache

key-cache-volume:
	docker create -v /opt/bldr/cache/keys --name bldr-keys-cache tianon/true /bin/true

key-cache-clean:
	docker rm bldr-key-cache

cargo-volume:
	docker create -v /bldr-cargo-cache --name bldr-cargo-cache tianon/true /bin/true

container:
	docker build -t chef/bldr --no-cache=${NO_CACHE} .

test:
	docker-compose run bldr cargo test

cargo-clean:
	docker-compose run bldr cargo clean

shell:
	docker-compose run bldr bash

pkg-shell:
	docker-compose run package bash

clean:
	docker images -q -f dangling=true | xargs docker rmi

redis:
	docker-compose run bldr cargo run -- start redis
