use tui::{
    style::{Color, Modifier, Style},
    widgets::{ListState, TableState},
};
use std::time::Duration;
use tokio::sync::Mutex;
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
    pub tick_rate: Duration,
    pub assignments_state: TableState,
    pub links_state: ListState,
    pub data: Data,
    pub active_widget: Widget,
}

impl App {
    pub fn new(data: Data, tick_rate: Duration) -> App {
        App {
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
    let handle = tokio::task::spawn(async move {
        let course_ids = vec![72125, 71983, 72567, 71447, 72767]; // Henry course IDs
        let data;
        match Data::from_course_ids(course_ids, false).await {
            Ok(d) => data = d,
            Err(e) => {
                eprintln!("Error fetching data: {}", e);
                return;
            }
        }
        let mut app = app_clone.lock().await;
        app.data = data;
    });
    handle.await?;
    app.lock().await.data.serialize_to_file("data.json")?;
    
    Ok(())
}
