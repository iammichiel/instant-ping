const MiniCssExtractPlugin = require("mini-css-extract-plugin")
const path = require('path');

module.exports = {
    entry: './assets/js/main.js',
    output: {
        path: path.resolve(__dirname, 'web'),
        filename: 'bundle.js'
    },
    module: {
        rules:  [{
            test: /\.scss$/,
            use: [
                { loader: MiniCssExtractPlugin.loader },
                { loader: "css-loader" },          
                { loader: "sass-loader" }
            ]
        }],  
    },
    plugins: [
        new MiniCssExtractPlugin({
          filename: "[name].css",
          chunkFilename: "[id].css"
        }),
    ]
};