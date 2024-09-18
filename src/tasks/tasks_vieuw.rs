use leptos::*; // Assuming you're using the Leptos framework
use leptos_router::Route;
use crate::tasks::tasks_p1::*; // Assuming `Task` is defined in your main or another module


use crate::comms::get_tasks_from_api;


//isn't used but probably better and more resposive if a signal is used for editing instead of a resource
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


//this is added in the app function to create a resource that is used to fill the task list. its
// for now its here and not in p1 becouse of when it was added maybe its better logically to put in p1 if the vieuw part gets more complex
//
// perhaps its better to have an init and an update and a vieuw whatever
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


pub fn task_routes(tasks: ReadSignal<Vec<Task>>) -> impl IntoView {
    view! {
        {/* Tasks Routes */}
        <Route path="/tasks" view=move || view! { <TaskList tasks={tasks} /> }>   {/* Added TaskList component */}
            <Route path="" view=|| view! {
                <p>"Select a task to view more info."</p>
            }/>
            <Route path=":id" view=move || view! { <TaskInfo tasks={tasks} /> }>   {/* Added TaskInfo component */}
                <Route path="" view=|| view! {
                    <div class="tab">"Task Info"</div>
                }/>
                <Route path="conversations" view=|| view! {
                    <div class="tab">"Task Conversations"</div>  {/* Task-specific tab */}
                }/>
            </Route>
        </Route>
    }
}
