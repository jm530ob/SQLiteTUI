# litelens-tui
![GitHub last commit](https://img.shields.io/github/last-commit/jm530ob/litelens-tui)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/jm530ob/litelens-tui)
![Crates.io Total Downloads](https://img.shields.io/crates/d/litelens-tui)
![GitHub Repo stars](https://img.shields.io/github/stars/jm530ob/litelens-tui)

Simple terminal-based user interface `TUI` app to view existing SQLite database files, built with pure Rust

![image](https://github.com/user-attachments/assets/2cc5110b-909b-41ba-af2b-bffed96990b8)

## Installation

```bash
cargo install litelens-tui
```

## Usage
To get started, simply provide at least one argument specifying the ``.db`` file to open — this can be a relative path

Run in your terminal
```
$ litelens-tui <ARGS>
```

## Args
| Arg  | Description |
| ------------- | ------------- |
| `-p` short for `-path` | relative / absolute path |

~~After being set, the path will be remembered~~

## Navigation
| Key  | Movement | Area
| --- | -- | -------- |
| `k` | Up | TreeNode |
| `j` | Down | TreeNode |
| `ArrowUp` | Up | TableView |
| `ArrowDown` | Down | TableView |

### Features
- [x] Vim like navigation
- [x] TUI Interface
- [x] View data
- [ ] Modify data
- [ ] Better optimized
- [ ] Error handler and display
- [ ] dump the contents of a database into stdout

## License

<a href="https://github.com/jm530ob/litelens-tui/blob/main/LICENSE">MIT</a>
