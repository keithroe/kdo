
use crate::draw;
use crate::state;

use tui_input::backend::crossterm::EventHandler;


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
            match app.mode {
                app::Mode::Normal => {
                    match key.code {
                        crossterm::event::KeyCode::Esc |
                        crossterm::event::KeyCode::Char('q') => {
                            app.should_quit = true // call func that can cleanup
                        }
                        crossterm::event::KeyCode::Char('j') => {
                            app.navigate_down();
                        }
                        crossterm::event::KeyCode::Char('k') => {
                            app.navigate_up();
                        }
                        crossterm::event::KeyCode::Char('h') => {
                            app.navigate_left();
                        }
                        crossterm::event::KeyCode::Char('l') => {
                            app.navigate_right();
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
                            app.toggle_completed();
                        }
                        crossterm::event::KeyCode::Char('X') => {
                            app.mark_selected_task_complete();
                        }
                        _ => {}
                    }
                }
                app::Mode::Edit => match key.code {
                    crossterm::event::KeyCode::Esc => {
                        app.exit_edit_mode(None);
                    }
                    crossterm::event::KeyCode::Enter => {
                        app.exit_edit_mode(
                            Some(ui_state.input.value().to_string())
                        );
                    }
                    _ => {
                        ui_state.input.handle_event(&crossterm::event::Event::Key(key));
                    }
                },
                app::Mode::Search => {}
                app::Mode::Confirm(_) => match key.code {
                    crossterm::event::KeyCode::Esc |
                    crossterm::event::KeyCode::Char('N') |
                    crossterm::event::KeyCode::Char('n') => {
                        app.cancel_action();
                    }
                    crossterm::event::KeyCode::Enter |
                    crossterm::event::KeyCode::Char('Y') |
                    crossterm::event::KeyCode::Char('y') 
                        => {
                        app.confirm_action();
                    }
                    _ => {}
                }
            }
            elapsed += handle_input_start.elapsed().unwrap().as_secs_f64();
        }
        app.end_frame(elapsed);

        if app.should_quit {
            return Ok(());
        }
    }
}
