
# 0-shell

A minimalist Unix-like shell written in Rust. Every command is implemented from
scratch on top of Rust's standard library and Unix syscalls — no external
binaries are spawned, and no `sh`/`bash` is used under the hood.

## Build & run

```sh
cargo build --release
cargo run            # or: ./target/release/shell
```

The shell prints a `$ ` prompt, runs one command per line, and returns to the
prompt when the command finishes. `Ctrl+D` (EOF) exits cleanly; so does `exit`.

## Built-in commands

| Command | Supported usage |
| --- | --- |
| `echo` | `echo [args...]` — prints its arguments separated by a space |
| `cd` | `cd [dir]` — no argument goes to `$HOME`; `cd -` returns to the previous directory and prints it; `cd --` and `cd ""` are handled like `sh` |
| `pwd` | `pwd` — prints the current working directory |
| `ls` | `ls [-l] [-a] [-F] [paths...]` — flags may be combined (`-la`), `--` ends flag parsing |
| `cat` | `cat [files...]` — prints file contents to stdout |
| `cp` | `cp <source> <destination>` — files only; directories are skipped with `cp: omitting directory` |
| `mv` | `mv <source...> <destination>` — several sources are allowed when the destination is a directory |
| `rm` | `rm [-r\|-R\|--recursive] <paths...>` — `-r` is required to remove a directory |
| `mkdir` | `mkdir <dirs...>` — creates each directory (parents are not created) |
| `exit` | `exit` — terminates the shell |

Anything else prints `Command '<name>' not found`.

## Parsing

The line parser (`src/helpers/parser.rs`) supports:

- single quotes (`'...'`) — literal, no escapes
- double quotes (`"..."`) — with backslash escapes
- backslash escaping outside quotes
- `#` comments at a word boundary
- multi-line input: an unterminated quote or a trailing `\` re-prompts with
  `quote> `, `dquote> ` or `> ` until the command is complete

Not supported (and out of scope): pipes `|`, redirection `>`, globbing `*`,
and variable expansion beyond `~` / `$HOME` in path arguments.

## Behaviour details

A few deliberate choices that match GNU/BusyBox behaviour:

- **Vanishing cwd.** If the current directory is deleted while the shell is
  running, `sync_cwd` in `src/main.rs` walks up to the nearest surviving
  ancestor instead of leaving the shell in a broken state.
- **Error messages.** The ` (os error N)` suffix that Rust's `io::Error` adds is
  stripped, so messages read like real Unix tools (`commands::err_msg`).
- **Same-file detection.** `cp` and `mv` compare device + inode, so `f.txt`,
  `./f.txt` and a hard link to it are recognised as the same file.
- **Empty operands** are printed as `''` in error messages, like GNU tools.
- `ls -l` resolves user and group names, and `-F` appends the type suffix
  (`/`, `*`, `@`, `|`, `=`) including on symlink targets.

## Layout

```
src/
├── main.rs              # prompt loop, line continuation, command dispatch
├── helpers/
│   ├── mod.rs
│   └── parser.rs        # tokenizer: quotes, escapes, comments, continuation
└── commands/
    ├── mod.rs           # shared helpers: resolve_path, err_msg, is_same_file
    ├── cat.rs   cd.rs   cp.rs    echo.rs  exit.rs
    └── ls.rs    mkdir.rs mv.rs   pwd.rs   rm.rs
```
