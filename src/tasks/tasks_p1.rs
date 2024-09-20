use leptos::*;
use leptos_router::*;
use serde::{Serialize,Deserialize};

// Task struct from your API
#[derive(PartialEq, Clone,Serialize, Deserialize, Debug)] // Ensure Task is Cloneable
pub struct Task {
    pub name: String,
    pub priority: i8,
    pub task_id: i32,
}

#[component]
pub fn TaskList(tasks: ReadSignal<Vec<Task>>) -> impl IntoView {
    view! {
        <div class="task-list">
            <h3>"Tasks"</h3>
            <div class="task-list-tasks">
                {move || tasks.get().into_iter().map(|task| {
                    // Use the name from the Task struct for href and display
                    let href = format!("/tasks/{}", task.name.to_lowercase());
                    view! {
                        <A href={href}>{&task.name}</A>
                    }
                }).collect_view()}
            </div>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn TaskInfo(tasks: ReadSignal<Vec<Task>>) -> impl IntoView {
    let params = use_params_map();
    let id = create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));

    // Reactively find the task with the matching name (id)
    let task_info = move || {
        let lowercase_id = id().to_lowercase();
        tasks.get().iter()
            .find(|task| task.name.to_lowercase() == lowercase_id)
            .cloned()
    };

    view! {
        <h4 key={id()}>
            {
                move || {
                    match task_info() {
                        Some(task) => format!("TASK: {}", task.name),  // Use Task's name
                        None => "Task not found.".to_string(),
                    }
                }
            }
        </h4>
        <div class="task-info">
            <div class="tabs">
                <A href="" exact=true>"Task Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>
            <Outlet/>
        </div>
    }
}


// Function to delete a task by ID
pub fn delete_task_resource_by_id(id: i32, set_tasks: WriteSignal<Vec<Task>>) {
    set_tasks.update(|tasks| {
        *tasks = tasks.iter()
                      .cloned() // Clone each task
                      .filter(|task| task.task_id != id) // Filter out the task with the given ID
                      .collect(); // Collect the remaining tasks
    });

}
