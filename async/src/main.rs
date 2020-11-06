
// variant using async_std

use actix_web::{get, post, web, App, HttpServer, Responder};

use async_std::fs::OpenOptions;
use async_std::io;
use async_std::io::prelude::WriteExt;

use chrono::prelude::*;

async fn maybe_write_entry(event_id: u32, client_id: &str, severity: &str, body: &str) -> io::Result<()> {
    
    let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("thelog.log").await?;
    
    write!(
        file, 
        "{}", 
        Utc::now().to_rfc3339_opts(SecondsFormat::AutoSi, true), 
    ).await?;
    write!(
        file, 
        " {} ", 
        client_id, 
    ).await?;

    write!(
        file, 
        "\"{}\"\n", 
        body.replace("\\", "\\\\").replace("\n", "\\n").replace("\"", "\"\"")
    ).await?;
    /*
    write!(
        file, 
        "{}: {} - [{}:{}] \"{}\"\n", 
        Utc::now().to_rfc3339_opts(SecondsFormat::AutoSi, true), 
        client_id, 
        event_id, 
        severity,
        body.replace("\\", "\\\\").replace("\n", "\\n").replace("\"", "\"\"")
    ).await?;
    */

    //write!(file, "\"{}\"\n", body.replace("\\", "\\\\").replace("\n", "\\n").replace("\"", "\"\"")).await?;
    //write!(file, "\"{}\"\n", body).await?;
    
    println!("{}: {} - [{}:{}] {}", Utc::now(), client_id, event_id, severity, body);
    Ok(())
}

#[get("/{id}/{name}/index.html")]
async fn index_id_name(path: web::Path<(u32, String)>) -> impl Responder {

    let web::Path((id, name)) = path;

    format!("Hello {}! id:{}", name, id)
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello Stranger!")
}

#[post("/log/{event_id}/{client_id}/{severity}")]
async fn log_item(web::Path((event_id, client_id, severity)): web::Path<(u32, String, String)>, body: web::Bytes) -> impl Responder {


    let payload = std::str::from_utf8(&body).unwrap();
    let result = maybe_write_entry(event_id, &client_id, &severity, payload).await;
    match result {
        Ok(_) => {
            // whatevs
        }
        Err(_e) => {
            // not sure what to do
        }
    }
    

    format!("Thanks {} for your contribution of {}! You say it is {}? This was your payload: {}", client_id, event_id, severity, payload)


}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(index)
                     .service(log_item)
                     .service(index_id_name)
    )
        .bind("127.0.0.1:9457")?
        .run()
        .await
}
