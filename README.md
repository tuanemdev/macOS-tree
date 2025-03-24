# macOS-tree

A Rust implementation of the `tree` command for macOS.

Original C codebase: Tree of Unix/Linux

-  [Gitlab](https://gitlab.com/OldManProgrammer/unix-tree)

-  [Github](https://github.com/Old-Man-Programmer/tree)

## Features

-   Displays directory structures in a tree-like format.
-   Customizable output depth.
-   Filters for files and directories.
-   Colorized output.
-   Support for hidden files and directories.

## Installation

1.  Clone the repository:

    ```bash
    git clone https://github.com/tuanemdev/macOS-tree.git
    ```

2.  Navigate to the project directory:

    ```bash
    cd macOS-tree
    ```

3.  Build and install:

    ```bash
    cargo install --path .
    ```

## Usage

```bash
tree [OPTIONS] [DIRECTORY]
```

### Options

-   `-a`, `--all`: List all files, including hidden files and directories.
-   `-d`, `--dirs-only`: List directories only.
-   `-L`, `--level <LEVEL>`: Descend only level directories deep.
-   `-f`, `--full-path`: Print the full path prefix for each file.
-   `-h`, `--help`: Print help information.
-   `-V`, `--version`: Print version information.

### Examples

-   List the current directory:

    ```bash
    tree
    ```

-   List the `Documents` directory:

    ```bash
    tree ~/Documents
    ```

-   List all files and directories in the current directory, including hidden ones:

    ```bash
    tree -a
    ```

-   List only directories in the current directory:

    ```bash
    tree -d
    ```

-   List the directory structure up to 2 levels deep:

    ```bash
    tree -L 2
    ```

- List the full path of each file and directory

    ```bash
    tree -f
    ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the GNU General Public License - see the [LICENSE](LICENSE) file for details.
