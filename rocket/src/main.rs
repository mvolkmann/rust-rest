#[macro_use]
extern crate rocket;

use parking_lot::RwLock;
//use rocket::config::{Config, Environment};
//use rocket::response::status::Created;
//use rocket::fairing::AdHoc;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" function.
#[derive(Clone, Deserialize, Serialize, Debug)]
struct Dog {
    id: String,
    breed: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewDog {
    breed: String,
    name: String,
}

struct MyState {
    dog_map: Arc<RwLock<HashMap<String, Dog>>>,
}

#[rocket::main]
async fn main() {
    let mut dog_map: HashMap<String, Dog> = HashMap::new();

    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    dog_map.insert(id, dog);

    let state = MyState {
        dog_map: Arc::new(RwLock::new(dog_map)),
    };

    #[post("/", format = "json", data = "<json>")]
    //fn create_dog(json: Json<NewDog>, state: State<MyState>) -> Created<Json> {
    fn create_dog(json: Json<NewDog>, state: State<MyState>) -> Json<Dog> {
        let new_dog = json.into_inner();
        let id = Uuid::new_v4().to_string();
        let dog = Dog {
            id: id.clone(),
            name: new_dog.name,
            breed: new_dog.breed,
        };
        let mut dog_map = state.dog_map.write();
        dog_map.insert(id, dog.clone());

        //let url = format!("http://localhost:1234/dog/{}", id);
        //Created(url, Some(Json(dog)))
        Json(dog)
    }

    #[delete("/<id>")]
    fn delete_dog(id: String, state: State<MyState>) {
        let mut dog_map = state.dog_map.write();
        dog_map.remove(&id);
    }

    #[get("/<id>", format = "json")]
    fn get_dog(id: String, state: State<MyState>) -> Option<Json<Dog>> {
        let dog_map = state.dog_map.read();
        if let Some(dog) = dog_map.get(&id) {
            Some(Json(dog.clone()))
        } else {
            None
        }
    }

    #[get("/")]
    fn get_dogs(state: State<MyState>) -> Json<Vec<Dog>> {
        let dog_map = state.dog_map.read();
        let dogs = dog_map.values().cloned().collect();
        Json(dogs)
    }

    #[put("/<id>", format = "json", data = "<json>")]
    fn update_dog(id: String, json: Json<Dog>, state: State<MyState>) -> Json<Dog> {
        let dog: Dog = json.into_inner();
        let mut dog_map = state.dog_map.write();
        dog_map.insert(id, dog.clone());
        Json(dog)
    }

    //TODO: Learn how to get this to use TLS/HTTPS.
    // Note that https://rocket.rs/v0.4/guide/configuration/ says
    // "Warning: Rocket's built-in TLS is not considered ready for
    // production use. It is intended for development use only."
    rocket::ignite()
        /*
        .attach(AdHoc::on_attach(|rocket| {
            //TODO: Can you use this to get the active host and port
            //TODO: for setting url in create_dog?
            let config = rocket.config();
            dbg!(config);
        }))
        */
        .manage(state)
        .mount(
            "/dog",
            routes![create_dog, delete_dog, get_dog, get_dogs, update_dog],
        )
        .launch()
        .await
        .expect("failed to start rocket");
}
