
#[derive(Default)]
pub struct State {
    pub task_list_state: ratatui::widgets::ListState,
    pub context_list_state: ratatui::widgets::ListState,
    pub project_list_state: ratatui::widgets::ListState,
    pub priority_list_state: ratatui::widgets::ListState,

    pub input: tui_input::Input,
}

impl State {
    pub fn new() -> Self {
        State {
            task_list_state : ratatui::widgets::ListState::default(),
            context_list_state : ratatui::widgets::ListState::default(),
            project_list_state : ratatui::widgets::ListState::default(),
            priority_list_state : ratatui::widgets::ListState::default(),
            input : tui_input::Input::new("".to_string()),
        }
    }
}
