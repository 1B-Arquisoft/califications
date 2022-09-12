// use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: i32,
    pub notas: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class{
    pub materia: String,
    // creditos: i32,
    // horario: String,
    // salon: String,
    pub estudiantes: Vec<Student> // -----------------------
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub materias: Vec<Class> // -----------------------
    // pub id: i32,
    // pub name: String,
    // pub materias: Vec<Class>
}
