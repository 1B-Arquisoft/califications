// use std::env;s
// // extern crate dotenv;
// use dotenv::dotenv;
use mongodb::bson::Bson;

use crate::models::teacher_model::Class;
use crate::models::teacher_model::Student;
use crate::models::teacher_model::Teacher;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId}, //modify here
    options::{ClientOptions, ResolverConfig},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Teacher>,
    // col: Collection<Class>,
    // col: Collection<Student>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        // Load the MongoDB connection string from an environment variable:
        let client_uri =  "mongodb+srv://ArquiSoft:ArquiSoft@cluster0.3gasdka.mongodb.net/?retryWrites=true&w=majority";
        // env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:

        let client = Client::with_uri_str(client_uri).unwrap();
        let db = client.database("ArquiSoft");
        let col: Collection<Teacher> = db.collection("TeachersCol");
        MongoRepo { col }
    }

    pub fn create_teacher(&self, new_teacher: Teacher) -> Result<InsertOneResult, Error> {
        let new_doc = Teacher {
            id: new_teacher.id,
            name: new_teacher.name,
            materias: new_teacher.materias,
        };
        let teacher = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating Teacher");
        Ok(teacher)
    }
    pub fn get_teacher(&self, id: &String) -> Result<Teacher, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        // println!("{:?}", self.col.find_one(filter, None));
        let teacher_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting teacher's detail");
        Ok(teacher_detail.unwrap())
    }
    pub fn update_teacher(&self, id: &String, new_teacher: Teacher) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_teacher.id,
                    "name": new_teacher.name

                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating teacher");
        Ok(updated_doc)
    }

    pub fn delete_teacher(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let teacher_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting teacher");
        Ok(teacher_detail)
    }

    pub fn get_teacher_by_id(&self, id: &i32) -> Result<Teacher, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"id": id};
        let teacher_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting teacher's detail");
        Ok(teacher_detail.unwrap())
    }

    pub fn get_student(
        &self,
        id: &i32,
        teacher_id: &i32,
        class_name: &String,
    ) -> Result<Student, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {
            "id": teacher_id,
            // "materias": [
            //     {
            //         "materia": class_name,
            //         "estudiantes": [
            //             {
            //                 "id": id
            //             }
            //         ]
            //     }
            // ]
        };

        let student_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting student's detail");

        let tch = student_detail.unwrap();

        let student: Student = Student {
            id: 0,
            notas: Vec::new(),
        };
        for m in tch.materias {
            if m.materia == *class_name {
                for s in m.estudiantes {
                    if s.id == *id {
                        return Ok(s);
                    }
                }
            }
        }

        return Ok(student);
    }

    fn update_grade(
        &self,
        id: &i32,
        teacher_id: &i32,
        class_name: &String,
        idx: usize,
        grade : i32
    ) -> Result<UpdateResult, Error> {

        let filter = doc! {
            "id": teacher_id,
            "$and": [
                {"materias.materia": class_name},
                {"$and": [
                    {"materias.estudiantes.id": id},
                    
                ]}
            ]
        };
        
        let student_detail = self
        .col
        .find_one(filter, None)
        .ok()
            .expect("Error getting student's detail");
            
            let mut new_student = student_detail.unwrap();
            
            
            // let mut tch = student_detail.unwrap();
            
            // let mut notas = Vec::new();
            
        let mut st = Student {
            id: *id,
            notas: Vec::new(),
        };

        // let mut student = tch.materias[0].estudiantes[0];
        // let mut changed_m = false;
        // for mut m in new_student.materias {
        //     if m.materia == *class_name {
        //         for mut s in m.estudiantes {
        //             if s.id == *id {
        //                 if idx < s.notas.len(){
        //                     s.notas[idx] = grade;
        //                 }
        //                 else {
        //                     s.notas.push(grade);
        //                 }
        //                 notas = s.notas;
        //                 st.notas = notas;
        //                 changed_m  = true;
        //             }
        //         }
        //         if changed_m{
        //             m.materia 
        //         }
        //     }
        // }

        // // let new_subject = tch.materias;
        
        let new_doc = doc! {
            "$set":
            {
                "id": teacher_id,
                // "materias": <Bson as From<Vec<Class>>>::from(new_subject),
                
            },
        };
        
        let a_filter = doc! {
            "id": teacher_id,
            "$and": [
                {"materias.materia": class_name},
                {"$and": [
                    {"materias.estudiantes.id": id},
                    
                ]}
            ]
        };
        
        let updated_doc = self
            .col
            .update_one(a_filter, new_doc, None)
            .ok()
            .expect("Error updating teacher");
        Ok(updated_doc)
    }
}
