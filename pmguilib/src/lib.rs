use pyo3::prelude::*;
mod tests;

#[pymodule]
mod pmguilib {
    use pyo3::prelude::*;

    /// # UserData
    /// Contains all user data and methods
    #[pyclass]
    pub struct UserData {
        #[pyo3(get)]
        pub tasks: Vec<Task>,
        #[pyo3(get)]
        pub shop: Vec<Item>,
        #[pyo3(get)]
        pub penalties: Vec<Item>,
        #[pyo3(get)]
        pub points: usize,
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

        // NT
        /// # add_penalty
        /// This function adds a penalty
        pub fn add_penalty(&mut self, name: &str, points: usize) {
            self.penalties.push(Item::new(name, points));
        }

        // NT
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

        // NT
        /// # penalty
        /// Assign penalty points. This operation may be used multiple times
        /// If a penalty does not found, this will return false
        /// If it is not enough points, it will return false
        /// If a penalty activated, it will return true
        pub fn penalty(&mut self, item_id: usize) -> bool {
            match self.penalties.get(item_id) {
                Some(i) => {
                    if self.points >= i.points {
                        self.points -= i.points;
                        true
                    } else {
                        false
                    }
                }
                None => false,
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
            }
        }
    }

    /// # Task
    /// Contains task name, reward and state (completed or not)
    #[pyclass(from_py_object)]
    #[derive(Clone, Debug, PartialEq)]
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
    #[derive(Clone, Debug, PartialEq)]
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
}
