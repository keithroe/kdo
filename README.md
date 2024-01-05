
## vdo 

`vdo` is a simple terminal application for editing and viewing [todo.txt](https://github.com/todotxt/todo.txt "todo.txt format specification") files.  It is designed to complement **todo.txt**'s monolithic list-based format by allowing easy browsing via filtering of contexts, projects, priorities, and task completion.  Tasks may be added, deleted, marked complete, or edited textually. `vdo` is written in [rust](https://www.rust-lang.org/).

![vdo_0](https://github.com/keithroe/kdo/assets/775667/9d6f00ca-6d1b-4c14-879f-70313f1003db)

## Building

## Component crates
- todo_txt - parsing, writing, and in-memory representation of todo.txt tasks
- ui - used for managing terminal state and user input (`ui::terminal`) and terminal interface (`ui::draw`)
- app - `vdo` application state representation and manipulation

## Customization
#### Colors
#### File location

## Key third party libraries used
- [ratatui](https://docs.rs/ratatui/latest/ratatui/) - for creating terminal-based UIs
- [crossterm](https://docs.rs/crossterm/latest/crossterm/) - lower level terminal manipulation


