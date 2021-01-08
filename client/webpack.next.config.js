const webpack = require("webpack");
const path = require("path");
const CssPlugin = require("mini-css-extract-plugin");

module.exports = {
  mode: "production",
  context: process.cwd(),
  entry: {
    app: "./src/app.js",
  },
  output: {
    path: path.resolve(process.cwd(), "..", "servers", "next-server", "assets"),
    publicPath: "/",
    filename: "[name].js",
    libraryTarget: "commonjs2",
  },
  devtool: false,
  plugins: [
    new CssPlugin({
      filename: "[name].css",
      chunkFilename: "[id].css",
    }),
  ],
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          {
            loader: "babel-loader",
            options: { compact: true },
          },
        ],
      },
      {
        test: /\.css$/,
        use: [
          CssPlugin.loader,
          "css-loader",
        ],
      },
    ],
  },
};
