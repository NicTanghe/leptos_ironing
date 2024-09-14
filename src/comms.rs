
use reqwest::Error;

use crate::tasks::Task;




pub async fn get_tasks_from_api() -> Result<Vec<Task>, Error> {
    let url = "http://localhost:3030/tasks";
    log::info!("Sending request to {}", url);

    let response = match reqwest::get(url).await {
        Ok(resp) => resp,
        Err(err) => {
            log::error!("Failed to fetch tasks: {}", err);
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


    //// add later case matching for 404 ects.


    if response.status().is_success() {
        let tasks: Vec<Task> = response.json().await?;
        Ok(tasks)
    } else {
        log::error!("Request failed with status: {}", response.status());
        // Return default tasks on HTTP error
        Ok(vec![
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
        ])
    }
}

