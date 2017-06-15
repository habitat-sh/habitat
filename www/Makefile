build: clean
	bundle install
	bundle exec middleman build
.PHONY: build

clean:
	rm -rf build

run: build
	bundle exec middleman serve

sync: build check-env
	bundle exec middleman s3_sync
.PHONY: sync

purge_cache: check-env
	curl -H "Fastly-Key: ${FASTLY_API_KEY}" -X POST "https://api.fastly.com/service/${FASTLY_SERVICE_KEY}/purge_all"

prep:
	sed -i '/^Disallow:/ s/$$/ \//' build/robots.txt
	zip -r website.zip build

deploy: build sync purge_cache

acceptance: build prep
	curl -H "Content-Type: application/zip" \
    -H "Authorization: Bearer $(NETLIFYKEY)" \
    --data-binary "@website.zip" \
    --url "https://api.netlify.com/api/v1/sites/habitat-acceptance.netlify.com/deploys"

check-env:
ifndef FASTLY_API_KEY
	$(error FASTLY_API_KEY is undefined)
endif
ifndef FASTLY_SERVICE_KEY
	$(error FASTLY_SERVICE_KEY is undefined)
endif
ifndef NETLIFYKEY
	$(error NETLIFYKEY is undefined)
endif

