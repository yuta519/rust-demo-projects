use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Not found: id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type ToDoDataset = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct InMemoryTodoRepository {
    store: Arc<RwLock<ToDoDataset>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }
    fn write_store_ref(&self) -> RwLockWriteGuard<ToDoDataset> {
        self.store.write().unwrap()
    }
    fn read_store_ref(&self) -> RwLockReadGuard<ToDoDataset> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for InMemoryTodoRepository {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.store.write().unwrap();
        let id = store.len() as i32 + 1;
        let todo = Todo::new(id, payload.text);
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.store.read().unwrap();
        store.get(&id).cloned()
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.store.read().unwrap();
        store.values().cloned().collect()
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.store.write().unwrap();
        let todo = store.get_mut(&id).ok_or(RepositoryError::NotFound(id))?;
        if let Some(text) = payload.text {
            todo.text = text;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }
        Ok(todo.clone())
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.store.write().unwrap();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}
