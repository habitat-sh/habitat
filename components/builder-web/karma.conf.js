// Karma configuration
// Generated on Fri Jan 29 2016 15:43:21 GMT-0600 (CST)
module.exports = function (config) {
    config.set({
        // frameworks to use
        // available frameworks: https://npmjs.org/browse/keyword/karma-adapter
        frameworks: ["jasmine"],

        // list of files / patterns to load in the browser
        files: [
          "node_modules/core-js/client/core.js",
          "node_modules/zone.js/dist/zone.js",
          "node_modules/zone.js/dist/async-test.js",
          "node_modules/zone.js/dist/fake-async-test.js",
          "node_modules/zone.js/dist/long-stack-trace-zone.js",
          "node_modules/zone.js/dist/proxy.js",
          "node_modules/zone.js/dist/sync-test.js",
          "node_modules/zone.js/dist/jasmine-patch.js",
          "app/tests-entry.ts",

          // handle asset requests
          { pattern: 'assets/**/*', watched: false, included: false, served: true },
        ],

        proxies: {
            "/assets": "/base/assets"
        },

        plugins: [
            require("karma-jasmine"),
            require("karma-phantomjs-launcher"),
            require("karma-webpack"),
            require("karma-sourcemap-loader"),
            require("karma-spec-reporter"),
            require("karma-coverage"),
        ],

        // preprocess matching files before serving them to the browser
        // available preprocessors: https://npmjs.org/browse/keyword/karma-preprocessor
        preprocessors: {
            "app/tests-entry.ts": ["webpack", "sourcemap", "coverage"]
        },

        // test results reporter to use
        // possible values: "dots", "progress"
        // available reporters: https://npmjs.org/browse/keyword/karma-reporter
        reporters: ["spec"],

        webpack: {
            devtool: "inline-source-map",
            resolve: {
                extensions: ["", ".webpack.js", ".web.js", ".ts", ".js"]
            },
            module: {
                loaders: [
                    { test: /\.ts$/, loader: "awesome-typescript-loader", exclude: /node_modules/ },
                    { test: /\.html$/, loader: "raw-loader" },
                ]
            },
            debug: false
        },

        webpackMiddleware: {
            noInfo: true
        },

        // web server port
        port: 9876,

        // enable / disable colors in the output (reporters and logs)
        colors: true,

        // level of logging
        // possible values: config.LOG_DISABLE || config.LOG_ERROR || config.LOG_WARN || config.LOG_INFO || config.LOG_DEBUG
        logLevel: config.LOG_INFO,

        // enable / disable watching file and executing tests whenever any file changes
        autoWatch: true,

        // start these browsers
        // available browser launchers: https://npmjs.org/browse/keyword/karma-launcher
        browsers: ["PhantomJS"],

        // Continuous Integration mode
        // if true, Karma captures browsers, runs the tests and exits
        singleRun: true,

        // Concurrency level
        // how many browser should be started simultaneous
        concurrency: Infinity
    });
};
