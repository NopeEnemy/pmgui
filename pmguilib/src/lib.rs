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
        /// Reqires a task name. If a task is not found this will return false
        /// Warning: it will delete all task with this task name
        pub fn remove_task(&mut self, task_name: &str) -> bool {
            let mut is_deleted = false;
            self.tasks.retain(|i| {
                if i.name == task_name {
                    is_deleted = true;
                    false
                } else {
                    true
                }
            });

            is_deleted
        }

        /// # change_task
        /// This function changes a task state. If task is not completed it will be set to completed, if not it will be set not completed.
        /// If a task not found, it will return false
        pub fn change_task(&mut self, task_name: &str) -> bool {
            let mut is_changed = false;
            for t in self.tasks.iter_mut() {
                if t.name == task_name {
                    is_changed = true;
                    t.state = !t.state;
                    if t.state {
                        self.points += t.reward;
                    }
                }
            }

            is_changed
        }

        /// # list_tasks
        /// Returns all tasks
        pub fn list_tasks(&self) -> Vec<Task> {
            self.tasks.clone()
        }

        // Shop:

        // NT
        /// # add_item
        /// This function adds an item
        pub fn add_item(&mut self, name: &str, price: usize) {
            self.shop.push(Item::new(name, price));
        }

        // NT
        /// # remove_item
        /// Reqires a item name. If a task is not found it will return false
        /// Warning: It will remove all items with this name
        pub fn remove_item(&mut self, item_name: &str) -> bool {
            let mut is_deleted = false;
            self.shop.retain(|i| {
                if i.name == item_name {
                    is_deleted = true;
                    false
                } else {
                    true
                }
            });
            is_deleted
        }

        // NT
        /// # buy_item
        /// Buy an item. This operation may be used multiple times
        /// If a task not found, this will return false
        /// If it is not enough points, it will return false
        /// If an item bought, it will return true
        /// Warning: It will buy all items with this name
        pub fn buy_item(&mut self, item_name: &str) -> bool {
            let mut is_bought = false;
            for i in self.shop.iter_mut() {
                if i.name == item_name {
                    if i.price <= self.points {
                        self.points -= i.price;
                        is_bought = true;
                    }
                }
            }
            is_bought
        }

        // Self:
        #[staticmethod]
        pub fn new() -> Self {
            Self {
                tasks: vec![],
                shop: vec![],
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
        pub price: usize,
    }

    impl Item {
        pub fn new(name: &str, price: usize) -> Self {
            Self {
                name: name.to_string(),
                price,
            }
        }
    }
}
