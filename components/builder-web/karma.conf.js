// Karma configuration
// Generated on Fri Jan 29 2016 15:43:21 GMT-0600 (CST)
module.exports = function (config) {
    config.set({
        // frameworks to use
        // available frameworks: https://npmjs.org/browse/keyword/karma-adapter
        frameworks: ["chai", "mocha", "sinon"],


        // list of files / patterns to load in the browser
        files: [
            "node_modules/es6-shim/es6-shim.js",
            "app/tests-entry.ts",
        ],

        plugins: [
            require("karma-mocha"),
            require("karma-chai"),
            require("karma-sinon"),
            require("karma-phantomjs-launcher"),
            require("karma-webpack"),
            require("karma-sourcemap-loader"),
            require("karma-spec-reporter"),
            require("karma-coverage"),
        ],

        // preprocess matching files before serving them to the browser
        // available preprocessors: https://npmjs.org/browse/keyword/karma-preprocessor
        preprocessors: {
            "app/tests-entry.ts": ["webpack", "sourcemap", "coverage"],
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
                ],
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
        concurrency: Infinity,
    })
}
