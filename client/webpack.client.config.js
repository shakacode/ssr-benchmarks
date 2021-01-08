const webpack = require("webpack");
const path = require("path");
const CssPlugin = require("mini-css-extract-plugin");

module.exports = function(env, argv) {
  return {
    mode: "production",
    context: process.cwd(),
    entry: {
      app:
        (() => {
          switch (env.target) {
            case "rust":
              return "./src/rust-client.js";
            case "node":
              return "./src/node-client.js";
            default:
              throw new Error(`Unexpected target: ${env.target}`);
          }
        })(),
    },
    output: {
      path:
        (() => {
          switch (env.target) {
            case "rust":
              return path.resolve(process.cwd(), "..", "servers", "rust-server", "assets");
            case "node":
              return path.resolve(process.cwd(), "..", "servers", "node-server", "assets");
            default:
              throw new Error(`Unexpected target: ${env.target}`);
          }
        })(),
      publicPath: "/",
      filename: "[name].js",
      chunkFilename: "[id].js",
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
}
