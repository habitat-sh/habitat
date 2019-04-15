build: clean
	gem install bundler
	bundle install
	bundle exec middleman build --no-parallel
.PHONY: build

clean:
	rm -rf build

run: build
	BUILDER_WEB_URL="http://localhost:3000" GITHUB_APP_URL="https://github.com/apps/habitat-builder" bundle exec middleman serve

sync: build check-env
	bundle exec middleman s3_sync
.PHONY: sync

purge_cache: check-env
	curl -H "Fastly-Key: ${FASTLY_API_KEY}" -X POST "https://api.fastly.com/service/${FASTLY_SERVICE_KEY}/purge_all"

prep:
	sed -i'' '/^Disallow:/ s/$$/ \//' build/robots.txt
	zip -r website.zip build

deploy: build sync purge_cache

acceptance: build prep
	curl -H "Content-Type: application/zip" \
    -H "Authorization: Bearer $(NETLIFYKEY)" \
    --data-binary "@website.zip" \
    --url "https://api.netlify.com/api/v1/sites/habitat-acceptance.netlify.com/deploys"

cli_docs:
	hab studio run "hab pkg install core/hab-studio && \
	hab pkg install core/hab-sup && \
	hab pkg install core/hab-launcher && \
	hab pkg install core/node --binlink && \
	node scripts/generate-cli-docs > source/docs/habitat-cli.html.md"

node_modules/json-schema-ref-parser:
	npm install json-schema-ref-parser@6.1.0

template_reference: node_modules/json-schema-ref-parser
	mkdir tmp
	cp ../components/sup/doc/* tmp/
	hab studio run "hab pkg install core/node --binlink && \
	node scripts/generate-template-reference.js tmp/render_context_schema.json > ./source/partials/docs/_reference-template-data.html.md.erb"
	rm -rf tmp

check-env:
ifndef AWS_ACCESS_KEY_ID
	$(error AWS_ACCESS_KEY_ID is undefined)
endif
ifndef AWS_BUCKET
	$(error AWS_BUCKET is undefined)
endif
ifndef AWS_DEFAULT_REGION
	$(error AWS_DEFAULT_REGION is undefined)
endif
ifndef AWS_SECRET_ACCESS_KEY
	$(error AWS_SECRET_ACCESS_KEY is undefined)
endif
ifndef FASTLY_API_KEY
	$(error FASTLY_API_KEY is undefined)
endif
ifndef FASTLY_SERVICE_KEY
	$(error FASTLY_SERVICE_KEY is undefined)
endif
ifndef NETLIFYKEY
	$(error NETLIFYKEY is undefined)
endif
