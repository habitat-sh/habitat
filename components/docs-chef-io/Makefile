# we use pushd/popd here, and /bin/sh of our chefes/buildkite image is not bash
# so we have to override the default shell here
SHELL=bash

preview_netlify: chef_web_docs
	cp -R content/* chef-web-docs/_vendor/github.com/habitat-sh/habitat/components/docs-chef-io/content
	cp -R static/* chef-web-docs/_vendor/github.com/habitat-sh/habitat/components/docs-chef-io/static
	cp -R config.toml chef-web-docs/_vendor/github.com/habitat-sh/habitat/components/docs-chef-io/
	pushd chef-web-docs && make assets; hugo --gc --minify --buildFuture && popd

serve: chef_web_docs
	echo "replace github.com/habitat-sh/habitat/components/docs-chef-io => ../" >> chef-web-docs/go.mod
	pushd chef-web-docs && make assets; hugo server --buildDrafts --buildFuture --noHTTPCache --ignoreVendorPaths "github.com/habitat-sh/habitat/**" && popd

chef_web_docs:
	if [ -d "chef-web-docs/" ]; then \
		pushd chef-web-docs && git reset HEAD --hard; git clean -fd; git pull --ff-only origin master; rm -rf public && popd;  \
	else \
		git clone https://github.com/chef/chef-web-docs.git; \
	fi

clean_all:
	pushd chef-web-docs && make clean_all && popd

lint: assets
	hugo -D
