# Calen'sCLI

Calen'sCLI is a basic CLI utility tool that provides similar functionality to commond shell commands
like echo, cat, ls, find, and grep.
---

## Features

- **`echo`**: Print a string to the terminal.
- **`cat`**: Display the contents of a file.
- **`ls`**: List files and directories in a specified directory.
- **`find`**: Search for files by name in a directory (recursively).
- **`grep`**: Search for patterns in a file and print matching lines.

---

## Installation

To install `Calen'sCLI` from [crates.io](https://crates.io):

1. Ensure you have Rust and Cargo installed. If not, install them via [rustup](https://rustup.rs/):
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	
2. Install Calen'sCLI using Cargo:
	cargo install Calen'sCLI
3. Verify installation:
	Calen'sCLI --help

## Usage

echo:
CLI echo "Hello, Rust!"

cat:
CLI cat <file_path>

ls:
CLI ls <directory_path>

find:
CLI find <directory_path> <filename>

mini-grep:
CLI mini-grep <pattern> <file_path>

## Contributing 

Pull requests are welcome!

## License

[MIT OR Apache-2.0"]