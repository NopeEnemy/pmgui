#[cfg(test)]
use crate::pmguilib::{Item, PenaltyResult, Task, UserData};

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

    assert!(!data.remove_task(4));
    assert!(data.remove_task(0));

    assert_eq!(data.tasks, vec![]);
}

#[test]
fn change_task_test() {
    let mut data = UserData::new();
    data.add_task("Task", 5);
    assert!(!data.change_task(3));
    assert!(data.change_task(0));

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

    assert!(!data.remove_item(10));
    assert!(data.remove_item(0));

    assert_eq!(data.shop, vec![]);
}

#[test]
fn buy_item_test() {
    let mut data = UserData::new();
    data.add_item("Item", 5);
    data.points = 10;

    assert!(!data.buy_item(1));
    assert!(data.buy_item(0));

    assert_eq!(
        data.shop,
        vec![Item {
            name: "Item".to_string(),
            points: 5
        }]
    );

    assert_eq!(data.points, 5);
}

// Penalties:
#[test]
fn add_penalty_test() {
    let mut data = UserData::new();
    data.add_penalty("Penalty", 5);

    assert_eq!(data.penalties, vec![Item::new("Penalty", 5)]);
}

#[test]
fn remove_penalty_test() {
    let mut data = UserData::new();
    data.add_penalty("Penalty", 5);

    assert!(!data.remove_penalty(10));
    assert!(data.remove_penalty(0));

    assert_eq!(data.penalties, vec![]);
}

#[test]
fn penalty_test() {
    let mut data = UserData::new();
    data.add_penalty("Penalty", 5);
    data.points = 10;

    assert_eq!(data.penalty(1), PenaltyResult::NotFound);
    assert_eq!(data.penalty(0), PenaltyResult::Ok);

    assert_eq!(
        data.penalties,
        vec![Item {
            name: "Penalty".to_string(),
            points: 5
        }]
    );

    assert_eq!(data.points, 5);
}
