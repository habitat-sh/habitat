# we use pushd/popd here, and /bin/sh of our chefes/buildkite image is not bash
# so we have to override the default shell here
SHELL=bash

assets:
	pushd themes/docs-new && make assets && popd

clean:
	pushd themes/docs-new && make clean && popd
	rm -rf resources/

clean_all:
	pushd themes/docs-new && make clean_all && popd
	rm -rf resources/
	rm -rf results/

serve: assets
	hugo server --buildDrafts --noHTTPCache

lint: assets
	hugo -D
