use leptos::*; // Assuming you're using the Leptos framework
use leptos_router::{Route,use_params_map};
use crate::tasks::tasks_p1::*; // Assuming `Task` is defined in your main or another module
use crate::comms::*;


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
//            logging::log!("Fetched tasks:\n{:#?}", fetched_tasks);
            fetched_tasks
        }
        Err(err) => {
            logging::log!("Error fetching tasks: {:?}", err);
            value // Returns old vector in case of error
        }
    }
}



fn handle_task_delete(id: i32, set_tasks: WriteSignal<Vec<Task>>) {
    // First delete from the resource synchronously
    //

    delete_task_resource_by_id(id, set_tasks);

    // Spawn the async API call in the background

    spawn_local(async move {
        match delete_task_from_api(id).await {

            //this error handeling is not working for shiz but the function seems to work.
            Ok(_) => {
                // Handle success
             logging::log!("Successfully deleted task from API with id: {}", id);                   
            }
            Err(e) => {
                // Handle error
            logging::log!("Failed to delete task from API: {}", e);
            }   
        }
    });
}


pub fn task_routes(
    tasks: ReadSignal<Vec<Task>>, 
    set_tasks: WriteSignal<Vec<Task>>
) -> impl IntoView {
    // Define the view
    view! {
        <Route path="/tasks" view=move || view! { <TaskList tasks={tasks} /> }>   
            <Route path="" view=|| view! {
                <p>"Select a task to view more info."</p>
            }/>
            <Route path=":id" view=move || {
                // Get the current task ID from route parameters
                let params = use_params_map();

                // Get the current task ID as a string
                let current_id = params.with(|params| {
                    params.get("id")
                        .map_or_else(|| String::from(""), |s| s.clone()) // Return a cloned string or an empty string
                });
                
                //this one doesnt print but it clones the other one doesnt clone but prints
                let id = create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));
                

                // Reactively find the task with the matching name (id)
                let task_info = move || {
                    let lowercase_id = id().to_lowercase();
                    tasks.get().iter()
                        .find(|task| task.name.to_lowercase() == lowercase_id)
                        .cloned()
                        .or_else(|| Some(Task {
                            name: "Request failed".to_string(),
                            priority: 0,
                            task_id: 505 as i32
                        }))
                 };



                view! {
                    <TaskInfo tasks={tasks} />
                    <div class="tab">"Task Info"</div>
                    <p>"Current Task ID: "{current_id}</p> // Print the current route ID (string)
                    <p>{
                        move || {
                            match task_info() {
                                Some(task) => format!("TASK: {}", task.task_id),  // Use Task's name
                                None => "Task not found.".to_string(),
                            }
                        }
                        }
                    </p> 
                    <button on:click=move |_| handle_task_delete(
                                {
                    // Immediately call the closure to get the task_id or 404
                        let task_id = move || {
                            match task_info() {
                                Some(task) => task.task_id,
                                None => 404,
                            }
                        };
                        task_id() // Call the closure to return the actual task ID
                    },
                        set_tasks)> // Pass the actual task_id to delete
                        "Delete Task"
                    </button>
                    <Route path="conversations" view=|| view! {
                        <div class="tab">"Task Conversations"</div>  
                    }/>
                }
            }/>
        </Route>
    }
}







