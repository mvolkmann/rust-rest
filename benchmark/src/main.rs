// This is for benchmarking REST service implementations.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

const BASE_URL: &str = "http://localhost:1234/dog";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    id: String,
    breed: String,
    name: String,
}

async fn delete_all_dogs(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let res = client.get(BASE_URL).send().await?;
    let dogs = res.json::<Vec<Dog>>().await?;
    for dog in dogs {
        let url = format!("{}/{}", BASE_URL, dog.id);
        client.delete(&url).send().await?.text().await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count: usize = 10000;
    //let count: usize = 3000; // 2500 works in Flask, 3000 gives "operation timed out"
    let client = reqwest::Client::new();

    // Delete all the current dogs to start with an empty collection.
    delete_all_dogs(&client).await?;

    let start = Instant::now();

    // Create new dogs.
    for i in 0..count {
        let id = Uuid::new_v4().to_string();
        let dog = Dog {
            id,
            name: format!("name-{}", i),
            breed: format!("breed-{}", i),
        };
        client.post(BASE_URL).json(&dog).send().await?;
    }
    let res = client.get(BASE_URL).send().await?;
    let dogs = res.json::<Vec<Dog>>().await?;
    assert_eq!(dogs.len(), count);

    // Update all the dogs.
    for dog in dogs {
        let id = dog.id.clone();
        let new_dog = Dog {
            id: id.clone(),
            name: format!("new-{}", dog.name),
            breed: format!("new-{}", dog.breed),
        };
        let url = format!("{}/{}", BASE_URL, id);
        client.put(&url).json(&new_dog).send().await?;
    }

    delete_all_dogs(&client).await?;

    println!("elapsed time: {:?}", start.elapsed());

    Ok(())
}
