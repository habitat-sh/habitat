const webpack = require("webpack");

module.exports = {
    devtool: "source-map",
    entry: "./app/boot.ts",
    output: {
        filename: "assets/app.js",
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
                warnings: false,
            },
            mangle: false,
            sourceMap: true,
        }),
    ],
    stats: {
        chunks: false,
    },
    tslint: {
        emitErrors: true,
        failOnHint: true,
    },
}
