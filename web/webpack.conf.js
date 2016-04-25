const webpack = require("webpack");

module.exports = {
    devtool: "source-map",
    entry: "./app/boot.ts",
    output: {
        filename: "app.js",
    },
    resolve: {
        extensions: ["", ".webpack.js", ".web.js", ".ts", ".js"],
    },
    module: {
        preLoaders: [
            { test: /\.ts$/, loader: "tslint" },
        ],
        loaders: [
            { test: /\.ts$/, loader: "ts-loader" },
            { test: "app.js", loader: "uglify" },
        ],
        noParse: [/angular2\/bundles\/.+/],
    },
    plugins: [
        new webpack.optimize.UglifyJsPlugin({
            compress: {
                drop_debugger: false,
            },
            mangle: false,
            sourceMap: true,
        }),
    ],
    tslint: {
        emitErrors: true,
        failOnHint: true,
    },
}
