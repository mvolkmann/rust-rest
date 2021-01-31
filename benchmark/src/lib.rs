use serde::{Deserialize, Serialize};

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" route.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dog {
    pub id: String,
    pub breed: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDog {
    pub breed: String,
    pub name: String,
}
