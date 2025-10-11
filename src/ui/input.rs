use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};
use super::app::{self, App, Dir, Mode, AssignmentField};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;

pub async fn handle_input(app: Arc<Mutex<App>>) -> Result<bool, Box<dyn Error>> {
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
            KeyCode::Char('j') => app.lock().await.next_assignment(),
            KeyCode::Char('k') => app.lock().await.prev_assignment(),
            KeyCode::Char('q') => {
                app.lock().await.serialize_data()?;
                return Ok(true);
            }
            KeyCode::Char('o') => app.lock().await.open_assignment().await,
            KeyCode::Char('n') => app.lock().await.new_assignment().await?,
            KeyCode::Char('r') => {
                app::refresh(app.clone()).await?;
                app.lock().await.serialize_data()?;
            },
            KeyCode::Char('d') => app.lock().await.mark_done(),
            KeyCode::Char('x') => app.lock().await.delete_assignment().await?,
            KeyCode::Enter => app.lock().await.enter(),
            KeyCode::Esc => app.lock().await.esc(),
            _ => (),
        },
        KeyModifiers::SHIFT => match key.code {
            KeyCode::Char('J') => app.lock().await.next_link(),
            KeyCode::Char('K') => app.lock().await.prev_link(),
            KeyCode::Char('O') => app.lock().await.open_link().await,
            _ => (),
        },
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Char('c') => {
                app.lock().await.serialize_data()?;
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
            KeyCode::Esc => app.lock().await.exit_new_assignment_mode().await?,
            _ => take_new_assignment_input(app.clone(), key).await,
        }
        KeyModifiers::SHIFT => take_new_assignment_input(app.clone(), key).await,
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Char('c') => app.lock().await.exit_new_assignment_mode().await?,
            _ => (),
        }
        _ => (),
    };
    Ok(false)
}

async fn take_new_assignment_input(app: Arc<Mutex<App>>, key: KeyEvent) {
    let mut app = app.lock().await;

    let field = match app.mode {
        Mode::Normal => unreachable!(),
        Mode::NewAssignment(field) => field,
    };

    let i = app.assignments_state.selected().unwrap();

    // Handle switching fields
    match key.code {
        KeyCode::Tab => {
            let new_field = match field {
                AssignmentField::Course => AssignmentField::Name,
                AssignmentField::Name => AssignmentField::DueDate,
                AssignmentField::DueDate => AssignmentField::DueDate,
            };
            app.mode = Mode::NewAssignment(new_field);
        },
        KeyCode::BackTab => {
            let new_field = match field {
                AssignmentField::Course => AssignmentField::Course,
                AssignmentField::Name => AssignmentField::Course,
                AssignmentField::DueDate => AssignmentField::Name,
            };
            app.mode = Mode::NewAssignment(new_field);
        },
        _ => (),
    }

    // Handle the DueDate case
    if field == AssignmentField::DueDate {
        match key.code {
            KeyCode::Char('j') => app.data.assignments[i].decrement_due_date(),
            KeyCode::Char('k') => app.data.assignments[i].increment_due_date(),
            _ => (),
        }
        return;
    }

    let mut text = match field {
        AssignmentField::Course => app.data.assignments[i].course.clone(),
        AssignmentField::Name => app.data.assignments[i].name.clone(),
        AssignmentField::DueDate => unreachable!(), 
    };

    // Handle text input
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Backspace => {text.pop();},
            KeyCode::Char(c) => text.push(c),
            _ => (),
        },
        KeyModifiers::SHIFT => match key.code {
            KeyCode::Char(c) => text.push(c),
            _ => (),
        },
        _ => (),
    }

    match field {
        AssignmentField::Course => app.data.assignments[i].course = text,
        AssignmentField::Name => app.data.assignments[i].name = text,
        AssignmentField::DueDate => (),
    }


}
