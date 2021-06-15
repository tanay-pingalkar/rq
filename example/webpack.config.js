const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require("webpack");

module.exports = {
  entry: "./web/index.ts",
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(["./web/index.html", "./web/logo.png"]),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"],
    }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "index.js",
    path: path.resolve(__dirname, "dist"),
  },
};
