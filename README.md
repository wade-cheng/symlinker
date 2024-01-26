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

Run the `symlinker` command from a directory with a `symlinker_cfg.toml` configuration file. 
As of version `0.1.0`, it must contain:
```
[Config]
symlinker_root_here = true
```