use leptos::*;
use leptos_router::*;

use crate::contacts::*;
use crate::tasks::*; // Assuming you have a tasks module for TaskList and TaskInfo


use crate::comms::get_tasks_from_api;
use crate::tasks::Task; // Assuming you updated comms.rs as shown above



/// Function to create the contact list signal
pub fn create_contact_signal() -> (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>){
    create_signal(vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Steve".to_string(),
        "Diana".to_string(),
        "Eve".to_string(),
        "Fred".to_string(),
        "Camille".to_string(),
        "Tron".to_string()
    ])
}

/// Function to create the task list signal

pub fn create_task_signal() -> (ReadSignal<Vec<Task>>, WriteSignal<Vec<Task>>) {
    let (tasks, set_tasks) = create_signal(vec![]);  // Initially empty vector

    // Fetch the tasks asynchronously
    spawn_local(async move {
        match get_tasks_from_api().await {
            Ok(fetched_tasks) => {
                // Update the signal with fetched tasks
                set_tasks(fetched_tasks);
            }
            Err(err) => {
                log::error!("Error fetching tasks: {:?}", err);
            }
        }
    });

    (tasks, set_tasks)
}




async fn get_task_vector(value: Vec<Task>) -> Vec<Task> {
    match get_tasks_from_api().await {
        Ok(fetched_tasks) => {
            logging::log!("Fetched tasks:\n{:#?}", fetched_tasks);  // Log fetched tasks properly
            fetched_tasks
        }
        Err(err) => {
            logging::log!("Error fetching tasks: {:?}", err);
            value // Returns old vector incase of error

        }
    }
}



#[component]
pub fn App() -> impl IntoView {
 

    let (contacts, _set_contacts) = create_contact_signal();
    let (tasks, set_tasks) = create_signal(vec![
        Task {
            name: "server not talked to".to_string(),
            priority: 0,
            task_id: 0 as i32,
        },
    ]);

    // Create a resource that fetches tasks from the API
    let async_data = create_resource(
        move || (),  // Pass an empty tuple as a dependency to ensure it runs once
        move |_| async move {
            logging::log!("RESOURCE: loading data from API");
            get_task_vector(tasks.get()).await
        },
    );

    // Update the tasks signal when data is loaded
    create_effect(move |_| {
        if let Some(fetched_tasks) = async_data.get() {
            set_tasks(fetched_tasks);
        }
    });


    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/contacts">"Contacts"</A>
                <A href="/tasks">"Tasks"</A>   // Added task navigation
            </nav>
            <Routes>
                <Route path="/" view=HomePage/>
                
                {/* Contacts Routes */}
                <Route path="/contacts" view=move || view!{ <ContactList contacts />}>
                    <Route path="" view=|| view! {
                        <p>"Select a contact to view more info."</p>
                    }/>
                    <Route path=":id" view=move || view!{ <ContactInfo contacts />}>
                        <Route path="" view=|| view! {
                            <div class="tab">"Contact Info"</div>
                        }/>
                        <Route path="conversations" view=|| view! {
                            <div class="tab">"Conversations"</div>
                        }/>
                    </Route>
                </Route>

                {/* Tasks Routes */}
                <Route path="/tasks" view=move || view!{ <TaskList tasks />}>   {/* Added TaskList component */}
                    <Route path="" view=|| view! {
                        <p>"Select a task to view more info."</p>
                    }/>
                    <Route path=":id" view=move || view!{ <TaskInfo tasks />}>   {/* Added TaskInfo component */}
                        <Route path="" view=|| view! {
                            <div class="tab">"Task Info"</div>
                        }/>
                        <Route path="conversations" view=|| view! {
                            <div class="tab">"Task Conversations"</div>  {/* Task-specific tab */}
                        }/>
                    </Route>
                </Route>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
