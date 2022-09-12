mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::teacher_api::{create_teacher, get_teacher, update_teacher, delete_teacher, get_teacher_by_id, get_student}; //import the handler here
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_teacher])
        .mount("/", routes![get_teacher])
        .mount("/", routes![update_teacher])
        .mount("/", routes![delete_teacher])
        .mount("/", routes![get_teacher_by_id])
        .mount("/", routes![get_student])
}
