# cli_assist

A CLI tool for correcting typos in UNIX shell command interfaces.

## Requirements

- git
- gcc
- [Rustup](https://rustup.rs)
- python >= 3.14 (only necessary for the evaluation script)

## Supported Shells

- Bash
- Fish
- Nushell

## Installation

1. Clone git repository

```
git clone https://github.com/schwoens/cli_assist
cd cli_assist

```

2. Build executable

```
cargo build --release
```

### Bash

The shell history has to be updated per command line for the software to work correctly. To get this functionality in bash add

```
export PROMPT_COMMAND="$PROMPT_COMMAND; history -a"

```

to your `.bashrc` file. This appends the last executed command line immediately after running it.


## Usage

1. Typo a command line and try to run it
2. Run the executable: ./target/release/cli_assist

For more usage information run: `cli_assist --help`

## Usage of the evaluation script

```
cd evaluation
python -m venv env
source /env/bin/activate
pip install -r requirements.txt
python evaluation.py
```

