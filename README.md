# THP: Typed Hypertext Processor

Types and a new syntax for PHP, because I'm forced to use it at work.

## Usage

TBD.

Requirements: A *nix system & cargo

```sh
# Clone the repo
git clone https://github.com/Araozu/thp-lang.git

# Generate an executable
cargo build --release

# The executable will be located in ./target/release/thp

# And then run it follow the instructions!
```

```sh
Usage: `thp [command] [options]`

Commands

  c _file_  Compiles _file_ in-place
  f _file_  Formats _file_
  r         Starts the REPL

  init      Initializes a new project in the current directory
  build     Builds the project
  fmt       Formats all files in the project
  watch, w  Starts compilation of the project in watch mode

  help, h   Print this message & exit

General options

  -h, --help    Print command-specific usage
  -v, --version Print version & exit
```

