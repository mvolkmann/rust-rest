use async_std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use tide::prelude::*;
use tide::{Body, Request, Response, StatusCode};
use uuid::Uuid;

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" route.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    #[serde(default)]
    id: Option<String>,
    breed: String,
    name: String,
}

type DogMap = HashMap<String, Dog>;

#[derive(Clone)]
struct State {
    dog_map: Arc<RwLock<DogMap>>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut dog_map: HashMap<String, Dog> = HashMap::new();

    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: Some(id.clone()),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    dog_map.insert(id, dog);

    let state = State {
        dog_map: Arc::new(RwLock::new(dog_map)),
    };
    let mut app = tide::with_state(state);

    app.at("/dog")
        // Get all dogs.
        .get(|req: Request<State>| async move {
            let dog_map = req.state().dog_map.read();
            let dogs: Vec<Dog> = dog_map.values().cloned().collect();
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&dogs)?);
            Ok(res)
        })
        // Create a dog.
        .post(|mut req: Request<State>| async move {
            let mut dog: Dog = req.body_json().await?;
            let id = Uuid::new_v4().to_string();
            dog.id = Some(id.clone());
            let mut dog_map = req.state().dog_map.write();
            dog_map.insert(id, dog.clone());
            let mut res = tide::Response::new(StatusCode::Created);
            res.set_body(Body::from_json(&dog)?);
            Ok(res)
        });

    app.at("/dog/:id")
        // Get a specific dog.
        .get(|req: Request<State>| async move {
            let id = req.param("id")?;
            let dog_map = req.state().dog_map.read();
            if let Some(dog) = dog_map.get(id.clone()) {
                let mut res = Response::new(StatusCode::Ok);
                res.set_body(Body::from_json(&dog)?);
                Ok(res)
            } else {
                Ok(Response::new(StatusCode::NotFound))
            }
        })
        // Update a dog.
        .put(|mut req: Request<State>| async move {
            let dog: Dog = req.body_json().await?;
            let id = req.param("id")?;
            let mut dog_map = req.state().dog_map.write();
            if let Some(_dog) = dog_map.get(id) {
                dog_map.insert(id.to_string(), dog.clone());
                let mut res = tide::Response::new(StatusCode::Ok);
                res.set_body(Body::from_json(&dog)?);
                Ok(res)
            } else {
                Ok(Response::new(StatusCode::NotFound))
            }
        })
        // Delete a dog.
        .delete(|req: Request<State>| async move {
            let id = req.param("id")?;
            let mut dog_map = req.state().dog_map.write();
            if let Some(_dog) = dog_map.remove(id) {
                Ok(Response::new(StatusCode::Ok))
            } else {
                Ok(Response::new(StatusCode::NotFound))
            }
        });

    app.listen("127.0.0.1:1234").await?;
    Ok(())
}
