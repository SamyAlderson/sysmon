# sysmon
System monitor CLI tool that keeps you informed

System monitor CLI tool that provides real-time system metrics and alerts.

## What it does
sysmon is a simple CLI tool that collects and displays system metrics, such as CPU usage and memory consumption. It also includes an alerting system that notifies you when certain thresholds are exceeded. This project exists because I needed a lightweight system monitor for my personal projects.

## Installation
To install sysmon, run `cargo install sysmon` in your terminal.

## Usage
To run sysmon, simply type `sysmon` in your terminal. You can also specify a configuration file using the `-c` flag, like this: `sysmon -c /path/to/config.toml`.

## Building from source
To build sysmon from source, run `cargo build` in the project directory. You can also run `cargo build --release` to build a release version.

## Running tests
To run the tests, run `cargo test` in the project directory.

## Project structure
* `src/main.rs`: The main entry point of the program.
* `src/monitor.rs`: The module responsible for collecting system metrics.
* `src/alert.rs`: The module responsible for sending alerts.
* `src/config.rs`: The module responsible for parsing configuration files.
* `tests`: The directory containing the test suite.

## License
Copyright (c) 2026 SamyAlderson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.