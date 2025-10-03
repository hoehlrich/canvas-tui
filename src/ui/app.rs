use ratatui::widgets::{ListState, TableState};
use tokio::sync::Mutex;
use std::time::Duration;
use std::sync::Arc;
use std::error::Error;
use crate::types::{assignment::Assignment, data::Data};

pub enum Dir {
    Up,
    Down,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Widget {
    Assignments,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AssignmentField {
    Course,
    Name,
    DueDate,
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    Normal,
    NewAssignment(AssignmentField),
}

pub struct App {
    pub path: String,
    pub tick_rate: Duration,
    pub assignments_state: TableState,
    pub links_state: ListState,
    pub data: Data,
    pub active_widget: Widget,
    pub mode: Mode,
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
            mode: Mode::Normal,
        }
    }

    pub async fn new_assignment(&mut self) -> Result<(), Box<dyn Error>> {
        self.mode = Mode::NewAssignment(AssignmentField::Course);
        match self.mode {
            Mode::Normal => return Err("Cannot create new assignment in normal mode.".into()),
            Mode::NewAssignment(_) => {
                self.data.assignments.insert(0, Assignment::empty());
                self.assignments_state.select(Some(0));
            }
        }

        Ok(())
    }

    pub async fn exit_new_assignment_mode(&mut self) {
        self.mode = Mode::Normal;
        self.data.sort_assignments();
    }

    pub async fn delete_assignment(&mut self) {
        if let Some(i) = self.assignments_state.selected() {
            // Do nothing if the assignment is not user-created
            if !self.data.assignments[i].custom {
                return;
            }

            self.data.assignments.remove(i);
            if self.data.assignments.len() == 0 {
                self.assignments_state.select(None);
            } else if self.data.assignments.len() <= i {
                self.assignments_state.select(Some(i - 1));
            }
        }
    }

    pub async fn open(&self) {
        match self.active_widget {
            Widget::Assignments => {
                if let Some(i) = self.assignments_state.selected() {
                    let url = self.data.assignments[i].html_url.clone();
                    tokio::task::spawn(async move {
                        let _ = open::that(url);
                    });
                }
            },
        }
    }

    pub fn mark_done(&mut self) {
        match self.active_widget {
            Widget::Assignments => {
                if let Some(a) = self.assignments_state.selected() {
                    let assignment = &mut self.data.assignments[a];
                    assignment.completed = !assignment.completed;
                    assignment.modified = true;
                }
            }
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
                selected
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
                0
            } else {
                selected - 1
            };
            self.assignments_state.select(Some(prev));
        } else if self.data.assignments.len() > 0 {
            self.assignments_state.select(Some(0));
        }
    }

    pub fn quit(&self) -> Result<(), Box<dyn Error>> {
        self.data.serialize_to_file(&self.path)
    }

    pub fn on_tick(&mut self) {
        ()
    }
}

pub async fn refresh(app: Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    let app_clone = Arc::clone(&app);
    tokio::task::spawn(async move {
        let course_ids = app.lock().await.data.course_ids.clone();
        let assignments = match crate::queries::assignments::query_assignments(&course_ids).await {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error fetching assignments: {}", e);
                return;
            }
        };

        app_clone.lock().await.data.update_assignments(assignments);
        let grades = match crate::queries::grades::query_grades(&course_ids).await {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error fetching grades: {}", e);
                return;
            }
        };
        app_clone.lock().await.data.remove_past_assignments();
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
