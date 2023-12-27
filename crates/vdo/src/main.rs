use clap::Parser;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// todo.txt filename and location
    #[arg(short, long, default_value = "./todo.txt")]
    file: String,
}

fn main() {
    let args = Args::parse();

    // Create a path to the desired file
    let file = match std::fs::File::open(&args.file) {
        Err(err) => {
            println!("Failed to open file '{}': {}", args.file, err);
            std::process::exit(0);
        }
        Ok(file) => file,
    };

    // Process tasks found in file
    let reader = std::io::BufReader::new(file);
    let tasks = todo_txt::read_tasks(&mut reader.lines());
    let mut app = app::App::new("vdo v0.1", &args.file, &tasks);
    let mut ui_state = ui::state::State::new();

    let res = ui::terminal::run(&mut app, &mut ui_state);
    if let Err(err) = res {
        println!("{:?}", err);
    }
}
