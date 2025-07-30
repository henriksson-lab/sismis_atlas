pub mod genbank;
pub mod sqlite;

use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufReader};

use actix_files::Files;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

use crate::genbank::convert_genbank;
use crate::sqlite::query_cluster;
use crate::genbank::query_genbank;

use rusqlite::{Connection, OpenFlags};

////////////////////////////////////////////////////////////
/// Backend state
pub struct ServerData {
    conn: Connection,
    path_zip: PathBuf,

}


#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    data: PathBuf,
    bind: String,
}




use actix_web::{get, Responder};


////////////////////////////////////////////////////////////
/// REST entry point
#[get("/get_cluster")]
async fn get_cluster(server_data: Data<Mutex<ServerData>>) -> impl Responder {
    //info!("metadata: {:?}", &server_data.db_metadata);
    let ret = query_cluster(&server_data, &String::new()).
    expect("failed to access sql").expect("failed to get cluster");
    serde_json::to_string(&ret)
}


////////////////////////////////////////////////////////////
/// REST entry point
#[get("/get_genbank")]
async fn get_genbank(server_data: Data<Mutex<ServerData>>) -> impl Responder {
    //info!("metadata: {:?}", &server_data.db_metadata);

    let e ="GUT_GENOME277127-scaffold_21_cluster_1".to_string();

    let ret = query_genbank(&server_data, &e).
            expect("failed to access genbank");
    ret
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


    // Open SQL database
    let path_store = std::path::Path::new(&config_file.data);
    let path_sql = path_store.join(std::path::Path::new("clusters.sqlite"));
    let conn = Connection::open_with_flags(&path_sql, OpenFlags::SQLITE_OPEN_READ_ONLY).expect("Could not open SQL database");


    // Convert data files if needed
    let gbk_in = config_file.data.join("genbank.gbk");
    let gbk_zip = config_file.data.join("genbank.zip");
    convert_genbank(&gbk_in, &gbk_zip).await.expect("Could not parse genbank");








    let data = Data::new(Mutex::new(
        ServerData {
            conn,
            path_zip: gbk_zip,
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())  //for debugging
            .service(get_cluster)
            .service(get_genbank)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::NotFound()),  //header("Location", "/").finish()
            )
    })
    .bind(config_file.bind)? /////////////// for dev, "127.0.0.1:8080"  ; 127.0.0.1:5199 for beagle deployment
    .run()
    .await
}




