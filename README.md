# cli_assist

A CLI tool for correcting typos in UNIX shell command interfaces.

## Requirements

- git
- gcc
- [Rustup](https://rustup.rs)
- python >= 3.14 (only necessary for the evaluation script)

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

