# macOS-tree

A Rust implementation of the `tree` command for macOS.

Original C codebase: Tree of Unix/Linux

-  [Gitlab](https://gitlab.com/OldManProgrammer/unix-tree)

-  [Github](https://github.com/Old-Man-Programmer/tree)

## Installation

0.  Install Rust
    ```zsh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1.  Clone the repository:

    ```zsh
    git clone https://github.com/tuanemdev/macOS-tree.git
    ```

2.  Navigate to the project directory:

    ```zsh
    cd macOS-tree
    ```

3.  Build and install:

    ```zsh
    cargo build --release
    cp target/release/tree /usr/local/bin/
    ```

## Usage

```zsh
tree [OPTIONS] [DIRECTORY...]
```

### Options

-   `-a`, `--all`: List all files, including hidden files and directories.
-   `-d`, `--dirs-only`: List directories only.
-   `-i`, `--no-indent`: Don't print indentation lines".
-   `-f`, `--full-path`: Print the full path prefix for each file.
-   `-g`, `--gitignore`: Ignore files specified in .gitignore and .git folder (root dir only).
-   `-l`, `--max-depth <LEVEL>`: Descend only level directories deep.
-   `-o`, `--output <FILE_PATH>`: Output tree to a file.
-   `-V`, `--version`: Print version information.
-   `-h`, `--help`: Print help information.

### Examples

-   List the current directory:

    ```zsh
    tree
    ```

-   List the `Documents` directory:

    ```zsh
    tree ~/Documents
    ```

-   List all files and directories in the current directory, including hidden ones:

    ```zsh
    tree -a
    ```

-   List only directories in the current directory:

    ```zsh
    tree -d
    ```

-   List the directory structure up to 2 levels deep:

    ```zsh
    tree -l 2
    ```

- List the full path of each file and directory

    ```zsh
    tree -f
    ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the GNU General Public License - see the [LICENSE](LICENSE) file for details.
