use crate::selection_list::SelectionList;
use std::io::Write;
use std::str::FromStr;

//------------------------------------------------------------------------------
//
// App
//
//------------------------------------------------------------------------------

/// Used to specify the App's current interaction mode
#[derive(PartialEq)]
pub enum Mode {
    Edit,
    Normal,
    Help,
    Confirm(ConfirmedAction),
}

/// Used to specify which list is currently under focus
#[derive(PartialEq)]
pub enum Focus {
    Tasks,
    Projects,
    Contexts,
    Priorities,
}

/// Actions requiring user confirmation
#[derive(PartialEq)]
pub enum ConfirmedAction {
    Save,
    Sort,
}

pub struct App<'a> {
    pub title: &'a str,
    pub filepath: &'a str,
    pub should_quit: bool,

    pub tasks: Vec<todo_txt::task::Task>,

    pub task_list: SelectionList<usize>,

    pub context_list: SelectionList<String>,
    pub project_list: SelectionList<String>,
    pub priority_list: SelectionList<String>,

    pub mode: Mode,
    pub focus: Focus,

    pub omit_completed: bool,

    pub error_msg: String,

    pub frame_time: f64,
}

pub static ALL_TOKEN: &str = "[all]";
pub static NEW_TOKEN: &str = "[new]";

impl<'a> App<'a> {
    pub fn new(title: &'a str, filepath: &'a str, tasks: &[todo_txt::task::Task]) -> App<'a> {
        let tasks = [
            vec![todo_txt::task::Task::from_str(NEW_TOKEN).unwrap()],
            tasks.to_vec(),
        ]
        .concat();

        App {
            title,
            filepath,
            should_quit: false,

            task_list: SelectionList::with_items(App::get_task_items(&tasks)),
            context_list: SelectionList::with_items(App::get_context_items(&tasks)),
            project_list: SelectionList::with_items(App::get_project_items(&tasks)),
            priority_list: SelectionList::with_items(App::get_priority_items(&tasks)),

            mode: Mode::Normal,
            focus: Focus::Tasks,

            omit_completed: true,

            //input: tui_input::Input::new("".to_string()),
            error_msg: "".to_string(),

            frame_time: 0f64,

            tasks, // NB: at end since it consumes local task object
        }
    }

    /// Create list of items for display, including ALL_TOKEN or NEW_TOKEN header
    pub fn get_task_items(tasks: &[todo_txt::task::Task]) -> Vec<usize> {
        // tasks already contains NEW_TOKEN task
        Vec::from_iter(0..tasks.len())
    }

    /// See [get_task_items]
    pub fn get_context_items(tasks: &[todo_txt::task::Task]) -> Vec<String> {
        [
            vec![ALL_TOKEN.to_string()],
            todo_txt::tasks::collect_contexts(tasks, true),
        ]
        .concat()
    }

    /// See [get_task_items]
    pub fn get_project_items(tasks: &[todo_txt::task::Task]) -> Vec<String> {
        [
            vec![ALL_TOKEN.to_string()],
            todo_txt::tasks::collect_projects(tasks, true),
        ]
        .concat()
    }

    /// See [get_task_items]
    pub fn get_priority_items(tasks: &[todo_txt::task::Task]) -> Vec<String> {
        [
            vec![ALL_TOKEN.to_string()],
            todo_txt::tasks::collect_priorities(tasks, true)
                .iter()
                .map(|p| p.to_string())
                .collect(),
        ]
        .concat()
    }

    fn get_selected_item<T: Clone>(list: &SelectionList<T>) -> Option<T> {
        match list.selection() {
            Some(i) if i > 0 => list.items().get(i).cloned(),
            _ => None,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut file = match std::fs::File::create(self.filepath) {
            Err(err) => {
                println!("Failed to open file '{}': {}", &self.filepath, err);
                std::process::exit(0);
            }
            Ok(file) => file,
        };
        for task in &self.tasks[1..] {
            writeln!(&mut file, "{}", task)?;
        }
        Ok(())
    }

    pub fn start_frame(&mut self) {}

    pub fn end_frame(&mut self, frame_time: f64) {
        self.frame_time = frame_time;
    }

    pub fn sort_tasks(&mut self) {
        self.tasks.sort();
    }

    pub fn toggle_completed(&mut self) {
        self.omit_completed = !self.omit_completed;
        self.filter_tasks();
    }

    pub fn mark_selected_task_complete(&mut self) {
        if let Some(task) = self.get_selected_task_mut() {
            if task.date_created.is_some() {
                task.date_completed = Some(chrono::Local::now().date_naive());
            }
            task.completed = true;
        }
    }

    pub fn get_selected_task_list_idx(&self) -> Option<usize> {
        match self.task_list.selection() {
            Some(i) if i > 0 => Some(i),
            _ => None,
        }
    }

    pub fn get_selected_task_idx(&self) -> Option<usize> {
        match self.task_list.selection() {
            Some(i) if i > 0 => self.task_list.items().get(i).cloned(),
            _ => None,
        }
    }

    pub fn get_selected_task(&self) -> Option<&todo_txt::task::Task> {
        match self.get_selected_task_idx() {
            Some(idx) => self.tasks.get(idx),
            _ => None,
        }
    }

    pub fn get_selected_task_mut(&mut self) -> Option<&mut todo_txt::task::Task> {
        match self.get_selected_task_idx() {
            Some(idx) => self.tasks.get_mut(idx),
            _ => None,
        }
    }

    pub fn get_selected_context(&self) -> Option<String> {
        App::get_selected_item(&self.context_list)
    }

    pub fn get_selected_project(&self) -> Option<String> {
        App::get_selected_item(&self.project_list)
    }

    pub fn get_selected_priority(&self) -> Option<char> {
        match App::get_selected_item(&self.priority_list) {
            Some(priority_string) => priority_string.chars().next(),
            None => None,
        }
    }

    pub fn filter_tasks(&mut self) {
        let tasks_filter = todo_txt::tasks::TasksFilter::new(&self.tasks[1..])
            .without_completed(self.omit_completed)
            .with_project(self.get_selected_project().as_deref())
            .with_context(self.get_selected_context().as_deref())
            .with_priority(self.get_selected_priority());

        let task_items = [
            vec![0usize],
            tasks_filter.task_indices.iter().map(|i| i + 1).collect(),
        ]
        .concat();

        self.task_list = SelectionList::with_items(task_items);
    }

    pub fn update_state_after_edit(&mut self) {
        // a) Cache selected context from previous frame
        // b) regenerate list of contexts after task list edit
        // c) if selected context is still present, reselect it
        let selected_context = self.get_selected_context();
        self.context_list = SelectionList::with_items(App::get_context_items(&self.tasks));
        if let Some(context) = selected_context {
            self.context_list
                .select(self.context_list.items().iter().position(|x| x == &context));
        }

        // repeat for projects
        let selected_project = self.get_selected_project();
        self.project_list = SelectionList::with_items(App::get_project_items(&self.tasks));
        if let Some(project) = selected_project {
            self.project_list
                .select(self.project_list.items().iter().position(|x| x == &project));
        }

        // repeat for priority
        let selected_priority = self.get_selected_priority();
        self.priority_list = SelectionList::with_items(App::get_priority_items(&self.tasks));
        if let Some(priority) = selected_priority {
            let priority = priority.to_string();
            self.priority_list.select(
                self.priority_list
                    .items()
                    .iter()
                    .position(|x| x == &priority),
            );
        }

        // regenerate task list and reselect current task
        let selected_task = self.get_selected_task().cloned();
        let selected_task_list_idx = self.task_list.selection();

        self.filter_tasks();
        self.task_list.select(selected_task_list_idx);

        // If current task is now filtered out, unselect
        if let Some(task) = selected_task {
            if task.priority != self.get_selected_priority() {
                self.task_list.select(Some(0));
            }
            if let Some(selected_context) = self.get_selected_context() {
                if !task.contexts().contains(&selected_context) {
                    self.task_list.select(Some(0));
                }
            }
            if let Some(selected_project) = self.get_selected_project() {
                if !task.projects().contains(&selected_project) {
                    self.task_list.select(Some(0));
                }
            }
        }
    }

    pub fn navigate_up(&mut self) {
        match self.mode {
            Mode::Normal => match self.focus {
                Focus::Tasks => {
                    self.task_list.previous();
                }
                Focus::Contexts => {
                    self.context_list.previous();
                    self.filter_tasks();
                }
                Focus::Projects => {
                    self.project_list.previous();
                    self.filter_tasks();
                }
                Focus::Priorities => {
                    self.priority_list.previous();
                    self.filter_tasks();
                }
            },
            Mode::Edit => {}
            Mode::Help => {}
            Mode::Confirm(_) => {}
        }
    }

    pub fn navigate_down(&mut self) {
        match self.mode {
            Mode::Normal => match self.focus {
                Focus::Tasks => {
                    self.task_list.next();
                }
                Focus::Contexts => {
                    self.context_list.next();
                    self.filter_tasks();
                }
                Focus::Projects => {
                    self.project_list.next();
                    self.filter_tasks();
                }
                Focus::Priorities => {
                    self.priority_list.next();
                    self.filter_tasks();
                }
            },
            Mode::Edit => {}
            Mode::Help => {}
            Mode::Confirm(_) => {}
        }
    }

    pub fn navigate_right(&mut self) {
        match self.mode {
            Mode::Normal => match self.focus {
                Focus::Tasks => {
                    self.focus = Focus::Contexts;
                }
                Focus::Contexts => {
                    self.focus = Focus::Projects;
                }
                Focus::Projects => {
                    self.focus = Focus::Priorities;
                }
                Focus::Priorities => {
                    self.focus = Focus::Tasks;
                }
            },
            Mode::Edit => {}
            Mode::Help => {}
            Mode::Confirm(_) => {}
        }
    }

    pub fn navigate_left(&mut self) {
        match self.mode {
            Mode::Normal => match self.focus {
                Focus::Tasks => {
                    self.focus = Focus::Priorities;
                }
                Focus::Contexts => {
                    self.focus = Focus::Tasks;
                }
                Focus::Projects => {
                    self.focus = Focus::Contexts;
                }
                Focus::Priorities => {
                    self.focus = Focus::Projects;
                }
            },
            Mode::Edit => {}
            Mode::Help => {}
            Mode::Confirm(_) => {}
        }
    }

    pub fn confirm_action(&mut self) {
        if let Mode::Confirm(action) = &self.mode {
            match action {
                ConfirmedAction::Save => {
                    self.save().expect("Failed to save file");
                    self.mode = Mode::Normal;
                }
                ConfirmedAction::Sort => {
                    self.sort_tasks();
                    self.mode = Mode::Normal;
                }
            }
        }
    }

    pub fn cancel_action(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn enter_confirm_mode(&mut self, action: ConfirmedAction) {
        self.mode = Mode::Confirm(action);
    }

    // enter editing mode and return the string of the currently selected task
    pub fn enter_edit_mode(&mut self) -> String {
        self.mode = Mode::Edit;

        match self.get_selected_task() {
            Some(task) => task.to_string(),
            None => "".to_string(),
        }
    }

    pub fn exit_edit_mode(&mut self, input_str: Option<String>) {
        self.mode = Mode::Normal;
        if input_str.is_none() {
            return;
        }

        let task_list_idx = self.task_list.selection().unwrap();
        let tasks_idx = self.task_list.items()[task_list_idx];
        let task_str = input_str.unwrap();

        if task_str.is_empty() {
            if task_list_idx != 0 {
                self.tasks.remove(tasks_idx);
                self.update_state_after_edit();
            }
            return;
        }

        if let Ok(mut task) = todo_txt::task::Task::from_str(&task_str) {
            if task_list_idx == 0 {
                task.date_created = Some(chrono::Local::now().date_naive());
                self.tasks.push(task);
                self.task_list.select(Some(0));
            } else {
                self.tasks[tasks_idx] = task;
            }
            self.update_state_after_edit();
        } else {
            self.error_msg = "Failed to parse Task".to_string();
        }
    }

    pub fn enter_help_mode(&mut self) {
        self.mode = Mode::Help;
    }

    pub fn exit_help_mode(&mut self) {
        self.mode = Mode::Normal;
    }
}
