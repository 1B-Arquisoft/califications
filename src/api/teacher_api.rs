use std::{vec, fmt::format};

use crate::{
    models::teacher_model::{Class, Student, Teacher},
    repository::mongodb_repo::MongoRepo,
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, futures::future::ok};

#[get("/")]
pub fn hello(){
    println!("Hello, world!");
    format!("Hello, world!");
}

#[post("/teacher", data = "<new_teacher>")]
pub fn create_teacher(
    db: &State<MongoRepo>,
    new_teacher: Json<Teacher>,
) -> Result<Json<InsertOneResult>, Status> {
    let student = Student {
        id: new_teacher.id,
        notas: {
            let mut notas = Vec::new();
            notas.push(1);
            notas.push(2);
            notas.push(3);
            notas
        },
    };

    let materia = Class {
        materia: new_teacher.materias[0].materia.to_owned(),
        estudiantes: vec![student],
    };
    let data = Teacher {
        id: new_teacher.id,
        name: new_teacher.name.to_owned(),
        // materias:
        materias: vec![materia],
        // materias: new_teacher.materias
    };
    let teacher_detail = db.create_teacher(data);
    match teacher_detail {
        Ok(teacher) => Ok(Json(teacher)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/teacher/<path>")]
pub fn get_teacher(db: &State<MongoRepo>, path: String) -> Result<Json<Teacher>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let teacher_detail = db.get_teacher(&id);
    match teacher_detail {
        Ok(teacher) => Ok(Json(teacher)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/teacher_by_id/<path>")]
pub fn get_teacher_by_id(db: &State<MongoRepo>, path: i32) -> Result<Json<Teacher>, Status> {
    let id = path;
    // if id.is_empty() {
    //     return Err(Status::BadRequest);
    // };
    let teacher_detail = db.get_teacher_by_id(&id);
    match teacher_detail {
        Ok(teacher) => Ok(Json(teacher)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/teacher/<path>", data = "<new_teacher>")]
pub fn update_teacher(
    db: &State<MongoRepo>,
    path: String,
    new_teacher: Json<Teacher>,
) -> Result<Json<Teacher>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let student = Student {
        id: new_teacher.id,
        notas: {
            let mut notas = Vec::new();
            notas.push(0);
            notas.push(0);
            notas.push(0);
            notas
        },
    };

    let materia = Class {
        materia: new_teacher.materias[0].materia.to_owned(),
        estudiantes: vec![student],
    };

    let data = Teacher {
        id: new_teacher.id,
        name: new_teacher.name.to_owned(),
        materias: vec![materia],
    };
    let update_result = db.update_teacher(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_teacher_info = db.get_teacher(&id);
                return match updated_teacher_info {
                    Ok(teacher) => Ok(Json(teacher)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/teacher/<path>")]
pub fn delete_teacher(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_teacher(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Teacher successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/teacher/<teacher_id>/class/<class_name>/student/<student_id>")]
pub fn get_student(
    db: &State<MongoRepo>,
    teacher_id: i32,
    class_name: String,
    student_id: i32,
) -> Result<Json<Student>, Status> {
    let result = db.get_student(&student_id, &teacher_id, &class_name);

    match result {
        Ok(student) => Ok(Json(student)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// #[put ]
