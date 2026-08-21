# Fixes needed for 0-shell

The shell builds, runs, and implements all mandatory commands from scratch (no
external binaries are spawned). The core audit flow works: prompt, `echo`,
`cd`, `pwd`, `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`, `exit`, unknown-command
message, and graceful `Ctrl+D` (EOF).

The following flaws were found while testing against `sh`/bash. They are listed
by severity.

## Bugs (incorrect behavior)

### 1. Empty-string arguments are dropped by the parser

File: `src/helpers/parser.rs`

The parser never pushes an empty `current` token when a quoted empty string is
the whole argument (a closing `"` leaves `current` empty, and the empty token is
only ever pushed via `if !current.is_empty()`). This makes every command lose
`""` arguments.

Reproduction / expected results (compared to `sh`):

| command       | `sh`                      | this shell                      |
| ------------- | ------------------------- | ------------------------------- |
| `cd ""`       | no-op, stays in place     | behaves like bare `cd` -> $HOME |
| `cat ""`      | `cat: '': No such file`   | reads from stdin (hangs)        |
| `ls ""`       | `ls: cannot access ''...` | lists the current directory     |
| `echo a "" b` | `a  b`                    | `a b`                           |

### 2. `rm -r .` / `rm -r ..` delete data before failing

File: `src/commands/rm.rs`

`remove_dir_all()` is called on the `.`/`..` path. On Linux the call fails with
"Invalid argument", but only _after_ the directory contents have been removed.
Reproduced: inside a test dir `rm -r .` deleted `f.txt` and `sub/`, then printed
`rm: cannot remove '.': Invalid argument (os error 22)`.

`sh` refuses outright: `rm: refusing to remove '.' or '..' directory`.

### 3. `mv` does not detect moving a file onto itself

File: `src/commands/mv.rs`

`mv f.txt f.txt` silently succeeds (a no-op rename). GNU prints
`mv: 'f.txt' and 'f.txt' are the same file`. Add the same source/destination
comparison already used by `cp`.

### 4. `#` is treated as a comment start mid-word

File: `src/helpers/parser.rs`

`echo a#b` prints `a`; GNU/bash prints `a#b`. A `#` only starts a comment when
it begins a word (in `Normal` mode outside quotes and preceded by a boundary).

## Deviations from Unix conventions (cosmetic)

### 5. Error messages include Rust's `(os error N)` suffix

All commands print e.g. `cat: nope: No such file or directory (os error 2)`.
Unix tools print `cat: nope: No such file or directory`. The `(os error N)` tail
comes from `std::io::Error`'s `Display`; it should be stripped so output matches
`sh`.

### 6. `ls` sort order differs from GNU `ls`

File: `src/commands/ls.rs` (`ls_cmp`)

- Dotfiles are not sorted first. `ls -a`/`ls -alF` in `sh` print `.hidden`
  before `Bbb`/`a1`; this shell sorts `.hidden` after `dir2`.
- Uppercase/lowercase ordering differs: `sh` prints `Bbb a1 aaa`, this shell
  prints `a1 aaa Bbb` (because everything is lowercased and non-alphanumerics
  are stripped before comparing).

The subject grading compares `ls` output to the real terminal ("output is
similar"), so matching GNU collation ordering is worth fixing.

### 7. `ls -l` date column always shows the time, never the year

File: `src/commands/ls.rs` (`format_mtime`)

For files not modified in the last ~6 months GNU prints the year instead of the
time and left-pads with an extra space (`Mar 15  2020`). This shell always
prints `Mar 15 10:30`.

### 8. `ls -l` does not show device major/minor numbers

File: `src/commands/ls.rs`

For character/block device files GNU prints `major, minor` instead of the size
(e.g. `crw-r--r-- 1 root root 10, 235 Aug 14 13:31 autofs`). This shell prints
the raw `len()` (`0`) in that column. The subject's own example output shows
`10,    58` for device files (`acpi_thermal_rel`), so this is expected
behavior. Use `std::os::unix::fs::MetadataExt::rdev()` to extract major/minor
and format it as `N, N` when the file is a device file.

### 9. `cat -` is not treated as stdin

File: `src/commands/cat.rs`

`sh`'s `cat` treats the argument `-` as stdin. This shell tries to open a file
named `-` and prints `cat: -: No such file or directory (os error 2)`. `cat`
(with any `-` argument) should fall back to reading stdin for that argument.

## Not a flaw (out of scope / verified OK)

- `mkdir -p`, pipes, redirection, globbing and command chaining are correctly
  unsupported per the constraints.
- `Ctrl+C` (SIGINT), auto-completion, history, and prompt-with-cwd are bonus
  features and are intentionally not implemented.
- `ls` per-file (`ls f.txt`, `ls -l f.txt`), `ls` on multiple targets, `-F`
  classification (`/`, `@`, `*`, `|`), `total` block counts, `cd -`/OLDPWD,
  `exit <code>`, and multi-file `cat` all behave correctly.
- Except for the deviations above, `ls -laF /dev` output (permissions, owners,
  symlinks, `@`/`/` classification) matches GNU `ls`.
