use leptos::*; // Assuming you're using the Leptos framework
use crate::tasks::tasks_p1::Task; // Assuming `Task` is defined in your main or another module


use crate::comms::get_tasks_from_api;

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

// Add any other task-related functions here
