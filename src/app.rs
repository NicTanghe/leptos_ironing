use leptos::*;
use leptos_router::*;

// use crate::contacts::*;


use crate::tasks::tasks_p1::*;
use crate::tasks::tasks_vieuw::*;



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





// note look at using a sagnal aswess for increased responsiveness

#[component]
pub fn App() -> impl IntoView {
 

 //   let (contacts, _set_contacts) = create_contact_signal();
    let (tasks, set_tasks) = create_signal(vec![
        Task {
            name: "server not talked to".to_string(),
            priority: 0,
            task_id: 0 as i32,
        },
    ]);

    // Create a resource that fetches tasks from the API
    let async_tasks = create_resource(
        move || (),  // Pass an empty tuple as a dependency to ensure it runs once
        move |_| async move {
           logging::log!("RESOURCE: loading data from API");
            get_task_vector(tasks.get()).await
        },
    );

    // Update the tasks signal when data is loaded
    create_effect(move |_| {
        if let Some(fetched_tasks) = async_tasks.get() {
            set_tasks(fetched_tasks);
        } 
    });

    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/tasks">"Tasks"</A>   // Added task navigation
            </nav>
            <Routes>

                    <Route path="/" view=HomePage/>
                   /// ContactList 


                {task_routes(tasks,set_tasks)} //there wher mistakes here that werent made by me
                
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
