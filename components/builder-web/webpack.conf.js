"use strict";

const webpack = require("webpack");
const isProduction = process.env.NODE_ENV == "production";

// Set up compression to only happen if NODE_ENV = production
let loaders = [
    { test: /\.ts$/, loader: "awesome-typescript-loader" },
];
let plugins = [];

if (isProduction) {
    loaders.push({ test: "app.js", loader: "uglify" });
    plugins.push(
        new webpack.optimize.UglifyJsPlugin({
            compress: {
                drop_debugger: false,
                warnings: false
            },
            mangle: false,
            sourceMap: true
        })
    );
}

module.exports = {
    devtool: "source-map",
    entry: "./app/main.ts",
    output: {
        filename: "assets/app.js"
    },
    resolve: {
        extensions: ["", ".webpack.js", ".web.js", ".ts", ".js"]
    },
    module: {
        preLoaders: [
            { test: /\.ts$/, loader: "tslint" },
        ],
        loaders: loaders
    },
    plugins: plugins,
    stats: {
        chunks: false
    },
    tslint: {
        emitErrors: true,
        failOnHint: true
    }
};
