use crate::state;
use app::app;

static FOCUS_COLOR : ratatui::style::Color = ratatui::style::Color::Reset; 
static UNFOCUS_COLOR : ratatui::style::Color = ratatui::style::Color::DarkGray; 
static BG_COLOR : ratatui::style::Color    = ratatui::style::Color::Reset; 
static SELECTION_COLOR : ratatui::style::Color = ratatui::style::Color::Yellow; 

pub fn draw<B: ratatui::backend::Backend>(
    frame: &mut ratatui::Frame<B>,
    app: &mut app::App,
    ui_state: &mut state::State,
) {
    //
    // Update selections lists
    //
    ui_state.task_list_state.select(app.task_list.selection());
    ui_state
        .context_list_state
        .select(app.context_list.selection());
    ui_state
        .project_list_state
        .select(app.project_list.selection());
    ui_state
        .priority_list_state
        .select(app.priority_list.selection());

    //
    // Create main body chunks
    //
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                ratatui::layout::Constraint::Length(3), // title
                ratatui::layout::Constraint::Min(3),    // body
                ratatui::layout::Constraint::Length(3), // command line
            ]
            .as_ref(),
        )
        .split(frame.size());

    //
    // Header: display application title
    //
    let header_block = ratatui::widgets::Paragraph::new(app.title)
        .style(ratatui::style::Style::default().fg(SELECTION_COLOR).bg(BG_COLOR))
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            ratatui::widgets::Block::default()
                .style(ratatui::style::Style::default().fg(UNFOCUS_COLOR).bg(BG_COLOR))
                .borders(ratatui::widgets::Borders::ALL), //.borders(ratatui::widgets::Borders::BOTTOM | ratatui::widgets::Borders::TOP),
        );
    frame.render_widget(header_block, chunks[0]);

    //
    // Body: main todo browser
    //
    if app.mode == app::Mode::Help {
        let help_paragraph = ratatui::widgets::Paragraph::new(crate::terminal::KEYBIND_HELP_STR)
            .block(
                ratatui::widgets::Block::default()
                    .title("help")
                    .borders(ratatui::widgets::Borders::ALL),
            );
        frame.render_widget(help_paragraph, chunks[1]);
    } else {
        let body_chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(
                [
                    ratatui::layout::Constraint::Percentage(60), // tasks
                    ratatui::layout::Constraint::Percentage(15), // project
                    ratatui::layout::Constraint::Percentage(15), // context
                    ratatui::layout::Constraint::Percentage(10), //
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        // We can now render the item list
        let tasks: Vec<String> = app
            .task_list
            .items()
            .iter()
            .map(|idx| app.tasks[*idx].to_string())
            .collect();

        frame.render_stateful_widget(
            render_list(
                "task",
                &tasks,
                app.mode == app::Mode::Normal && app.focus == app::Focus::Tasks,
            ),
            body_chunks[0],
            &mut ui_state.task_list_state,
        );

        frame.render_stateful_widget(
            render_list(
                "context",
                app.context_list.items(),
                app.mode == app::Mode::Normal && app.focus == app::Focus::Contexts,
            ),
            body_chunks[1],
            &mut ui_state.context_list_state,
        );

        frame.render_stateful_widget(
            render_list(
                "project",
                app.project_list.items(),
                app.mode == app::Mode::Normal && app.focus == app::Focus::Projects,
            ),
            body_chunks[2],
            &mut ui_state.project_list_state,
        );

        frame.render_stateful_widget(
            render_list(
                "priority",
                app.priority_list.items(),
                app.mode == app::Mode::Normal && app.focus == app::Focus::Priorities,
            ),
            body_chunks[3],
            &mut ui_state.priority_list_state,
        );
    }

    //
    // Edit line at bottom
    //
    let edit_block = match &app.mode {
        app::Mode::Edit => ratatui::widgets::Paragraph::new(ui_state.input.value())
            .style(ratatui::style::Style::default().fg(FOCUS_COLOR).bg(BG_COLOR))
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
        app::Mode::Confirm(action) => {
            let action_str = match action {
                app::ConfirmedAction::Save => "Save file",
                app::ConfirmedAction::Sort => "Sort tasks",
            };
            ratatui::widgets::Paragraph::new(format!("{}? [Y/n]", action_str))
                .style(ratatui::style::Style::default().fg(FOCUS_COLOR).bg(BG_COLOR))
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL))
        }
        _ => {
            ratatui::widgets::Paragraph::new(
                "", //format!("{:.2}", app.frame_time*1000.0f64)
                   //"hjkl: navigate  <ent>: begin/save edit  <esc>: cancel edit  q: quit  s: save  S:sort",
            )
            .style(ratatui::style::Style::default().fg(UNFOCUS_COLOR).bg(BG_COLOR))
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL))
        }
    };
    frame.render_widget(edit_block, chunks[2]);

    if app.mode == app::Mode::Edit {
        let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = ui_state.input.visual_scroll(width as usize);
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        frame.set_cursor(
            // Put cursor past the end of the input text
            chunks[2].x + ((ui_state.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[2].y + 1,
        )
    }
}

fn render_list<'a>(
    title: &'a str,
    item_strings: &'a [String],
    is_focus: bool,
) -> ratatui::widgets::List<'a> {
    let items: Vec<ratatui::widgets::ListItem> = item_strings
        .iter()
        .map(|s| ratatui::widgets::ListItem::new(ratatui::text::Line::from(s.as_str())))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    ratatui::widgets::List::new(items)
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .style(ratatui::style::Style::default()
                    .fg(if is_focus {
                            FOCUS_COLOR
                        } else {
                            UNFOCUS_COLOR
                        })
                    .bg(BG_COLOR)
                )
                .title(title),
        )
        .highlight_style(ratatui::style::Style::default().fg(SELECTION_COLOR).bg(BG_COLOR))
}
