mod app;

use crate::types::data::Data;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    symbols,
    text::Span,
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, GraphType, List, ListItem, Paragraph, Row,
        Table, Wrap,
    },
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use app::{App, Dir};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn render_assignments(app: Arc<Mutex<App>>) -> Table<'static> {
    let app = app.lock().await;
    let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let header_cells = ["Course", "Name", "Due Date"].iter().map(|h| Cell::from(*h));
    let header = Row::new(header_cells)
        .style(bold)
        .height(1);
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
        Row::new(cells)
        
    });
    let selected_style = Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD);
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Upcoming Assignments")
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
    let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let header_cells = ["Course", "Grade"].iter().map(|h| Cell::from(*h));
    let header = Row::new(header_cells)
        .style(bold)
        .height(1);
    let rows = app.data.grades.iter().map(|g| {
        let cells = vec![
            format!("{}", g.course),
            format!("{}", g.grade),
        ];
        Row::new(cells)
        
    });
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Grades")
        )
        .widths(&[
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 2),
        ]);
    return table;
}

async fn render_welcome(app: Arc<Mutex<App>>) -> Paragraph<'static> {
    let app = app.lock().await;
    Paragraph::new(format!("\nToday is {}, there are {} upcoming assignments", chrono::Local::now().format("%A %d %B"), app.data.assignments.len()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Welcome to CanvasTUI")
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
                .title("Assignment Summary")
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
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
            .split(f.size());

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 10), Constraint::Ratio(6, 10), Constraint::Ratio(3, 10)].as_ref())
            .split(chunks[0]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(3, 10), Constraint::Ratio(7, 10)].as_ref())
            .split(chunks[1]);

        let bottom_left_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(left_chunks[2]);

        f.render_widget(welcome, left_chunks[0]);
        f.render_stateful_widget(assignments, left_chunks[1], &mut assignments_state);
        f.render_widget(summary, bottom_left_chunks[0]);
        f.render_widget(grades, right_chunks[0]);
    });

}


pub async fn run(data: Data) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = Arc::new(Mutex::new(App::new(data, Duration::from_millis(1000))));

    // Main loop and tick logic
    let mut last_tick = Instant::now();
    loop {
        render_default(&mut terminal, Arc::clone(&app)).await;

        // Non-blocking key detection
        let timeout = app.lock().await
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


    // restore terminal
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
        match key.modifiers {
            KeyModifiers::NONE => match key.code {
                KeyCode::Char('j') => app.lock().await.mv(Dir::Down),
                KeyCode::Char('k') => app.lock().await.mv(Dir::Up),
                KeyCode::Char('q') => {
                    app.lock().await.data.serialize_to_file("data.json")?;
                    return Ok(true)
                },
                KeyCode::Char('o') => app.lock().await.open(),
                KeyCode::Char('r') => app::refresh(app).await?,
                KeyCode::Enter => app.lock().await.enter(),
                KeyCode::Esc => app.lock().await.esc(),
                _ => (),
            },
            KeyModifiers::CONTROL => match key.code {
                _ => (),
            },
            _ => (),
        }
    }
    Ok(false)
}

