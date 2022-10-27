// use chrono::{TimeZone, Utc};
// use mongodb::bson::doc;
// use mongodb::bson::oid::ObjectId;
// use rocket::http::Status;
// models::courses::Student
use crate::{models::courses::Course, repository::mongodb::MongoRepo};//, models::courses::CourseNameAndGroupFilter};
use mongodb::results::InsertOneResult;
// use mongodb::bson::doc;
// use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[post("/create_course", data = "<new_course>")]
pub fn create_course(
    db: &State<MongoRepo>,
    new_course: Json<Course>,
) -> Result<Json<InsertOneResult>, Status> {
    let inserted_course = db.create_course_json(new_course.into_inner());
    match inserted_course {
        Ok(inserted_course) => Ok(Json(inserted_course)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/insert_new_student/<group>/<student_id>")]
pub fn insert_new_student(
    db: &State<MongoRepo>,
    group: String,
    student_id: String,
) -> Result<String, Status> {
    let course_response = db.insert_new_student(group, student_id);
    match course_response {
        Ok(_course_response) => Ok("Student was inserted".to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[put("/change_grade_of_student/<group>/<student_id>/<new_grade>")]
pub fn change_grade_of_student(
    db: &State<MongoRepo>,
    group: String,
    student_id: String,
    new_grade: String,
) -> Result<String, Status> {
    let course_response = db.change_grade_of_student(group, student_id, new_grade);
    match course_response {
        Ok(_course_response) => Ok("Grade was changed".to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/delete_student_on_group/<group>/<student_id>")]
pub fn delete_student_on_group(
    db: &State<MongoRepo>,
    group: String,
    student_id: String,
) -> Result<String, Status> {
    let course_response = db.delete_student_on_group(group, student_id);
    match course_response {
        Ok(_course_response) => Ok("Student was deleted".to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}