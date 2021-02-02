use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, Reply};
use warp::{Filter, Rejection};

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" route.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    id: String,
    breed: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NewDog {
    breed: String,
    name: String,
}

type DogMap = HashMap<String, Dog>;

type State = Arc<RwLock<DogMap>>;

#[tokio::main]
async fn main() {
    // Add one dog for testing.
    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    let mut dog_map = HashMap::new();
    dog_map.insert(id, dog);

    let state: State = Arc::new(RwLock::new(dog_map));

    fn with_state(state: State) -> impl Filter<Extract = (State,), Error = Infallible> + Clone {
        warp::any().map(move || state.clone())
    }

    let get_dogs = warp::path!("dog")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handle_get_dogs);

    // In routes that cannot return an Err,
    // the compiler cannot infer the error type for the Result.
    // This must be an async fn instead of a closure passed to and_then
    // until proper support for async closures is added to Rust.
    async fn handle_get_dogs(state: State) -> Result<Json, Rejection> {
        let dog_map = state.read();
        let dogs: Vec<Dog> = dog_map.values().cloned().collect();
        Ok(warp::reply::json(&dogs))
    }

    let get_dog = warp::path!("dog" / String)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(|id, state: State| async move {
            let dog_map = state.read();
            if let Some(dog) = dog_map.get(&id) {
                Ok(warp::reply::json(&dog))
            } else {
                Err(warp::reject::not_found())
            }
        })
        .recover(not_found);

    async fn not_found(_err: Rejection) -> Result<impl warp::Reply, Rejection> {
        Ok(StatusCode::NOT_FOUND)
    }

    let create_dog = warp::path!("dog")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_create_dog);

    // See the comment above the handle_get_dogs function.
    async fn handle_create_dog(new_dog: NewDog, state: State) -> Result<impl Reply, Rejection> {
        let id = Uuid::new_v4().to_string();
        let dog = Dog {
            id: id.clone(),
            name: new_dog.name,
            breed: new_dog.breed,
        };
        let mut dog_map = state.write();
        dog_map.insert(id, dog.clone());
        Ok(with_status(json(&dog), StatusCode::CREATED))
    }

    let update_dog = warp::path!("dog" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(|id: String, dog: Dog, state: State| async move {
            let mut dog_map = state.write();
            if let Some(_dog) = &dog_map.get(&id) {
                dog_map.insert(id, dog.clone());
                Ok(warp::reply::json(&dog))
            } else {
                Err(warp::reject::not_found())
            }
        });

    let delete_dog = warp::path!("dog" / String)
        .and(warp::delete())
        .and(with_state(state.clone()))
        .and_then(|id: String, state: State| async move {
            let mut dog_map = state.write();
            if let Some(_dog) = dog_map.remove(&id) {
                Ok(with_status("", StatusCode::NO_CONTENT))
            } else {
                Err(warp::reject::not_found())
            }
        });

    //TODO: Learn how to get this to use TLS/HTTPS.
    let routes = get_dogs
        .or(get_dog)
        .or(create_dog)
        .or(update_dog)
        .or(delete_dog);
    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await;
}
