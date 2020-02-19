build: clean
    # See https://bundler.io/blog/2019/01/04/an-update-on-the-bundler-2-release.html
	# Due to bug in ruby 2.x, need to install same version of bundler lock file was built with
	gem install bundler --version 2.0.1
	bundle install
	bundle exec middleman build
.PHONY: build

clean: clean_tmp
	rm -rf build

run: build
	BUILDER_WEB_URL="http://localhost:3000" GITHUB_APP_URL="https://github.com/apps/habitat-builder" bundle exec middleman serve

deploy_live: check-env
	cd ..; .expeditor/scripts/website-deploy.sh live
.PHONY: deploy_live

deploy_acceptance: check-env
	cd ..; .expeditor/scripts/website-deploy.sh acceptance
.PHONY: deploy_acceptance

check-env:
ifndef AWS_DEFAULT_REGION
	$(error AWS_DEFAULT_REGION is undefined)
endif
ifndef FASTLY_SERVICE_KEY
	$(error FASTLY_SERVICE_KEY is undefined)
endif

clean_tmp:
	rm -Rf tmp
