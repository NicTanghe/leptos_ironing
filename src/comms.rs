use reqwest::Error;
use crate::tasks::Task;
use leptos::logging;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Task>,
  //  success: bool,
}

pub async fn get_tasks_from_api() -> Result<Vec<Task>, Error> {
    let url = "http://localhost:3030/tasks";
    logging::log!("Sending request to {}", url);

    // Send request
    let response = match reqwest::get(url).await {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to fetch tasks: {}", err);
            // Return default tasks on error
            return Ok(vec![
                Task {
                    name: "ERROR!".to_string(),
                    priority: 1, 
                    task_id: 0,
                },
                Task {
                    name: "Something went wrong".to_string(),
                    priority: 1,
                    task_id: 0,
                }
            ]);
        }
    };

    // Check if the response status is OK
    if !response.status().is_success() {
        logging::log!("Request failed with status: {}", response.status());
        // Return default tasks on HTTP error
        return Ok(vec![
            Task {
                name: "HTTP Error".to_string(),
                priority: 0,
                task_id: response.status().as_u16() as i32
            },
            Task {
                name: "Request failed".to_string(),
                priority: 0,
                task_id: response.status().as_u16() as i32
            }
        ]);
    }

    // Deserialize the response body into ApiResponse
    let api_response: ApiResponse = match response.json().await {
        Ok(data) => data,
        Err(err) => {
            logging::log!("Failed to deserialize response: {}", err);
            // Return default tasks on deserialization error
            return Ok(vec![
                Task {
                    name: "Deserialization Error".to_string(),
                    priority: 0,
                    task_id: 0,
                }
            ]);
        }
    };

    // Log success status
    logging::log!("Successfully received response from API.");
    logging::log!("COMMS: \n Loaded data from API: {:?}", api_response.data);

    // Return the tasks from the data field
    Ok(api_response.data)
}
