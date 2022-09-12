// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;
// mod rocket_script;
// mod mongo_script;

// fn main() {
//     // rocket::ignite().mount("/", routes![index, world]).launch();
//     mongo_script::query_db();
//     // print!();
//     // rocket_script::ignition();
// }

// use std::collections::HashMap;

pub(crate) mod mongoDB {
    use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
    // This trait is required to use `try_next()` on the cursor
    use futures::stream::{Collect, TryStreamExt};
    use serde::{Deserialize, Serialize};
    // use mongodb::{bson::doc, options::FindOptions};

    #[derive(Debug, Serialize, Deserialize)]

    struct Student {
        id: i32,
        notas: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Class {
        materia: String,
        // creditos: i32,
        // horario: String,
        // salon: String,
        estudiantes: Vec<Student>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Teacher {
        id: i32,
        name: String,
        materias: Vec<Class>,
    }
    #[tokio::main]
    pub async fn query_db() {
        // Parse your connection string into an options struct
        let mut client_options =
        ClientOptions::parse("mongodb+srv://ArquiSoft:ArquiSoft@cluster0.3gasdka.mongodb.net/?retryWrites=true&w=majority")
            .await?;

        // Manually set an option
        client_options.app_name = Some("Rust Demo".to_string());

        // Get a handle to the cluster
        let client = Client::with_options(client_options)?;

        // Ping the server to see if you can connect to the cluster
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        println!("Connected successfully.");

        // List the names of the databases in that cluster
        for db_name in client.list_database_names(None, None).await? {
            println!("- {}", db_name);
        }

        let db = client.database("ArquiSoft");

        for name in db.list_collection_names(None).await? {
            println!("Collection: {}", name);
        }

        let teachers: Collection<Teacher> = client.database("ArquiSoft").collection("TeachersCol");
        // teachers.find(None, None).await?;

        // Get every document on teachers collection

        // let mut cursor = teachers.find(None, None).await?;

        let reg = teachers.find_one(doc! {"id": 100123456}, None).await?;

        println!("reg: {:?}", reg);

        Ok(());
    }
}
// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//     // Parse your connection string into an options struct
//     let mut client_options =
//         ClientOptions::parse("mongodb+srv://ArquiSoft:ArquiSoft@cluster0.3gasdka.mongodb.net/?retryWrites=true&w=majority")
//             .await?;

//     // Manually set an option
//     client_options.app_name = Some("Rust Demo".to_string());

//     // Get a handle to the cluster
//     let client = Client::with_options(client_options)?;

//     // Ping the server to see if you can connect to the cluster
//     client
//         .database("admin")
//         .run_command(doc! {"ping": 1}, None)
//         .await?;
//     println!("Connected successfully.");

//     // List the names of the databases in that cluster
//     for db_name in client.list_database_names(None, None).await? {
//         println!("- {}", db_name);
//     }

//     let db = client.database("ArquiSoft");

//     for name in db.list_collection_names(None).await? {
//         println!("Collection: {}", name);
//     }

//     let teachers: Collection<Teacher> = client.database("ArquiSoft").collection("TeachersCol");
//     // teachers.find(None, None).await?;

//     // Get every document on teachers collection

//     // let mut cursor = teachers.find(None, None).await?;

//     let reg = teachers.find_one(doc! {"id": 100123456}, None).await?;

//     println!("reg: {:?}", reg);

//     Ok(())
// }
