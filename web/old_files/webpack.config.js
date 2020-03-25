const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.bundle.js'
  },
  module : {
    rules: [
      { test: /\.js$/, exclude: /node_modules/, use: 'babel-loader' },
      {
         test: /\.(csv)$/,
         use: [
           'csv-loader',
         ],
      }
    ]
  },
  plugins : [
    new HtmlWebpackPlugin({
      template: 'src/index.html'
    })
  ] 
};
