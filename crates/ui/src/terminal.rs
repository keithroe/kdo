use crate::draw;
use crate::state;

use tui_input::backend::crossterm::EventHandler;

pub static KEYBIND_HELP_STR: &str = r"
A simple viewer/editor of TODO lists in the todo.txt format
(https://github.com/todotxt/todo.txt).

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
";

/// Run the application.  Setup terminal, run the application loop, then cleanup
/// on exit.
pub fn run(
    app: &mut app::App,
    ui_state: &mut state::State,
) -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Run application logic
    let res = run_app(&mut terminal, app, ui_state);

    // restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    // Report any errors and exit run
    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}

/// The main application loop. Checks for user input, updates application state,
/// then draws application UI to terminal.
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut app::App,
    ui_state: &mut state::State,
) -> std::io::Result<()> {
    loop {
        app.start_frame();

        // Draw the current state to the terminal
        let draw_start = std::time::SystemTime::now();
        terminal.draw(|f| draw::draw(f, app, ui_state))?;
        let mut elapsed = draw_start.elapsed().unwrap().as_secs_f64();

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            let handle_input_start = std::time::SystemTime::now();
            match app.mode() {
                app::Mode::Normal => {
                    match key.code {
                        crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
                            app.quit()
                        }
                        crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
                            app.navigate_down();
                        }
                        crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
                            app.navigate_up();
                        }
                        crossterm::event::KeyCode::Char('h') | crossterm::event::KeyCode::Left => {
                            app.navigate_left();
                        }
                        crossterm::event::KeyCode::Char('l') | crossterm::event::KeyCode::Right => {
                            app.navigate_right();
                        }
                        crossterm::event::KeyCode::Char('H')
                        | crossterm::event::KeyCode::Char(' ') => {
                            app.enter_help_mode();
                        }
                        crossterm::event::KeyCode::Char('e') | crossterm::event::KeyCode::Enter => {
                            let input_string = app.enter_edit_mode();
                            ui_state.input = tui_input::Input::new(input_string);
                        }
                        crossterm::event::KeyCode::Char('s') => {
                            app.enter_confirm_mode(app::ConfirmedAction::Save);
                        }
                        crossterm::event::KeyCode::Char('S') => {
                            app.enter_confirm_mode(app::ConfirmedAction::Sort);
                        }
                        crossterm::event::KeyCode::Char('x') => {
                            app.toggle_view_completed();
                        }
                        crossterm::event::KeyCode::Char('X') => {
                            app.toggle_task_complete();
                        }
                        _ => {}
                    }
                }
                app::Mode::Edit => match key.code {
                    crossterm::event::KeyCode::Esc => {
                        app.exit_edit_mode(None);
                    }
                    crossterm::event::KeyCode::Enter => {
                        app.exit_edit_mode(Some(ui_state.input.value().to_string()));
                    }
                    _ => {
                        ui_state
                            .input
                            .handle_event(&crossterm::event::Event::Key(key));
                    }
                },
                app::Mode::Help => match key.code {
                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char(' ') => {
                        app.exit_help_mode();
                    }
                    _ => {}
                },
                app::Mode::Confirm(_) => match key.code {
                    crossterm::event::KeyCode::Esc
                    | crossterm::event::KeyCode::Char('N')
                    | crossterm::event::KeyCode::Char('n') => {
                        app.cancel_action();
                    }
                    crossterm::event::KeyCode::Enter
                    | crossterm::event::KeyCode::Char('Y')
                    | crossterm::event::KeyCode::Char('y') => {
                        app.confirm_action();
                    }
                    _ => {}
                },
            }
            elapsed += handle_input_start.elapsed().unwrap().as_secs_f64();
        }
        app.end_frame(elapsed);

        if app.should_quit() {
            return Ok(());
        }
    }
}
