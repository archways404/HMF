
# How Much Faster (HMF)

How Much Faster (HMF) is a simple CLI application written in Rust to calculate time differences and percentage differences between two time values. It supports input in seconds (s) and milliseconds (ms), and it is useful for quickly comparing two durations.

## Features

- Calculate the absolute time difference between two values.
- Calculate the percentage difference between two time values.
- Accepts inputs in seconds (e.g., `2s`) and milliseconds (e.g., `300ms`).

## Installation

### Download and install

- You can download and install the program via the [Releases](https://github.com/archways404/HMF/releases/)

### Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Build and Install

You can build and install the HMF CLI by cloning the repository and using `cargo`:

```bash
git clone https://github.com/archways404/hmf.git
cd hmf
cargo build --release
```

This will compile the project and generate the `hmf` executable inside the `target/release/` directory.

Optionally, you can install the binary globally:

```bash
cargo install --path .
```

This will allow you to run `hmf` from anywhere in your terminal.

## Usage

The CLI accepts two time values as input and calculates the difference and percentage difference between them.

### Example

```bash
hmf 888s 2ms
```

**Output:**

```
Difference: 887.998s
v2 (2ms) is 44400000.00% faster than v1 (888s)
```

### Help

To see the available options, you can run:

```bash
hmf --help
```

The output will be:

```
How Much Faster 0.1
Calculate time differences and percentage differences between two time values

USAGE:
    hmf <TIME1> <TIME2>

ARGS:
    <TIME1>    First time value (e.g., "2s", "300ms")
    <TIME2>    Second time value (e.g., "888s", "500ms")

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/archways404/HMF/blob/master/LICENSE) file for details.
