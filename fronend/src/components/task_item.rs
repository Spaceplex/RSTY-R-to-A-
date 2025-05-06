use yew::prelude::*;

use crate::models::Task;

#[derive(Properties, PartialEq)]
pub struct TaskItemProps {
    pub task: Task,
}

#[function_component(TaskItem)]
fn task() -> Html {
    html!()
}
