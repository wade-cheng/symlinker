# Symlinker

A command line program that saves storage space by symlinking all repeating file names to a cache.

Example resultant folder layout:
```
.
├── .cache/
│   ├── file_one
│   ├── file_two
│   ├── file_three
│   └── file_eight
├── folder01/
│   ├── file_one (symlink)
│   ├── file_two (symlink)
│   └── file_three (symlink)
├── folder02/
│   ├── file_two (symlink)
│   └── file_three (symlink)
├── folder03/
│   ├── file_eight (symlink)
│   ├── file_one (symlink)
│   └── file_two (symlink)
└── symlinker_cfg.toml
```

## Usage

Run `symlinker new --name <NAME>` to create a new symlinker archive.

Run `symlinker run` from a directory with a `symlinker_cfg.toml` configuration file. 
We use this as a "[magic number](https://en.wikipedia.org/wiki/Magic_number_(programming))" identifier 
to prevent accidental file system modifications. It must contain, at minimum:
```
[Config]
symlinker_root_here = true
```

Note: this program probably has a bug where it is not able to process directories past a depth of 1 (ie the `folder01/*` level). It doesn't come up in my personal use case, and I'm busy with other things right now, so it hasn't been fixed. I'll probably come back to bugfix this, though.

## Installation

Symlinks are a Unix-only feature. On a Unix system with Rust [installed](https://doc.rust-lang.org/book/ch01-01-installation.html) (which includes its `cargo` package manager), run 
```cargo install --git https://github.com/wade-cheng/symlinker```
