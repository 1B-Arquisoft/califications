use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Student {
    id: i32,
    notas: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class{
    materia: String,
    // creditos: i32,
    // horario: String,
    // salon: String,
    estudiantes: Vec<Student>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) materias: Vec<Class>
}
