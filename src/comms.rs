use reqwest::Error;
use crate::tasks::Task;

use leptos::logging;

pub async fn get_tasks_from_api() -> Result<Vec<Task>, Error> {
    let url = "http://localhost:3030/tasks";
    logging::log!("Sending request to {}", url);

    let response = match reqwest::get(url).await {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to fetch tasks: {}", err);
            // Return default tasks on error
            return Ok(vec![
                Task {
                    name: "ERROR!".to_string(),
                    priority: 1,
                    status: 500,
                },
                Task {
                    name: "Something went wrong".to_string(),
                    priority: 1,
                    status: 500,
                }
            ]);
        }
    };

    // Log success status
    if response.status().is_success() {
        logging::log!("Successfully received response from API.");
        let tasks: Vec<Task> = response.json().await?;
        logging::log!("Loaded data from API: {:?}", tasks);  // Logging the fetched tasks
        return Ok(tasks);
    } else {
        logging::log!("Request failed with status: {}", response.status());
        // Return default tasks on HTTP error
        return Ok(vec![
            Task {
                name: "ERROR!".to_string(),
                priority: response.status().as_u16() as i32,
                status: response.status().as_u16() as i32,
            },
            Task {
                name: "Something went wrong".to_string(),
                priority: response.status().as_u16() as i32,
                status: response.status().as_u16() as i32,
            }
        ]);
    }
}
