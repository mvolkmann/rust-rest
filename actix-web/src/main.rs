use actix_web::{delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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

struct AppState {
    dog_map: DogMap,
}

#[get("/dog")]
async fn get_dogs(state: web::Data<RwLock<AppState>>) -> Result<HttpResponse> {
    let state = state.read();
    let dogs: Vec<Dog> = state.dog_map.values().cloned().collect();
    Ok(HttpResponse::Ok().json(dogs))
}

#[get("/dog/{id}")]
async fn get_dog(req: HttpRequest, state: web::Data<RwLock<AppState>>) -> Result<HttpResponse> {
    let id = req.match_info().get("id").unwrap();
    let state = state.read();
    if let Some(dog) = state.dog_map.get(id) {
        Ok(HttpResponse::Ok().json(dog))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[post("/dog")]
async fn create_dog(
    json: web::Json<NewDog>,
    state: web::Data<RwLock<AppState>>,
) -> Result<HttpResponse> {
    let id = Uuid::new_v4().to_string();
    let new_dog = json.into_inner();
    let dog = Dog {
        id: id.clone(),
        name: new_dog.name,
        breed: new_dog.breed,
    };

    let mut state = state.write();
    state.dog_map.insert(id, dog.clone());
    Ok(HttpResponse::Created().json(dog))
}

#[put("/dog/{id}")]
async fn update_dog(
    json: web::Json<Dog>,
    state: web::Data<RwLock<AppState>>,
) -> Result<HttpResponse> {
    let dog = json.into_inner();
    let id = dog.id.clone();
    //println!("updating dog with id {}", id);
    let mut state = state.write();
    state.dog_map.insert(id, dog.clone());
    Ok(HttpResponse::Ok().json(dog))
}

#[delete("/dog/{id}")]
async fn delete_dog(req: HttpRequest, state: web::Data<RwLock<AppState>>) -> Result<HttpResponse> {
    let id = req.match_info().get("id").unwrap();
    //println!("deleting dog with id {}", id);
    let mut state = state.write();
    if let Some(_dog) = state.dog_map.remove(id) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut dog_map: HashMap<String, Dog> = HashMap::new();

    // Start with one dog already created.
    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    dog_map.insert(id, dog);

    let data = web::Data::new(RwLock::new(AppState { dog_map }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_dog)
            .service(get_dogs)
            .service(delete_dog)
            .service(create_dog)
            .service(update_dog)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
