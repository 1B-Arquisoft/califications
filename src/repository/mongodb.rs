// use std::env;s
// extern crate dotenv;
// use dotenv::dotenv;
use std::env;
extern crate dotenv;
use dotenv::dotenv;
// use mongodb::bson;
// use mongodb::bson::bson;

use crate::models::courses::Course;
// use crate::models::courses::Student;
// use crate::models::courses::CourseNameAndGroupFilter;
// use mongodb::bson::Bson;

use mongodb::{
    bson::{doc, extjson::de::Error}, //modify hereo id::ObjectId
    // options::{ClientOptions, ResolverConfig},
    results::{InsertOneResult, UpdateResult}, //DeleteResult,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    courses_collection: Collection<Course>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let client_uri =
            env::var("DATABASE_URI").expect("You must set the DATABASE_URI environment var!");
        let client = Client::with_uri_str(client_uri).unwrap();
        let db = client.database("ArquiSoft");
        let actual_courses_collection: Collection<Course> = db.collection("actualCourses");
        MongoRepo {
            courses_collection: actual_courses_collection,
        }
    }

    pub fn create_course_json(&self, course: Course) -> Result<InsertOneResult, Error> {
        let course_creation_document = Course {
            _id: None,
            course_name: course.course_name,
            teacher_name: course.teacher_name,
            teacher_id: course.teacher_id,
            group: course.group,
            students_in_course: course.students_in_course,
        };

        let inserted_course = self
            .courses_collection
            .insert_one(course_creation_document, None)
            .ok()
            .expect("Error inserting course");

        Ok(inserted_course)
    }

    pub fn insert_new_student(
        &self,
        group: String,
        student_id: String,
    ) -> Result<UpdateResult, Error> {
        let update_result = self
            .courses_collection
            .update_one(
                doc! {
                    "group": group
                },
                doc! {
                    "$push": {
                        "students_in_course": {
                            "student_id": student_id,
                            "grade": "0",
                        }
                    }
                },
                None,
            )
            .ok()
            .expect("Error updating course");

        Ok(update_result)
    }

    pub fn change_grade_of_student(
        &self,
        group: String,
        student_id: String,
        new_grade: String,
    ) -> Result<UpdateResult, Error> {
        let update_result = self
            .courses_collection
            .update_one(
                doc! {
                    "group": group,
                    "students_in_course.student_id": student_id
                },
                doc! {
                    "$set": {
                        "students_in_course.$.grade": new_grade
                    }
                },
                None,
            )
            .ok()
            .expect("Error updating course");

        Ok(update_result)
    }

    pub fn delete_student_on_group(
        &self,
        group: String,
        student_id: String,
    ) -> Result<UpdateResult, Error> {
        let update_result = self
            .courses_collection
            .update_one(
                doc! {
                    "group": group
                },
                doc! {
                    "$pull": {
                        "students_in_course": {
                            "student_id": student_id
                        }
                    }
                },
                None,
            )
            .ok()
            .expect("Error updating course");

        Ok(update_result)
    }
}
