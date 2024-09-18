use leptos::*; // Assuming you're using the Leptos framework
use leptos_router::Route;
use crate::tasks::tasks_p1::*; // Assuming `Task` is defined in your main or another module
use crate::comms::get_tasks_from_api;


use std::rc::Rc;
// Create task signals
pub fn create_task_signal() -> (ReadSignal<Vec<Task>>, WriteSignal<Vec<Task>>) {
    let (tasks, set_tasks) = create_signal(vec![]);

    spawn_local(async move {
        match get_tasks_from_api().await {
            Ok(fetched_tasks) => {
                set_tasks(fetched_tasks);
            }
            Err(err) => {
                log::error!("Error fetching tasks: {:?}", err);
            }
        }
    });

    (tasks, set_tasks)
}

// Fetch task vector
pub async fn get_task_vector(value: Vec<Task>) -> Vec<Task> {
    match get_tasks_from_api().await {
        Ok(fetched_tasks) => {
            logging::log!("Fetched tasks:\n{:#?}", fetched_tasks);
            fetched_tasks
        }
        Err(err) => {
            logging::log!("Error fetching tasks: {:?}", err);
            value // Returns old vector in case of error
        }
    }
}

// Define task routes
pub fn task_routes(
    tasks: ReadSignal<Vec<Task>>, 
    set_tasks: WriteSignal<Vec<Task>>
) -> impl IntoView {

    // Handle delete action
    let handle_task_delete = move |id: i32| {
        delete_task_by_id(id, set_tasks);  // Ensure `set_tasks` has a proper lifetime
    };

    // Define the view
    view! {
        <Route path="/tasks" view=move || view! { <TaskList tasks={tasks} /> }>   {/* Added TaskList component */}
            <Route path="" view=|| view! {
                <p>"Select a task to view more info."</p>
            }/>
            <Route path=":id" view=move || view! { <TaskInfo tasks={tasks} /> }>   {/* Added TaskInfo component */}
                <Route path="" view=|| view! {
                    <div class="tab">"Task Info"</div>
                    <button on:click=move |_| handle_task_delete(1)> // Pass a task id, e.g., 1
                        "Delete Task 1"
                    </button>
                }/>
                <Route path="conversations" view=|| view! {
                    <div class="tab">"Task Conversations"</div>  {/* Task-specific tab */}
                }/>
            </Route>
        </Route>
    }
}
