pub mod parse;
pub mod task;
pub mod tasks;

use std::str::FromStr;

/// Read tasks from a line buffer.  Can be used to read from file as such:
/// ```
/// let reader = std::io::BufReader::new(file);
/// let tasks = todo_txt::read_tasks(&mut reader.lines());
/// ```
pub fn read_tasks<B: std::io::BufRead>(lines: &mut std::io::Lines<B>) -> Vec<task::Task> {
    let mut tasks = Vec::new();
    for line in lines.map(|line| line.unwrap()) {
        if !line.trim().is_empty() {
            if let Ok(task) = task::Task::from_str(&line) {
                tasks.push(task);
            } else {
                println!("Failed to parse Task from '{}'", line);
            }
        }
    }
    tasks
}
