use pyo3::prelude::*;
mod tests;

#[pymodule]
mod pmguilib {
    use serde::{Deserialize, Serialize};
    use std::process::exit;
    use std::{
        fs::{self, File, create_dir_all},
        io::{self, Read},
    };

    use pyo3::prelude::*;

    /// # UserData
    /// Contains all user data and methods
    #[pyclass]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserData {
        #[pyo3(get)]
        pub tasks: Vec<Task>,
        #[pyo3(get)]
        pub shop: Vec<Item>,
        #[pyo3(get)]
        pub penalties: Vec<Item>,
        #[pyo3(get)]
        pub points: usize,
        #[pyo3(get)]
        pub lifes: usize,
    }

    #[pymethods]
    impl UserData {
        // Tasks:

        /// # add_task
        /// This function adds a task, state is false by default.
        pub fn add_task(&mut self, name: &str, reward: usize) {
            self.tasks.push(Task::new(name, reward));
        }

        /// # remove_task
        /// Reqires a task id. If a task is not found this will return false
        pub fn remove_task(&mut self, task_id: usize) -> bool {
            if self.tasks.len() > task_id {
                self.tasks.remove(task_id);
                true
            } else {
                false
            }
        }

        /// # change_task
        /// This function changes a task state. If task is not completed it will be set to completed, if not it will be set not completed.
        /// If a task not found, it will return false
        pub fn change_task(&mut self, task_id: usize) -> bool {
            match self.tasks.get_mut(task_id) {
                Some(t) => {
                    t.state = !t.state;
                    if t.state {
                        self.points += t.reward;
                    }
                    true
                }
                None => false,
            }
        }

        /// # list_tasks
        /// Returns all tasks
        pub fn list_tasks(&self) -> Vec<Task> {
            self.tasks.clone()
        }

        // Shop:

        /// # add_item
        /// This function adds an item
        pub fn add_item(&mut self, name: &str, price: usize) {
            self.shop.push(Item::new(name, price));
        }

        /// # remove_item
        /// Reqires a item id. If a item is not found it will return false
        pub fn remove_item(&mut self, item_id: usize) -> bool {
            if self.shop.len() > item_id {
                self.shop.remove(item_id);
                true
            } else {
                false
            }
        }

        /// # buy_item
        /// Buy an item. This operation may be used multiple times
        /// If a task not found, this will return false
        /// If it is not enough points, it will return false
        /// If an item bought, it will return true
        pub fn buy_item(&mut self, item_id: usize) -> bool {
            match self.shop.get(item_id) {
                Some(i) => {
                    if i.points <= self.points {
                        self.points -= i.points;
                        true
                    } else {
                        false
                    }
                }
                None => false,
            }
        }

        // Penalties

        /// # add_penalty
        /// This function adds a penalty
        pub fn add_penalty(&mut self, name: &str, points: usize) {
            self.penalties.push(Item::new(name, points));
        }

        /// # remove_penalty
        /// Reqires a item id. If a item is not found it will return false
        pub fn remove_penalty(&mut self, item_id: usize) -> bool {
            if self.penalties.len() > item_id {
                self.penalties.remove(item_id);
                true
            } else {
                false
            }
        }

        /// # penalty
        /// Assign penalty points. This operation may be used multiple times
        /// If a penalty does not found, this will return NotFound
        /// If it is not enough points, it will return LifeConsumed
        /// If a penalty activated, it will return Ok
        pub fn penalty(&mut self, item_id: usize) -> PenaltyResult {
            match self.penalties.get(item_id) {
                Some(i) => {
                    if self.points >= i.points {
                        self.points -= i.points;
                        PenaltyResult::Ok
                    } else {
                        self.lifes -= 1;
                        PenaltyResult::LifeConsumed
                    }
                }
                None => PenaltyResult::NotFound,
            }
        }

        // Self:
        #[staticmethod]
        pub fn new() -> Self {
            Self {
                tasks: vec![],
                shop: vec![],
                penalties: vec![],
                points: 0,
                lifes: 3,
            }
        }

        /// # load
        /// This function loads UserData from data_local_dir()/pmgui/user_data.json. On Linux it is ~/.local/share/pmgui/user_data.json.
        #[staticmethod]
        pub fn load() -> Result<Self, io::Error> {
            let dir = dirs::data_local_dir()
                .expect("Can not get data dir")
                .join("pmgui");
            let path = dir.join("user_data.json");

            let mut file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound
                        || e.kind() == io::ErrorKind::UnexpectedEof
                    {
                        create_dir_all(dir)?;
                        File::create(&path)?;
                        File::open(&path)?
                    } else {
                        eprint!("Cannot parse data, maybe user_data.json is broken: {e}");
                        exit(1);
                    }
                }
            };

            let mut user_data = String::new();
            file.read_to_string(&mut user_data)?;

            let user_data: Self = match serde_json::from_str(&user_data) {
                Ok(p) => p,
                Err(e) => {
                    if e.is_eof() {
                        Self::new()
                    } else {
                        eprint!("Cannot parse data, maybe save.json is broken: {e}");
                        exit(1);
                    }
                }
            };

            Ok(user_data)
        }

        /// # save
        /// This function saves UserData in data_local_dir()/pmgui/user_data.json. On Linux it is ~/.local/share/pmgui/user_data.json.
        pub fn save(&self) -> Result<(), io::Error> {
            let str = serde_json::to_string(self)?;
            let file = dirs::data_local_dir()
                .expect("Can not get data dir")
                .join("pmgui")
                .join("user_data.json");

            fs::write(file, str)?;

            Ok(())
        }
    }

    /// # Task
    /// Contains task name, reward and state (completed or not)
    #[pyclass(from_py_object)]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Task {
        #[pyo3(get)]
        pub name: String,
        #[pyo3(get)]
        pub state: bool,
        #[pyo3(get)]
        pub reward: usize,
    }

    impl Task {
        pub fn new(name: &str, reward: usize) -> Self {
            Self {
                name: name.to_string(),
                state: false,
                reward,
            }
        }
    }

    /// # Item
    /// Item for a shop
    #[pyclass(from_py_object)]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Item {
        #[pyo3(get)]
        pub name: String,
        #[pyo3(get)]
        pub points: usize,
    }

    impl Item {
        pub fn new(name: &str, points: usize) -> Self {
            Self {
                name: name.to_string(),
                points,
            }
        }
    }

    /// # PenaltyResult
    /// Used by function penalty.
    /// If points consumed it is Ok
    /// If penalty doesnt found it is NotFound
    /// If it doesnt enough points returns LifeConsumed
    #[pyclass]
    #[derive(Debug, PartialEq)]
    pub enum PenaltyResult {
        Ok,
        NotFound,
        LifeConsumed,
    }
}
