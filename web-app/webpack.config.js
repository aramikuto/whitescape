const path = require("path")
const HtmlWebpackPlugin = require("html-webpack-plugin")

module.exports = {
  mode: "production",
  entry: {
    index: "./src/index.tsx",
  },
  output: {
    path: __dirname + "/dist",
    filename: "[name].bundle.js",
  },
  devServer: {
    static: {
      directory: __dirname + "/dist",
    },
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        include: path.resolve(__dirname, "src"),
        use: ["style-loader", "css-loader", "postcss-loader"],
      },
    ],
  },
  plugins: [new HtmlWebpackPlugin({ template: "./src/index.html" })],
  experiments: {
    asyncWebAssembly: true,
  },
}
