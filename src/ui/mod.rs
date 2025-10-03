mod app;
mod input;

use crate::types::data::Data;
use self::input::handle_input;
use app::{App, Mode, AssignmentField};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::sync::Arc;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, Cell, Paragraph, Row,
        Table, List, ListItem, Wrap,
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
            date.format("%A %-d, %H:%M").to_string()
        } else {
            "(No due date)".to_string()
        };
        let name = if a.locked {
            format!("🔒{}", a.name)
        } else {
            a.name.clone()
        };
        let cells = vec![
            format!("{}", a.course),
            name,
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
    let table = Table::default()
        .rows(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Upcoming Assignments"),
        )
        .row_highlight_style(selected_style)
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
    let table = Table::default()
        .rows(rows)
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

async fn render_links(app: Arc<Mutex<App>>) -> List<'static> {
    let app = app.lock().await;
    if let Some(i) = app.assignments_state.selected() {
        let links = app.data.assignments[i].links.clone().into_iter().map(|link| {
            ListItem::new(link.title.clone())
        }).collect::<Vec<_>>();
        let selected_style = Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD);
            return List::new(links)
                .block(
                    Block::default()
                    .borders(Borders::ALL)
                    .title("Links")
                )
                .highlight_style(selected_style);
    } else {
        return List::new::<Vec<String>>(vec![])
            .block(
                Block::default()
                .borders(Borders::ALL)
                .title("Links")
            );
    }
}

async fn render_default<B: Backend>(terminal: &mut Terminal<B>, app: Arc<Mutex<App>>) {
    let welcome = render_welcome(app.clone()).await;
    let assignments = render_assignments(app.clone()).await;
    let mut assignments_state = app.lock().await.assignments_state.clone();
    let summary = render_summary(app.clone()).await;
    let links = render_links(app.clone()).await;
    let mut links_state = app.lock().await.links_state.clone();
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
            .split(f.area());

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)].as_ref())
            .split(chunks[2]);

        f.render_widget(welcome, chunks[0]);
        f.render_stateful_widget(assignments, chunks[1], &mut assignments_state);
        f.render_widget(summary, bottom_chunks[0]);
        f.render_stateful_widget(links, bottom_chunks[1], &mut links_state);
        f.render_widget(grades, bottom_chunks[2]);
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


