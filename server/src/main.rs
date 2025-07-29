use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::io::BufReader;

use actix_files::Files;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde::Serialize;

////////////////////////////////////////////////////////////
/// Backend state
pub struct ServerData {
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    bind: String,
}






////////////////////////////////////////////////////////////
/// Backend entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Read the config file
    let f_meta = File::open("config.json").expect("Could not open config.json");
    let config_reader = BufReader::new(f_meta);
    let config_file:ConfigFile = serde_json::from_reader(config_reader).expect("Could not open config file");


    let data = Data::new(Mutex::new(
        ServerData {
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())  //for debugging
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::NotFound()),  //header("Location", "/").finish()
            )
    })
    .bind(config_file.bind)? /////////////// for dev, "127.0.0.1:8080"  ; 127.0.0.1:5199 for beagle deployment
    .run()
    .await
}




