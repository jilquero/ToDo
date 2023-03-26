use std::ops::Deref;

use common::model::{
    task::{Task, TaskState},
    user::User,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;
use yewdux::prelude::*;

#[derive(Clone, Serialize, Deserialize, Store, PartialEq)]
#[store(storage = "session", storage_tab_sync)]
pub struct Store {
    pub user: Option<User>,
    pub tasks: Vec<Task>,
    pub filter: Filter,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            user: None,
            tasks: vec![],
            filter: Filter::All,
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    NotStarted,
    InProgress,
    Completed,
}

impl Filter {
    pub fn fits(&self, task: &Task) -> bool {
        match *self {
            Filter::All => true,
            Filter::NotStarted => task.state == TaskState::NotStarted,
            Filter::InProgress => task.state == TaskState::InProgress,
            Filter::Completed => task.state == TaskState::Completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::NotStarted => "#/not-started",
            Filter::InProgress => "#/in-progres",
            Filter::Completed => "#/completed",
        }
    }
}

pub fn register_user(dispatch: Dispatch<Store>, user: User) {
    dispatch.reduce(move |store| {
        let mut store = store.deref().clone();
        store.user = Some(user);
        store.tasks = vec![];
        store.into()
    });
}

pub fn login_user(dispatch: Dispatch<Store>, user: User) {
    dispatch.reduce(move |store| {
        let mut store = store.deref().clone();
        store.user = Some(user);
        store.into()
    });
}

pub fn update_user(dispatch: Dispatch<Store>, user: User) {
    dispatch.reduce(move |store| {
        let mut store = store.deref().clone();
        store.user = Some(user);
        store.into()
    });
}

pub fn logout_user(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.user = None;
        store.tasks = vec![];
        store.filter = Filter::All;
    })
}

pub fn set_tasks(dispatch: Dispatch<Store>, tasks: Vec<Task>) {
    dispatch.reduce_mut(move |store| {
        store.tasks = tasks;
    })
}

pub fn add_task(dispatch: Dispatch<Store>, task: Task) {
    dispatch.reduce_mut(move |store| {
        store.tasks.push(task);
    })
}

pub fn edit_task(dispatch: Dispatch<Store>, task: Task) {
    dispatch.reduce_mut(move |store| {
        store
            .tasks
            .iter_mut()
            .find(|t| t.task_uuid == task.task_uuid)
            .map(|t| *t = task.clone());
    })
}

pub fn remove_task(dispatch: Dispatch<Store>, uuid: Uuid) {
    dispatch.reduce_mut(move |store| {
        store.tasks.retain(|t| t.task_uuid != uuid);
    })
}

pub fn set_filter(dispatch: Dispatch<Store>, filter: Filter) {
    dispatch.reduce_mut(move |store| {
        store.filter = filter;
    })
}
