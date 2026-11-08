#[cfg(test)]
use crate::pmguilib::{Item, Task, UserData};

// Tasks:
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

    assert!(!data.remove_task("Not a task"));
    assert!(data.remove_task("Task"));

    assert_eq!(data.tasks, vec![]);
}

#[test]
fn change_task_test() {
    let mut data = UserData::new();
    data.add_task("Task", 5);
    assert!(!data.change_task("Not a task"));
    assert!(data.change_task("Task"));

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

// Shop:
#[test]
fn add_item_test() {
    let mut data = UserData::new();
    data.add_item("Item", 5);

    assert_eq!(data.shop, vec![Item::new("Item", 5)]);
}

#[test]
fn remove_item_test() {
    let mut data = UserData::new();
    data.add_item("Item", 5);

    assert!(!data.remove_item("Not an item"));
    assert!(data.remove_item("Item"));

    assert_eq!(data.shop, vec![]);
}

#[test]
fn buy_item_test() {
    let mut data = UserData::new();
    data.add_item("Item", 5);
    data.points = 10;

    assert!(!data.buy_item("Not an item"));
    assert!(data.buy_item("Item"));

    assert_eq!(
        data.shop,
        vec![Item {
            name: "Item".to_string(),
            price: 5
        }]
    );

    assert_eq!(data.points, 5);
}
