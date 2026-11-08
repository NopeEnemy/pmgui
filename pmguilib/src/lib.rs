use pyo3::prelude::*;

#[pymodule]
mod pmguilib {
    use pyo3::prelude::*;

    /// # UserData
    /// Contains all user data and methods
    #[pyclass]
    pub struct UserData {
        #[pyo3(get)]
        tasks: Vec<Task>,
        #[pyo3(get)]
        points: usize,
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
        /// Reqires a task id (position). If a task is not found this will do nothing
        pub fn remove_task(&mut self, task_id: usize) {
            if self.tasks.len() > task_id {
                self.tasks.remove(task_id);
            }
        }

        /// # change_task
        /// This function changes a task state. If task is not completed it will be set to completed, if not it will be set not completed.
        pub fn change_task(&mut self, task_id: usize) {
            match self.tasks.get_mut(task_id) {
                Some(t) => {
                    t.state = !t.state;
                    if t.state {
                        self.points += t.reward
                    }
                }
                None => return,
            }
        }

        /// # list_tasks
        /// Returns all tasks
        pub fn list_tasks(&self) -> Vec<Task> {
            self.tasks.clone()
        }

        // Self:
        #[staticmethod]
        pub fn new() -> Self {
            Self {
                tasks: vec![],
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
        fn new(name: &str, reward: usize) -> Self {
            Self {
                name: name.to_string(),
                state: false,
                reward,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::pmguilib::{Task, UserData};

        #[test]
        fn add_task_test() {
            let mut data = UserData::new();
            data.add_task("Task", 5);

            assert_eq!(data.tasks, vec![Task::new("Task", 5)]);
        }

        #[test]
        fn remove_task_test() {
            let mut data = UserData::new();
            data.add_task("Task", 5);
            data.remove_task(0);

            assert_eq!(data.tasks, vec![]);
        }

        #[test]
        fn change_task_test() {
            let mut data = UserData::new();
            data.add_task("Task", 5);
            data.change_task(0);

            assert_eq!(
                data.tasks,
                vec![Task {
                    name: "Task".to_string(),
                    state: true,
                    reward: 5
                }]
            );

            assert_eq!(data.points, 5);
        }
    }
}
