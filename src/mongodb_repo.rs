use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::teachers_model::Teacher;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = "mongodb+srv://ArquiSoft:ArquiSoft@cluster0.3gasdka.mongodb.net/?retryWrites=true&w=majority";
        // let uri = match env::var("MONGOURI") {
        //     Ok(v) => v.to_string(),
        //     Err(_) => format!("Error loading env variable"),
        // };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("ArquiSoft");
        let col: Collection<User> = db.collection("TeachersCol");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}
