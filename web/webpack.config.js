module.exports = {
  devtool: "source-map",
  entry: "./app/boot.ts",
  output: {
    filename: "app.js"
  },
  resolve: {
    extensions: ["", ".webpack.js", ".web.js", ".ts", ".js"]
  },
  module: {
    loaders: [
      { test: /\.ts$/, loader: "ts-loader" }
    ],
    noParse: [ /angular2\/bundles\/.+/ ]
  }
}
