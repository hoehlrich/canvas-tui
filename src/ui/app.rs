use tui::{
    style::{Color, Modifier, Style},
    widgets::{ListState, TableState},
};
use tokio::sync::Mutex;
use std::time::Duration;
use std::sync::Arc;
use std::error::Error;
use crate::types::data::Data;

pub enum Dir {
    Up,
    Down,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Widget {
    Assignments,
}

pub struct App {
    pub path: String,
    pub tick_rate: Duration,
    pub assignments_state: TableState,
    pub links_state: ListState,
    pub data: Data,
    pub active_widget: Widget,
}

impl App {
    pub fn new(data: Data, path: String, tick_rate: Duration) -> Self {
        App {
            path,
            tick_rate,
            assignments_state: TableState::default(),
            links_state: ListState::default(),
            data,
            active_widget: Widget::Assignments,
        }
    }

    pub fn open(&self) {
        match self.active_widget {
            Widget::Assignments => {
                if let Some(i) = self.assignments_state.selected() {
                    let assignment = &self.data.assignments[i];
                    open::that(&assignment.html_url).unwrap();
                }
            },
        }
    }

    pub fn enter(&mut self) {
        ()
    }

    pub fn esc(&mut self) {
        ()
    }

    pub fn mv(&mut self, dir: Dir) {
        match self.active_widget {
            Widget::Assignments => match dir {
                Dir::Down => self.next_assignment(),
                Dir::Up => self.prev_assignment(),
            }
        }
        
    }

    pub fn next_assignment(&mut self) {
        if let Some(selected) = self.assignments_state.selected() {
            let next = if selected >= self.data.assignments.len() - 1 {
                0
            } else {
                selected + 1
            };
            self.assignments_state.select(Some(next));
        } else if self.data.assignments.len() > 0 {
            self.assignments_state.select(Some(0));
        }
    }

    pub fn prev_assignment(&mut self) {
        if let Some(selected) = self.assignments_state.selected() {
            let prev = if selected == 0 {
                self.data.assignments.len() - 1
            } else {
                selected - 1
            };
            self.assignments_state.select(Some(prev));
        } else if self.data.assignments.len() > 0 {
            self.assignments_state.select(Some(0));
        }
    }

    pub fn on_tick(&mut self) {
        ()
    }
}

pub async fn refresh(app: Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    let app_clone = Arc::clone(&app);
    tokio::task::spawn(async move {
        let course_ids = vec![72125, 71983, 72567, 71447, 72767]; // Henry course IDs
        let assignments = match crate::queries::assignments::query_assignments(&course_ids).await {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error fetching assignments: {}", e);
                return;
            }
        };
        app_clone.lock().await.data.assignments = assignments;
        let grades = match crate::queries::grades::query_grades(&course_ids).await {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error fetching grades: {}", e);
                return;
            }
        };
        app_clone.lock().await.data.grades = grades;
        let path = app_clone.lock().await.path.clone();
        match app_clone.lock().await.data.serialize_to_file(&path) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error saving data: {}", e);
                return;
            }
        };
    });
    
    Ok(())
}
