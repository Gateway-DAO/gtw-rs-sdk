use dotenv::dotenv;
use gtw_rs_sdk::{models::DtoDataModelCreateRequest, GtwSDK};
use serde_json::{json, Map};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not set");
    let gtw_sdk = GtwSDK::new(Some(bearer_token)).await?;

    // Retrieve all data models
    match gtw_sdk.data_model.get_all(None, None).await {
        Ok(paginated_response) => {
            println!("Data Models Retrieved: {:?}", paginated_response.links);
        }
        Err(e) => {
            eprintln!("Failed to retrieve data models: {}", e);
        }
    }

    let example_value = DtoDataModelCreateRequest {
        title: String::from("Personal Data Model"),
        description: String::from(
            "A data model for storing personal information including name and age.",
        ),
        schema: {
            let mut schema_map = Map::new();
            schema_map.insert("type".to_string(), json!("object"));
            schema_map.insert(
                "properties".to_string(),
                json!({
                    "firstName": {
                        "type": "string",
                        "title": "First Name",
                        "description": "The person's first name.",
                    },
                    "lastName": {
                        "type": "string",
                        "title": "Last Name",
                        "description": "The person's last name.",
                    },
                    "age": {
                        "type": "number",
                        "title": "Age",
                        "minimum": 0,
                        "description": "The person's age in years."
                    }
                }),
            );
            schema_map
        },
        tags: vec![String::from("Personal"), String::from("Data Model")],
    };

    // Create a new data model

    // match gtw_sdk.data_model.create(example_value).await {
    //     Ok(response) => {
    //         println!("Data Model Created: {:?}", response);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to create data model: {}", e);
    //     }
    // }

    // // Update an existing data model (replace `data_model_id` with a valid ID)
    // let data_model_id: u64 = 1; // Replace with the actual ID you want to update
    // let updated_data_model = DtoDataModelUpdateRequest {
    //     description: Some("Updated description".to_string()),
    //     id: data_model_id,
    //     schema: json!({ "type": "object", "properties": { "updated": { "type": "string" } } }),
    //     tags: Some(vec!["updated".to_string()]),
    //     title: Some("Updated Data Model".to_string()),
    // };

    // match data_model_client
    //     .update(data_model_id, updated_data_model)
    //     .await
    // {
    //     Ok(response) => {
    //         println!("Data Model Updated: {:?}", response);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to update data model: {}", e);
    //     }
    // }

    // Get data models created by the user
    match gtw_sdk.data_model.get_my(Some(1), Some(10)).await {
        Ok(paginated_response) => {
            println!("User's Data Models: {:?}", paginated_response);
        }
        Err(e) => {
            eprintln!("Failed to retrieve user's data models: {}", e);
        }
    }

    Ok(())
}
