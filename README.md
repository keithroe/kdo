
## kdo 

`kdo` is a simple terminal application for editing and viewing [todo.txt](https://github.com/todotxt/todo.txt "todo.txt format specification") files.  It is designed to complement **todo.txt**'s monolithic list-based format by allowing easy task browsing via filtering of contexts, projects, priorities, and completion status.  Tasks may be added, deleted, marked complete, or edited textually. `kdo` is written in [Rust](https://www.rust-lang.org/).

![kdo_0](https://github.com/keithroe/kdo/assets/775667/5e056b38-f98a-4180-8893-d08fa18bfc09)

## Building
If not already done, [install the Rust toolchain](https://www.rust-lang.org/tools/install).

Compile the executable:
```
cd kdo
cargo build --release
```
And move or link to a location in your path:
```
cp ./target/release/kdo ~/bin
```
## kdo controls
Controls can be shown in-app by pressing `<SHFT>-h`.
```
Key bindings:
Normal mode:
  [s]:       Save task list to todo.txt file
  [S]:       Sort task list
  [q/ESC]:   Quit
  [h/LEFT]:  Move focus one pane to left
  [j/RIGHT]: Move selection up one item in current pane
  [k/UP]:    Move selection down one item in current pane
  [l/DOWN]:  Move focus one pane to right
  [e/ENT]:   Enter edit mode on current task selection
  [x]:       Toggle visibility of all completed tasks
  [X]:       Toggle completion of current task
  [H/SPC]:   Enter help mode display
Edit mode:
  [ESC]:     Exit edit mode without saving any modifications
  [ENT]:     Exit edit mode and save modifications
Help mode:
  [ESC/SPC]: Exit help mode
```

## Component crates
- todo_txt - parsing, writing, and in-memory representation of todo.txt tasks
- ui - used for managing terminal state and user input (`ui::terminal`) and terminal interface (`ui::draw`)
- app - `kdo` application state representation and manipulation

## Customization
#### Colors
`kdo` uses the current shells default colors for the background and for focused foreground text.  The highlight color is ANSI Yellow and the out-of-focus text color is ANSI DarkGray.  These can all be modified by changing the color variables at the top of the file `kdo\crates\ui\src\draw.rs`.

#### File location
By default `kdo` looks for a `todo.txt` file in your current working directory.  A file path can be specified via command line.
```
kdo -f ~/tmp/work_todo.txt
```
A default file location can be specified via the environment variable `KDO_DEFAULT_FILE`.

## Key third party libraries used
- [ratatui](https://docs.rs/ratatui/latest/ratatui/) - for creating terminal-based UIs
- [crossterm](https://docs.rs/crossterm/latest/crossterm/) - lower level terminal manipulation


