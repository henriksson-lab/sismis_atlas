pub mod genbank;
pub mod sqlite;

use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufReader};

use actix_files::Files;
use actix_web::web::Json;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer, post, get};
use my_web_app::{ClusterRequest, Genbank, UmapData};
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




use actix_web::{Responder};  //get, 


////////////////////////////////////////////////////////////
/// REST entry point
#[post("/get_cluster")]
async fn get_cluster(server_data: Data<Mutex<ServerData>>, req_body: web::Json<ClusterRequest>) -> impl Responder {

    println!("{:?}",req_body);
    let Json(req) = req_body;

    //info!("metadata: {:?}", &server_data.db_metadata);
    let ret = query_cluster(&server_data, &req.cluster_id).
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

//    let e ="GUT_GENOME277127-scaffold_21_cluster_1".to_string();

    let ret = query_genbank(&server_data, &req.cluster_id).
            expect("failed to access genbank");
//    ret   ///// would be more efficient!!

    let ret = Genbank {
        data: String::from_utf8_lossy(ret.as_slice()).to_string()
    };

    let ret = vec![ret];

    serde_json::to_string(&ret)
}





////////////////////////////////////////////////////////////
/// REST entry point
#[get("/get_umap")]  //would be simpler if we used get
async fn get_umap(_server_data: Data<Mutex<ServerData>>) -> impl Responder {

    let ret = load_umap_data();

    serde_json::to_string(&ret)
}






pub fn load_umap_data() -> UmapData {



    // This list of vertices
    let num_points = 10;
    let mut vertices: Vec<f32> = vec![];
    let mut ids: Vec<String> = Vec::new();

    use rand::Rng;
    let mut rng = rand::rng();
    for i in 0..(num_points*2) {
        let v = rng.random_range(0.0..1023.0);//.round();
//        let v = rng.random_range(-1.0..1.0);
        vertices.push(v);
        ids.push(format!("c{}",i));
    }
 
    UmapData {
        num_point: vertices.len()/2,
        data: vertices,
        ids: ids,
    }



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
            .service(get_umap)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::NotFound()),  //header("Location", "/").finish()
            )
    })
    .bind(config_file.bind)? /////////////// for dev, "127.0.0.1:8080"  ; 127.0.0.1:5199 for beagle deployment
    .run()
    .await
}




