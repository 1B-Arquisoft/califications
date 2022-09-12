// #![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!!!"
}



#[get("/califications/<teacher_id>/<class_name>/<student_id>/<calification_num>")]
fn get_calification(teacher_id: String, class_name: String, student_id: String, calification_num: i32) -> String {
    format!("{} {} {} {}", teacher_id, class_name, student_id, calification_num)
}

#[get("/califications/<teacher_id>/<class_name>/<student_id>")]
fn get_califications(teacher_id: String, class_name: String, student_id: String) -> String {
    format!("{} {} {}", teacher_id, class_name, student_id)
}

#[get("/<teacher_id>")]
fn get_teacher(teacher_id: String) -> String {
    format!("{} ", teacher_id)
}

pub fn ignition() {
    rocket::ignite().mount("/", routes![index, world, get_calification, get_califications]).launch();
}