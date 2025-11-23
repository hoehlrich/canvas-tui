use ratatui::widgets::{ListState, TableState};
use tokio::sync::Mutex;
use std::hash::{DefaultHasher, Hasher};
use std::{hash::Hash, time::Duration};
use std::sync::Arc;
use std::error::Error;
use crate::types::{assignment::Assignment, data::Data};

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
    pub data_path: String,
    pub tick_rate: Duration,
    course_ids: Vec<u32>,
    pub assignments_state: TableState,
    pub links_state: ListState,
    pub data: Data,
    pub mode: Mode,
}

impl App {
    pub fn new(data_path: String, tick_rate: Duration, course_ids: Vec<u32>, data: Data) -> Self {
        App {
            data_path,
            tick_rate,
            course_ids,
            assignments_state: TableState::default(),
            links_state: ListState::default(),
            data,
            mode: Mode::Normal,
        }
    }

    pub fn select_by_hash(&mut self, hash: Option<u64>) {
        if let Some(h) = hash {
            let new_i = self
                .data
                .assignments
                .iter()
                .enumerate()
                .find(|a| {
                    let mut hasher = DefaultHasher::new();
                    a.1.hash(&mut hasher);
                    hasher.finish() == h
                })
                .map_or_else(|| 0, |v| v.0);
            self.assignments_state.select(Some(new_i));
        }
    }

    pub fn get_selected_hash(&self) -> Option<u64> {
        let mut selected_hash = None;
        if let Some(selected_i) = self.assignments_state.selected() {
            let mut hasher = DefaultHasher::new();
            self.data.assignments[selected_i].hash(&mut hasher);
            selected_hash = Some(hasher.finish());
        }
        selected_hash
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

    pub async fn exit_new_assignment_mode(&mut self) -> Result<(), Box<dyn Error>> {
        self.mode = Mode::Normal;
        self.data.sort_assignments();
        self.serialize_data()?;
        Ok(())
    }

    pub async fn delete_assignment(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(i) = self.assignments_state.selected() {
            // Do nothing if the assignment is not user-created
            if !self.data.assignments[i].custom {
                return Ok(());
            }

            self.data.assignments.remove(i);
            if self.data.assignments.len() == 0 {
                self.assignments_state.select(None);
            } else if self.data.assignments.len() <= i {
                self.assignments_state.select(Some(i - 1));
            }
        }
        self.serialize_data()?;
        Ok(())
    }

    pub async fn open_assignment(&self) {
        if let Some(i) = self.assignments_state.selected() {
            let url = self.data.assignments[i].html_url.clone();
            tokio::task::spawn(async move {
                let _ = open::that(url);
            });
        }
    }

    pub async fn open_link(&self) {
        if let (Some(link_i), Some(assignment_i)) = (self.links_state.selected(), self.assignments_state.selected()) {
            if self.data.assignments[assignment_i].links.len() == 0 {
                return;
            }
            let url = self.data.assignments[assignment_i].links[link_i].url.clone();
            tokio::task::spawn(async move {
                let _ = open::that(url);
            });
        }
    }

    pub fn mark_done(&mut self) {
        if let Some(a) = self.assignments_state.selected() {
            let assignment = &mut self.data.assignments[a];
            assignment.completed = !assignment.completed;
            assignment.modified = true;
        }
        let selected_hash = self.get_selected_hash();
        self.data.sort_assignments();
        self.select_by_hash(selected_hash);
    }

    pub fn enter(&mut self) {
        ()
    }

    pub fn esc(&mut self) {
        ()
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

        // Select first link if an assignment is selected
        if let Some(i) = self.assignments_state.selected() {
            if self.data.assignments[i].links.len() > 0 {
                self.links_state.select(Some(0));
            }
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

    pub fn next_link(&mut self) {
        if let (Some(link_i), Some(assignment_i)) = (self.links_state.selected(), self.assignments_state.selected()) {
            let next = if link_i >= self.data.assignments[assignment_i].links.len() - 1 {
                link_i
            } else {
                link_i + 1
            };
            self.links_state.select(Some(next));
        }
    }

    pub fn prev_link(&mut self) {
        if let Some(selected) = self.links_state.selected() {
            let prev = if selected == 0 {
                0
            } else {
                selected - 1
            };
            self.links_state.select(Some(prev));
        }
    }

    pub fn serialize_data(&self) -> Result<(), Box<dyn Error>> {
        self.data.serialize_to_file(&self.data_path)
    }

    pub fn on_tick(&mut self) {
        ()
    }
}

pub async fn refresh(app: Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    let app_clone = Arc::clone(&app);
    tokio::task::spawn(async move {
        let course_ids = app.lock().await.course_ids.clone();

        // query full list of assignments
        let assignments = match crate::queries::assignments::query_assignments(&course_ids).await {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error fetching assignments: {}", e);
                return;
            }
        };

        // Store pre-refresh selected assignment hash
        let selected_hash = app_clone.lock().await.get_selected_hash();

        app_clone.lock().await.data.update_assignments(assignments);

        // Restore selection to select pre-refresh hash
        app_clone.lock().await.select_by_hash(selected_hash);

        let grades = match crate::queries::grades::query_grades(&course_ids).await {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error fetching grades: {}", e);
                return;
            }
        };
        app_clone.lock().await.data.remove_past_assignments();
        app_clone.lock().await.data.grades = grades;
        let path = app_clone.lock().await.data_path.clone();
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
