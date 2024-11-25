# litelens-tui
![GitHub last commit](https://img.shields.io/github/last-commit/jm530ob/litelens-tui)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/jm530ob/litelens-tui)
![Crates.io Total Downloads](https://img.shields.io/crates/d/litelens-tui)
![GitHub Repo stars](https://img.shields.io/github/stars/jm530ob/litelens-tui)

Simple tool to view existing SQLite database files, built with Rust, runs as terminal-based user interface (TUI) .

![image](https://github.com/user-attachments/assets/2cc5110b-909b-41ba-af2b-bffed96990b8)

## Installation

```bash
cargo install litelens-tui
```

## Usage
To get started, simply provide at least one argument specifying the ``.db`` file to open — this can be a relative path

Run in your terminal
```
$ litelens-tui <COMMAND>
```

## Args
| Arg  | Description |
| ------------- | ------------- |
| `-p` | relative / absolute path |

*After being set, the path will be remembered

### Feature Checklist
- [x] Vim like navigation
- [x] TUI Interface
- [x] View data
- [ ] Modify data
- [ ] More optimized

## License
MIT License
