use reqwasm::{http::Request, Error};

use crate::models::Task;

const BASE_URL: &str = "localhost:8080";

pub async fn fetch_all() -> Result<Vec<Task>, Error> {
    Request::get(&format!("{BASE_URL}/tasks/"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}
