
// variant using sqlx; async

use actix_web::{get, post, web, App, HttpServer, Responder};
use std::io;
use structopt::StructOpt;
//use sqlx::prelude::*;
use sqlx::sqlite::SqlitePool;
use sqlx::types::chrono::Utc;

async fn maybe_write_entry(pool: web::Data<SqlitePool>, event_id: i32, client_id: &str, severity: &str, body: &str) -> io::Result<()> {

    let msg =  body.replace("\\", "\\\\").replace("\n", "\\n").replace("\"", "\"\"");

    sqlx::query!(
        "
        INSERT 
            INTO events
                   (whenithappened, event, client, severity, msg)
            VALUES (?,    ?,      ?,     ?,        ?)
        ",
        Utc::now().timestamp(),
        event_id,
        client_id,
        severity,
        msg
    ).execute(pool.get_ref()).await.expect("insert failed, damn!");

    /*
    write!(
        file, 
        "{}", 
        Utc::now().to_rfc3339_opts(SecondsFormat::AutoSi, true), 
    )?;
    write!(
        file, 
        " {} ", 
        client_id, 
    )?;

    write!(
        file, 
        "\"{}\"\n", 
        body.replace("\\", "\\\\").replace("\n", "\\n").replace("\"", "\"\"")
    )?;
    */
    
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
async fn log_item(pool: web::Data<SqlitePool>, web::Path((event_id, client_id, severity)): web::Path<(i32, String, String)>, body: web::Bytes) -> impl Responder {


    let payload = std::str::from_utf8(&body).unwrap();
    let result = maybe_write_entry(pool, event_id, &client_id, &severity, payload).await;
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

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CommandLineOptions {
    #[structopt(short, long)]
    debug: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let opt = CommandLineOptions::from_args();
    println!("{:#?}", opt);

    let pool = match SqlitePool::new("sqlite::file:logdb.sqlite").await {
        Ok(pool) => pool,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, err))
    };

    /*
    sqlx::query!(
        "
        create table if not exists events (whenithappened timestamp, event integer, client text, severity text, msg text);
        ").execute(&pool).await;
    */

    let result = HttpServer::new(
        move || App::new().data(pool.clone())
                     .service(index)
                     .service(log_item)
                     .service(index_id_name)
    )
        .bind("127.0.0.1:9457")?
        .run()
        .await;

    result
}
