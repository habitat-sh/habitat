
assets: assets/js static/fonts assets/sass
clean: clean_assets
clean_all: clean_node_modules clean_assets

clean_assets:
	rm -rf assets/js/vendor
	rm -rf assets/sass/vendor
	rm -rf static/fonts/fontawesome

clean_node_modules:
	rm -rf node_modules

#
# Assets
#

node_modules:
	npm install

assets/js: assets/js/vendor

assets/sass: assets/sass/vendor

static/fonts: static/fonts/fontawesome

assets/js/vendor: node_modules
	mkdir -p assets/js/vendor
	cp node_modules/jquery/dist/jquery.min.js assets/js/vendor/jquery.min.js
	cp node_modules/showdown/dist/showdown.min.js assets/js/vendor/showdown.min.js
	cp node_modules/ua-parser-js/dist/ua-parser.min.js assets/js/vendor/ua-parser.min.js
	cp node_modules/what-input/dist/what-input.min.js assets/js/vendor/what-input.min.js
	cp node_modules/@fortawesome/fontawesome-free/js/all.min.js assets/js/vendor/fontawesome-all.min.js
	cp node_modules/foundation-sites/dist/js/foundation.min.js assets/js/vendor/foundation.min.js
	cp node_modules/redoc/bundles/redoc.standalone.js assets/js/vendor/redoc.standalone.js

assets/sass/vendor: assets/sass/vendor/fontawesome

assets/sass/vendor/fontawesome: node_modules
	mkdir -p assets/sass/vendor
	cp -R node_modules/@fortawesome/fontawesome-free/scss/ assets/sass/vendor/fontawesome

static/fonts/fontawesome: node_modules
	cp -R node_modules/@fortawesome/fontawesome-free/webfonts/ static/fonts/fontawesome



