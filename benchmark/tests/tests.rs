use benchmark::{Dog, NewDog};
use reqwest::Client;

const BASE_URL: &str = "http://localhost:1234/dog";

async fn assert_dog_count(
    client: &Client,
    count: usize,
) -> Result<Vec<Dog>, Box<dyn std::error::Error>> {
    let dogs = get_all_dogs(client).await?;
    assert_eq!(dogs.len(), count);
    Ok(dogs)
}

async fn get_all_dogs(client: &Client) -> Result<Vec<Dog>, Box<dyn std::error::Error>> {
    let res = client.get(BASE_URL).send().await?;
    let dogs = res.json::<Vec<Dog>>().await?;
    Ok(dogs)
}

async fn delete_all_dogs(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let dogs = get_all_dogs(client).await?;
    for dog in dogs {
        let url = format!("{}/{}", BASE_URL, dog.id);
        client.delete(&url).send().await?.text().await?;
    }
    Ok(())
}

#[tokio::test]
async fn it_uses_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Delete all the current dogs to start with an empty collection.
    delete_all_dogs(&client).await?;
    assert_dog_count(&client, 0).await?;

    // Create Comet dog.
    let dog = NewDog {
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    let res = client.post(BASE_URL).json(&dog).send().await?;
    assert_eq!(res.status(), 201);
    let comet: Dog = res.json().await?;
    assert_eq!(comet.name, "Comet");
    assert_eq!(comet.breed, "Whippet");

    // Create Oscar dog.
    let dog = NewDog {
        name: "Oscar".to_string(),
        breed: "German Shorthaired Pointer".to_string(),
    };
    let res = client.post(BASE_URL).json(&dog).send().await?;
    assert_eq!(res.status(), 201);
    let mut oscar: Dog = res.json().await?;
    assert_eq!(oscar.name, "Oscar");
    assert_eq!(oscar.breed, "German Shorthaired Pointer");

    assert_dog_count(&client, 2).await?;

    // Delete Comet.
    let url = format!("{}/{}", BASE_URL, comet.id);
    let res = client.delete(&url).send().await?;
    assert_eq!(res.status(), 200);
    let text = res.text().await?;
    assert!(text == "" || text == "OK");
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 404);

    // Update Oscar.
    oscar.name = "Oscar Wilde".to_string();
    let url = format!("{}/{}", BASE_URL, oscar.id);
    let res = client.put(&url).json(&oscar).send().await?;
    assert_eq!(res.status(), 200);
    let dog: Dog = res.json().await?;
    assert_eq!(dog.name, "Oscar Wilde");
    assert_eq!(dog.breed, "German Shorthaired Pointer");

    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 200);
    let new_oscar: Dog = res.json().await?;
    assert_eq!(new_oscar.name, "Oscar Wilde");

    Ok(())
}
