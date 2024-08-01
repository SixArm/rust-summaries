# BurntSushi programs

<https://github.com/BurntSushi>

**ripgrep** is a line-oriented search tool that recursively searches the current directory for a regex pattern. By default, ripgrep can respect gitignore rules and automatically skip hidden files/directories and binary files. ripgrep has first-class support on Windows, macOS and Linux, with binary downloads available for every release. ripgrep is similar to other popular search tools like The Silver Searcher, ack and grep.

**regex** is a Rust library for parsing, compiling, and executing regular expressions. Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and backreferences. In exchange, all searches execute in linear time with respect to the size of the regular expression and search text. Much of the syntax and implementation is inspired by RE2.

**walkdir** is cross platform Rust library for efficiently walking a directory recursively. Comes with support for following symbolic links, controlling the number of open file descriptors and efficient mechanisms for pruning the entries in the directory tree.

**xsv** is a command line program for indexing, slicing, analyzing, splitting, and joining CSV files. Commands should be simple, fast and composable. The xsv command can also concatenate, flatten, formation, partition, sample, search, slice, sort, and more.
