use std::borrow::Borrow;
use std::str::FromStr;

use crate::task::Task;

//------------------------------------------------------------------------------
//
// operatons on collections of tasks
//
//------------------------------------------------------------------------------

pub fn read<B: std::io::BufRead>(lines: &mut std::io::Lines<B>) -> Vec<Task> {
    let mut tasks = Vec::new();
    for line in lines.map(|line| line.unwrap()) {
        if !line.trim().is_empty() {
            if let Ok(task) = Task::from_str(&line) {
                tasks.push(task);
            } else {
                println!("Failed to parse Task from '{}'", line);
            }
        }
    }
    tasks
}

pub struct TasksFilter<'a> {
    pub tasks: Vec<&'a Task>,
    pub task_indices: Vec<usize>,
}

impl<'a> TasksFilter<'a> {
    pub fn new(tasks: &'a [Task]) -> TasksFilter {
        TasksFilter {
            tasks: tasks.iter().collect(),
            task_indices: (0..tasks.len()).collect(),
        }
    }
    pub fn num_contexts(&self, include_completed: bool) -> usize {
        num_contexts(&self.tasks, include_completed)
    }

    pub fn num_projects(&self, include_completed: bool) -> usize {
        num_projects(&self.tasks, include_completed)
    }

    pub fn num_priorities(&self, include_completed: bool) -> usize {
        num_priorities(&self.tasks, include_completed)
    }

    pub fn collect_contexts(&self, include_completed: bool) -> Vec<String> {
        collect_contexts(&self.tasks, include_completed)
    }

    pub fn collect_projects(&self, include_completed: bool) -> Vec<String> {
        collect_projects(&self.tasks, include_completed)
    }

    pub fn collect_priorities(&self, include_completed: bool) -> Vec<char> {
        collect_priorities(&self.tasks, include_completed)
    }

    // TODO: avoid the zip/unzip in the with_* functions
    pub fn with_context(mut self, context_opt: Option<&str>) -> TasksFilter<'a> {
        if let Some(context) = context_opt {
            let tasks_with_indices: Vec<(&Task, usize)> = self
                .tasks
                .into_iter()
                .zip(self.task_indices)
                .filter(|(task, _index)| {
                    for c in task.contexts() {
                        if c == context {
                            return true;
                        }
                    }
                    false
                })
                .collect();

            (self.tasks, self.task_indices) = tasks_with_indices.into_iter().unzip();
        }
        self
    }

    pub fn with_project(mut self, project_opt: Option<&str>) -> TasksFilter<'a> {
        if let Some(project) = project_opt {
            let tasks_with_indices: Vec<(&Task, usize)> = self
                .tasks
                .into_iter()
                .zip(self.task_indices)
                .filter(|(task, _index)| {
                    for p in task.projects() {
                        if p == project {
                            return true;
                        }
                    }
                    false
                })
                .collect();

            (self.tasks, self.task_indices) = tasks_with_indices.into_iter().unzip();
        }
        self
    }

    pub fn with_priority(mut self, priority_opt: Option<char>) -> TasksFilter<'a> {
        if priority_opt.is_some() {
            let tasks_with_indices: Vec<(&Task, usize)> = self
                .tasks
                .into_iter()
                .zip(self.task_indices)
                .filter(|(task, _index)| priority_opt == task.priority)
                .collect();

            (self.tasks, self.task_indices) = tasks_with_indices.into_iter().unzip();
        }
        self
    }

    pub fn without_completed(mut self, omit_completed: bool) -> TasksFilter<'a> {
        if omit_completed {
            let tasks_with_indices: Vec<(&Task, usize)> = self
                .tasks
                .into_iter()
                .zip(self.task_indices)
                .filter(|(task, _index)| !task.completed)
                .collect();

            (self.tasks, self.task_indices) = tasks_with_indices.into_iter().unzip();
        }
        self
    }
}

/*
impl Tasks for Vec<Task> {
}
*/

/*
impl Tasks<'a, 'a> for Vec<&'a Task> {
}

impl<'a, 'b> Tasks<'a, 'b> for &'a [&'b Task] {
}
*/

pub fn num_contexts<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> usize {
    let mut contexts = std::collections::HashSet::new();
    for task in tasks {
        if include_completed || !task.borrow().completed {
            for context in task.borrow().contexts() {
                contexts.insert(context.as_str());
            }
        }
    }
    contexts.len()
}

pub fn num_projects<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> usize {
    let mut projects = std::collections::HashSet::new();
    for task in tasks {
        if include_completed || !task.borrow().completed {
            for context in task.borrow().projects() {
                projects.insert(context.as_str());
            }
        }
    }
    projects.len()
}

pub fn num_priorities<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> usize {
    let mut priorities = std::collections::HashSet::new();
    for task in tasks {
        if let Some(priority) = task.borrow().priority {
            if include_completed || !task.borrow().completed {
                priorities.insert(priority);
            }
        }
    }
    priorities.len()
}

pub fn collect_contexts<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> Vec<String> {
    let mut contexts: std::collections::HashSet<&String> = std::collections::HashSet::new();
    for task in tasks {
        if include_completed || !task.borrow().completed {
            for context in task.borrow().contexts() {
                contexts.insert(context);
            }
        }
    }

    let mut contexts: Vec<String> = contexts.iter().map(|p| p.to_string()).collect();
    contexts.sort_unstable();
    contexts
}

pub fn collect_projects<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> Vec<String> {
    let mut projects: std::collections::HashSet<&String> = std::collections::HashSet::new();
    for task in tasks {
        if include_completed || !task.borrow().completed {
            for project in task.borrow().projects() {
                projects.insert(project);
            }
        }
    }

    let mut projects: Vec<String> = projects.iter().map(|p| p.to_string()).collect();
    projects.sort_unstable();
    projects
}

pub fn collect_priorities<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> Vec<char> {
    let mut priorities = std::collections::HashSet::new();
    for task in tasks {
        if include_completed || !task.borrow().completed {
            if let Some(priority) = task.borrow().priority {
                priorities.insert(priority);
            }
        }
    }
    let mut priorities: Vec<char> = priorities.into_iter().collect();
    priorities.sort_unstable();
    priorities
}

pub fn include_completed<T: Borrow<Task>>(tasks: &[T], include_completed: bool) -> Vec<Task> {
    tasks
        .iter()
        .map(|borrow| borrow.borrow())
        .filter(|task| include_completed || !task.completed)
        .cloned()
        .collect()
}

pub fn with_context<T: Borrow<Task>>(tasks: &[T], context: &str) -> Vec<Task> {
    tasks
        .iter()
        .map(|borrow| borrow.borrow())
        .filter(|task| {
            for p in task.contexts() {
                if p == context {
                    return true;
                }
            }
            false
        })
        .cloned()
        .collect()
}

pub fn with_project<T: Borrow<Task>>(tasks: &[T], project: &str) -> Vec<Task> {
    tasks
        .iter()
        .map(|borrow| borrow.borrow())
        .filter(|task| {
            for p in task.projects() {
                if p == project {
                    return true;
                }
            }
            false
        })
        .cloned()
        .collect()
}

pub fn with_priority<T: Borrow<Task>>(tasks: &[T], priority: char) -> Vec<Task> {
    tasks
        .iter()
        .map(|borrow| borrow.borrow())
        .filter(|task| Some(priority) == task.priority)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
