# Rust QOI Converter
A very basic [QOI format](https://qoiformat.org/) encoder/decoder written in Rust (because if it can be written, it should be written in Rust!)

⚠️ **DISCLAIMER** ⚠️
This is in no way supposed to be a particularly fast, feature-rich, or versatile implementation of this format. This was purely written as a fun side project. If you would like to use better versions of this, go check out [aldanor/qoi-rust](https://github.com/aldanor/qoi-rust), [zakarumych/rapid-qoi](https://github.com/zakarumych/rapid-qoi), or [10maurycy10/libqoi](https://github.com/10maurycy10/libqoi/). If you are looking for something NOT written in Rust (but why would you?), the official README on [phoboslab/qoi](https://github.com/phoboslab/qoi/) has many other implementations of the image format in plenty of other languages and software.

### Usage
If after reading the previous disclaimer, you still would like to use this program, pull the files from the repository and run `cargo run --release <input_file> <output_file>`. This implementation currently only supports converting to and from QOI & PNG.