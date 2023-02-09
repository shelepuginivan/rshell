# rshell

UNIX shell in Rust

It is an experimental shell developed as a pet-project to improve my understanding of the operating system, processes, and the shell itself. I do not recommend to use `rshell` as main shell since there are possibly bugs or unexpected behaviour.

## Features

1. Built-in commands such as `cd`, `exit` etc.
2. Base system commands such as `ls`, `mkdir` etc.
3. Logical AND (`&&`) implementation
4. Sequential execution (`;`) implementation
5. Pipes
6. I/O redirection (both `>` for write and `>>` for append)
7. Script execution (`rshell` can run files as interpretor)
8. Profile - file that executed as the shell process started
9. Variables
10. Aliases
11. String literals
12. Functions
13. Instants (instant commands)
14. Command history
15. Comments

See the [documentation](./docs.md)

## Usage

Clone this repo:

```shell
git clone https://github.com/shelepuginivan/rshell.git
cd rshell
```

You can run it in development mode with

```shell
cargo run
```

or build with

```shell
cargo build --release
```

After building, you can find binary `rsh` file in `./target/release` directory. You can move it to directory that is accessable in global scope or export path to it.