mod app;

use crate::types::data::Data;
use app::{App, Dir, Mode, AssignmentField};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::sync::Arc;
use std::{
    env,
    error::Error,
    io,
    path::Path,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, GraphType, List, ListItem, Paragraph, Row,
        Table, Wrap,
    },
};

async fn render_assignments(app: Arc<Mutex<App>>) -> Table<'static> {
    let app = app.lock().await;
    let bold = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let mut header_cells: Vec<Cell> = ["Course", "Name", "Due Date"]
        .iter()
        .map(|h| Cell::from(*h))
        .collect();

    // If creating a new assignment highlight the property being modified
    match app.mode {
        Mode::NewAssignment(field) => {
            match field {
                AssignmentField::Course => header_cells[0] = Cell::from("Course").style(Style::default().fg(Color::LightYellow)),
                AssignmentField::Name => header_cells[1] = Cell::from("Name").style(Style::default().fg(Color::LightYellow)),
                AssignmentField::DueDate => header_cells[2] = Cell::from("Due Date").style(Style::default().fg(Color::LightYellow)),
            }
        },
        _ => (),
    }

    let header = Row::new(header_cells).style(bold).height(1);
    let rows = app.data.assignments.iter().map(|a| {
        let date = if let Some(date) = a.date {
            date.format("%A %d, %H:%M").to_string()
        } else {
            "(No due date)".to_string()
        };
        let cells = vec![
            format!("{}", a.course),
            format!("{}", a.name),
            format!("{}", date),
        ];
        let style = if a.completed {
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::CROSSED_OUT)
        } else {
            Style::default()
        };
        Row::new(cells).style(style)
    });
    let selected_style = match app.mode {
        Mode::Normal => Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
        Mode::NewAssignment(_) => Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
    };
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Upcoming Assignments"),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Ratio(1, 10),
            Constraint::Ratio(6, 10),
            Constraint::Ratio(3, 10),
        ]);
    return table;
}

async fn render_grades(app: Arc<Mutex<App>>) -> Table<'static> {
    let app = app.lock().await;
    let bold = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let header_cells = ["Course", "Grade"].iter().map(|h| Cell::from(*h));
    let header = Row::new(header_cells).style(bold).height(1);
    let rows = app.data.grades.iter().map(|g| {
        let cells = vec![format!("{}", g.course), format!("{}", g.grade)];
        Row::new(cells)
    });
    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Grades"))
        .widths(&[Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
    return table;
}

async fn render_welcome(app: Arc<Mutex<App>>) -> Paragraph<'static> {
    let app = app.lock().await;
    Paragraph::new(format!(
        "\nToday is {}, there are {} upcoming assignments",
        chrono::Local::now().format("%A %B %-d"),
        app.data.get_number_incomplete()
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Welcome to CanvasTUI"),
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
}

async fn render_summary(app: Arc<Mutex<App>>) -> Paragraph<'static> {
    let app = app.lock().await;
    let summary = if let Some(i) = app.assignments_state.selected() {
        let assignment = &app.data.assignments[i];
        let s = format!("Course: {}\nName: {}\n", assignment.course, assignment.name);
        s
    } else {
        "No assignment selected".to_string()
    };
    Paragraph::new(summary)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Assignment Summary"),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

async fn render_default<B: Backend>(terminal: &mut Terminal<B>, app: Arc<Mutex<App>>) {
    let welcome = render_welcome(app.clone()).await;
    let assignments = render_assignments(app.clone()).await;
    let mut assignments_state = app.lock().await.assignments_state.clone();
    let summary = render_summary(app.clone()).await;
    let grades = render_grades(app.clone()).await;

    let _ = terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 10),
                    Constraint::Ratio(6, 10),
                    Constraint::Ratio(3, 10),
                ]
                .as_ref(),
            )
            .split(f.size());

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(chunks[2]);

        f.render_widget(welcome, chunks[0]);
        f.render_stateful_widget(assignments, chunks[1], &mut assignments_state);
        f.render_widget(summary, bottom_chunks[0]);
        f.render_widget(grades, bottom_chunks[1]);
    });
}


pub async fn run(data: Data, path: String) -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = Arc::new(Mutex::new(App::new(
        data,
        path,
        Duration::from_millis(1000),
    )));

    // Initial refresh
    app::refresh(app.clone()).await?;

    // Main loop and tick logic
    let mut last_tick = Instant::now();
    loop {
        render_default(&mut terminal, Arc::clone(&app)).await;

        // Non-blocking key detection
        let timeout = app
            .lock()
            .await
            .tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));
        if event::poll(timeout)? {
            if handle_input(Arc::clone(&app)).await? {
                break;
            }
        }
        if last_tick.elapsed() >= app.lock().await.tick_rate {
            app.lock().await.on_tick();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn handle_input(app: Arc<Mutex<App>>) -> Result<bool, Box<dyn Error>> {
    if let Event::Key(key) = event::read()? {
        let mode = app.lock().await.mode.clone();
        match mode {
            Mode::Normal => handle_input_normal(app.clone(), key).await,
            Mode::NewAssignment(_) => handle_input_new_assignment(app.clone(), key).await,
        }
    } else {
        Ok(false)
    }
}

async fn handle_input_normal(app: Arc<Mutex<App>>, key: KeyEvent) -> Result<bool, Box<dyn Error>> {
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Char('j') => app.lock().await.mv(Dir::Down),
            KeyCode::Char('k') => app.lock().await.mv(Dir::Up),
            KeyCode::Char('q') => {
                app.lock().await.quit()?;
                return Ok(true);
            }
            KeyCode::Char('o') => app.lock().await.open().await,
            KeyCode::Char('n') => app.lock().await.new_assignment().await?,
            KeyCode::Char('r') => app::refresh(app).await?,
            KeyCode::Char('d') => app.lock().await.mark_done(),
            KeyCode::Char('x') => app.lock().await.delete_assignment().await,
            KeyCode::Enter => app.lock().await.enter(),
            KeyCode::Esc => app.lock().await.esc(),
            _ => (),
        },
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Char('c') => {
                app.lock().await.quit()?;
                return Ok(true);
            }
            _ => (),
        },
        _ => (),
    }
    Ok(false)
}

async fn handle_input_new_assignment(app: Arc<Mutex<App>>, key: KeyEvent) -> Result<bool, Box<dyn Error>> {
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Esc => app.lock().await.exit_new_assignment_mode().await,
            _ => app.lock().await.take_new_assignment_input(key).await,
        }
        KeyModifiers::SHIFT => app.lock().await.take_new_assignment_input(key).await,
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Char('c') => app.lock().await.exit_new_assignment_mode().await,
            _ => (),
        }
        _ => (),
    };
    Ok(false)
}

