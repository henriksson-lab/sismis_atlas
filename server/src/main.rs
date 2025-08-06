pub mod genbank_query_rszip;
pub mod genbank_query_unzip;
pub mod sqlite;
pub mod umap;
pub mod genbank_convert_zip;

use std::path::{PathBuf};
use std::io::{Cursor};

use actix_files::Files;
use actix_web::web::Json;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer, post, get};
use actix_web::{Responder}; 
use my_web_app::{ClusterRequest, ConfigFile, UmapData, UmapMetadata};
use std::sync::Mutex;

use crate::genbank_convert_zip::convert_genbank_rszip;
use crate::sqlite::get_sequence_sql;
use crate::genbank_query_unzip::query_genbank;
use crate::umap::load_umap_data;

use rusqlite::{Connection, OpenFlags};

////////////////////////////////////////////////////////////
/// Backend state
pub struct ServerData {
    conn: Connection,
    path_zip: PathBuf,
    umeta: UmapMetadata,
    umap_points: UmapData,
}








////////////////////////////////////////////////////////////
/// REST entry point
#[get("/get_coloring")]
async fn get_coloring(server_data: Data<Mutex<ServerData>>) -> impl Responder {
    let server_data =server_data.lock().unwrap();
    serde_cbor::to_vec(&server_data.umeta).expect("Failed to serialize")
}


////////////////////////////////////////////////////////////
/// REST entry point
#[post("/get_sequence")]
async fn get_sequence(server_data: Data<Mutex<ServerData>>, req_body: web::Json<ClusterRequest>) -> impl Responder {

    println!("{:?}",req_body);
    let Json(req) = req_body;

    //info!("metadata: {:?}", &server_data.db_metadata);
    let ret = get_sequence_sql(&server_data, &req).
        expect("failed to access sql");
        //expect("failed to get cluster");
    serde_json::to_string(&ret)
}


////////////////////////////////////////////////////////////
/// REST entry point
#[post("/get_genbank")]  //would be simpler if we used get
async fn get_genbank(server_data: Data<Mutex<ServerData>>, req_body: web::Json<ClusterRequest>) -> impl Responder {
    //info!("metadata: {:?}", &server_data.db_metadata);

    println!("{:?}",req_body);
    let Json(req) = req_body;
    let ret = query_genbank(&server_data, &req)
        .await
        .expect("failed to access genbank"); 

    serde_json::to_string(&ret)
}





////////////////////////////////////////////////////////////
/// REST entry point
#[get("/get_umap")]  //would be simpler if we used get
async fn get_umap(server_data: Data<Mutex<ServerData>>) -> impl Responder {
    let server_data =server_data.lock().unwrap();
//    serde_json::to_string(&server_data.umap_points)
    serde_cbor::to_vec(&server_data.umap_points).expect("Failed to serialize")

}















////////////////////////////////////////////////////////////
/// Backend entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "info");
    //std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Read the config file   .. or revert to previous?
    let config_file = serde_json::from_reader(Cursor::new(include_bytes!("../../config.json"))).expect("Could not open config.json");

    //UMAP meta
    let (umap_points,umeta) = load_umap_data(&config_file);

    // Open SQL database
    let path_store = std::path::Path::new(&config_file.data);
    let path_sql = path_store.join(std::path::Path::new("clusters.sqlite"));
    let conn = Connection::open_with_flags(&path_sql, OpenFlags::SQLITE_OPEN_READ_ONLY).expect("Could not open SQL database");


    // Convert data files if needed
    let gbk_in = config_file.data.join("genbank.gbk");
    let gbk_zip = config_file.data.join("genbank.zip");
    if !gbk_zip.exists() {
        println!("Converting genbank to zip");
        convert_genbank_rszip(&gbk_in, &gbk_zip).await.expect("Could not parse genbank");
    }



    //Gather server data / state
    let data = Data::new(Mutex::new(
        ServerData {
            conn,
            path_zip: gbk_zip,
            umeta: umeta,
            umap_points: umap_points,
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())  //for debugging
            .service(get_coloring)
            .service(get_sequence)
            .service(get_genbank)
            .service(get_umap)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::NotFound()),  //header("Location", "/").finish()
            )
    })
    .bind((config_file.bind, config_file.port))? /////////////// for dev, "127.0.0.1:8080"  ; 127.0.0.1:5199 for beagle deployment   ; 0.0.0.0 should be fine!
    .run()
    .await
}




