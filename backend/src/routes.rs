use axum::{extract::Path, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{opt::PatchOp, RecordId};

use crate::DB;

type E = crate::error::Error;

const TASKS: &str = "tasks";

#[derive(Serialize, Deserialize)]
struct TaskData {
    title: String,
    completed: bool,
    created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: RecordId,
    title: String,
    completed: bool,
    created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskOut {
    id: String,
    title: String,
    completed: bool,
    created_at: Option<DateTime<Utc>>,
}

pub async fn create_task(title: Path<String>) -> Result<Json<Option<TaskOut>>, E> {
    let task: Option<Task> = DB
        .create(TASKS)
        .content(TaskData {
            title: title.to_string(),
            completed: false,
            created_at: Some(Utc::now()),
        })
        .await?;

    let task = task.map(|t| TaskOut {
        id: t.id.to_string(),
        completed: t.completed,
        title: t.title.clone(),
        created_at: t.created_at,
    });

    Ok(Json(task))
}

pub async fn get_task(id: Path<String>) -> Result<Json<Option<TaskOut>>, E> {
    let id = id.trim_start_matches("task:").to_string();
    let task: Option<Task> = DB.select((TASKS, &*id)).await?;
    let task = task.map(|t| TaskOut {
        id: t.id.to_string(),
        completed: t.completed,
        title: t.title.clone(),
        created_at: t.created_at,
    });

    Ok(Json(task))
}

pub async fn get_all_tasks() -> Result<Json<Vec<TaskOut>>, E> {
    let tasks: Vec<Task> = DB.select(TASKS).await?;
    let tasks = tasks
        .iter()
        .map(|t| TaskOut {
            id: t.id.to_string(),
            completed: t.completed,
            title: t.title.clone(),
            created_at: t.created_at,
        })
        .collect();

    Ok(Json(tasks))
}

#[derive(Deserialize, Serialize)]
pub struct AffectedRows {
    affected_rows: i32,
}

pub async fn update_task(id: Path<String>) -> Result<Json<AffectedRows>, E> {
    let id = id.trim_start_matches("task:").to_string();
    let t: Option<Task> = DB.select((TASKS, &*id)).await?;

    match t {
        Some(t) => {
            let _: Option<Task> = DB
                .update((TASKS, &*id))
                .patch(PatchOp::replace("/completed", !t.completed))
                .await?;

            Ok(Json(AffectedRows { affected_rows: 1 }))
        }
        None => Ok(Json(AffectedRows { affected_rows: 0 })),
    }
}

pub async fn delete_task(id: Path<String>) -> Result<Json<AffectedRows>, E> {
    let id = id.trim_start_matches("task:").to_string();
    let t: Option<Task> = DB.delete((TASKS, &*id)).await?;
    if t.is_some() {
        Ok(Json(AffectedRows { affected_rows: 1 }))
    } else {
        Ok(Json(AffectedRows { affected_rows: 0 }))
    }
}
