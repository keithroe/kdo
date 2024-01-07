use clap::Parser;
use std::io::BufRead;

pub static ABOUT_STR: &str = r"
A simple viewer/editor of TODO lists in the todo.txt format
(https://github.com/todotxt/todo.txt).";

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = ABOUT_STR, long_about = ui::terminal::KEYBIND_HELP_STR)]
struct Args {
    /// todo.txt file path. DEFAULT: $KDO_FILE_DEFAULT if set, else ./todo.txt
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let filename = if let Some(file_arg) = args.file {
        file_arg
    } else {
        std::env::var("KDO_FILE_DEFAULT").unwrap_or("./todo.txt".to_string())
    };
    // Create a path to the desired file
    let file = match std::fs::File::open(&filename) {
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err);
            std::process::exit(0);
        }
        Ok(file) => file,
    };

    // Process tasks found in file
    let reader = std::io::BufReader::new(file);
    let tasks = todo_txt::read_tasks(&mut reader.lines());
    let mut app = app::App::new("kdo v0.1", &filename, &tasks);
    let mut ui_state = ui::state::State::new();

    let res = ui::terminal::run(&mut app, &mut ui_state);
    if let Err(err) = res {
        println!("{:?}", err);
    }
}
