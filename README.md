# catsay

## Introduction

`catsay` is a fun and versatile Rust program that gives you a kitty that speaks. It utilizes the powerful `clap` crate for command-line argument parsing and the `colorized` crate for vibrant output customization.

## Features

- Customize your cat's message with ease.
- Optionally depict the cat as deceased (for those dramatic moments).
- Import unique cat ASCII art from your own files.

## Installation

1. **Prerequisites:** Ensure you have Rust installed.
2. **Build:** Navigate to your project directory and run `cargo build`.

## Usage

Run `catsay` with the desired message or explore the available options:

```powershell
catsay.exe [OPTIONS] [MESSAGE]
```

**Arguments**

- `[MESSAGE] (default: Meow)`: The text your cat wants to express.

**Options**

- `-d, --dead`: Makes the cat appear deceased (eyes closed, tongue out).
- `-f, --file <CATFILE>`: Imports a custom cat ASCII art file (must be valid ASCII art).
- `-h, --help`: Prints this helpful usage information.

## Examples

1. **Basic Meow:**

```powershell
catsay.exe Hello, world!
```

2. **"Dead" Cat:**

```powershell
catsay.exe --dead I'm not feeling well... (눈‸눈)
```

3. **Custom Cat Art (assuming 'mycat.txt' contains valid ASCII art):**

```powershell
catsay.exe -f mycat.txt This is my custom cat!
```

**Built-in Cat Art**

`catsay` comes with a default cat ASCII art that you can use without specifying a file. 

