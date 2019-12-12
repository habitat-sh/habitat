build: clean
    # See https://bundler.io/blog/2019/01/04/an-update-on-the-bundler-2-release.html
	# Due to bug in ruby 2.x, need to install same version of bundler lock file was built with
	gem install bundler --version 2.0.1
	bundle install
	bundle exec middleman build
.PHONY: build

clean:
	rm -rf build

run: build
	BUILDER_WEB_URL="http://localhost:3000" GITHUB_APP_URL="https://github.com/apps/habitat-builder" bundle exec middleman serve

deploy_live: check-env
	cd ..; .expeditor/scripts/website-deploy.sh live
.PHONY: deploy_live

deploy_acceptance: check-env
	cd ..; .expeditor/scripts/website-deploy.sh acceptance
.PHONY: deploy_acceptance

cli_docs:
	hab studio run "hab pkg install core/hab-studio && \
	hab pkg install core/hab-sup && \
	hab pkg install core/hab-launcher && \
	hab pkg install core/node && \
	hab pkg binlink core/node --dest=/bin && \
	/bin/node scripts/generate-cli-docs > source/docs/habitat-cli.html.md"

template_reference:
	mkdir tmp
	cp ../components/sup/doc/* tmp/
	hab studio run "hab pkg install core/node && \
	hab pkg binlink core/node --dest=/bin && \
	npm install json-schema-ref-parser@6.1.0 && \
	/bin/node scripts/generate-template-reference.js tmp/render_context_schema.json > ./source/partials/docs/_reference-template-data.html.md.erb"
	rm -rf tmp

check-env:
ifndef AWS_DEFAULT_REGION
	$(error AWS_DEFAULT_REGION is undefined)
endif
ifndef FASTLY_SERVICE_KEY
	$(error FASTLY_SERVICE_KEY is undefined)
endif
