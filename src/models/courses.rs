use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
// use serde::Deserialize;
// use rocket_contrib::json::Json;
#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: String,
    pub grade: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub _id: Option<ObjectId>,
    pub course_name: String,
    pub teacher_name: String,
    pub teacher_id: String,
    pub group: String,
    // pub studentsInCourse: Vec<Student>,
    pub students_in_course: Vec<Student>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseNameAndGroupFilter {
    pub course_name: String,
    pub group: String,
}
