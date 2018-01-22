'use strict';

const webpack = require('webpack');
const isProduction = process.env.NODE_ENV === 'production';

let loaders = [
    { test: /\.ts$/, loader: 'awesome-typescript-loader' },
    { test: /\.html$/, loader: 'raw-loader' }
];

let plugins = [];
let devtool = 'source-map';

if (isProduction) {
    devtool = false;

    // Set up compression to only happen if NODE_ENV = production
    loaders.push({ test: 'app.js', loader: 'uglify' });
    plugins.push(
        new webpack.optimize.UglifyJsPlugin({
            compress: {
                drop_debugger: false,
                warnings: false
            },
            mangle: false,
            sourceMap: false
        })
    );
}

module.exports = {
    devtool: devtool,
    entry: './app/main.ts',
    output: {
        filename: 'assets/app.js'
    },
    resolve: {
        extensions: ['', '.webpack.js', '.web.js', '.ts', '.js']
    },
    module: {
        preLoaders: [
            { test: /\.ts$/, loader: 'tslint' }
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
